use std::time::{Instant};
use crate::attacks::movegen::all_moves;
use crate::engines::engine_manager::Ctx;
use crate::mov::Move;
use crate::position::{Position, Status};

const MATE_SCORE: i16 = 10_000;

pub(crate) fn negamax(pos: &mut Position, depth: u8, a: i16, b: i16, color: i16, deadline: Instant, ctx: &mut Ctx) -> Option<(i16, Move)> {
    ctx.nodes += 1;


    if Instant::now() >= deadline {
        return None;
    }
    if depth == 0 {
        return Some((color * (ctx.eval_fn)(pos), Move::null()));
    }

    let moves = all_moves(pos);
    if moves.is_empty() {
        let s = match pos.get_game_result() {
            Status::Checkmate(_) => -color * (MATE_SCORE + depth as i16),
            _                    => 0,
        };
        return Some((color * s, Move::null()));
    }

    let mut best_score = i16::MIN;
    let mut best_move  = Move::null();

    for m in moves.iter() {
        pos.do_move(m);

        // recurse from opponent’s view
        let reply = negamax(pos, depth - 1, a, b, -color, deadline, ctx);
        pos.undo_move();

        match reply {
            None => return None,
            Some((child_score, _)) => {
                let score = -child_score;
                if score > best_score {
                    best_score = score;
                    best_move  = m;
                }
            }
        }
    }
    Some((best_score, best_move))
}
