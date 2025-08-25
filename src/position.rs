#![cfg_attr(not(debug_assertions), allow(unused_unsafe))]

use std::str::SplitWhitespace;
use crate::attacks::movegen::{all_moves};
use crate::attacks::sliding::{diagonal_attacks, orthogonal_attacks};
use crate::bitboards::{FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H};
use crate::castling_rights::{CastlingRights, CASTLE_RIGHT_MASK, ROOK_END_FROM_KING_TO, ROOK_START_FROM_KING_TO};
use crate::color::Color;
use crate::color::Color::{Black, White};
use crate::tables::{zobrist, KING_MOVES, KNIGHT_MOVES, PAWN_ATTACKS, RAYS};
use crate::mov::{en_passant_capture_pawn, flag, index_to_algebraic, is_flag_capture_promo, is_flag_quiet_promo, new_en_passant_square, Move};
use crate::direction::Dir;
use crate::eval::{build_eval, mirror, EvalCache, EG_VALUE, MG_VALUE, PHASE_INC, PST_EG, PST_MG};
use crate::piece::{is_empty, is_slider_val, piece_to_val, to_color, to_piece, to_str, ColoredPiece, Piece, EMPTY_PIECE, SEE_SCORES};
use crate::position::Status::{Checkmate, Draw, Ongoing};
pub(crate) use crate::state_info::StateInfo;
use crate::undo::{Undo, UndoStack};

pub const NO_CAPTURE: ColoredPiece = EMPTY_PIECE; // 0
pub const NO_SQ     : u8           = 64;
const MAX_PIECES: usize = 10;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Status {
    Checkmate(Color),   // winner (the *other* side to move)
    Draw,
    Ongoing,
}

