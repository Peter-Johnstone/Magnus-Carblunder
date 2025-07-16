use std::str::SplitWhitespace;
use crate::attacks::sliding::{diagonal_attacks, orthogonal_attacks};
use crate::castling_rights::CastlingRights;
use crate::color::Color;
use crate::attacks::tables::{KING_MOVES, KNIGHT_MOVES, PAWN_ATTACKS, RAYS};
use crate::mov::{Move, MoveKind};
use crate::direction::Dir;
use crate::piece::{decode_piece, encode_piece, EncodedPiece, Piece, EMPTY_PIECE};
pub(crate) use crate::state_info::StateInfo;

const MAX_PIECES: usize = 10;

#[derive(Copy, Clone, Debug)]
pub struct Undo {
    captured_piece: Option<(EncodedPiece, u8)>, // (encoded piece, square)
    castling_rights: CastlingRights,
    en_passant: u8, // or Option<u8> if you refactor it
    half_move: u8,
    turn: Color,
}


#[derive(Debug)]
pub struct Position {
    board: [EncodedPiece; 64],
    piece_list: [[[u8; MAX_PIECES]; 2]; 6],  // [piece][color][index]
    piece_count: [[usize; 2]; 6],
    reverse_piece_index: [[[Option<usize>; 64]; 2]; 6], // [piece][color][square]
    piece_bitboards: [u64; 6], // [p, n, b, r, q]
    color_bitboards: [u64; 2], // [w, b]
    undo_stack: Vec<Undo>,
    turn: Color,
    castling_rights: CastlingRights,
    en_passant: u8,
    half_move: u8,
}
impl Default for Position {
    fn default() -> Self {
        Self {
            board: [0; 64],
            piece_list: Default::default(),
            piece_count: Default::default(),
            reverse_piece_index: [[[None; 64]; 2]; 6],
            piece_bitboards: Default::default(),
            color_bitboards: Default::default(),
            undo_stack: Vec::new(),
            turn: Default::default(),
            castling_rights: Default::default(),
            en_passant: 0,
            half_move: 0,
        }
    }
}

impl Position {
    pub fn start() -> Position {
        Self::load_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0")
    }

    pub fn load_position_from_fen(fen: &str) -> Position {
        let mut position = Position::default();
        let mut iter: SplitWhitespace = fen.split_whitespace();

        let piece_positions: &str = iter.next().expect("FEN is missing piece positions");
        position.load_board_from_fen(piece_positions);

        let turn_str: &str = iter.next().expect("FEN is missing current turn");
        position.turn = Color::from_str(turn_str);

        let castling_rights_str: &str = iter.next().expect("FEN is missing castling rights");
        position.castling_rights = CastlingRights::from_str(castling_rights_str);

        let en_passant_str: &str = iter.next().expect("FEN is missing en passant");
        position.en_passant = if en_passant_str != "-" {
            let file = en_passant_str.chars().nth(0).unwrap();
            let rank = en_passant_str.chars().nth(1).unwrap();

            let col = file as u8 - b'a';
            let row = rank.to_digit(10).unwrap() as u8;

            row*8 + col
        } else {
            64
        };

        let half_move_str: &str = iter.next().expect("FEN is missing half move count");
        position.half_move = half_move_str.parse::<u8>().expect("Invalid half move count");

        position
    }

    fn load_board_from_fen(&mut self, fen_board : &str) {
        for (row_index, row) in fen_board.split('/').enumerate() {
            let mut col_index = 0;
            for ch in row.chars() {
                if ch.is_ascii_digit() {
                    col_index += ch.to_digit(10).unwrap() as usize;
                } else {
                    let square: usize = square_index(row_index, col_index);
                    update_bitboards_pieces(self, ch, square as u8);
                    self.board[square] = piece_from_char(ch);
                    col_index += 1;
                }
            }
        }
    }

