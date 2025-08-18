use crate::attacks::movegen::all_moves;
use crate::engines::constants::MAX_DEPTH;
use crate::engines::engine_manager::Ctx;
use crate::engines::transposition_table::Bound;
use crate::mov::{MAX_MOVES, Move, MoveList};
use crate::position::{Position, Status};
use std::time::Instant;

pub const MVV_LVA: [[u8; 6]; 6] = [
    [15, 14, 13, 12, 11, 10],
    [25, 24, 23, 22, 21, 20],
    [35, 34, 33, 32, 31, 30],
    [45, 44, 43, 42, 41, 40],
    [55, 54, 53, 52, 51, 50],
    [0,  0,  0,  0,  0,  0 ],
];

#[inline(always)]
fn score_move(
    pos: &Position,
    ctx: &mut Ctx,
    mv: Move,
    pv: Move,
    hash: Move,
) -> u8 {
    // ⓵ PV & TT
    if mv == pv   { return 255; }
    if mv == hash { return 254; }

    // ⓶ Captures first
    if mv.is_capture() {
        let victim    = pos.piece_at_sq(mv.to());
        let aggressor = pos.piece_at_sq(mv.from());
        let base      = MVV_LVA[victim as usize][aggressor as usize]; // 10-55

        // capture-promotion outranks everything except PV/TT
        if mv.is_promotion() {
            return (190u8 + base).min(253);   // clamp below TT
        }

        // recapture bonus (only if *not* a promotion)
        if let Some(last) = pos.last_move() {
            if last.to() == mv.to() {
                return 190u8 + base;          // 200-245
            }
        }
        return base;                           // ordinary capture
    }

    // ⓷ Quiet promotion (rare, but still strong)
    if mv.is_promotion() { return 190; }

    // ⓸ Killer quiets (before history)
    let killers = &ctx.killers[ctx.ply as usize];
    if mv == killers[0] {
        //ctx.stats.killer_hit0 += 1;
        return 189;
    }
    if mv == killers[1] {
        //ctx.stats.killer_hit1 += 1;
        return 188;
    }


    // ⓹ Quiet: history heuristic
    let h = ctx.history.index(mv, pos.side_to_move());
    (h >> 4).min(180) as u8                 // 0-180
}





#[inline(always)]
fn score_all(pos: &Position, ctx: &mut Ctx, moves: &MoveList, pv: Move, hash: Move, scores: &mut [u8; MAX_MOVES]) {
    for i in 0..moves.len {
        scores[i] = score_move(pos, ctx, moves.moves[i], pv, hash);
    }
}


#[inline(always)]
fn select_next(moves: &mut MoveList, scores: &mut [u8; MAX_MOVES], start: usize) {
    let len = moves.len;
    let mut best = start;
    let mut best_sc = scores[start];
    let mut i = start + 1;
    while i < len {
        let sc = scores[i];
        if sc > best_sc {
            best_sc = sc;
            best = i;
        }
        i += 1;
    }
    if best != start {
        moves.moves.swap(start, best);
        scores.swap(start, best);
    }
}


pub(crate) fn quiescence(
    pos: &mut Position,
    mut alpha: i16,
    beta: i16,
    color: i16,
    deadline: Instant,
    ctx: &mut Ctx,
) -> Option<(i16, Move)> {
    /* ---- 0.  time check --------------------------------------- */
    if Instant::now() >= deadline {
        return None;
    }

    let mut qhash = Move::null();

    /* ---- 1.  TT probe ----------------------------------------- */
    if let Some(e) = ctx.tt.probe(pos.zobrist()) {
        // depth == 0 entries are quiescence results
        qhash = e.mv;
        match e.bound {
            Bound::Exact => return Some((e.score, e.mv)),
            Bound::Lower if e.score >= beta => return Some((e.score, e.mv)),
            Bound::Upper if e.score <= alpha => return Some((e.score, e.mv)),
            _ => {} // fall through
        }
    }

    /* ---- 2.  stand-pat ---------------------------------------- */
    let stand_pat = color * pos.evaluate();
    if stand_pat >= beta {
        ctx.tt.store(
            pos.zobrist(),
            0,
            Bound::Lower,
            stand_pat,
            Move::null(),
            ctx.generation,
        );
        return Some((beta, Move::null())); // fail-high
    }
    if stand_pat > alpha {
        alpha = stand_pat;
    }

    /* ---- 3.  generate noisy moves ----------------------------- */
    let moves = all_moves(pos);
    let mut noisy = MoveList::new();
    for m in moves.iter() {
        if m.is_capture() || m.is_promotion() {
            noisy.push(m);
        }
    }
    if noisy.is_empty() {
        ctx.tt.store(
            pos.zobrist(),
            0,
            Bound::Exact,
            stand_pat,
            Move::null(),
            ctx.generation,
        );
        return Some((stand_pat, Move::null()));
    }

    /* ---- 4.  MVV-LVA ordering -------------------------------- */

    let mut best_move = Move::null();

    let mut scores = [0u8; MAX_MOVES];

    score_all(pos, ctx, &noisy, Move::null(), qhash, &mut scores);
    for i in 0..noisy.len {
        select_next(&mut noisy, &mut scores, i);
        let m = noisy.moves[i];

        pos.do_move(m);
        let child = quiescence(pos, -beta, -alpha, -color, deadline, ctx);
        pos.undo_move();

        let score = match child {
            None => return None, // time-out bubbled
            Some((sc, _)) => -sc,
        };

        if score >= beta {
            ctx.tt.store(pos.zobrist(), 0, Bound::Lower, score, m, ctx.generation);
            return Some((beta, m)); // fail-high
        }
        if score > alpha {
            alpha = score;
            best_move = m;
        }
    }

    /* ---- 6.  store in TT ------------------------------------- */
    let bound = if best_move.is_null() {
        Bound::Exact // stand-pat is the best
    } else {
        Bound::Upper // searched moves but didn’t reach beta
    };
    ctx.tt
        .store(pos.zobrist(), 0, bound, alpha, best_move, ctx.generation);

    Some((alpha, best_move))
}