#[derive(Debug, Clone)]
pub struct Position {
    board: [ColoredPiece; 64],
    piece_list: [[[u8; MAX_PIECES]; 2]; 6],  // [piece][color][index]
    piece_count: [[usize; 2]; 6],
    reverse_piece_index: [[[u8; 64]; 2]; 6], // [piece][color][square]
    bitboards: [[u64; 6]; 2],
    occupancy: [u64; 2],
    pub(crate) undo_stack: UndoStack,
    zobrist: u64,
    eval: EvalCache,
    turn: Color,
    castling_rights: CastlingRights,
    state_info: StateInfo,
    en_passant: u8,
    half_move: u8,
}
impl Default for Position {
    fn default() -> Self {
        Self {
            board: [0; 64],
            piece_list: [[[64; MAX_PIECES]; 2]; 6],
            piece_count: Default::default(),
            reverse_piece_index: [[[255; 64]; 2]; 6],
            bitboards: [[0u64; 6]; 2],
            occupancy: [0u64; 2],
            undo_stack: UndoStack::new(),
            zobrist: 0,
            eval: EvalCache::default(),
            turn: Default::default(),
            castling_rights: Default::default(),
            state_info: Default::default(),
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
            let row = rank.to_digit(10).unwrap() as u8 - 1; // subtract 1 to make it 0-based
            position.zobrist ^= zobrist::EN_PASSANT[col as usize];
            row * 8 + col
        } else {
            NO_SQ
        };


        let half_move_str: Option<&str> = iter.next();
        position.half_move = half_move_str
            .map(|s| s.parse::<u8>().expect("Invalid half move count"))
            .unwrap_or(0);
        position.half_move = 0;

        if position.turn == Color::Black {
            position.zobrist ^= zobrist::TURN_IS_BLACK;
        }
        position.zobrist ^= position.castling_rights.castling_zobrist();


        position.eval = build_eval(&position);

        position.state_info  = position.compute_pins_checks(position.turn);

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

    pub fn to_fen(&self) -> String {
        let mut s = "".to_owned();
        let mut row = "".to_owned();
        let mut consec_empties: u8 = 0;
        for (i, cp_ref) in self.board.iter().rev().enumerate()

        // board
        {
            let cp = *cp_ref;

            if is_empty(cp) {
                consec_empties += 1;
            }
            else {
                if consec_empties != 0 {
                    row += &*consec_empties.to_string();
                    consec_empties = 0;
                }
                row += to_str(cp);
            }

            if (i+1) % 8 == 0 {
                // add the reversed
                if consec_empties != 0 {
                    row += &*consec_empties.to_string();
                    consec_empties = 0;
                }
                s += &*(row.chars().rev().collect::<String>());
                if i != 63 {
                    s += "/";
                }
                row = "".to_owned();
            }
        }

        // turn
        s += " ";
        s += &*self.turn.to_str();

        // castling
        s += " ";
        s += &*self.castling_rights.to_str();

        // en passant
        s += " ";
        if self.en_passant != NO_SQ {
            s += &*index_to_algebraic(self.en_passant);
        } else {
            s += "-";
        }

        // half move
        s += " ";
        s += &*self.half_move.to_string();
        s
    }

    pub fn do_null_move(&mut self) {
        self.zobrist ^= zobrist::TURN_IS_BLACK;
        self.turn = !self.turn;
        self.state_info = self.compute_pins_checks(self.turn);
    }

    pub fn undo_null_move(&mut self) {
        self.zobrist ^= zobrist::TURN_IS_BLACK;
        self.turn = !self.turn;
        self.state_info = self.compute_pins_checks(self.turn);
    }

    pub fn do_move(&mut self, mov: Move) {
        debug_assert!(!mov.is_null());
        let (from, to) = (mov.from() as usize, mov.to() as usize);

        let colored = self.board[from];
        let piece   = to_piece(colored);
        let color   = to_color(colored);
        let flag    = mov.flag();

        /* ─── NEW: incremental-eval deltas initialisation ─────────── */
        let mut d_mg    = 0i32;
        let mut d_eg    = 0i32;
        let mut d_phase = 0i32;
        let sgn    = if color.is_white() { 1 } else { -1 };
        let from_i = if color.is_white() { mirror(from)  } else { from };
        let to_i   = if color.is_white() { mirror(to)   } else { to   };
        let p_idx  = piece as usize;

        // Moving piece leaves its square: remove its contribution from eval & phase
        d_mg    -= sgn * (PST_MG[p_idx][from_i] + MG_VALUE[p_idx]) as i32;
        d_eg    -= sgn * (PST_EG[p_idx][from_i] + EG_VALUE[p_idx]) as i32;
        d_phase -= PHASE_INC[p_idx];
        /* ─────────────────────────────────────────────────────────── */

        /* 2. capture info before mutating board --------------------- */
        let mut captured_piece  = EMPTY_PIECE;
        let mut captured_square = NO_SQ;

        if flag == flag::CAPTURE || flag == flag::EN_PASSANT || is_flag_capture_promo(flag) {
            captured_square = if flag == flag::EN_PASSANT {
                en_passant_capture_pawn(to) as u8
            } else {
                to as u8
            };
            unsafe {
                captured_piece = *self.board.get_unchecked(captured_square as usize);

                let cp              = to_piece(captured_piece) as usize;
                let captured_color  = to_color(captured_piece);
                let csgn            = if captured_color.is_white() { 1 } else { -1 };
                let cap_sq          = captured_square as usize;
                let cap_i           = if captured_color.is_white() { mirror(cap_sq) } else { cap_sq };

                // Removing a piece means subtracting its contribution from the eval & phase
                d_mg    -= csgn * (PST_MG[cp][cap_i] + MG_VALUE[cp]) as i32;
                d_eg    -= csgn * (PST_EG[cp][cap_i] + EG_VALUE[cp]) as i32;
                d_phase -= PHASE_INC[cp];
            }
        }

        /* 3. push undo **before** making changes -------------------- */
        self.undo_stack.push(Undo {
            captured_piece,
            captured_square,
            castling:    self.castling_rights,
            en_passant:  self.en_passant,
            half_move:   self.half_move,
            zobrist:     self.zobrist,
            delta_mg:    0,           // filled later
            delta_eg:    0,
            delta_phase: 0,
            state_info: self.state_info,
            mov,
        });

        /* ---- hash out old en-passant ------------------------------ */
        if self.en_passant != NO_SQ {
            self.zobrist ^= zobrist::EN_PASSANT[(self.en_passant % 8) as usize];
        }
        self.en_passant = NO_SQ;               // cleared (maybe re-set)

        /* ---- hash out moving piece on FROM ------------------------ */
        self.zobrist ^= zobrist::PIECE_SQUARES[from][p_idx][color as usize];
        self.zobrist ^= zobrist::PIECE_SQUARES[to  ][p_idx][color as usize];

        /* 5. main move ladder (unchanged logic) --------------------- */
        if flag == flag::QUIET {
            self.move_piece(piece, color, from, to);

        } else if flag == flag::DOUBLE_PAWN_PUSH {
            self.move_piece(piece, color, from, to);
            self.en_passant = new_en_passant_square(from);
            self.zobrist ^= zobrist::EN_PASSANT[(self.en_passant % 8) as usize];

        } else if flag == flag::CAPTURE {
            self.capture_and_move_piece(piece, color, from, to);

        } else if flag == flag::QUEEN_CASTLE || flag == flag::KING_CASTLE {
            let r_from = ROOK_START_FROM_KING_TO[to];
            let r_to   = ROOK_END_FROM_KING_TO[to];

            self.zobrist ^= zobrist::PIECE_SQUARES[r_from][Piece::Rook as usize][color as usize];
            self.zobrist ^= zobrist::PIECE_SQUARES[r_to  ][Piece::Rook as usize][color as usize];

            self.move_piece(piece,        color, from,   to);
            self.move_piece(Piece::Rook,  color, r_from, r_to);

            let r_from_i = if color.is_white() { mirror(r_from) } else { r_from };
            let r_to_i   = if color.is_white() { mirror(r_to)   } else { r_to   };
            let rp = Piece::Rook as usize;

            d_mg -= sgn * PST_MG[rp][r_from_i] as i32;
            d_eg -= sgn * PST_EG[rp][r_from_i] as i32;
            d_mg += sgn * PST_MG[rp][r_to_i]   as i32;
            d_eg += sgn * PST_EG[rp][r_to_i]   as i32;

        } else if is_flag_quiet_promo(flag) {
            self.move_piece(piece, color, from, to);
            self.promote_to(mov.promotion_piece(), to, color);

        } else if is_flag_capture_promo(flag) {
            self.capture_and_move_piece(piece, color, from, to);
            self.promote_to(mov.promotion_piece(), to, color);

        } else if flag == flag::EN_PASSANT {
            self.remove_piece(captured_square as usize);
            self.move_piece(piece, color, from, to);

        } else {
            unreachable!();
        }

        /* 6. add piece on TO (final piece after promotions) ---------- */
        let to_piece_idx = if is_flag_quiet_promo(flag) || is_flag_capture_promo(flag) {
            mov.promotion_piece() as usize
        } else {
            p_idx
        };

        d_mg += sgn * (PST_MG[to_piece_idx][to_i] + MG_VALUE[to_piece_idx]) as i32;
        d_eg += sgn * (PST_EG[to_piece_idx][to_i] + EG_VALUE[to_piece_idx]) as i32;
        d_phase += PHASE_INC[to_piece_idx];

        /* 7. update eval cache -------------------------------------- */
        self.eval.mg    += d_mg;
        self.eval.eg    += d_eg;
        self.eval.phase += d_phase;

        /* 8. update last pushed Undo with the deltas ---------------- */
        {
            let u = self.undo_stack.last_mut();
            u.delta_mg    = d_mg;
            u.delta_eg    = d_eg;
            u.delta_phase = d_phase;
        }

        /* 9. hash castling rights ----------------------------------- */
        let old_rights = self.castling_rights.0;
        self.castling_rights.0 &= CASTLE_RIGHT_MASK[from] & CASTLE_RIGHT_MASK[to];
        let lost = old_rights ^ self.castling_rights.0;
        if lost & 0b0001 != 0 { self.zobrist ^= zobrist::CASTLING[0]; }
        if lost & 0b0010 != 0 { self.zobrist ^= zobrist::CASTLING[1]; }
        if lost & 0b0100 != 0 { self.zobrist ^= zobrist::CASTLING[2]; }
        if lost & 0b1000 != 0 { self.zobrist ^= zobrist::CASTLING[3]; }


        /* 10. half-move clock ---------------------------------------- */
        self.half_move = if piece == Piece::Pawn || captured_piece != EMPTY_PIECE {
            0
        } else {
            self.half_move + 1
        };

        /* 11. flip turn & hash ---------------------------------------- */
        self.zobrist ^= zobrist::TURN_IS_BLACK;
        self.turn = !self.turn;
        self.state_info = self.compute_pins_checks(self.turn);
    }




    #[inline(always)]
    pub fn undo_move(&mut self) {
        debug_assert!(!self.undo_stack.is_empty());

        /* 1. pop record ------------------------------------------------ */
        let undo = self.undo_stack.pop();

        /* 2. reverse the board move ----------------------------------- */
        let (to, from) = (undo.mov.from() as usize, undo.mov.to() as usize); // reversed
        let coloured   = unsafe { *self.board.get_unchecked(from) };
        let piece      = to_piece(coloured);
        let color      = to_color(coloured);

        self.move_piece(piece, color, from, to);

        match undo.mov.flag() {
            flag::QUIET => {}
            flag::CAPTURE | flag::EN_PASSANT => {
                self.add_piece(undo.captured_piece as ColoredPiece, undo.captured_square);
            }
            flag if is_flag_quiet_promo(flag) => {
                self.replace_piece(Piece::Pawn, color, to);
            }
            flag if is_flag_capture_promo(flag) => {
                self.add_piece(undo.captured_piece as ColoredPiece, undo.captured_square);
                self.replace_piece(Piece::Pawn, color, to);
            }
            flag::KING_CASTLE | flag::QUEEN_CASTLE => {
                self.move_piece(
                    Piece::Rook,
                    color,
                    ROOK_END_FROM_KING_TO[from],
                    ROOK_START_FROM_KING_TO[from],
                );
            }
            _ => {}
        }

        /* 3. restore hash, rights, counters --------------------------- */
        self.zobrist         = undo.zobrist;
        self.castling_rights = undo.castling;
        self.en_passant      = undo.en_passant;
        self.half_move       = undo.half_move;
        self.state_info      = undo.state_info;


        /* 4. restore incremental evaluation --------------------------- */
        self.eval.mg    -= undo.delta_mg;
        self.eval.eg    -= undo.delta_eg;
        self.eval.phase -= undo.delta_phase;


        /* 5. flip side-to-move ---------------------------------------- */
        self.turn = !self.turn;

    }


    #[inline(always)]
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }
    #[inline(always)]
    pub fn capture_and_move_piece(&mut self, piece: Piece, color: Color, from: usize, to: usize) {
        /* --- remove captured piece ----------------------------------- */
        let cap_p = to_piece(self.board[to]) as usize;
        let cap_c = to_color(self.board[to]) as usize;

        self.zobrist ^= zobrist::PIECE_SQUARES[to][cap_p][cap_c];


        let idx        = self.rev_idx(cap_p, cap_c, to) as usize;
        let last_idx   = self.piece_count[cap_p][cap_c] - 1;
        let last_sq    = self.piece_list[cap_p][cap_c][last_idx] as usize;

        // swap‑remove without branch
        unsafe {
            *self.piece_list[cap_p][cap_c].get_unchecked_mut(idx)      = last_sq as u8;
            *self.reverse_piece_index[cap_p][cap_c].get_unchecked_mut(last_sq) = idx as u8;
        }
        self.piece_count[cap_p][cap_c] -= 1;

        let bb_to = 1u64 << to;
        self.bitboards[cap_c][cap_p] ^= bb_to;
        self.occupancy[cap_c]        ^= bb_to;

        /* --- move attacking piece ------------------------------------ */
        let p = piece as usize;
        let c = color as usize;

        unsafe {
            *self.board.get_unchecked_mut(from) = EMPTY_PIECE;
            *self.board.get_unchecked_mut(to)   = piece_to_val(piece, color);
        }

        let idx_from = self.rev_idx(p, c, from) as usize;
        unsafe {
            *self.reverse_piece_index[p][c].get_unchecked_mut(to)   = idx_from as u8;
            *self.piece_list[p][c].get_unchecked_mut(idx_from)      = to as u8;
        }

        let delta = (1u64 << from) ^ bb_to;
        self.bitboards[c][p] ^= delta;
        self.occupancy[c]    ^= delta;

    }