    pub fn do_move(&mut self, mov: Move) {
        let (from, to) = (mov.from() as usize, mov.to() as usize);
        let encoded = self.board[from];
        let (piece, color) = decode_piece(encoded).expect("Tried to move from an empty square");

        // Save undo info
        let mut captured_piece = None;
        if mov.is_capture() || mov.is_en_passant() {
            let capture_square = if mov.is_capture() {
                to
            } else {
                self.en_passant_capture_pawn() as usize
            };
            let captured_encoded = self.board[capture_square];
            captured_piece = Some((captured_encoded, capture_square as u8));

            // remove the captured piece from the board representations
            self.handle_capture(capture_square);
        }

        self.undo_stack.push(Undo {
            captured_piece,
            castling_rights: self.castling_rights,
            en_passant: self.en_passant,
            half_move: self.half_move,
            turn: self.turn,
        });

        // fuck up castling rights without being a castle itself
        if piece == Piece::King {self.castling_rights.remove_castling_rights(self.turn)}
        self.handle_rook_castling_change(from, to);

        if mov.is_castling() {self.handle_castle(from, to)}

        // Move the piece
        self.move_piece(piece, color, from, to);

        // Update en passant square (or clear)
        self.en_passant = mov.new_en_passant_square();

        // Rule-50 clock reset on pawn move or capture
        self.update_rule50(piece, mov.is_capture());

        // Switch sides
        self.turn = !self.turn;
    }

    fn handle_rook_castling_change(&mut self, from: usize, to: usize) {
        if to == 0 || to == 7 || to == 56 || to == 63 {
            self.castling_rights.remove_castling_right(to);
        }
        if from == 0 || from == 7 || from == 56 || from == 63 {
            self.castling_rights.remove_castling_right(from);
        }
    }

    fn move_piece(&mut self, piece: Piece, color: Color, from: usize, to: usize) {
        let p = piece as usize;
        let c = color as usize;

        // 1. Update board squares
        self.board[from] = EMPTY_PIECE;
        self.board[to] = encode_piece(piece, color);

        // 2. Update reverse index
        let count = self.reverse_piece_index[p][c][from]
            .expect("Piece missing from reverse piece index!");
        self.reverse_piece_index[p][c][from] = None;
        self.reverse_piece_index[p][c][to] = Some(count);

        // 3. Update piece list
        self.piece_list[p][c][count] = to as u8;

        // 4. Update bitboards
        let from_bb = 1u64 << from;
        let to_bb = 1u64 << to;

        self.piece_bitboards[p] = (self.piece_bitboards[p] & !from_bb) | to_bb;
        self.color_bitboards[c] = (self.color_bitboards[c] & !from_bb) | to_bb;
    }

    fn handle_capture(&mut self, to: usize) {
        let (piece, col) = decode_piece(self.board[to]).unwrap();
        let p = piece as usize;
        let c = col as usize;

        // 1. Update board square
        self.board[to] = EMPTY_PIECE;

        // 2. Update reverse piece index
        let count = self.reverse_piece_index[p][c][to]
            .expect("Piece missing from reverse piece index!");
        self.reverse_piece_index[p][c][to] = None;

        // 3. Update piece list
        self.piece_list[p][c][count] = to as u8;

        // 4. Update bitboards
        let captured_bb = 1u64 << to;
        self.piece_bitboards[p] &= !captured_bb;
        self.color_bitboards[c] &= !captured_bb;

    }

    fn handle_castle(&mut self, from: usize, to: usize) {
        let rook_to = (from + to) / 2;
        let rook_from =  if to == 2 || to == 58 {to - 2} else {to + 1}; // if queenside rook came from left, else came from right
        self.move_piece(Piece::Rook, self.turn, rook_from, rook_to);
    }

    fn update_rule50(&mut self, piece: Piece, is_capture: bool) {
        if piece == Piece::Pawn || is_capture {
            self.half_move = 0;
        } else {
            self.half_move += 1;
        }
    }


