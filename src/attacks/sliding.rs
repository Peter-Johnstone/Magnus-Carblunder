use crate::color::Color;
use crate::tables::{BISHOP_ATTACKS, BISHOP_MAGICS, BISHOP_MASKS, BISHOP_SHIFTS, ROOK_ATTACKS, ROOK_MAGICS, ROOK_MASKS, ROOK_SHIFTS};
use crate::bitboards::{pop_lsb, FULL_BB};
use crate::mov::{Move, flag, MoveList};
use crate::piece::Piece;
use crate::position::{Position, StateInfo};

pub(in crate::attacks) fn rook_moves(position: &Position, info: &StateInfo, allies: u64, enemies: u64, us: Color, moves: &mut MoveList) {

    let (squares, count) = position.piece_list(Piece::Rook, us);
    let pinned = info.blockers_for_king & position.get_allies(Piece::Rook);
    let king = position.king_square(us) as usize;

    for &sq in &squares[..count] {
        let mask = if pinned & (1u64 << sq) != 0 {
            StateInfo::pin_ray(king, sq as usize, info.pinners)
        } else {
            FULL_BB // unrestricted
        };
        orthogonal_moves(allies, enemies, mask, moves, sq);
    }
}

pub(in crate::attacks) fn queen_moves(position: &Position, info: &StateInfo, allies: u64, enemies: u64, us: Color, moves: &mut MoveList) {

    let (squares, count) = position.piece_list(Piece::Queen, us);
    let pinned = info.blockers_for_king & position.get_allies(Piece::Queen);
    let king = position.king_square(us) as usize;

    for &sq in &squares[..count] {
        let mask = if pinned & (1u64 << sq) != 0 {
            StateInfo::pin_ray(king, sq as usize, info.pinners)
        } else {
            FULL_BB // unrestricted
        };
        diagonal_moves(allies, enemies, mask, moves, sq);
        orthogonal_moves(allies, enemies, mask, moves, sq);
    }
}

pub(in crate::attacks) fn bishop_moves(position: &Position, info: &StateInfo, allies: u64, enemies: u64, us: Color, moves: &mut MoveList) {

    let (squares, count) = position.piece_list(Piece::Bishop, us);

    let pinned = info.blockers_for_king & position.get_allies(Piece::Bishop);
    let king = position.king_square(us) as usize;

    for &sq in &squares[..count] {
        let mask = if pinned & (1u64 << sq) != 0 {
            StateInfo::pin_ray(king, sq as usize, info.pinners)
        } else {
            FULL_BB // unrestricted
        };
        diagonal_moves(allies, enemies, mask, moves, sq);
    }
}

pub fn diagonal_attacks(sq: usize, occupied: u64) -> u64 {
    let blockers: u64 = BISHOP_MASKS[sq] & occupied;
    let idx: usize = ((blockers.wrapping_mul(BISHOP_MAGICS[sq]))
        >> BISHOP_SHIFTS[sq]) as usize;
    let row: &[u64; 512] = &BISHOP_ATTACKS[sq];
    row[idx]

}

fn diagonal_moves(allies: u64, enemies: u64, move_mask: u64, moves: &mut MoveList, sq: u8) {
    let attacks: u64 = diagonal_attacks(sq as usize, allies | enemies) & !allies & move_mask;
    let quiet_bb: u64 = attacks & !enemies;
    let capture_bb: u64 = attacks & enemies;

    pop_lsb(quiet_bb, |to| {
        moves.push(Move::encode(sq, to, flag::QUIET))
    });
    pop_lsb(capture_bb, |to| {
        moves.push(Move::encode(sq, to, flag::CAPTURE))
    });
}

pub fn orthogonal_attacks(sq: usize, occupied: u64) -> u64 {
    let blockers: u64 = ROOK_MASKS[sq] & occupied;
    let idx: usize = ((blockers.wrapping_mul(ROOK_MAGICS[sq]))
        >> ROOK_SHIFTS[sq]) as usize;


    let row: &[u64; 4096] = &ROOK_ATTACKS[sq];
    row[idx]
}

