use crate::color::Color;
use crate::mov::{Move, MoveFlags, MoveKind, MoveList};
use crate::piece::Piece;
use crate::position::Position;

pub (in crate::attacks) fn pawn_moves(position: &Position, pawns: u64, allies: u64, enemies: u64, us: Color, moves: &mut MoveList) {
    let occupied: u64 = allies|enemies;

    #[inline]
    fn advance(bb: u64, amount: u8, color: Color) -> u64 {
        if color.is_white() { bb << amount } else { bb >> amount }
    }

    #[inline]
    fn reverse(sq: u8, amount: u8, color: Color) -> u8 {
        if color.is_white() { sq - amount } else { sq + amount }
    }


    let push_rank: u64 = if us.is_white() {0x0000_0000_00FF_0000} else {0x0000_FF00_0000_0000};
    const FILE_A: u64 = 0x0101_0101_0101_0101;
    const FILE_H: u64 = 0x8080_8080_8080_8080;
    const PROMO_RANKS: u64 = 0xFF00_0000_0000_00FF;

    let en_passant_bb: u64 = if position.en_passant() == 64 {0} else {1u64 << position.en_passant()} ;

    let (lshift, rshift) = if us.is_white() { (7u8, 9u8) } else { (9u8, 7u8) };

    // Get the destinations bitboards
    let single: u64 = advance(pawns, 8, us)&(!occupied);
    let double: u64 = advance(single&push_rank, 8, us)&(!occupied);

    let left_captures  = advance(pawns & !FILE_A, lshift, us) & enemies;
    let right_captures = advance(pawns & !FILE_H, rshift, us) & enemies;

    let left_en_passant  = advance(pawns & !FILE_A, lshift, us) & en_passant_bb;
    let right_en_passant = advance(pawns & !FILE_H, rshift, us) & en_passant_bb;


    // Loop used for extracting the origins (from) and the destinations (to) for each move from the pawn and destination bitboards
    let pop_lsb_loop = |bitboard: u64, shift: u8, move_kind: MoveKind, moves: &mut MoveList| {
        let mut bb = bitboard;
        while bb != 0 {
            let to = bb.trailing_zeros() as u8;
            let from = reverse(to, shift, us);
            if PROMO_RANKS & (1u64 << to) != 0 {
                // four under-promotion variants
                for piece in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                    moves.push(Move::encode(from, to, MoveFlags::with_promote(move_kind, piece)));
                }
            } else {
                moves.push(Move::encode(from, to, MoveFlags::new(move_kind)));
            }
            bb &= bb - 1;
        }
    };

    pop_lsb_loop(single, 8, MoveKind::Quiet, moves);
    pop_lsb_loop(double,16, MoveKind::DoublePush, moves);
    pop_lsb_loop(left_captures, lshift, MoveKind::Capture, moves);
    pop_lsb_loop(right_captures, rshift, MoveKind::Capture, moves);
    pop_lsb_loop(left_en_passant, lshift, MoveKind::EnPassant, moves);
    pop_lsb_loop(right_en_passant, rshift, MoveKind::EnPassant, moves);
}