    pub fn compute_pins_checks(&self, us: Color) -> StateInfo {
        let king_sq = self.king_square(us) as usize;
        let occ     = self.occupied();

        // enemy sliders
        let rooks   = self.rooks()   & self.occupancy(!us);
        let bishops = self.bishops() & self.occupancy(!us);
        let queens  = self.queens()  & self.occupancy(!us);
        let ortho_sliders = rooks | queens;
        let diag_sliders  = bishops | queens;

        let mut checkers          = 0u64;
        let mut blockers_for_king = 0u64;
        let mut pinners           = 0u64;

        for dir in Dir::ALL {
            // pieces (friend+foe) in that direction
            let mut ray = RAYS[dir.idx()][king_sq] & occ;
            if ray == 0 { continue; }

            // isolate nearest piece on the ray
            let first_bb = if dir.is_positive() {        // N, NE, E, NW
                ray & ray.wrapping_neg()                 // LS1B
            } else {                                     // S, SW, W, SE
                1u64 << (63 - ray.leading_zeros())       // MS1B
            };
            let first_sq = first_bb.trailing_zeros() as u8;    // BSF works either way
            let sliders  = if dir.is_ortho() { ortho_sliders } else { diag_sliders };

            // ① direct check?
            if first_bb & sliders != 0 {
                checkers |= first_bb;
                continue;                                // nothing behind a checker matters
            }

            // ② maybe pinned?
            if first_bb & self.occupancy(us) != 0 {
                ray ^= first_bb;                         // drop the first blocker
                if ray != 0 {
                    let second_bb = if dir.is_positive() {
                        ray & ray.wrapping_neg()
                    } else {
                        1u64 << (63 - ray.leading_zeros())
                    };
                    if second_bb & sliders != 0 {
                        blockers_for_king |= first_bb;   // our piece is absolutely pinned
                        pinners           |= second_bb;  // pinning slider
                    }
                }
            }
        }

        // ③ pawn and knight checks
        let enemy_pawns   = self.pawns()   & self.occupancy(!us);
        let enemy_knights = self.knights() & self.occupancy(!us);

        checkers |= PAWN_ATTACKS[us as usize][king_sq] & enemy_pawns;
        checkers |= KNIGHT_MOVES[king_sq]              & enemy_knights;

        StateInfo { checkers, blockers_for_king, pinners }
    }



    pub fn square_under_attack(&self, sq: u8, by: Color) -> bool {
        let enemies = self.occupancy(by);

        let pawns = self.pawns() & enemies;

        // !by because our attacks (!by) is the fields of where enemy pawns can attack us
        if PAWN_ATTACKS[!by as usize][sq as usize] & pawns != 0 {
            return true;
        }
        let king = self.kings() & enemies;
        if KING_MOVES[sq as usize] & king != 0 {
            return true;
        }

        let knights = self.knights() & enemies;
        if KNIGHT_MOVES[sq as usize] & knights != 0 {
            return true;
        }

        let bishops = self.bishops() & enemies;
        let queens = self.queens() & enemies;
        let occ = self.occupied();

        if diagonal_attacks(sq as usize, occ) & (bishops | queens) != 0 {
            return true;
        }
        let rooks = self.rooks() & enemies;
        if orthogonal_attacks(sq as usize, occ) & (rooks | queens) != 0 {
            return true;
        }
        false
    }

    pub fn turn(&self) -> Color {
        self.turn
    }

    pub fn occupancy(&self, color: Color) -> u64 {
        match color {
            Color::White => self.white(),
            Color::Black => self.black()
        }
    }

    pub fn pawns(&self) -> u64 {
        self.piece_bitboards[Piece::Pawn as usize]
    }
    pub fn knights(&self) -> u64 {
        self.piece_bitboards[Piece::Knight as usize]
    }
    pub fn bishops(&self) -> u64 {
        self.piece_bitboards[Piece::Bishop as usize]
    }
    pub fn rooks(&self) -> u64 {
        self.piece_bitboards[Piece::Rook as usize]
    }
    pub fn queens(&self) -> u64 {
        self.piece_bitboards[Piece::Queen as usize]
    }
    pub fn kings(&self) -> u64 {
        self.piece_bitboards[Piece::King as usize]
    }
    pub fn white(&self) -> u64 {
        self.color_bitboards[Color::White as usize]
    }
    pub fn black(&self) -> u64 {
        self.color_bitboards[Color::Black as usize]
    }