    #[inline(always)]
    fn promote_to(&mut self, new_piece: Piece, to: usize, color: Color) {
        // remove the pawn
        self.remove_piece(to);
        // add the new promotion piece to the board
        let colored_piece = piece_to_val(new_piece, color);
        self.add_piece(colored_piece, to as u8);
        self.zobrist ^= zobrist::PIECE_SQUARES[to][new_piece as usize][color as usize];

    }


    #[inline(always)]
    fn move_piece(&mut self, piece: Piece, color: Color, from: usize, to: usize) {
        let p = piece as usize;
        let c = color as usize;


        // 1. Update board squares
        unsafe {
            *self.board.get_unchecked_mut(from) = EMPTY_PIECE;
            *self.board.get_unchecked_mut(to)   = piece_to_val(piece, color);
        }


        // 2. Update reverse index
        let count = self.rev_idx(p, c, from) as usize;
        self.set_rev_idx(p, c, to, count as u8);


        debug_assert!(
            count != 255,
            "Invalid reverse index for {:?} {:?} at square {}",
            piece, color, from
        );

        if count == 255 {
            self.print_move_history();
            self.print_board();
        }

        // 3. Update piece list
        self.piece_list[p][c][count] = to as u8;

        let delta = (1u64 << from) ^ (1u64 << to);
        self.bitboards[color as usize][piece as usize] ^= delta;
        self.occupancy[color as usize]                 ^= delta;

    }

    #[inline(always)]
    fn replace_piece(&mut self, new_piece: Piece, new_color: Color, sq: usize) {
        /* -------- 1. remove the piece currently on `sq` ----------------- */
        let old_p = to_piece(self.board[sq])  as usize;
        let old_c = to_color(self.board[sq])  as usize;

        self.zobrist ^= zobrist::PIECE_SQUARES[sq][old_p][old_c];


        let idx       = self.rev_idx(old_p, old_c, sq) as usize;
        let last_idx  = self.piece_count[old_p][old_c] - 1;
        let last_sq   = self.piece_list[old_p][old_c][last_idx] as usize;

        // unconditional swap‑remove (branch‑free)
        unsafe {
            *self.piece_list[old_p][old_c].get_unchecked_mut(idx) = last_sq as u8;
            *self.reverse_piece_index[old_p][old_c].get_unchecked_mut(last_sq) = idx as u8;
        }
        self.piece_count[old_p][old_c] -= 1;

        // clear board & bitboards
        let bb_sq = 1u64 << sq;
        self.board[sq]                    = EMPTY_PIECE;
        self.bitboards[old_c][old_p]     ^= bb_sq;
        self.occupancy[old_c]            ^= bb_sq;

        /* -------- 2. add the new piece on the same square --------------- */
        let new_p = new_piece as usize;
        let new_c = new_color as usize;

        let idx_new = self.piece_count[new_p][new_c];
        self.piece_list[new_p][new_c][idx_new]           = sq as u8;
        self.reverse_piece_index[new_p][new_c][sq]       = idx_new as u8;
        self.piece_count[new_p][new_c]                  += 1;

        self.zobrist ^= zobrist::PIECE_SQUARES[sq][new_p][new_c];

        self.board[sq] = piece_to_val(new_piece, new_color);
        self.bitboards[new_c][new_p] ^= bb_sq;   // set bit
        self.occupancy[new_c]        ^= bb_sq;
    }

    #[inline(always)]
    fn add_piece(&mut self, new_piece: ColoredPiece, sq: u8) {
        let p = to_piece(new_piece) as usize;
        let c = to_color(new_piece) as usize;

        let idx = self.piece_count[p][c];          // first free slot
        self.piece_list[p][c][idx] = sq;           // append
        self.reverse_piece_index[p][c][sq as usize] = idx as u8;
        self.piece_count[p][c] += 1;               // list grows by 1

        self.board[sq as usize] = new_piece;
        let bb = 1u64 << sq;
        self.bitboards[c][p] |= bb;
        self.occupancy[c] |= bb;
    }

    #[inline(always)]
    fn remove_piece(&mut self, sq: usize) {
        let p = to_piece(self.board[sq]) as usize;
        let c = to_color(self.board[sq]) as usize;

        self.zobrist ^= zobrist::PIECE_SQUARES[sq][p][c];


        // index of the captured piece in the list
        let idx = self.rev_idx(p, c, sq) as usize;
        let last_idx  = self.piece_count[p][c] - 1;
        let last_sq   = self.piece_list[p][c][last_idx] as usize;
        // move the *last* piece into `idx`
        if idx != last_idx {
            self.piece_list[p][c][idx]            = last_sq as u8;
            self.reverse_piece_index[p][c][last_sq] = idx as u8;
        }

        self.piece_count[p][c] -= 1;

        // board & bitboards …
        unsafe {
            *self.board.get_unchecked_mut(sq) = EMPTY_PIECE;
        }
        let bb = 1u64 << sq;
        self.bitboards[c][p] &= !bb;
        self.occupancy[c]    &= !bb;
    }


