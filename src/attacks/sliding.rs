use crate::color::Color;
use crate::attacks::tables::{BISHOP_ATTACKS, BISHOP_MAGICS, BISHOP_MASKS, BISHOP_SHIFTS, ROOK_ATTACKS, ROOK_MAGICS, ROOK_MASKS, ROOK_SHIFTS};
use crate::bitboards::pop_lsb;
use crate::mov::{Move, MoveFlags, MoveKind, MoveList};
use crate::piece::Piece;
use crate::position::Position;

pub(in crate::attacks) fn rook_moves(position: &Position, allies: u64, enemies: u64, us: Color, moves: &mut MoveList) {

    let (squares, count) = position.piece_list(Piece::Rook, us);

    for &sq in &squares[..count] {
        orthogonal_moves(allies, enemies, moves, sq);
    }
}

pub(in crate::attacks) fn bishop_moves(position: &Position, allies: u64, enemies: u64, us: Color, moves: &mut MoveList) {

    let (squares, count) = position.piece_list(Piece::Bishop, us);

    for &sq in &squares[..count] {
        diagonal_moves(allies, enemies, moves, sq);
    }
}

pub(in crate::attacks) fn queen_moves(position: &Position, allies: u64, enemies: u64, us: Color, moves: &mut MoveList) {

    let (squares, count) = position.piece_list(Piece::Queen, us);

    for &sq in &squares[..count] {
        orthogonal_moves(allies, enemies, moves, sq);
        diagonal_moves(allies, enemies, moves, sq);
    }
}

pub fn orthogonal_attacks(sq: usize, occupied: u64) -> u64 {
    let blockers = ROOK_MASKS[sq] & occupied;
    let idx = ((blockers.wrapping_mul(ROOK_MAGICS[sq]))
        >> ROOK_SHIFTS[sq]) as usize;


    let row = &ROOK_ATTACKS[sq];
    row[idx]
}

fn orthogonal_moves(allies: u64, enemies: u64, moves: &mut MoveList, sq: u8) {
    let attacks = orthogonal_attacks(sq as usize, allies|enemies) & !allies;

    let quiet_bb = attacks & !enemies;
    let capture_bb = attacks & enemies;

    pop_lsb(quiet_bb, |to| {
        moves.push(Move::encode(sq, to, MoveFlags::new(MoveKind::Quiet)))
    });
    pop_lsb(capture_bb, |to| {
        moves.push(Move::encode(sq, to, MoveFlags::new(MoveKind::Capture)))
    });
}

pub fn diagonal_attacks(sq: usize, occupied: u64) -> u64 {
    let blockers = BISHOP_MASKS[sq] & occupied;
    let idx = ((blockers.wrapping_mul(BISHOP_MAGICS[sq]))
        >> BISHOP_SHIFTS[sq]) as usize;

    let row = &BISHOP_ATTACKS[sq];
    row[idx]
    
}

fn diagonal_moves(allies: u64, enemies: u64, moves: &mut MoveList, sq: u8) {
    let attacks = diagonal_attacks(sq as usize, allies | enemies) & !allies;

    let quiet_bb = attacks & !enemies;
    let capture_bb = attacks & enemies;

    pop_lsb(quiet_bb, |to| {
        moves.push(Move::encode(sq, to, MoveFlags::new(MoveKind::Quiet)))
    });
    pop_lsb(capture_bb, |to| {
        moves.push(Move::encode(sq, to, MoveFlags::new(MoveKind::Capture)))
    });
}
