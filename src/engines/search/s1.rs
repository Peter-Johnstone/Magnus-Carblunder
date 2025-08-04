use std::time::Instant;
use crate::attacks::movegen::all_moves;
use crate::engines::engine_manager::Ctx;
use crate::mov::Move;
use crate::position::Position;

pub fn pick(position: &mut Position, _: u8, _: i16, _: i16, _: i16, _: Instant, _: &mut Ctx) -> Option<(i16, Move)>
{
    // Random move. Brilliant.
    Some((0, all_moves(position).random()))
}



