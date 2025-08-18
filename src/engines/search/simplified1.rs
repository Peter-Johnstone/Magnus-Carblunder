use std::time::Instant;
use crate::attacks::movegen::all_moves;
use crate::engines::engine_manager::Ctx;
use crate::mov::{Move, MoveList};
use crate::position::{Position};



// Basic Minimax
pub fn minimax(pos: &mut Position,
               depth: u8,
               a: i16,
               b: i16,
               _: i16,
               deadline: Instant,
               ctx: &mut Ctx)

            -> Option<(i16, Move)>
{

    if depth == 0 {
        // leaf node
        return Some((pos.evaluate(), Move::null()))
    }

    if Instant::now() >= deadline {
        // Ran out of time. End early.
        return None;
    }

    // generate all the moves
    let mut mvs: MoveList = all_moves(pos);

    if mvs.is_empty() {
        // terminating node. Either checkmate or stalemate
        return Some((pos.game_result_eval(depth), Move::null()))
    }


    let mut top: i16;
    let mut top_mv: Move = Move::null();

    if pos.side_to_move().is_white()

    {

        // Maximizing player
        top = i16::MIN;

        for mv in mvs.iter() {
            pos.do_move(mv);
            let reply = minimax(pos, depth - 1, a, b, 0, deadline, ctx);
            pos.undo_move();

            match reply {
                None => return None, // Hard-deadline on time reached.
                Some((child_eval, _)) => {
                    if child_eval >= top {
                        top = child_eval;
                        top_mv = mv;
                    }
                }
            }

        }

        Some((top, top_mv))
    } else {

        // Minimizing player
        top = i16::MAX;

        for mv in mvs.iter() {
            pos.do_move(mv);
            let reply = minimax(pos, depth - 1, a, b, 0, deadline, ctx);
            pos.undo_move();

            match reply {
                None => return None, // Hard-deadline on time reached.
                Some((child_eval, _)) => {
                    if child_eval <= top {
                        top = child_eval;
                        top_mv = mv;
                    }
                }
            }

        }


        Some((top, top_mv))
    }
}