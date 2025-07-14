use crate::attacks::tables::{KNIGHT_MOVES};
use crate::bitboards;
use crate::color::Color;
use crate::mov::{Move, MoveFlags, MoveKind, MoveList};
use crate::piece::Piece;
use crate::position::Position;

pub (in crate::attacks) fn knight_moves(position: &Position, allies: u64, enemies: u64, us: Color, moves: &mut MoveList) {

    let (squares, count) = position.piece_list(Piece::Knight, us);
    for &from in &squares[..count] {
        let all = KNIGHT_MOVES[from as usize] & !allies;
        let quiet = all & !enemies;
        let capture = all & enemies;

        bitboards::pop_lsb(quiet, |to| {
            moves.push(Move::encode(from, to, MoveFlags::new(MoveKind::Quiet)));
        });

        bitboards::pop_lsb(capture, |to| {
            moves.push(Move::encode(from, to, MoveFlags::new(MoveKind::Capture)));
        });
    }
}