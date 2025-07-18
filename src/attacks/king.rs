use crate::attacks::tables::KING_MOVES;
use crate::color::Color;
use crate::mov::{Move, MoveFlags, MoveKind, MoveList};
use crate::position::Position;
use crate::bitboards::{pop_lsb};
use crate::piece::{EMPTY_PIECE};

pub (in crate::attacks) fn king_moves(position: &Position, allies: u64, enemies: u64, unsafe_squares: u64, us: Color, moves: &mut MoveList) {
    let sq: u8 = position.king_square(us);
    let to_bb: u64 = KING_MOVES[sq as usize] & !allies & !unsafe_squares;
    let capture_bb: u64 = to_bb & enemies;
    let quiet_bb: u64 = to_bb & !capture_bb;


    pop_lsb(quiet_bb, |to| {moves.push(Move::encode(sq, to, MoveFlags::new(MoveKind::Quiet)));});
    pop_lsb(capture_bb, |to| {moves.push(Move::encode(sq, to, MoveFlags::new(MoveKind::Capture)));});

    
    let in_check = (unsafe_squares & (1u64 << sq)) != 0;

    if !in_check {
        // check castling
        if can_castle_queenside(position, us) {
            moves.push(Move::encode(sq, sq - 2, MoveFlags::new(MoveKind::Castling)));
        }

        if can_castle_kingside(position, us) {
            moves.push(Move::encode(sq, sq + 2, MoveFlags::new(MoveKind::Castling)));
        }
    }
}


fn can_castle_kingside(position: &Position, us: Color) -> bool {
    let (between, king_path) = match us {
        Color::White => ([5, 6], [5, 6]),
        Color::Black => ([61, 62], [61, 62]),
    };

    // 1. Squares between king and rook must be empty
    if between.iter().any(|&sq| position.board_sq(sq) != EMPTY_PIECE) {
        return false;
    }

    // 2. Castling right must still be up
    if !position.kingside(us) {
        return false;
    }

    // 3. King may not move through or into check
    if king_path
        .iter()
        .any(|&sq| position.square_under_attack(sq, !us))
    {
        return false;
    }

    true
}


fn can_castle_queenside(position: &Position, us: Color) -> bool {
    let (between, king_path) = match us {
        Color::White => ([1, 2, 3], [2, 3]),
        Color::Black => ([57, 58, 59], [58, 59]),
    };

    // 1. Squares between king and rook must be empty
    if between.iter().any(|&sq| position.board_sq(sq) != EMPTY_PIECE) {
        return false;
    }

    // 2. Castling right must still be up
    if !position.queenside(us) {
        return false;
    }

    // 3. King may not move through or into check
    if king_path
        .iter()
        .any(|&sq| position.square_under_attack(sq, !us))
    {
        return false;
    }

    true
}



pub fn king_attacks(position: &Position, color: Color) -> u64 {
    KING_MOVES[position.king_square(color) as usize]
}