    #[inline(always)]
    pub fn evaluate(&self) -> i16 {
        // self.eval.{mg,eg} are already White−Black totals
        let mut mg_phase = self.eval.phase;
        if mg_phase > 24 { mg_phase = 24; }
        if mg_phase < 0  { mg_phase = 0; }

        let eg_phase = 24 - mg_phase;

        ((self.eval.mg * mg_phase + self.eval.eg * eg_phase) / 24) as i16
    }

    #[inline(always)]
    pub fn evaluate_2(&self) -> i16 {
        // Clamp phase
        let mut mg_phase = self.eval.phase;
        if mg_phase > 24 { mg_phase = 24; }
        if mg_phase < 0  { mg_phase = 0; }
        let eg_phase = 24 - mg_phase;

        // --- simplification bias ---
        // 1) Get material lead in centipawns (White−Black). Prefer pure material, not full eval.
        // If you already keep material separately, use that. Fallback: use a material() helper.
        let material_lead_cp: i32 = self.raw_material_diff();

        // 2) Count remaining non-pawn pieces (both sides). Start-of-game is 14 (7 per side).
        let nonpawn_count: i32 = self.count_nonpawn_pieces_total();
        const MAX_NONPAWN_START: i32 = 14;

        // 3) How simplified are we? (0 at start, grows as pieces come off)
        let simplified: i32 = (MAX_NONPAWN_START - nonpawn_count).max(0);

        // 4) Tunable coefficients (in CP per piece of simplification).
        //    Stronger in endgame, milder in middlegame.
        const SIMPLIFY_MG: i32 = 2;  // cp per piece of simplification when ahead
        const SIMPLIFY_EG: i32 = 6;  // cp per piece of simplification when ahead

        // Bias sign follows the material lead:
        //  - If material_lead_cp > 0 => bonus for simplification (trading).
        //  - If material_lead_cp < 0 => penalty for simplification (discourage trades).
        // Use sign only to avoid blowing up with big leads.
        let lead_sign: i32 = material_lead_cp.signum();

        let simplify_mg: i32 = lead_sign * SIMPLIFY_MG * simplified;
        let simplify_eg: i32 = lead_sign * SIMPLIFY_EG * simplified;

        // Blend your base eval and the simplification term with the usual tapered scheme
        let blended_eval: i32 =
            (self.eval.mg + simplify_mg) * mg_phase +
                (self.eval.eg + simplify_eg) * eg_phase;

        (blended_eval / 24) as i16
    }

    #[inline(always)]
    pub fn evaluate_3(&self) -> i16 {
        // Clamp phase
        let mut mg_phase = self.eval.phase;
        if mg_phase > 24 { mg_phase = 24; }
        if mg_phase < 0  { mg_phase = 0; }
        let eg_phase = 24 - mg_phase;

        // --- simplification bias ---
        // 1) Get material lead in centipawns (White−Black). Prefer pure material, not full eval.
        // If you already keep material separately, use that. Fallback: use a material() helper.
        let material_lead_cp: i32 = self.raw_material_diff();

        // 2) Count remaining non-pawn pieces (both sides). Start-of-game is 14 (7 per side).
        let nonpawn_count: i32 = self.count_nonpawn_pieces_total();
        const MAX_NONPAWN_START: i32 = 14;

        // 3) How simplified are we? (0 at start, grows as pieces come off)
        let simplified: i32 = (MAX_NONPAWN_START - nonpawn_count).max(0);

        // 4) Tunable coefficients (in CP per piece of simplification).
        //    Stronger in endgame, milder in middlegame.
        const SIMPLIFY_MG: i32 = 0;  // cp per piece of simplification when ahead
        const SIMPLIFY_EG: i32 = 28;  // cp per piece of simplification when ahead

        // Bias sign follows the material lead:
        //  - If material_lead_cp > 0 => bonus for simplification (trading).
        //  - If material_lead_cp < 0 => penalty for simplification (discourage trades).
        // Use sign only to avoid blowing up with big leads.
        let lead_sign: i32 = material_lead_cp.signum();

        let simplify_mg: i32 = lead_sign * SIMPLIFY_MG * simplified;
        let simplify_eg: i32 = lead_sign * SIMPLIFY_EG * simplified;

        let wp = self.pawns(White);
        let bp = self.pawns(Black);
        let dp_white = doubled_pawns(wp);
        let dp_black = doubled_pawns(bp);

        const DP_MG: i32 = 30; // cp per extra pawn in a file (middlegame)
        const DP_EG: i32 = 15;  // cp (endgame)

        // Positive if Black has more doubles → increases White’s eval; negative if White has more → lowers eval.
        let dp_mg: i32 = (dp_black - dp_white) * DP_MG;
        let dp_eg: i32 = (dp_black - dp_white) * DP_EG;

        // Blend
        let blended_eval: i32 =
            (self.eval.mg + simplify_mg + dp_mg) * mg_phase +
                (self.eval.eg + simplify_eg + dp_eg) * eg_phase;

        (blended_eval / 24) as i16
    }


    #[inline(always)]
    pub fn evaluate_4(&self) -> i16 {
        // Clamp phase
        let mut mg_phase = self.eval.phase;
        if mg_phase > 24 { mg_phase = 24; }
        if mg_phase < 0  { mg_phase = 0; }
        let eg_phase = 24 - mg_phase;

        // --- simplification bias ---
        // 1) Get material lead in centipawns (White−Black). Prefer pure material, not full eval.
        // If you already keep material separately, use that. Fallback: use a material() helper.
        let material_lead_cp: i32 = self.raw_material_diff();

        // 2) Count remaining non-pawn pieces (both sides). Start-of-game is 14 (7 per side).
        let nonpawn_count: i32 = self.count_nonpawn_pieces_total();
        const MAX_NONPAWN_START: i32 = 14;

        // 3) How simplified are we? (0 at start, grows as pieces come off)
        let simplified: i32 = (MAX_NONPAWN_START - nonpawn_count).max(0);

        // 4) Tunable coefficients (in CP per piece of simplification).
        //    Stronger in endgame, milder in middlegame.
        const SIMPLIFY_MG: i32 = 0;  // cp per piece of simplification when ahead
        const SIMPLIFY_EG: i32 = 28;  // cp per piece of simplification when ahead

        // Bias sign follows the material lead:
        //  - If material_lead_cp > 0 => bonus for simplification (trading).
        //  - If material_lead_cp < 0 => penalty for simplification (discourage trades).
        // Use sign only to avoid blowing up with big leads.
        let lead_sign: i32 = material_lead_cp.signum();

        let simplify_mg: i32 = lead_sign * SIMPLIFY_MG * simplified;
        let simplify_eg: i32 = lead_sign * SIMPLIFY_EG * simplified;

        let wp = self.pawns(White);
        let bp = self.pawns(Black);
        let dp_white = doubled_pawns(wp);
        let dp_black = doubled_pawns(bp);

        const DP_MG: i32 = 30; // cp per extra pawn in a file (middlegame)
        const DP_EG: i32 = 25;  // cp (endgame)

        // Positive if Black has more doubles → increases White’s eval; negative if White has more → lowers eval.
        let dp_mg: i32 = (dp_black - dp_white) * DP_MG;
        let dp_eg: i32 = (dp_black - dp_white) * DP_EG;

        // Blend
        let blended_eval: i32 =
            (self.eval.mg + simplify_mg + dp_mg) * mg_phase +
                (self.eval.eg + simplify_eg + dp_eg) * eg_phase;

        (blended_eval / 24) as i16
    }