fn orthogonal_moves(allies: u64, enemies: u64, move_mask: u64, moves: &mut MoveList, sq: u8) {
    let attacks: u64 = orthogonal_attacks(sq as usize, allies|enemies) & !allies & move_mask;

    let quiet_bb: u64 = attacks & !enemies;
    let capture_bb: u64 = attacks & enemies;

    pop_lsb(quiet_bb, |to| {
        moves.push(Move::encode(sq, to, flag::QUIET))
    });
    pop_lsb(capture_bb, |to| {
        moves.push(Move::encode(sq, to, flag::CAPTURE))
    });
}


pub fn bishop_attacks(position: &Position, color: Color) -> u64 {
    let mut attacks: u64 = 0;
    let bishops: u64 = position.piece_bb(Piece::Bishop, color);
    let occ: u64 = position.occupied();
    pop_lsb(bishops, |sq| {
        attacks |= diagonal_attacks(sq as usize, occ);
    });
    attacks
}

pub fn rook_attacks(position: &Position, color: Color) -> u64 {
    let mut attacks: u64 = 0;
    let rooks: u64 = position.piece_bb(Piece::Rook, color);
    let occ : u64= position.occupied();
    pop_lsb(rooks, |sq| {
        attacks |= orthogonal_attacks(sq as usize, occ);
    });
    attacks
}

pub fn queen_attacks(position: &Position, color: Color) -> u64 {
    let mut attacks: u64 = 0;
    let queens: u64 = position.piece_bb(Piece::Queen, color);
    let occ: u64 = position.occupied();
    pop_lsb(queens, |sq| {
        attacks |= diagonal_attacks(sq as usize, occ) | orthogonal_attacks(sq as usize, occ);
    });
    attacks
}

pub fn bishop_moves_evasion(position: &Position, info: &StateInfo, allies: u64, enemies: u64, block_mask: u64, us: Color, moves: &mut MoveList) {
    let (squares, count) = position.piece_list(Piece::Bishop, us);

    // Bitboard of *our* absolutely‑pinned pieces
    let pinned = info.blockers_for_king & position.get_allies(Piece::Bishop);
    let king   = position.king_square(us) as usize;

    for &sq in &squares[..count] {
        // Fast bit‑test: is this bishop pinned?
        if pinned & (1u64 << sq) != 0 {
            // Allowed squares are only on the pin ray
            let ray = StateInfo::pin_ray(king, sq as usize, info.pinners);
            diagonal_moves(allies, enemies, block_mask & ray, moves, sq);
        } else {
            // Normal blocker/capture mask
            diagonal_moves(allies, enemies, block_mask, moves, sq);
        }
    }
}

pub fn rook_moves_evasion(position: &Position, info: &StateInfo, allies: u64, enemies: u64, block_mask: u64, us: Color, moves: &mut MoveList) {
    let (squares, count) = position.piece_list(Piece::Rook, us);

    // Bitboard of *our* absolutely‑pinned pieces
    let pinned = info.blockers_for_king & position.get_allies(Piece::Rook);
    let king   = position.king_square(us) as usize;

    for &sq in &squares[..count] {
        // Fast bit‑test: is this rook pinned?
        if pinned & (1u64 << sq) != 0 {
            // Allowed squares are only on the pin ray
            let ray = StateInfo::pin_ray(king, sq as usize, info.pinners);
            orthogonal_moves(allies, enemies, block_mask & ray, moves, sq);
        } else {
            // Normal blocker/capture mask
            orthogonal_moves(allies, enemies, block_mask, moves, sq);
        }
    }
}

pub fn queen_moves_evasion(position: &Position, info: &StateInfo, allies: u64, enemies: u64, block_mask: u64, us: Color, moves: &mut MoveList) {
    let (squares, count) = position.piece_list(Piece::Queen, us);

    // Bitboard of *our* absolutely‑pinned pieces
    let pinned = info.blockers_for_king & position.get_allies(Piece::Queen);
    let king   = position.king_square(us) as usize;

    for &sq in &squares[..count] {
        // Fast bit‑test: is this queen pinned?
        if pinned & (1u64 << sq) != 0 {
            // Allowed squares are only on the pin ray
            let ray = StateInfo::pin_ray(king, sq as usize, info.pinners);
            orthogonal_moves(allies, enemies, block_mask & ray, moves, sq);
            diagonal_moves(allies, enemies, block_mask & ray, moves, sq);

        } else {
            // Normal blocker/capture mask
            orthogonal_moves(allies, enemies, block_mask, moves, sq);
            diagonal_moves(allies, enemies, block_mask, moves, sq);

        }
    }
}