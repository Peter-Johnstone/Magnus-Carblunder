use std::time::Instant;
use crate::attacks::movegen::all_moves;
use crate::engines::constants::MAX_DEPTH;
use crate::engines::engine_manager::Ctx;
use crate::engines::transposition_table::Bound;
use crate::mov::Move;
use crate::position::{Position, Status};

/// Mate score base value (distance-to-mate is added/subtracted on top of it)
const MATE_SCORE: i16 = 10_000; // any value ≫ evaluate range

/// Alpha–beta Negamax **with Transposition Table**
///
/// * returns `None` as soon as the deadline is hit
/// * probes / stores in `ctx.tt`
/// * uses `ctx.eval_fn` for the static evaluation
pub(crate) fn negamax(
    pos: &mut Position,
    depth: u8,
    mut alpha: i16,
    beta: i16,
    color: i16,
    deadline: Instant,
    ctx: &mut Ctx,
) -> Option<(i16, Move)> {
    /* ----- 0. abort if out of time -------------------------------- */
    if Instant::now() >= deadline {
        return None; // bubble up timeout
    }

    ctx.nodes += 1;

    let orig_alpha = alpha;

    /* ----- 1. TT probe ------------------------------------------- */
    let mut hash_move = Move::null();

    if let Some(e) = ctx.tt.probe(pos.zobrist()) {
        hash_move = e.mv;
        let entry_ok = e.depth >= depth;

        if entry_ok {
            ctx.pv_array[ctx.pv_index] = e.mv;
            match e.bound {
                Bound::Exact if true => return Some((e.score, e.mv)),
                Bound::Lower if e.score >= beta => return Some((e.score, e.mv)),
                Bound::Upper if e.score <= alpha => return Some((e.score, e.mv)),
                _ => {}
            }
        }
    }

    /* ----- 2. leaf ------------------------------------------------ */
    if depth == 0 {
        let sc = color * (ctx.eval_fn)(pos);
        ctx.tt.store(
            pos.zobrist(),
            0,
            Bound::Exact,
            sc,
            Move::null(),
            ctx.generation,
        );
        return Some((sc, Move::null()));
    }

    /* ----- 3. generate moves & check terminal positions ----------- */
    let mut moves = all_moves(pos);
    if moves.is_empty() {
        let s = match pos.get_game_result() {
            Status::Checkmate(_) => -color * (MATE_SCORE + depth as i16),
            _ => 0,
        };
        return Some((color * s, Move::null()));
    }

    /* -- 3a. put hash move first ----------------------------------- */
    if !hash_move.is_null() {
        let idx_opt = moves.iter().position(|mv| mv == hash_move);

        // 2. borrow is over, we may now take a mutable one
        if let Some(i) = idx_opt {
            moves.swap(0, i);
        }
    }

    /* -- 3b. PV move to front -------------------------------------- */
    let pv_move = ctx.pv_array[ctx.pv_index];
    if !pv_move.is_null() {
        // 1. immutable borrow ends as soon as `idx_opt` is materialised
        let idx_opt = moves.iter().position(|mv| mv == pv_move);

        // 2. borrow is over, we may now take a mutable one
        if let Some(i) = idx_opt {
            moves.swap(0, i);
        }
    }

    /* ----- 4. prepare PV bookkeeping ----------------------------- */
    ctx.pv_array[ctx.pv_index] = Move::null();
    let pv_next_index = ctx.pv_index + (MAX_DEPTH - ctx.ply) as usize;

    /* ----- 5. search loop ---------------------------------------- */
    let mut best_move = Move::null();
    let mut best_score = i16::MIN + 1;

    for m in moves.iter() {
        pos.do_move(m);

        // descend
        let parent_pv_index = ctx.pv_index;
        ctx.pv_index = pv_next_index;
        ctx.ply += 1;

        let child = negamax(
            pos,
            depth - 1,
            -beta,
            -alpha,
            -color,
            deadline,
            ctx,
        );

        ctx.ply -= 1;
        ctx.pv_index = parent_pv_index;
        pos.undo_move();

        let child_score = match child {
            None => return None, // timeout bubbled up
            Some((sc, _)) => sc,
        };
        let score = -child_score;


        if score > best_score {
            best_score = score;
            best_move = m;
        }

        if score > alpha {
            alpha = score;
            ctx.pv_array[ctx.pv_index] = m;

            // copy child PV one cell downwards
            let mut i = 0;
            while {
                let mv_from_child = ctx.pv_array[pv_next_index + i];
                ctx.pv_array[ctx.pv_index + 1 + i] = mv_from_child;
                !mv_from_child.is_null()
            } {
                i += 1;
            }
        }

        if alpha >= beta {
            break; // beta cut-off
        }
    }

    /* ----- 6. store in TT ---------------------------------------- */
    let returned = alpha; // value we are going to return
    let bound = if returned <= orig_alpha {
        Bound::Upper
    } else if returned >= beta {
        Bound::Lower
    } else {
        Bound::Exact
    };

    ctx.tt.store(
        pos.zobrist(),
        depth,
        bound,
        returned,
        best_move,
        ctx.generation,
    );

    Some((returned, best_move))
}
