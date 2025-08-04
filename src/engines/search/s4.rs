use std::time::{Instant};
use crate::attacks::movegen::all_moves;
use crate::engines::engine_manager::Ctx;
use crate::engines::transposition_table::{Bound};
use crate::mov::Move;
use crate::position::{Position, Status};

/// Pure material values, index = `Piece as usize`



const MATE_SCORE: i16 = 10_000;   // any value ≫ evaluate range


/// alpha‑beta Negamax **with TT** that fits the new `SearchFn` signature.
///
/// • returns `None` as soon as the deadline is hit
/// • uses `ctx.tt` for probing / storing
/// • uses `ctx.eval_fn` for the static evaluation
pub(crate) fn negamax(pos: &mut Position, depth: u8, mut alpha: i16, beta: i16, color: i16, deadline: Instant, ctx: &mut Ctx,) -> Option<(i16, Move)> {

    /* ----- 0. abort if out of time ------------------------------------ */
    if Instant::now() >= deadline {
        return None;                     // bubble up timeout
    }

    ctx.nodes += 1;

    let orig_alpha = alpha;

    ctx.tt_probes += 1;

    /* ----- 1. TT probe ------------------------------------------------- */
    let mut entry_ok  = false;          // is depth high enough for pruning?
    let mut entry     = None;

    if let Some(e) = ctx.tt.probe(pos.zobrist()) {
        ctx.tt_hits += 1;
        entry_ok   = e.depth >= depth;  // evaluate later
        entry      = Some(e);
    }

    /* pruning only if depth sufficient */
    if depth > 1 {
        if let Some(e) = entry {
            if entry_ok {
                match e.bound {
                    Bound::Exact                       => return Some((e.score, e.mv)),
                    Bound::Lower if e.score >= beta    => return Some((e.score, e.mv)),
                    Bound::Upper if e.score <= alpha   => return Some((e.score, e.mv)),
                    _ => {}
                }
            }
        }
    }

    /* ----- 2. leaf ----------------------------------------------------- */
    if depth == 0 {
        let sc = color * (ctx.eval_fn)(pos);
        return Some((sc, Move::null()));
    }

    /* ----- 3. generate moves & check terminal positions --------------- */
    let moves = all_moves(pos);
    if moves.is_empty() {
        let s = match pos.get_game_result() {
            Status::Checkmate(_) => -color * (MATE_SCORE + depth as i16),
            _                    => 0,
        };
        return Some((color * s, Move::null()));
    }

    /* ----- 4. search loop --------------------------------------------- */
    let mut best      = -MATE_SCORE;
    let mut best_move = Move::null();

    for m in moves.iter() {
        pos.do_move(m);
        let child = negamax(pos, depth - 1, -beta, -alpha,
                            -color, deadline, ctx);
        pos.undo_move();

        // propagate timeout as‑is
        let child_score = match child {
            None            => return None,
            Some((sc, _))   => sc,
        };

        let score = -child_score;

        if score > best { best = score; }
        if score > alpha {
            alpha    = score;
            best_move = m;
        }
        if alpha >= beta { break; }
    }


    /* ----- 5. store in TT --------------------------------------------- */
    let bound = if best <= orig_alpha { Bound::Upper } else if best >= beta { Bound::Lower } else { Bound::Exact };

    ctx.tt.store(pos.zobrist(), depth, bound, alpha, best_move, ctx.generation);


    Some((best, best_move))
}