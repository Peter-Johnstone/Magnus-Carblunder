use crate::attacks::king::king_moves;
use crate::attacks::knight::knight_moves;
use crate::attacks::pawn::pawn_moves;
use crate::attacks::sliding::{bishop_moves, queen_moves, rook_moves};
use crate::mov::MoveList;
use crate::piece::Piece;
use crate::color::Color;
use crate::position::Position;

pub fn all_moves(position: &Position) -> MoveList {
    let us: Color = position.turn();
    let allies: u64 = position.occupancy(us);
    let enemies: u64 = position.occupancy(!us);

    let mut moves = MoveList::new();
    pawn_moves(position, position.get_allied(Piece::Pawn), allies, enemies, us, &mut moves);
    king_moves(position, allies, enemies, us, &mut moves);
    knight_moves(position, allies, enemies, us, &mut moves);
    bishop_moves(position, allies, enemies, us, &mut moves);
    rook_moves(position, allies, enemies, us, &mut moves);
    queen_moves(position, allies, enemies, us, &mut moves);
    moves
}