    pub fn occupied(&self) -> u64 {
        self.black() | self.white()
    }

    pub fn en_passant_capture_pawn(&self) -> u8 {
        if self.en_passant == 64 {
            panic!("No en passant possible!");
        }
        match self.turn {
            Color::White => self.en_passant - 8, // white to move, the en passant square is PAST the pawn
            Color::Black => self.en_passant + 8 // black to move, vice versa
        }
    }

    pub fn en_passant(&self) -> u8 {
        self.en_passant
    }

    pub fn kingside(&self, color: Color) -> bool {
        self.castling_rights.kingside(color)
    }

    pub fn queenside(&self, color: Color) -> bool {
        self.castling_rights.queenside(color)
    }

    pub fn board_sq(&self, sq: u8) -> EncodedPiece {
        self.board[sq as usize]
    }

    pub fn piece_list(&self, piece: Piece, color: Color) -> ([u8; MAX_PIECES], usize)  {
        (self.piece_list[piece as usize][color as usize], self.piece_count[piece as usize][color as usize])
    }

    pub fn king_square(&self, color: Color) -> u8 {
        self.piece_list[Piece::King as usize][color as usize][0]
    }

    pub fn get_allies(&self, piece: Piece) -> u64 {
        self.piece_bitboards[piece as usize]&self.color_bitboards[self.turn as usize]
    }

    pub fn get_enemies(&self, piece: Piece) -> u64 {
        self.piece_bitboards[piece as usize]&self.color_bitboards[!self.turn as usize]
    }

    pub fn piece_bb(&self, piece: Piece, color: Color) -> u64 {
        let color_bb = self.occupancy(color);
        match piece {
            Piece::Pawn => self.pawns() & color_bb,
            Piece::Knight => self.knights() & color_bb,
            Piece::Bishop => self.bishops() & color_bb,
            Piece::Rook => self.rooks() & color_bb,
            Piece::Queen => self.queens() & color_bb,
            Piece::King => self.kings() & color_bb,
        }
    }

    pub fn print_board(&self) {
        println!();
        for rank in (0..8).rev() {
            print!("{} ", rank + 1);
            for file in 0..8 {
                let sq = rank * 8 + file;
                let piece = self.board[sq as usize];
                let symbol = if piece == EMPTY_PIECE {
                    '.'
                } else {
                    let (p, c) = decode_piece(piece).unwrap();
                    let ch = match p {
                        Piece::Pawn => 'p',
                        Piece::Knight => 'n',
                        Piece::Bishop => 'b',
                        Piece::Rook => 'r',
                        Piece::Queen => 'q',
                        Piece::King => 'k',
                    };
                    if c.is_white() { ch.to_ascii_uppercase() } else { ch }
                };
                print!("{} ", symbol);
            }
            println!();
        }
        println!("  a b c d e f g h");
        println!();
    }
}

fn square_index(row: usize, col: usize) -> usize {
    (7 - row)* 8 + col // (7 - row) because fens start from black pieces for some reason
}

fn piece_from_char(ch: char) -> EncodedPiece {
    let color = if ch.is_uppercase() { Color::White } else { Color::Black };

    let piece = match ch.to_ascii_lowercase() {
        'p' => Piece::Pawn,
        'n' => Piece::Knight,
        'b' => Piece::Bishop,
        'r' => Piece::Rook,
        'q' => Piece::Queen,
        'k' => Piece::King,
        _ => return EMPTY_PIECE, // invalid input
    };

    encode_piece(piece, color)
}


fn update_bitboards_pieces(position: &mut Position, ch: char, square: u8) {
    let encoded = piece_from_char(ch);

    if encoded != EMPTY_PIECE {
        if let Some((piece, color)) = decode_piece(encoded) {
            let bit = 1u64 << square;
            let p = piece as usize;
            let c = color as usize;

            let count = position.piece_count[p][c];
            position.piece_list[p][c][count] = square;
            position.piece_count[p][c] += 1;
            position.reverse_piece_index[p][c][square as usize] = Some(count);

            position.piece_bitboards[p] |= bit;
            position.color_bitboards[c] |= bit;
            position.board[square as usize] = encoded;
        }
    }
}














































