    #[inline(always)]
    pub fn captured_pieces(&self) -> [u8; 10] {
        [
            (8 - self.piece_count(Piece::Pawn, White)) as u8,
            (2 - self.piece_count(Piece::Knight, White)) as u8,
            (2 - self.piece_count(Piece::Bishop, White)) as u8,
            (2 - self.piece_count(Piece::Rook, White)) as u8,
            (1 - self.piece_count(Piece::Queen, White)) as u8,
            (8 - self.piece_count(Piece::Pawn, Black)) as u8,
            (2 - self.piece_count(Piece::Knight, Black)) as u8,
            (2 - self.piece_count(Piece::Bishop, Black)) as u8,
            (2 - self.piece_count(Piece::Rook, Black)) as u8,
            (1 - self.piece_count(Piece::Queen, Black)) as u8,
        ]
    }


    #[inline(always)]
    pub fn game_result_eval(&self, depth: u8) -> i16 {
        // Unchecked function to evaluate a finished chess game. Cannot be called on ongoing games.
        let result = self.get_game_result();

        match result {
            Checkmate(White) =>  10000 + (depth as i16),
            Checkmate(Black) => -10000 - (depth as i16),
            Draw             =>  0,
            Ongoing          =>  !unreachable!(),
        }
    }

    #[inline(always)]
    pub fn see(&self, mv: Move) -> bool {
        if !mv.is_capture() { return false; }

        let from = mv.from() as usize;
        let to   = mv.to()   as usize;

        let attacker = self.piece_at_sq(mv.from()) as usize;
        let victim   = self.piece_at_sq(mv.to())   as usize;

        // Cheap fast-path for threshold 0
        if SEE_SCORES[attacker] <= SEE_SCORES[victim] {
            return true;
        }

        // --- local copies (do NOT mutate self) ---
        let mut bb  = self.bitboards;   // [[u64;6];2]
        let mut occ = self.occupancy;   // [u64;2]

        let us   = self.turn as usize;
        let them = (!self.turn) as usize;

        let from_bb = 1u64 << from;
        let to_bb   = 1u64 << to;

        // 1) remove mover from its origin square
        bb[us][attacker] &= !from_bb;
        occ[us]          &= !from_bb;

        // 2) remove the captured piece from TO
        bb[them][victim] &= !to_bb;
        occ[them]        &= !to_bb;

        // 3) keep TO considered occupied by the "last mover"
        occ[us]          |= to_bb;

        // initial gain = victim + (optional) promotion bonus
        let mut gain  = [0i32; 32];
        let mut depth = 0usize;

        let promo_bonus = if is_flag_capture_promo(mv.flag()) {
            let promo_idx = mv.promotion_piece() as usize;
            SEE_SCORES[promo_idx] - SEE_SCORES[Piece::Pawn as usize]
        } else { 0 };
        gain[0] = SEE_SCORES[victim] + promo_bonus;

        // the piece currently on TO (what the opponent would win by recapturing)
        let mut last_moved_val = if is_flag_capture_promo(mv.flag()) {
            SEE_SCORES[mv.promotion_piece() as usize]
        } else {
            SEE_SCORES[attacker]
        };

        #[inline(always)]
        fn attackers_to(
            sq: usize,
            bb: &[[u64;6];2],
            occ_w: u64,
            occ_b: u64
        ) -> (u64, u64) {
            let occ_all = occ_w | occ_b;
            let w = (PAWN_ATTACKS[Black as usize][sq] & bb[White as usize][Piece::Pawn  as usize]) |
                (KNIGHT_MOVES[sq]                & bb[White as usize][Piece::Knight as usize]) |
                (diagonal_attacks(sq, occ_all)   & (bb[White as usize][Piece::Bishop as usize] | bb[White as usize][Piece::Queen as usize])) |
                (orthogonal_attacks(sq, occ_all) & (bb[White as usize][Piece::Rook   as usize] | bb[White as usize][Piece::Queen as usize])) |
                (KING_MOVES[sq]                  & bb[White as usize][Piece::King   as usize]);

            let b = (PAWN_ATTACKS[White as usize][sq] & bb[Black as usize][Piece::Pawn  as usize]) |
                (KNIGHT_MOVES[sq]                 & bb[Black as usize][Piece::Knight as usize]) |
                (diagonal_attacks(sq, occ_all)    & (bb[Black as usize][Piece::Bishop as usize] | bb[Black as usize][Piece::Queen as usize])) |
                (orthogonal_attacks(sq, occ_all)  & (bb[Black as usize][Piece::Rook   as usize] | bb[Black as usize][Piece::Queen as usize])) |
                (KING_MOVES[sq]                   & bb[Black as usize][Piece::King   as usize]);
            (w, b)
        }

        // opponent to move first after our capture
        let mut side = them;

        loop {
            let (w_atk, b_atk) = attackers_to(to, &bb, occ[White as usize], occ[Black as usize]);
            let a = if side == White as usize { w_atk } else { b_atk };
            if a == 0 { break; }

            // LVA: Pawn..King
            let mut picked_sq = None;
            let mut picked_pt = 0usize;
            for pt in 0..=5 {
                let cand = bb[side][pt] & a;
                if cand != 0 {
                    let lsb = cand & cand.wrapping_neg();
                    picked_sq = Some(lsb.trailing_zeros() as usize);
                    picked_pt = pt;
                    break;
                }
            }
            let sq = match picked_sq { Some(s) => s, None => break };

            // ---- FIX #1: store alternating net gains, not raw values ----
            depth += 1;
            gain[depth] = last_moved_val - gain[depth - 1];

            // the recapturing piece becomes the last mover
            last_moved_val = SEE_SCORES[picked_pt];

            // remove that attacker from its origin (it "moves" onto TO)
            let bit = 1u64 << sq;
            bb[side][picked_pt] &= !bit;
            occ[side]           &= !bit;

            side ^= 1;
        }

        // ---- FIX #2: classic fold with -max(-a, b) ----
        while depth > 0 {
            let m = std::cmp::max(-gain[depth - 1], gain[depth]);
            gain[depth - 1] = -m;
            depth -= 1;
        }

        gain[0] >= 0
    }



