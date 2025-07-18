use crate::attacks::sliding::orthogonal_attacks;
use crate::attacks::tables::PAWN_ATTACKS;
use crate::bitboards::{FILE_A, FILE_H, PROMO_RANKS, RANK_3, RANK_6};
use crate::bitboards::pop_lsb;
use crate::color::Color;
use crate::mov::{Move, MoveFlags, MoveKind, MoveList};
use crate::piece::Piece;
use crate::state_info::StateInfo;
use crate::position::{Position};


#[inline]
fn advance(bb: u64, amount: u8, color: Color) -> u64 {
    if color.is_white() { bb << amount } else { bb >> amount }
}

#[inline]
pub fn generate_pawn_moves_from(mut bitboard: u64, shift: u8, color: Color, move_kind: MoveKind, moves: &mut MoveList) {
    while bitboard != 0 {
        let to = bitboard.trailing_zeros() as u8;
        let from = if color.is_white() { to - shift } else { to + shift };
        if PROMO_RANKS & (1u64 << to) != 0 {
            for piece in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                moves.push(Move::encode(from, to, MoveFlags::with_promote(move_kind, piece)));
            }
        } else {
            moves.push(Move::encode(from, to, MoveFlags::new(move_kind)));
        }
        bitboard &= bitboard - 1;
    }
}

#[inline]
pub fn generate_pawn_moves_from_en_passant(position: &Position, mut bitboard: u64, shift: u8, color: Color, occupied: u64, move_kind: MoveKind, moves: &mut MoveList) {
    while bitboard != 0 {
        let to = bitboard.trailing_zeros() as u8;
        let from = if color.is_white() { to - shift } else { to + shift };
        if ep_legal(position, from, to, color, occupied) {
            moves.push(Move::encode(from, to, MoveFlags::new(move_kind)));
        }
        bitboard &= bitboard - 1;
    }
}


pub (in crate::attacks) fn pawn_moves(position: &Position, enemies: u64, info: &StateInfo, us: Color, moves: &mut MoveList) {
    let occupied: u64 = position.occupied();
    let king_sq  = position.king_square(us) as usize;
    let all_pawns  = position.get_allies(Piece::Pawn);
    let pinned     = info.blockers_for_king & all_pawns;   // only *our* pinned pieces

    let en_passant_bb: u64 = if position.en_passant() == 64 { 0 } else { 1u64 << position.en_passant() };

    unpinned_pawns(position, enemies, us, moves, occupied, en_passant_bb, all_pawns & !pinned);

    // ── pinned pawns – treat one by one ──────────────────
    let mut bb = pinned;
    while bb != 0 {
        let from = bb.trailing_zeros() as usize;
        bb &= bb - 1;

        // ❶ build the pin ray mask (inclusive king↔slider)
        let mask = StateInfo::pin_ray(king_sq, from, info.pinners);

        // ❷ generate moves for just this pawn, then AND with mask
        pawn_from_pinned_sq(position, from as u8, enemies, en_passant_bb ,mask, us, moves);
    }
}

fn pawn_from_pinned_sq(position: &Position, sq: u8, enemies: u64, en_passant_bb: u64, mask:  u64, us: Color, moves: &mut MoveList) {
    let occupied = position.occupied();
    let push_rank = if us.is_white() { RANK_3 } else { RANK_6 };
    let (lshift, rshift) = if us.is_white() { (7, 9) } else { (9, 7) };

    let from_bb = 1u64 << sq;

    let pseudo_single: u64 = advance(from_bb, 8, us) & (!occupied);
    let single: u64 = pseudo_single & !enemies & mask;
    let double: u64 = advance(pseudo_single & push_rank, 8, us) &!enemies & mask;
    // captures on the two diagonals
    let capture_left = advance(from_bb & !FILE_A, lshift, us) & enemies & mask;
    let capture_right = advance(from_bb & !FILE_H, rshift, us) & enemies & mask;
    let left_en_passant = advance(from_bb & !FILE_A, lshift, us) & en_passant_bb & mask;
    let right_en_passant = advance(from_bb & !FILE_H, rshift, us) & en_passant_bb & mask;

    generate_pawn_moves_from(single, 8,  us, MoveKind::Quiet,       moves);
    generate_pawn_moves_from(double, 16, us, MoveKind::DoublePush,  moves);
    generate_pawn_moves_from(capture_left, lshift, us, MoveKind::Capture, moves);
    generate_pawn_moves_from(capture_right, rshift, us, MoveKind::Capture, moves);


    // generate_pawn_moves_from(left_en_passant, lshift, us, MoveKind::EnPassant, moves);
    // generate_pawn_moves_from(right_en_passant, rshift, us, MoveKind::EnPassant, moves);

    generate_pawn_moves_from_en_passant(position, left_en_passant,  lshift, us, occupied, MoveKind::EnPassant, moves);
    generate_pawn_moves_from_en_passant(position, right_en_passant, rshift, us, occupied, MoveKind::EnPassant, moves);
}

/// Returns `true` if the en‑passant capture from `from` to `to` is **legal**
#[inline]
fn ep_legal(position: &Position, from: u8, to: u8, us: Color, occ: u64) -> bool {
    // square of the pawn that will be captured
    let cap_sq = if us.is_white() { to - 8 } else { to + 8 };

    let from_bb = 1u64 << from;
    let to_bb   = 1u64 << to;
    let cap_bb  = 1u64 << cap_sq;

    let ksq = position.king_square(us) as usize;

    // 1.  build occupancy *after* EP
    let occ_after = occ ^ from_bb ^ cap_bb ^ to_bb;

    let them = !us;
    // 2.  are we now hit by a rook/queen or bishop/queen?
    let rook_attack   = orthogonal_attacks(ksq, occ_after) &
        (position.piece_bb(Piece::Rook, them)
            | position.piece_bb(Piece::Queen, them));

    rook_attack == 0
}