#[cfg(test)]
mod tests {
    use crate::mov::MoveFlags;
    use super::*;
    #[test]
    fn test_starting_position_piece_list() {
        let pos = Position::start();
        let (white_pawns, count) = pos.piece_list(Piece::Pawn, Color::White);
        assert_eq!(count, 8);
        for &square in &white_pawns[..count] {
            assert!(square >= 8 && square < 16);
        }

        let (black_rooks, count) = pos.piece_list(Piece::Rook, Color::Black);
        assert_eq!(count, 2);
        assert_eq!(black_rooks[0], 56);
        assert_eq!(black_rooks[1], 63);
    }

    #[test]
    fn test_starting_position_bitboards() {
        let pos = Position::start();
        assert_eq!(pos.pawns() & pos.white(), 0xFF00);
        assert_eq!(pos.pawns() & pos.black(), 0x00FF000000000000);
        assert_eq!(pos.rooks() & pos.white(), 0x81);
        assert_eq!(pos.rooks() & pos.black(), 0x8100000000000000);
    }

    #[test]
    fn test_en_passant_square_none_in_startpos() {
        let pos = Position::start();
        assert_eq!(pos.en_passant(), 64);
    }

    #[test]
    fn test_move_piece_updates_board_and_bitboards() {
        let mut pos = Position::start();
        let from = 12; // d2
        let to = 28; // d4
        let encoded = encode_piece(Piece::Pawn, Color::White);

        pos.board[from] = encoded;
        pos.board[to] = EMPTY_PIECE;
        pos.reverse_piece_index[Piece::Pawn as usize][Color::White as usize][from] = Some(0);
        pos.piece_list[Piece::Pawn as usize][Color::White as usize][0] = from as u8;

        pos.move_piece(Piece::Pawn, Color::White, from, to);

        assert_eq!(pos.board[to], encoded);
        assert_eq!(pos.board[from], EMPTY_PIECE);
        assert_ne!(pos.pawns() & (1u64 << to), 0);
        assert_eq!(pos.pawns() & (1u64 << from), 0);
    }

    #[test]
    fn test_do_move_basic_pawn_push() {
        let mut pos = Position::start();
        let from = 12; // d2
        let to = 28; // d4
        let encoded = encode_piece(Piece::Pawn, Color::White);
        pos.board[from] = encoded;
        pos.reverse_piece_index[Piece::Pawn as usize][Color::White as usize][from] = Some(0);
        pos.piece_list[Piece::Pawn as usize][Color::White as usize][0] = from as u8;

        let mov = Move::encode(from as u8, to as u8, MoveFlags::new(MoveKind::DoublePush));
        pos.do_move(mov);

        assert_eq!(pos.board[to], encoded);
        assert_eq!(pos.board[from], EMPTY_PIECE);
        assert_eq!(pos.en_passant(), 20); // square between d2 and d4
        assert_eq!(pos.turn(), Color::Black);
    }

    #[test]
    fn test_handle_capture_removes_piece_correctly() {
        let mut pos = Position::default();
        let square = 36;
        let encoded = encode_piece(Piece::Pawn, Color::Black);
        pos.board[square] = encoded;
        pos.piece_list[Piece::Pawn as usize][Color::Black as usize][0] = square as u8;
        pos.reverse_piece_index[Piece::Pawn as usize][Color::Black as usize][square] = Some(0);
        pos.piece_bitboards[Piece::Pawn as usize] = 1u64 << square;
        pos.color_bitboards[Color::Black as usize] = 1u64 << square;

        pos.handle_capture(square);

        assert_eq!(pos.board[square], EMPTY_PIECE);
        assert_eq!(pos.piece_bitboards[Piece::Pawn as usize] & (1u64 << square), 0);
        assert_eq!(pos.color_bitboards[Color::Black as usize] & (1u64 << square), 0);
    }
}


