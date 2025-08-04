use crate::tables::KING_MOVES;
use crate::color::Color;
use crate::mov::{Move, flag, MoveList};
use crate::position::Position;
use crate::bitboards::{pop_lsb};

pub (in crate::attacks) fn king_moves(position: &Position, allies: u64, enemies: u64, unsafe_squares: u64, us: Color, moves: &mut MoveList) {
    let sq: u8 = position.king_square(us);
    let to_bb: u64 = &KING_MOVES[sq as usize] & !allies & !unsafe_squares;
    let capture_bb: u64 = to_bb & enemies;
    let quiet_bb: u64 = to_bb & !capture_bb;


    pop_lsb(quiet_bb, |to| {moves.push(Move::encode(sq, to, flag::QUIET));});
    pop_lsb(capture_bb, |to| {moves.push(Move::encode(sq, to, flag::CAPTURE));});

    
    let in_check = (unsafe_squares & (1u64 << sq)) != 0;

    if !in_check {
        // check castling
        if can_castle_queenside(position, us, unsafe_squares) {
            moves.push(Move::encode(sq, sq - 2, flag::QUEEN_CASTLE));
        }

        if can_castle_kingside(position, us, unsafe_squares) {
            moves.push(Move::encode(sq, sq + 2, flag::KING_CASTLE));
        }
    }
}

const BETWEEN_KS_WHITE: u64 = 0x60;
const KINGPATH_KS_WHITE: u64 = 0x60;
const BETWEEN_KS_BLACK: u64 = 0x6000000000000000;
const KINGPATH_KS_BLACK: u64 = 0x6000000000000000;

const BETWEEN_QS_WHITE: u64 = 0xe;
const BETWEEN_QS_BLACK: u64 = 0xe00000000000000;
const KINGPATH_QS_WHITE: u64 = 0xc;
const KINGPATH_QS_BLACK: u64 = 0xc00000000000000;


#[inline(always)]
fn can_castle_kingside(pos: &Position, us: Color, unsafe_squares: u64) -> bool {
    let (between, path, right_ok) = match us {
        Color::White => (BETWEEN_KS_WHITE, KINGPATH_KS_WHITE, pos.kingside(us)),
        Color::Black => (BETWEEN_KS_BLACK, KINGPATH_KS_BLACK, pos.kingside(us)),
    };

    // 1) squares empty, 2) right still present, 3) king path safe
    pos.occupied() & between == 0 &&
        right_ok &&
        unsafe_squares & path == 0
}

#[inline(always)]
fn can_castle_queenside(pos: &Position, us: Color, unsafe_squares: u64) -> bool {
    let (between, path, right_ok) = match us {
        Color::White => (BETWEEN_QS_WHITE, KINGPATH_QS_WHITE, pos.queenside(us)),
        Color::Black => (BETWEEN_QS_BLACK, KINGPATH_QS_BLACK, pos.queenside(us)),
    };

    pos.occupied() & between == 0 &&
        right_ok &&
        unsafe_squares & path == 0
}





pub fn king_attacks(position: &Position, color: Color) -> u64 {
    KING_MOVES[position.king_square(color) as usize]
}