    fn raw_material_diff(&self) -> i32 {
        const VALUES: [i32; 6] = [  100, 320, 330, 500, 900,   0];

        VALUES[0] * self.piece_count(Piece::Pawn, White)
            + VALUES[1] * self.piece_count(Piece::Knight, White)
            + VALUES[2] * self.piece_count(Piece::Bishop, White)
            + VALUES[3] * self.piece_count(Piece::Rook, White)
            + VALUES[4] * self.piece_count(Piece::Queen, White)

            - VALUES[0] * self.piece_count(Piece::Pawn, Black)
            - VALUES[1] * self.piece_count(Piece::Knight, Black)
            - VALUES[2] * self.piece_count(Piece::Bishop, Black)
            - VALUES[3] * self.piece_count(Piece::Rook, Black)
            - VALUES[4] * self.piece_count(Piece::Queen, Black)
    }

    #[inline(always)]
    pub fn count_nonpawn_pieces_total(&self) -> i32 {
        self.piece_count(Piece::Knight, Black) +
            self.piece_count(Piece::Bishop, Black) +
            self.piece_count(Piece::Rook, Black) +
            self.piece_count(Piece::Queen, Black) +
            self.piece_count(Piece::Knight, White) +
            self.piece_count(Piece::Bishop, White) +
            self.piece_count(Piece::Rook, White) +
            self.piece_count(Piece::Queen, White)
    }

    #[inline(always)]
    pub fn en_passant_capture_square(&self) -> usize{
        en_passant_capture_pawn(self.en_passant as usize)
    }


    /// Return `true` if the current side‑to‑move position
    /// has occurred at least **once** earlier in the 50‑move window.
    #[inline(always)]
    pub fn is_repeat_towards_three_fold_repetition(&self) -> bool {
        // Current Zobrist key
        let key = self.zobrist;

        if self.half_move < 4 { return false; }

        // Step back two plies at a time (same side to move)
        let mut remaining = self.half_move as i32 - 2;           // reversible plies left
        let mut idx       = self.undo_stack.len as i32 - 2;      // last same‑side position

        while remaining >= 0 && idx >= 0 {
            if self.undo_stack.peek_index(idx as usize).zobrist == key {
                return true;                                     // first earlier hit
            }
            idx       -= 2;
            remaining -= 2;
        }
        false
    }

    /// Return `true` if the current side-to-move position
    /// has already occurred **twice** before in the 50-move window,
    /// i.e. this occurrence would make it a *threefold repetition*.
    #[inline(always)]
    pub fn is_three_fold_repetition(&self) -> bool {
        // Current Zobrist key
        let key = self.zobrist;

        // No chance if fewer than 8 reversible plies have passed
        // (you need at least two prior same-side positions).
        if self.half_move < 8 {
            return false;
        }

        let mut count     = 0;
        let mut remaining = self.half_move as i32 - 2;      // reversible plies left
        let mut idx       = self.undo_stack.len as i32 - 2; // last same-side position

        // Step back two plies at a time (same side to move)
        while remaining >= 0 && idx >= 0 {
            if self.undo_stack.peek_index(idx as usize).zobrist == key {
                count += 1;
                if count >= 2 {
                    return true; // found two earlier identical positions
                }
            }
            idx       -= 2;
            remaining -= 2;
        }
        false
    }

    pub fn half_move_over_ninety_nine(&self) -> bool {
        self.half_move > 99
    }


    pub fn compute_pins_checks(&self, us: Color) -> StateInfo {
        let king_sq = self.king_square(us) as usize;
        let occ     = self.occupied();

        let them = !us;
        // enemy sliders
        let rooks   = self.rooks(them);
        let bishops = self.bishops(them);
        let queens  = self.queens(them);
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
            let first_bb = if dir.is_positive() {  // N, NE, E, NW
                ray & ray.wrapping_neg()                 // LS1B
            } else {                                     // S, SW, W, SE
                1u64 << (63 - ray.leading_zeros())       // MS1B
            };
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
        let enemy_pawns   = self.pawns(them);
        let enemy_knights = self.knights(them);

        checkers |= PAWN_ATTACKS[us as usize][king_sq] & enemy_pawns;
        checkers |= KNIGHT_MOVES[king_sq]              & enemy_knights;

        StateInfo { checkers, blockers_for_king, pinners }
    }


    pub fn game_status(&self) -> Status {

        if self.half_move >= 100 || self.is_three_fold_repetition() {
            return Draw;
        }

        // 1.  Generate every legal move for the side to move.
        if all_moves(self).is_empty() {
            // 2.  No moves → either mate or stalemate.
            //
            // `all_attacks(self)` should return a bitboard (or Vec) of every
            // square attacked by the *opponent*.  Replace the helper call with
            // your own `is_in_check()` if you already have one.
            self.get_game_result()
        } else {
            // Game still in progress
            Ongoing
        }
    }

    pub fn get_game_result(&self) -> Status {
        if self.in_check() {
            // Side to move is mated; the *opposite* color wins.
            let winner = !self.side_to_move();
            Checkmate(winner)
        } else {
            Draw
        }
    }

    pub fn square_under_attack(&self, sq: u8, by: Color) -> bool {
        let pawns = self.pawns(by);

        // !by because our attacks (!by) is the fields of where enemy pawns can attack us
        if PAWN_ATTACKS[!by as usize][sq as usize] & pawns != 0 {
            return true;
        }
        let king = self.kings(by);
        if KING_MOVES[sq as usize] & king != 0 {
            return true;
        }

        let knights = self.knights(by);
        if KNIGHT_MOVES[sq as usize] & knights != 0 {
            return true;
        }

        let bishops = self.bishops(by);
        let queens = self.queens(by);
        let occ = self.occupied();

        if diagonal_attacks(sq as usize, occ) & (bishops | queens) != 0 {
            return true;
        }
        let rooks = self.rooks(by);
        if orthogonal_attacks(sq as usize, occ) & (rooks | queens) != 0 {
            return true;
        }
        false
    }

    #[inline(always)]
    fn rev_idx(&self, piece: usize, color: usize, sq: usize) -> u8 {
        self.reverse_piece_index[piece][color][sq]
    }

    #[inline(always)]
    fn set_rev_idx(&mut self, piece: usize, color: usize, sq: usize, idx: u8) {
        self.reverse_piece_index[piece][color][sq] = idx;
    }

    #[inline(always)]
    pub fn piece_count(&self, piece: Piece, color: Color) -> i32 {
        self.piece_count[piece as usize][color as usize] as i32
    }

    #[inline(always)]
    pub fn in_check(&self) -> bool {
        self.state_info.checkers != 0
    }

    #[inline(always)]
    pub fn side_to_move(&self) -> Color {
        self.turn
    }

    #[inline(always)]
    pub fn occupancy(&self, color: Color) -> u64 {
        self.occupancy[color as usize]
    }

