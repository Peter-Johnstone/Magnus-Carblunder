use crate::color::Color;
use crate::piece::Piece;
use crate::position::Position;

pub(crate) fn evaluate(pos: &Position) -> i16 {
    if pos.is_repeat_towards_three_fold_repetition() || pos.half_move() >= 100 {
        return 0;
    }
    let pc = |p, c| pos.piece_count(p, c);

         (1 * pc(Piece::Pawn,     Color::White)
        + 3 * pc(Piece::Knight,   Color::White)
        + 3 * pc(Piece::Bishop,   Color::White)
        + 5 * pc(Piece::Rook,     Color::White)
        + 9 * pc(Piece::Queen,    Color::White)
        - 1 * pc(Piece::Pawn,     Color::Black)
        - 3 * pc(Piece::Knight,   Color::Black)
        - 3 * pc(Piece::Bishop,   Color::Black)
        - 5 * pc(Piece::Rook,     Color::Black)
        - 9 * pc(Piece::Queen,    Color::Black)) as i16
}