use std::time::Instant;
use crate::attacks::movegen::all_moves;
use crate::engines::constants::MAX_DEPTH;
use crate::engines::engine_manager::Ctx;
use crate::engines::transposition_table::Bound;
use crate::mov::{Move, MoveList, MAX_MOVES};
use crate::piece::Piece::{Pawn, Queen};
use crate::position::{Position, };

pub const MVV_LVA: [[u16; 6]; 6] =
    [
        [10150,    14,    13,    12,    11, 15010], // victim = pawn
        [15250, 10240, 10005,    22,    21, 15020], // victim = knight
        [15350, 11340, 10330,    32,    31, 15030], // victim = bishop
        [15450, 15440, 15430, 10420,    41, 15040], // victim = rook
        [15550, 15540, 15530, 15520, 10510, 15050], // victim = queen
        [30000, 30000, 30000, 30000, 30000, 30000], // victim = king
    ];



struct MovePicker {
    moves: MoveList,
    head: usize,
    prio_end: usize, // index where the priority moves end (pv and tt)
    scores: [u16; MAX_MOVES],
    is_quiescence: bool,
}

impl MovePicker {
    #[inline]
    fn new(pv_mv: Move, tt_mv: Move, mut mvs: MoveList) -> MovePicker {

        let mut i = 0;

        // put pv move at the front of the list
        if !pv_mv.is_null() && let Some(pv_mv_i) = mvs.index_opt(pv_mv) {
            mvs.swap(pv_mv_i, i);
            i += 1;
        }

        if !tt_mv.is_null() && pv_mv != tt_mv {
            // put tt move at the front of the list behind pv move (if it exists)
            if let Some(tt_mv_i) = mvs.index_opt(tt_mv) {
                mvs.swap(tt_mv_i, i);
                i += 1;
            }
        }

        MovePicker
        {
            moves: mvs,
            head: 0,
            prio_end: i,
            scores: [0; MAX_MOVES],
            is_quiescence: false,
        }
    }

    fn is_quiescence_move(pos: &Position, mv: Move) -> bool {
        (   (mv.is_capture() && pos.see(mv))||
            mv.is_promotion() ||
            mv.is_en_passant())


            && !mv.is_null()
    }

    fn quiescence_mvs(pos: &Position, mvs: &MoveList) -> MoveList {
        let mut quiescence_mvs = MoveList::new();
        for mv in mvs.iter() {
            if Self::is_quiescence_move(pos, mv) {
                quiescence_mvs.push(mv);
            }
        }
        quiescence_mvs
    }

    #[inline]
    /// mvs must already be quiescence form
    fn new_quiescence(pos: &Position, tt_mv: Move, mut mvs: MoveList) -> MovePicker {
        let mut i = 0;

        if Self::is_quiescence_move(pos, tt_mv) {
            // put tt move at the front of the list behind pv move (if it exists)
            let tt_mv_i = mvs.index(tt_mv);
            mvs.swap(tt_mv_i, i);
            i += 1;
        }

        MovePicker
        {
            moves: mvs,
            head: 0,
            prio_end: i,
            scores: [0; MAX_MOVES],
            is_quiescence: true,
        }
    }

    #[inline(always)]
    fn score_move(&self, pos: &Position, ctx: &Ctx, mv: Move) -> u16 {
        let mut score = 0;
        if mv.is_capture() {
            // apply MVV_LVA
            let victim    = pos.piece_at_sq(mv.to());
            let aggressor = pos.piece_at_sq(mv.from());
            score += MVV_LVA[victim as usize][aggressor as usize];
            if pos.see(mv) {score += 15_000};

        }
        if mv.is_promotion() {
            if mv.promotion_piece() == Queen {
                score += 20_000;
            } else {
                // Promotions to pieces other than queens are overwhelmingly unlikely
                return 0;
            }
        }
        if self.is_quiescence {
            return score;
        }
        let killers = ctx.killers[ctx.ply as usize];
        if mv == killers[0] {
            score += 10002;
        } else if mv == killers[1] {
            score += 10001;
        } else if mv.is_castling() {
            score += 1000;
        } else if mv.is_en_passant() {
            score += 10_200;
        } else if mv.is_quiet() || mv.is_double_push() {
            let h = ctx.history.index(mv, pos.side_to_move());
            return h.min(10_000)
        }
        score
    }


    #[inline]
    fn score_moves(&self, pos: &Position, ctx: &Ctx, mvs: &MoveList) -> [u16; MAX_MOVES] {
        let mut scores = [0; MAX_MOVES];

        // No need to calculate scores for pv/tt mvs that come first anyway
        for (i, mv) in mvs.iter().enumerate() {
            if i < self.prio_end {
                continue;
            }
            scores[i] = self.score_move(pos, ctx, mv); // keep tail relative
        }

        scores
    }

