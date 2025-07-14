use crate::attacks::tables::{KING_MOVES};
use crate::color::Color;
use crate::mov::{Move, MoveFlags, MoveKind, MoveList};
use crate::position::Position;
use crate::bitboards::{pop_lsb, print_bitboard};


pub (in crate::attacks) fn king_moves(position: &Position, allies: u64, enemies: u64, us: Color, moves: &mut MoveList) {
    let sq: u8 = position.king_square(us);
    let to_bb: u64 = KING_MOVES[sq as usize]&!allies;
    let capture_bb: u64 = to_bb & enemies;
    let quiet_bb: u64 = to_bb & !capture_bb;

    pop_lsb(quiet_bb, |to| moves.push(Move::encode(sq, to, MoveFlags::new(MoveKind::Quiet))));
    pop_lsb(capture_bb, |to| moves.push(Move::encode(sq, to, MoveFlags::new(MoveKind::Capture))));
}