    #[inline(always)]
    pub fn pawns(&self, color: Color) -> u64 {
        self.bitboards[color as usize][Piece::Pawn as usize]
    }
    #[inline(always)]
    pub fn knights(&self, color: Color) -> u64 {
        self.bitboards[color as usize][Piece::Knight as usize]
    }
    #[inline(always)]
    pub fn bishops(&self, color: Color) -> u64 {
        self.bitboards[color as usize][Piece::Bishop as usize]
    }
    #[inline(always)]
    pub fn rooks(&self, color: Color) -> u64 {
        self.bitboards[color as usize][Piece::Rook as usize]
    }
    #[inline(always)]
    pub fn queens(&self, color: Color) -> u64 {
        self.bitboards[color as usize][Piece::Queen as usize]
    }
    #[inline(always)]
    pub fn kings(&self, color: Color) -> u64 {
        self.bitboards[color as usize][Piece::King as usize]
    }
    #[inline(always)]
    pub fn white(&self) -> u64 {
        self.occupancy[Color::White as usize]
    }
    #[inline(always)]
    pub fn black(&self) -> u64 {
        self.occupancy[Color::Black as usize]
    }
    #[inline(always)]
    pub fn occupied(&self) -> u64 {
        self.black() | self.white()
    }
    #[inline(always)]
    pub fn zobrist(&self) -> u64 {
        self.zobrist
    }

    #[inline(always)]
    pub fn half_move(&self) -> u8 {
        self.half_move
    }

    #[inline(always)]
    pub fn state_info(&self) -> StateInfo {
        self.state_info
    }
    #[inline(always)]
    pub fn piece_exists_sq(&self, sq: u8) -> bool {
        self.board[sq as usize] != EMPTY_PIECE
    }

    #[inline(always)]
    pub fn colored_piece_at_sq(&self, sq: u8) -> ColoredPiece {
        self.board[sq as usize]
    }
    

    #[inline(always)]
    pub fn piece_at_sq(&self, sq: u8) -> Piece {
        let colored_piece = self.board[sq as usize];
        to_piece(colored_piece)
    }

    #[inline(always)]
    pub fn is_slider(&self, sq: u8) -> bool {
        is_slider_val(self.board[sq as usize])
    }

    #[inline(always)]
    pub fn en_passant(&self) -> u8 {
        self.en_passant
    }
    #[inline(always)]
    pub fn kingside(&self, color: Color) -> bool {
        self.castling_rights.kingside(color)
    }
    #[inline(always)]
    pub fn queenside(&self, color: Color) -> bool {
        self.castling_rights.queenside(color)
    }
    #[inline(always)]
    pub fn board_sq(&self, sq: u8) -> ColoredPiece {
        self.board[sq as usize]
    }
    #[inline(always)]
    pub fn piece_list(&self, piece: Piece, color: Color) -> ([u8; MAX_PIECES], usize)  {
        (self.piece_list[piece as usize][color as usize], self.piece_count[piece as usize][color as usize])
    }

    #[inline(always)]
    pub fn king_square(&self, color: Color) -> u8 {
        self.piece_list[Piece::King as usize][color as usize][0]
    }

    #[inline(always)]
    pub fn get_allies(&self, piece: Piece) -> u64 {
        self.bitboards[self.turn as usize][piece as usize]
    }

    #[inline(always)]
    pub fn get_enemies(&self, piece: Piece) -> u64 {
        self.bitboards[!self.turn as usize][piece as usize]
    }
    #[inline(always)]
    pub fn piece_bb(&self, piece: Piece, color: Color) -> u64 {
        self.bitboards[color as usize][piece as usize]
    }


    #[inline(always)]
    pub fn last_move(&self) -> Option<Move> {
        if self.undo_stack.len < 1 {
            return None;
        }
        Some(self.undo_stack.peek_index((self.undo_stack.len - 1) as usize).mov)
    }
    
    #[inline(always)]
    pub fn third_last_move(&self) -> Option<Move> {
        if self.undo_stack.len < 4 {
            return None;
        }
        Some(self.undo_stack.peek_index((self.undo_stack.len - 4) as usize).mov)
    }

    #[inline(always)]
    pub fn undo_stack(&self) -> UndoStack {
        self.undo_stack.clone()
    }

    pub fn print_move_history(&self) {
        println!("Move History: ");
        for i in 0..self.undo_stack.len {
            println!("{}", self.undo_stack.data[i as usize].mov);
        }
    }