    #[inline]
    fn next(&mut self, ctx: &Ctx, pos: &Position) -> Move {
        if self.head >= self.moves.len {
            return Move::null();
        } else if self.head < self.prio_end {
            // We still need to process either pv_mv or tt_mv
            let mv = self.moves.get(self.head);
            self.head += 1;
            return mv
        } else if self.head == self.prio_end {
            // We need to score the moves bc neither hash mv nor pv mv caused a cutoff
            self.scores = self.score_moves(pos, ctx, &self.moves);
        }

        // regular moves
        let mut top = 0;
        let mut top_i = self.head;
        let mut top_mv = self.moves.get(self.head);

        // moves to consider: between start and moves.len
        for i in self.head..self.moves.len {

            let mv_score = self.scores[i];
            if mv_score > top {
                // new top scorer found.
                top = mv_score;
                top_i = i;
                top_mv = self.moves.get(i);
            }
        }

        // to make sure our MoveList is ordered by already picked moves, we need to swap.
        if top_i != self.head {
            self.moves.swap(top_i, self.head);
            self.scores.swap(top_i, self.head);
        }

        self.head += 1;
        top_mv
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
    if Instant::now() >= deadline {
        return None;
    }

    let mut hash_mv = Move::null();

    if let Some(e) = ctx.tt.probe(pos.zobrist()) {
        hash_mv = e.mv;
        match e.bound {
            Bound::Exact => return Some((e.score, e.mv)),
            Bound::Lower if e.score >= beta => return Some((e.score, e.mv)),
            Bound::Upper if e.score <= alpha => return Some((e.score, e.mv)),
            _ => {}
        }
    }

    let orig_alpha = alpha;

    let stand_pat = color * pos.evaluate_2();
    if stand_pat >= beta {
        ctx.tt.store(
            pos.zobrist(),
            0,
            Bound::Lower,
            stand_pat,
            Move::null(),
            ctx.generation,
        );
        // FAIL-SOFT: return the true score (not β)
        return Some((stand_pat, Move::null()));
    }
    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let all_mvs = all_moves(pos);
    if all_mvs.is_empty() {
        return Some((color * pos.game_result_eval((MAX_DEPTH - ctx.ply) as u8), Move::null()));
    }

    let mvs = MovePicker::quiescence_mvs(pos, &all_mvs);
    let num_mvs = mvs.len;
    if mvs.is_empty() {
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
    let mut mv_picker = MovePicker::new_quiescence(pos, hash_mv, mvs);

    let mut best_move = Move::null();

    for _ in 0..num_mvs {
        let mv = mv_picker.next(ctx, pos);

        pos.do_move(mv);
        ctx.ply += 1;
        let child = quiescence(pos, -beta, -alpha, -color, deadline, ctx);
        ctx.ply -= 1;
        pos.undo_move();

        let score = match child {
            None => return Some((alpha, best_move)),
            Some((sc, _)) => -sc,
        };

        if score >= beta {
            ctx.tt.store(pos.zobrist(), 0, Bound::Lower, score, mv, ctx.generation);
            // FAIL-SOFT: return the true score (not β)
            return Some((score, mv));
        }
        if score > alpha {
            alpha = score;
            best_move = mv;
        }
    }

    // With fail-soft, final bound depends on whether we improved over the original α
    let bound = if alpha <= orig_alpha { Bound::Upper } else { Bound::Exact };
    ctx.tt.store(pos.zobrist(), 0, bound, alpha, best_move, ctx.generation);

    Some((alpha, best_move))
}

pub(crate) fn negamax(
    pos: &mut Position,
    depth: u8,
    mut alpha: i16,
    beta: i16,
    color: i16,
    deadline: Instant,
    ctx: &mut Ctx,
) -> Option<(i16, Move)> {
    if (ctx.ply) >= MAX_DEPTH {
        return Some((color * pos.evaluate_2(), Move::null()));
    }

    if depth == 0 {
        return quiescence(pos, alpha, beta, color, deadline, ctx);
    }

    if ctx.ply > 0 && (pos.half_move() >= 100 || pos.is_repeat_towards_three_fold_repetition()) {
        return Some((0, Move::null()));
    }

    if ctx.ply > 0 {
        if pos.half_move() >= 100 || pos.is_repeat_towards_three_fold_repetition() {
            return Some((0, Move::null()));
        }

        if depth >= 3 && !pos.in_check() {
            pos.do_null_move();
            ctx.ply += 1;
            let child = negamax(
                pos,
                reduction(depth),
                -beta,
                -beta + 1,
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
                // FAIL-SOFT here: return the true score, not β
                return Some((score, Move::null()));
            }
        }
    }
    if Instant::now() >= deadline {
        return None;
    }

    let orig_alpha = alpha;

    let mut hash_mv = Move::null();

    if let Some(e) = ctx.tt.probe(pos.zobrist()) {
        hash_mv = e.mv;
        let entry_ok = e.depth >= depth;

        if entry_ok {
            match e.bound {
                Bound::Exact if true => return Some((e.score, e.mv)),
                Bound::Lower if e.score >= beta => return Some((e.score, e.mv)),
                Bound::Upper if e.score <= alpha => return Some((e.score, e.mv)),
                _ => {}
            }
        }
    }

    let mvs = all_moves(pos);
    if mvs.is_empty() {
        return Some((color * pos.game_result_eval(depth), Move::null()));
    }
    let num_mvs = mvs.len;

    let pv_mv = ctx.pv.mv(ctx.ply);
    let mut mv_picker = MovePicker::new(pv_mv, hash_mv, mvs);
    ctx.pv.clear_node(ctx.ply);

    let mut best_move = Move::null();
    let mut best_score = i16::MIN + 1;
    let is_root = ctx.ply == 0;

    for i in 0..num_mvs {
        let mv = mv_picker.next(&ctx, pos);
        pos.do_move(mv);
        ctx.ply += 1;

        let new_depth = depth -1 + extension(pos, mv);
        let child = if i == 0 {
            negamax(pos, new_depth, -beta, -alpha, -color, deadline, ctx)
        } else {
            let do_pvs =
                depth >= 3 &&
                    alpha.saturating_add(1) < beta &&
                    !(mv.is_capture() || mv.is_promotion());

            if !do_pvs {
                negamax(pos, new_depth, -beta, -alpha, -color, deadline, ctx)
            } else {
                let a1 = alpha.saturating_add(1);
                let probe = negamax(pos, new_depth, -a1, -alpha, -color, deadline, ctx);

                match probe {
                    None => None,
                    Some((s_child, _)) => {
                        let probe_parent = -s_child;
                        if probe_parent > alpha && probe_parent < beta {
                            negamax(pos, new_depth, -beta, -alpha, -color, deadline, ctx)
                        } else {
                            probe
                        }
                    }
                }
            }
        };
        ctx.ply -= 1;
        pos.undo_move();

        if child.is_none() {
            return if !is_root || i == 0 {
                None
            } else {
                Some((alpha, best_move))
            }
        }

        let child_score = child.unwrap().0;
        let score = -child_score;

        if score > best_score {
            best_score = score;
            best_move = mv;
        }
        if score > alpha {
            alpha = score;
            ctx.pv.adopt(ctx.ply, mv);
        }
        if alpha >= beta {
            if !mv.is_capture() && depth >= 2 {
                ctx.history.update_non_captures(mv, pos.side_to_move(), depth);
                let killers = &mut ctx.killers[ctx.ply as usize];
                if mv != killers[0] {
                    killers[1] = killers[0];
                    killers[0] = mv;
                }
            }
            ctx.tt.store(pos.zobrist(), depth, Bound::Lower, alpha, mv, ctx.generation);
            break;
        }
    }


    let returned = alpha;
    let bound = if returned <= orig_alpha {
        Bound::Upper
    } else if returned >= beta {
        Bound::Lower
    } else {
        Bound::Exact
    };
    ctx.tt.store(pos.zobrist(), depth, bound, returned, best_move, ctx.generation);
    Some((returned, best_move))
}

fn reduction(depth: u8) -> u8 {
    let r_u8 = if depth <= 4 { 2 } else { (((depth as i16) - 4 + 2) / 3 + 2) as u8 };
    depth.saturating_sub(1 + r_u8)
}

#[inline]
fn pawn_to_penultimate(pos: &Position, mv: Move) -> bool {
    if pos.piece_at_sq(mv.to()) != Pawn { return false; }
    let to_rank = (mv.to() / 8) as u8; // 0..7
    // White penultimate: rank 6 (squares 48..55); Black penultimate: rank 1 (8..15)
    (pos.side_to_move().is_white() && to_rank == 6) ||
        (pos.side_to_move().is_black() && to_rank == 1)
}

fn extension(pos: &Position, mv: Move) -> u8 {
    if pos.in_check() || pawn_to_penultimate(&pos, mv) {
        return 1
    }
    0
}