use crate::attacks::tables::{KNIGHT_MOVES};
use crate::bitboards::{pop_lsb};
use crate::color::Color;
use crate::mov::{Move, MoveFlags, MoveKind, MoveList};
use crate::piece::Piece;
use crate::position::{Position, StateInfo};

pub (in crate::attacks) fn knight_moves(position: &Position, allies: u64, enemies: u64, info: &StateInfo, us: Color, moves: &mut MoveList) {
    let (squares, count) = position.piece_list(Piece::Knight, us);

    let all_knights: u64 = position.get_allies(Piece::Knight);
    let pinned     = info.blockers_for_king & all_knights;

    for &from in &squares[..count] {
        if pinned & (1u64 << from) != 0 {
            continue;
        }
        let all = KNIGHT_MOVES[from as usize] & !allies;
        let quiet = all & !enemies;
        let capture = all & enemies;

        pop_lsb(quiet, |to| {
            moves.push(Move::encode(from, to, MoveFlags::new(MoveKind::Quiet)));
        });

        pop_lsb(capture, |to| {
            moves.push(Move::encode(from, to, MoveFlags::new(MoveKind::Capture)));
        });
    }
}

pub fn knight_moves_evasion(position: &Position, info: &StateInfo, enemies: u64, block_mask: u64, us: Color, moves: &mut MoveList) {
    let all_knights: u64 = position.get_allies(Piece::Knight);
    let pinned     = info.blockers_for_king & all_knights;

    let (squares, count) = position.piece_list(Piece::Knight, us);
    for &from in &squares[..count] {
        if pinned & (1u64 << from) != 0 {
            continue;
        }

        // Since block mask has no allies (guaranteed) we don't need to filter allies
        let all = KNIGHT_MOVES[from as usize];
        let quiet = all & !enemies & block_mask;
        let capture = all & enemies & block_mask;

        pop_lsb(quiet, |to| {
            moves.push(Move::encode(from, to, MoveFlags::new(MoveKind::Quiet)));
        });

        pop_lsb(capture, |to| {
            moves.push(Move::encode(from, to, MoveFlags::new(MoveKind::Capture)));
        });
    }


}

pub fn knight_attacks(position: &Position, color: Color) -> u64{
    let mut attacks: u64 = 0;
    let knights = position.piece_bb(Piece::Knight, color);
    pop_lsb(knights, |sq| {
        attacks |= KNIGHT_MOVES[sq as usize];
    });
    attacks
}