    pub fn print_board(&self) {
        for rank in (0..8).rev() {
            print!("{} ", rank + 1);
            for file in 0..8 {
                let sq = rank * 8 + file;
                let colored_piece = self.board[sq as usize];
                let symbol = if is_empty(colored_piece) {
                    '.'
                } else {
                    let p = to_piece(colored_piece);
                    let c = to_color(colored_piece);
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

fn piece_from_char(ch: char) -> ColoredPiece {
    let color = if ch.is_uppercase() { Color::White } else { Color::Black };

    let piece = match ch.to_ascii_lowercase() {
        'p' => Piece::Pawn,
        'n' => Piece::Knight,
        'b' => Piece::Bishop,
        'r' => Piece::Rook,
        'q' => Piece::Queen,
        'k' => Piece::King,
        _ => unreachable!("Input is broken")
    };

    piece_to_val(piece, color)
}


fn update_bitboards_pieces(position: &mut Position, ch: char, square: u8) {
    let colored_piece = piece_from_char(ch);
    if is_empty(colored_piece) {
        return;
    }


    let bit = 1u64 << square;
    let p = to_piece(colored_piece) as usize;
    let c = to_color(colored_piece) as usize;

    position.zobrist ^= zobrist::PIECE_SQUARES[square as usize][p][c];

    let count = position.piece_count[p][c];
    position.piece_list[p][c][count] = square;
    position.piece_count[p][c] += 1;
    position.reverse_piece_index[p][c][square as usize] = count as u8;

    position.bitboards[c][p] |= bit;
    position.occupancy[c]    |= bit;
    position.board[square as usize] = colored_piece;
}



#[inline(always)]
fn doubled_pawns(bitboard: u64) -> i32 {
    // Count excess pawns per file: 0 if ≤1 pawn, 1 for doubled, 2 for tripled, etc.
    const FILES: [u64; 8] = [FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H];
    FILES.iter()
        .map(|f| ((bitboard & *f).count_ones() as i32 - 1).max(0))
        .sum()
}










































































mod tests {
    use super::*;


    /// Cheap in release, exhaustive in debug.
    #[allow(dead_code)]
    pub fn assert_consistent(position: &Position, yuh: &str) {
        let consistent = is_consistent(position);
        if !consistent {
            println!("Yuh: {}", yuh);
        }
        debug_assert!(consistent,
                      "Position corruption detected; see stderr for details");
    }

    /// Thorough but slow: call only under `debug_assert!`.
    ///
    ///
    // c5c4
    // h2g3
    // e8e1
    // g3g4
    // e1g1
    // g2g4

    #[allow(dead_code)]
    pub fn is_consistent(position: &Position) -> bool {

        // 1.  Board ↔ bitboards
        for sq in 0..64 {
            let colored_piece = position.board[sq];
            let on_board = colored_piece != EMPTY_PIECE;
            let in_bb    =
                (position.occupancy[0] | position.occupancy[1]) & (1u64 << sq) != 0;
            if on_board != in_bb {
                position.print_move_history();
                position.print_board();
                eprintln!("square {} board/bitboard mismatch", square_name(sq as u8));
                return false
            }
        }

        // 2.  Board ↔ reverse_piece_index / piece_list
        for p in 0..6 {
            for c in 0..2 {
                for idx in 0..position.piece_count[p][c] {
                    let sq = position.piece_list[p][c][idx] as usize;
                    if sq > 63 {
                        eprintln!("NUMBER OF PAWNS: {} ", position.piece_count[p][c]);
                        eprintln!("HALF MOVE: {}", position.half_move());
                        eprintln!("square {sq} board/bitboard mismatch");
                        eprintln!("piece list and reverse index disagree for {p:?}/{c} idx {idx}");
                    }
                    if position.reverse_piece_index[p][c][sq] != idx as u8 {
                        eprintln!("NUMBER OF PAWNS: {} ", position.piece_count[p][c]);

                        eprintln!("Reverse piece index: {:?}", position.reverse_piece_index[p][c][sq]);
                        eprintln!("square {sq} board/bitboard mismatch");
                        eprintln!("piece list and reverse index disagree for {p:?}/{c} idx {idx}");
                        return false
                    }
                    if position.board[sq] == EMPTY_PIECE {
                        eprintln!("piece list says a piece is on empty square {sq}");
                        return false
                    }
                }
            }
        }

        // 3.  Side to move sanity, castling rights squares contain correct rook/king, etc.
        //      (Fill in the details that matter for your engine.)
        true
    }


    #[test]
    fn test_starting_position_piece_list() {
        let pos = Position::start();
        let (white_pawns, count) = pos.piece_list(Piece::Pawn, Color::White);
        assert_eq!(count, 8);
        for &square in &white_pawns[..count] {
            assert!(square >= 8 && square < 16);
        }

        let (black_rooks, count) = pos.piece_list(Piece::Rook, Color::Black);
        assert_consistent(&pos, "");
        assert_eq!(count, 2);
        assert_eq!(black_rooks[0], 56);
        assert_eq!(black_rooks[1], 63);
    }

    #[test]
    fn test_starting_position_bitboards() {
        let pos = Position::start();
        assert_eq!(pos.pawns(Color::White), 0xFF00);
        assert_eq!(pos.pawns(Color::Black), 0x00FF000000000000);
        assert_eq!(pos.rooks(Color::White), 0x81);
        assert_eq!(pos.rooks(Color::Black), 0x8100000000000000);
    }

    #[test]
    fn test_en_passant_square_none_in_startpos() {
        let pos = Position::start();
        assert_eq!(pos.en_passant(), NO_SQ);
    }

    #[test]
    fn test_move_piece_updates_board_and_bitboards() {
        let mut pos = Position::start();
        let from = 12; // d2
        let to = 28; // d4
        let colored_piece = piece_to_val(Piece::Pawn, Color::White);

        pos.board[from] = colored_piece;
        pos.board[to] = EMPTY_PIECE;
        pos.reverse_piece_index[Piece::Pawn as usize][Color::White as usize][from] = 0;
        pos.piece_list[Piece::Pawn as usize][Color::White as usize][0] = from as u8;

        pos.move_piece(Piece::Pawn, Color::White, from, to);

        assert_eq!(pos.board[to], colored_piece);
        assert_eq!(pos.board[from], EMPTY_PIECE);
    }

    #[test]
    fn test_square_under_attack() {
        let pos = Position::load_position_from_fen("8/8/8/3q4/8/8/4K3/8 w - - 0 1");
        assert!(pos.square_under_attack(3, Color::Black)); // e2 is attacked by d5 queen
    }

    #[test]
    fn test_compute_pins_checks() {
        // test check detection

        let pos = Position::load_position_from_fen("4k3/8/8/8/4r3/8/4K3/8 w - - 0 1");
        let info = pos.compute_pins_checks(Color::White);
        assert_ne!(info.checkers, 0); // king is in check
        assert_eq!(info.blockers_for_king, 0); // no pin
        assert_eq!(info.pinners, 0); // no pinners

        // test pin detection
        let pos2 = Position::load_position_from_fen("qrb1knnr/pppp1ppp/4p3/8/4P2b/8/PPPP1PPP/QRBBKNNR w - - 0 1");
        let info2 = pos2.compute_pins_checks(Color::White);
        assert_eq!(info2.checkers, 0); // no check
        assert_ne!(info2.blockers_for_king, 0); // pin
        assert_ne!(info2.pinners, 0); // pinners
    }



    #[test]
    fn test_do_move_basic_pawn_push() {
        let mut pos = Position::start();
        let from = 12; // d2
        let to = 28; // d4
        let colored_piece = piece_to_val(Piece::Pawn, Color::White);
        pos.board[from] = colored_piece;
        pos.reverse_piece_index[Piece::Pawn as usize][Color::White as usize][from] = 0;
        pos.piece_list[Piece::Pawn as usize][Color::White as usize][0] = from as u8;

        let mov = Move::encode(from as u8, to as u8, flag::DOUBLE_PAWN_PUSH);
        println!("{}", mov);
        pos.do_move(mov);

        assert_eq!(pos.board[to], colored_piece);
        assert_eq!(pos.board[from], EMPTY_PIECE);
        assert_eq!(pos.en_passant(), 20); // square between d2 and d4
        assert_eq!(pos.side_to_move(), Color::Black);
    }

    #[test]
    fn test_undo_move_restores_position() {
        let mut pos = Position::start();
        let original_pos = pos.clone(); // Make sure Position derives Clone

        let from = 12; // d2
        let to = 28;   // d4
        let mov = Move::encode(from, to, flag::DOUBLE_PAWN_PUSH);

        pos.do_move(mov);
        pos.undo_move();

        assert_eq!(pos.board, original_pos.board);
        assert_eq!(pos.bitboards, original_pos.bitboards);
        assert_eq!(pos.occupancy, original_pos.occupancy);
        assert_eq!(pos.side_to_move(), original_pos.side_to_move());
        assert_consistent(&pos,"");
    }

    #[test]
    fn test_pawn_promotion() {
        let mut pos = Position::load_position_from_fen("8/P7/8/8/8/8/8/4k3 w - - 0 1");
        let from = 48;  // a7
        let to = 56;    // a8

        let mov = Move::encode(from, to, flag::PROMO_QUEEN);
        pos.do_move(mov);

        assert_eq!(pos.piece_at_sq(to), Piece::Queen);
        assert_eq!(pos.board[to as usize], piece_to_val(Piece::Queen, Color::White));
    }

    #[test]
    fn test_castling_kingside_white() {
        let mut pos = Position::load_position_from_fen("4k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
        let from = 4; // e1
        let to = 6;   // g1

        let mov = Move::encode(from, to, flag::KING_CASTLE);
        pos.do_move(mov);

        assert_eq!(pos.piece_at_sq(6), Piece::King);
        assert_eq!(pos.piece_at_sq(5), Piece::Rook);
    }



}

pub fn square_name(index: u8) -> String {
    let file = index % 8;
    let rank = index / 8;
    let file_char = (b'a' + file) as char;
    let rank_char = (b'1' + rank) as char;
    format!("{}{}", file_char, rank_char)
}