const MATE_SCORE: i16 = 10_000; // any value ≫ evaluate range

pub(crate) fn negamax(
    pos: &mut Position,
    depth: u8,
    mut alpha: i16,
    beta: i16,
    color: i16,
    deadline: Instant,
    ctx: &mut Ctx,
) -> Option<(i16, Move)> {
    ctx.nodes += 1;
    /* ----- 0. abort if out of time -------------------------------- */
    if Instant::now() >= deadline {
        return None; // bubble up timeout
    }

    // Only treat draw states *inside* the tree, not at root
    if ctx.ply > 0 {
        if pos.half_move() >= 100 || pos.is_repeat_towards_three_fold_repetition() {
            return Some((0, Move::null()));
        }

        // ─── Null-move pruning ─────────────────────────────────────────────
        if depth >= 3 && !pos.in_check() {
            let r = if depth > 6 { 3 } else { 2 }; // dynamic reduction
            pos.do_null_move();

            ctx.ply += 1;
            let child = negamax(
                pos,
                depth - 1 - r, // depth reduction
                -beta,
                -beta + 1, // “zero-window” re-search
                -color,
                deadline,
                ctx,
            );
            ctx.ply -= 1;
            pos.undo_null_move();

            let score = match child {
                Some((s, _)) => -s,
                None => return None,
            };

            if score >= beta {
                // caused cutoff
                return Some((beta, Move::null()));
            }
        }
    }

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
        return quiescence(pos, alpha, beta, color, deadline, ctx);
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


    let mut scores = [0u8; MAX_MOVES];
    score_all(pos, ctx, &moves, ctx.pv_array[ctx.pv_index], hash_move, &mut scores);


    ctx.pv_array[ctx.pv_index] = Move::null();
    let pv_next_index = ctx.pv_index + (MAX_DEPTH - ctx.ply) as usize;

    // search loop (index-based)
    let mut best_move = Move::null();
    let mut best_score = i16::MIN + 1;

    for i in 0..moves.len {
        select_next(&mut moves, &mut scores, i);
        let m = moves.moves[i];

        pos.do_move(m);

        // descend
        let parent_pv_index = ctx.pv_index;
        ctx.pv_index = pv_next_index;
        ctx.ply += 1;

        let child = negamax(pos, depth - 1, -beta, -alpha, -color, deadline, ctx);

        ctx.ply -= 1;
        ctx.pv_index = parent_pv_index;
        pos.undo_move();

        let child_score = match child {
            None => return None,
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

        // if alpha >= beta {
        //     let mut was_killer = false;
        //     if !m.is_capture() {
        //         ctx.stats.quiet_beta_cutoffs += 1;
        //
        //         // was this move already a killer?
        //         let k = &ctx.killers[ctx.ply as usize];
        //         was_killer = m == k[0] || m == k[1];
        //
        //         // update killers
        //         if depth >= 2 {
        //             let killers = &mut ctx.killers[ctx.ply as usize];
        //             if m != killers[0] {
        //                 killers[1] = killers[0];
        //                 killers[0] = m;
        //                 ctx.stats.killer_updates += 1;
        //             }
        //         }
        //     }
        //     if was_killer { ctx.stats.killer_cutoffs += 1; }
        //
        //     if !m.is_capture() && depth >= 2 {
        //         ctx.history.update_non_captures(m, pos.turn(), depth);
        //     }
        //     break;

        if alpha >= beta {
            if !m.is_capture() && depth >= 2 {
                ctx.history.update_non_captures(m, pos.side_to_move(), depth);
                let killers = &mut ctx.killers[ctx.ply as usize]; // parent ply (ctx.ply was decremented already)
                if m != killers[0] { killers[1] = killers[0]; killers[0] = m; }
            }
            break;
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
