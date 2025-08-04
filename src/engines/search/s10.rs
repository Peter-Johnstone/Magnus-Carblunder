use std::time::Instant;
use crate::attacks::movegen::all_moves;
use crate::engines::constants::MAX_DEPTH;
use crate::engines::engine_manager::Ctx;
use crate::engines::transposition_table::Bound;
use crate::mov::{Move, MoveList, MAX_MOVES};
use crate::piece::Piece;
use crate::position::{Position, Status};



pub const PIECE_VALUE: [i16; 6] = [100, 320, 330, 500, 900, 0];

#[inline(always)]
fn mvv_lva_score(victim: Piece, attacker: Piece) -> i16 {
    // 10× victim – attacker   (both taken from your PIECE_VALUE table)
    10 * PIECE_VALUE[victim as usize] - PIECE_VALUE[attacker as usize]
}



/// Give every move a numeric priority.
/// Bigger score ⇒ searched earlier.
#[inline(always)]
fn score_move(pos: &Position, mv: Move, pv: Move, hash: Move) -> i32 {
    // ❶  PV move always first
    if mv == pv { return 1_000_000; }

    // ❷  TT best move just after PV
    if mv == hash { return 999_999; }

    // ❸  Winning captures (MVV-LVA)
    if mv.is_capture() {
        let victim = pos.piece_at_sq(mv.to());          // you have this helper
        let aggressor = pos.piece_at_sq(mv.from());
        return  mvv_lva_score(victim, aggressor) as i32;
    }

    // ❹  Quiet promotions (e.g. e8=Q with nothing taken)
    if mv.is_promotion() { return 80_000; }

    // ❺  All other quiets
    0
}


fn order_moves(pos: &Position,
               moves: &mut MoveList,
               pv_move: Move,
               hash_move: Move)
{
    // 1.  Pre-compute scores once
    let mut scores: [i32; MAX_MOVES] = [0; MAX_MOVES];
    for i in 0..moves.len {
        scores[i] = score_move(pos, moves.moves[i], pv_move, hash_move);
    }

    // 2.  Insertion sort on the two parallel arrays
    for i in 1..moves.len {
        let mut j = i;
        let key_mv   = moves.moves[i];
        let key_sc   = scores[i];

        while j > 0 && scores[j - 1] < key_sc {
            moves.moves[j] = moves.moves[j - 1];
            scores[j]      = scores[j - 1];
            j -= 1;
        }
        moves.moves[j] = key_mv;
        scores[j]      = key_sc;
    }
}






pub(crate) fn quiescence(
    pos:      &mut Position,
    mut alpha: i16,
    beta:      i16,
    color:     i16,
    deadline:  Instant,
    ctx:       &mut Ctx,
) -> Option<(i16, Move)>
{
    /* ---- 0.  time check --------------------------------------- */
    if Instant::now() >= deadline { return None; }
    ctx.nodes += 1;

    /* ---- 1.  TT probe ----------------------------------------- */
    if let Some(e) = ctx.tt.probe(pos.zobrist()) {
        // depth == 0 entries are quiescence results
        match e.bound {
            Bound::Exact          => return Some((e.score, e.mv)),
            Bound::Lower if e.score >= beta  => return Some((e.score, e.mv)),
            Bound::Upper if e.score <= alpha => return Some((e.score, e.mv)),
            _ => {} // fall through
        }
    }

    /* ---- 2.  stand-pat ---------------------------------------- */
    let stand_pat = color * pos.evaluate();
    if stand_pat >= beta {
        ctx.tt.store(pos.zobrist(), 0, Bound::Lower, stand_pat, Move::null(), ctx.generation);
        return Some((beta, Move::null()));          // fail-high
    }
    if stand_pat > alpha { alpha = stand_pat; }

    let in_check = pos.in_check();

    let moves = all_moves(pos);
    let mut noisy = MoveList::new();

    if in_check {
        // When in check we must search *all* evasions.
        noisy = moves;          // cheap copy; keeps original ordering
    } else {
        for m in moves.iter() {
            if m.is_capture() || m.is_promotion() {   // just “noisy” stuff
                noisy.push(m);
            }
        }
    }

    if noisy.is_empty() {
        ctx.tt.store(pos.zobrist(), 0, Bound::Exact, stand_pat, Move::null(), ctx.generation);
        return Some((stand_pat, Move::null()));
    }

    /* ---- 4.  MVV-LVA ordering -------------------------------- */
    order_moves(pos, &mut noisy, Move::null(), Move::null());

    /* ---- 5.  search loop ------------------------------------- */
    let mut best_move = Move::null();

    for m in noisy.iter() {
        pos.do_move(m);
        let child = quiescence(pos, -beta, -alpha, -color, deadline, ctx);
        pos.undo_move();

        let score = match child {
            None          => return None,          // time-out bubbled
            Some((sc, _)) => -sc,
        };

        if score >= beta {
            ctx.tt.store(pos.zobrist(), 0, Bound::Lower, score, m, ctx.generation);
            return Some((beta, m));               // fail-high
        }
        if score > alpha {
            alpha     = score;
            best_move = m;
        }
    }

    /* ---- 6.  store in TT ------------------------------------- */
    let bound = if best_move.is_null() {
        Bound::Exact              // stand-pat is the best
    } else {
        Bound::Upper              // searched moves but didn’t reach beta
    };
    ctx.tt.store(pos.zobrist(), 0, bound, alpha, best_move, ctx.generation);

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
            let r = if depth > 6 { 3 } else { 2 };  // dynamic reduction
            pos.do_null_move();

            ctx.ply += 1;
            let child = negamax(
                pos,
                depth - 1 - r,     // depth reduction
                -beta,
                -beta + 1,         // “zero-window” re-search
                -color,
                deadline,
                ctx,
            );
            ctx.ply -= 1;
            pos.undo_null_move();

            let score = match child {
                Some((s, _)) => -s,
                None         => return None,
            };

            if score >= beta {
                return Some((beta, Move::null()));   // fail-high — prune
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



    /* -- 3a. PV move to front -------------------------------------- */
    order_moves(pos, &mut moves, ctx.pv_array[ctx.pv_index], hash_move);


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
