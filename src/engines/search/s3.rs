use std::time::{Instant};
use crate::attacks::movegen::all_moves;
use crate::engines::engine_manager::Ctx;
use crate::mov::Move;
use crate::position::{Position, Status};

const MATE_SCORE: i16 = 10_000;

/// α‑β NegaMax that aborts cleanly when `deadline` is hit.
///
/// Returns **`None`** instead of a bogus score when time is up.       ★
pub(crate) fn negamax(pos: &mut Position, depth: u8, mut a: i16, b: i16, color: i16, deadline: Instant, ctx: &mut Ctx) -> Option<(i16, Move)> {

    ctx.nodes += 1;

    if Instant::now() >= deadline { return None; }

    // base‑case: leaf
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

    let mut best_move = Move::null();

    for m in moves.iter() {
        pos.do_move(m);
        let reply = negamax(pos, depth - 1, -b, -a, -color, deadline, ctx);
        pos.undo_move();

        match reply {                                                  // ★
            None => return None,                                       // propagate timeout
            Some((child, _)) => {
                let score = -child;
                if score > a {
                    a     = score;
                    best_move = m;
                }
                if a >= b { break; }
            }
        }
    }

    Some((a, best_move))
}