fn unpinned_pawns(position: &Position, enemies: u64, us: Color, moves: &mut MoveList, occupied: u64, en_passant_bb: u64, pawns: u64) {
    let push_rank: u64 = if us.is_white() { RANK_3 } else { RANK_6 };

    let (lshift, rshift) = if us.is_white() { (7u8, 9u8) } else { (9u8, 7u8) };

    // Get the destinations bitboards
    let single: u64 = advance(pawns, 8, us) & (!occupied);
    let double: u64 = advance(single & push_rank, 8, us) & (!occupied);

    let left_captures = advance(pawns & !FILE_A, lshift, us) & enemies;
    let right_captures = advance(pawns & !FILE_H, rshift, us) & enemies;

    let left_en_passant = advance(pawns & !FILE_A, lshift, us) & en_passant_bb;
    let right_en_passant = advance(pawns & !FILE_H, rshift, us) & en_passant_bb;

    generate_pawn_moves_from(single, 8, us, MoveKind::Quiet, moves);
    generate_pawn_moves_from(double, 16, us, MoveKind::DoublePush, moves);
    generate_pawn_moves_from(left_captures, lshift, us, MoveKind::Capture, moves);
    generate_pawn_moves_from(right_captures, rshift, us, MoveKind::Capture, moves);
    //
    //
    // generate_pawn_moves_from(left_en_passant, lshift, us, MoveKind::EnPassant, moves);
    // generate_pawn_moves_from(right_en_passant, rshift, us, MoveKind::EnPassant, moves);
    generate_pawn_moves_from_en_passant(position, left_en_passant,  lshift, us, occupied, MoveKind::EnPassant, moves);
    generate_pawn_moves_from_en_passant(position, right_en_passant, rshift, us, occupied, MoveKind::EnPassant, moves);
}

pub fn pawn_moves_evasion(position: &Position, info: &StateInfo, enemies: u64, block_mask: u64, checker_bb: u64, us: Color, moves: &mut MoveList) {
    let occupied: u64 = position.occupied();

    let all_pawns: u64 = position.get_allies(Piece::Pawn);
    let pinned     = info.blockers_for_king & all_pawns;   // only *our* pinned pieces


    let en_passant_bb: u64 = if position.en_passant() == 64 { 0 } else { 1u64 << position.en_passant() };
    let mut pinned_bb = pinned & position.get_allies(Piece::Pawn);
    while pinned_bb != 0 {
        let from = pinned_bb.trailing_zeros() as usize;
        pinned_bb &= pinned_bb - 1;

        let ray_mask = StateInfo::pin_ray(position.king_square(us) as usize, from, info.pinners) & block_mask;
        pawn_from_pinned_sq(position, from as u8, enemies, en_passant_bb, ray_mask, us, moves);
    }

    let unpinned = all_pawns & !pinned;


    let push_rank: u64 = if us.is_white() {RANK_3} else {RANK_6};
    let en_passant_bb: u64 = if position.en_passant() == 64 {0} else {1u64 << position.en_passant()} ;
    let en_passant_piece_bb: u64 = if en_passant_bb == 0 {0} else {1u64 << position.en_passant_capture_pawn()} ;
    let en_passant_legal: bool = en_passant_piece_bb == checker_bb;

    let (lshift, rshift) = if us.is_white() { (7u8, 9u8) } else { (9u8, 7u8) };

    // Get the destinations bitboards
    let pseudo_single: u64 = advance(unpinned, 8, us) & (!occupied);
    let single: u64 = pseudo_single & block_mask;
    let double: u64 = advance(pseudo_single & push_rank, 8, us) & block_mask; // block mask already ensures its !occupied

    let left_captures  = advance(unpinned & !FILE_A, lshift, us) & checker_bb;
    let right_captures = advance(unpinned & !FILE_H, rshift, us) & checker_bb;

    generate_pawn_moves_from(single, 8, us, MoveKind::Quiet, moves);
    generate_pawn_moves_from(double, 16, us, MoveKind::DoublePush, moves);
    generate_pawn_moves_from(left_captures, lshift, us, MoveKind::Capture, moves);
    generate_pawn_moves_from(right_captures, rshift, us, MoveKind::Capture, moves);

    if en_passant_legal {
        let left_en_passant = advance(unpinned & !FILE_A, lshift, us) & en_passant_bb;
        let right_en_passant = advance(unpinned & !FILE_H, rshift, us) & en_passant_bb;

        generate_pawn_moves_from_en_passant(position, left_en_passant,  lshift, us, occupied, MoveKind::EnPassant, moves);
        generate_pawn_moves_from_en_passant(position, right_en_passant, rshift, us, occupied, MoveKind::EnPassant, moves);
    }

}


pub fn pawn_attacks(position: &Position, color: Color) -> u64{
    let mut attacks: u64 = 0;
    let pawns = position.piece_bb(Piece::Pawn, color);
    pop_lsb(pawns, |sq| {
        attacks |= PAWN_ATTACKS[color as usize][sq as usize];
    });
    attacks
}