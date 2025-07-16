use crate::attacks::king::{king_attacks, king_moves};
use crate::attacks::knight::{knight_attacks, knight_moves, knight_moves_evasion};
use crate::attacks::pawn::{pawn_attacks, pawn_moves, pawn_moves_evasion};
use crate::attacks::sliding::{bishop_attacks, bishop_moves, bishop_moves_evasion, queen_attacks, queen_moves, queen_moves_evasion, rook_attacks, rook_moves, rook_moves_evasion};
use crate::attacks::tables::{BETWEEN_EXCLUSIVE};
use crate::bitboards::print_bitboard;
use crate::mov::MoveList;
use crate::color::Color;
use crate::position::{StateInfo, Position};



pub fn all_moves(position: &Position) -> MoveList {

    let us: Color = position.turn();
    let allies: u64 = position.occupancy(us);
    let enemies: u64 = position.occupancy(!us);
    let mut moves = MoveList::new();

    let info = position.compute_pins_checks(us);
    print_bitboard(info.checkers());
    let in_check = info.is_check();

    let unsafe_squares = all_attacks(position, !us);
    if !in_check {
        all_pseudolegal_moves(position, allies, enemies, unsafe_squares, us, &info, &mut moves);
    } else if info.is_double_check() {
        king_moves(position, allies, enemies, unsafe_squares, us, &mut moves);
    } else {
        check_evasions(position, allies, enemies, unsafe_squares, &info, us, &mut moves);
    }
    moves
}

pub fn all_pseudolegal_moves(position: &Position, allies: u64, enemies: u64, unsafe_squares: u64, us: Color, info: &StateInfo, moves: &mut MoveList) {
    pawn_moves(position, enemies, info, us, moves);
    king_moves(position, allies, enemies, unsafe_squares, us, moves);
    knight_moves(position, allies, enemies, info, us, moves);
    bishop_moves(position, info, allies, enemies, us, moves);
    rook_moves(position, info, allies, enemies, us, moves);
    queen_moves(position, info, allies, enemies, us, moves);
}

pub fn check_evasions(position: &Position, allies: u64, enemies: u64, unsafe_squares: u64, info: &StateInfo, us: Color, moves: &mut MoveList) {
    // If we get to this point, there's only one checker

    // Single check
    let checker_bb = info.checkers();
    let checker_sq = checker_bb.trailing_zeros() as u8;
    let checker_bb = 1u64 << checker_sq;

    let king_sq = position.king_square(us);

    // Squares we can block the check on (between king and slider)
    let mut block_mask = BETWEEN_EXCLUSIVE[king_sq as usize][checker_sq as usize];

    // pawns require separation between block mask and checker "capture" mask, whereas other pieces don't
    pawn_moves_evasion(position, info, enemies, block_mask, checker_bb, us, moves);
    block_mask |= checker_bb;

    knight_moves_evasion(position, info, enemies, block_mask, us, moves);
    bishop_moves_evasion(position, info, allies, enemies, block_mask, us, moves);
    rook_moves_evasion(position, info, allies, enemies, block_mask, us, moves);
    queen_moves_evasion(position, info, allies, enemies, block_mask, us, moves);

    king_moves(position, allies, enemies, unsafe_squares, us, moves);
}

pub fn all_attacks(position: &Position, color: Color) -> u64 {
    pawn_attacks(position, color) |
    knight_attacks(position, color) |
    bishop_attacks(position, color) |
    rook_attacks(position, color) |
    queen_attacks(position, color) |
    king_attacks(position, color)
}


