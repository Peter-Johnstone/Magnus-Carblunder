use std::time::Instant;
use crate::attacks::movegen::all_moves;
use crate::engines::engine_manager::Ctx;
use crate::engines::transposition_table::Bound;
use crate::mov::{Move, MoveList, MAX_MOVES};
use crate::position::{Position};

// Includes PV mode ordering


pub const MVV_LVA: [[u16; 6]; 5] =
[
    [15, 14, 13, 12, 11, 10], // victim = pawn
    [25, 24, 23, 22, 21, 20], // victim = knight
    [35, 34, 33, 32, 31, 30], // victim = bishop
    [45, 44, 43, 42, 41, 40], // victim = rook
    [55, 54, 53, 52, 51, 50], // victim = queen
];


fn score_move(pos: &Position, mv: Move) -> u16 {
    let mut score = 0;
    if mv.is_capture() {
        // apply MVV_LVA
        let victim    = pos.piece_at_sq(mv.to());
        let aggressor = pos.piece_at_sq(mv.from());
        score += MVV_LVA[victim as usize][aggressor as usize]
    }

    score
}


struct MovePicker {
    moves: MoveList,
    head: usize,
    prio_end: usize, // index where the priority moves end (pv and tt)
    scores: [u16; MAX_MOVES],
}

impl MovePicker {
    fn new(pv_mv: Move, tt_mv: Move, mut mvs: MoveList) -> MovePicker {

        let mut i = 0;
        if !pv_mv.is_null() {
            // put pv move at the front of the list
            let pv_mv_i = mvs.index(pv_mv);
            mvs.swap(pv_mv_i, i);
            i += 1;
        }
        if !tt_mv.is_null() && pv_mv != tt_mv {
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
        }
    }

    fn score_moves(&self, pos: &Position, mvs: &MoveList) -> [u16; MAX_MOVES] {
        let mut scores = [0; MAX_MOVES];

        // No need to calculate scores for pv/tt mvs that come first anyway
        for (i, mv) in mvs.iter().enumerate() {
            if i < self.prio_end {
                continue;
            }
            scores[i] = score_move(pos, mv); // keep tail relative
        }

        scores
    }


    fn next_move(&mut self, pos: &Position) -> Move {
        if self.head < self.prio_end {
            // We still need to process either pv_mv or tt_mv
            let mv = self.moves.get(self.head);
            self.head += 1;
            return mv
        } else if self.head == self.prio_end {
            // We need to score the moves bc neither hash mv nor pv mv caused a cutoff
            self.scores = self.score_moves(pos, &self.moves);
        }

        // regular moves
        let mut top = 0;
        let mut top_i = self.head;
        let mut top_mv = Move::null();

        // moves to consider: between start and moves.len
        for i in self.head..self.moves.len {

            let mv_score = self.scores[i];
            if mv_score >= top {
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


pub fn minimax(pos: &mut Position,
               depth: u8,
               mut a: i16,
               mut b: i16,
               _: i16,
               deadline: Instant,
               ctx: &mut Ctx)

               -> Option<(i16, Move)>
{
    if depth == 0 {
        // leaf node
        return Some((pos.evaluate(), Move::null()))
    }

    let mut hash_move = Move::null();

    if let Some(tt_entry) = ctx.tt.probe(pos.zobrist()) {
        hash_move = tt_entry.mv;
        let entry_ok = tt_entry.depth >= depth;

        if entry_ok {
            let bound = tt_entry.bound;
            if bound == Bound::Exact ||
                (bound == Bound::Lower && tt_entry.score >= b) ||
                (bound == Bound::Upper && tt_entry.score <= a)
            {
                return Some((tt_entry.score, tt_entry.mv));
            }
        }
    }

    if Instant::now() >= deadline {
        // Ran out of time. End early.
        return None;
    }

    // original alpha and beta
    let (a0, b0) = (a, b);

    // generate all the moves
    let mut mvs: MoveList = all_moves(pos);
    let num_mvs = mvs.len;

    if mvs.is_empty() {
        // terminal node. checkmate or stalemate
        return Some((pos.game_result_eval(depth), Move::null()))
    }

    let mut top_mv: Move = Move::null();

    let mut mv_picker = MovePicker::new(ctx.pv.mv(ctx.ply), hash_move, mvs);

    ctx.pv.clear_node(ctx.ply);

    if pos.side_to_move().is_white()

    {

        // Maximizing player

        for i in 0..num_mvs {

            // pick next move from the move ordering scheme
            let mv = mv_picker.next_move(pos);

            pos.do_move(mv);
            ctx.ply += 1;
            let reply = if i == 0 {
                // first move in the ordering is searched with full window
                minimax(pos, depth - 1, a, b, 0, deadline, ctx)
            } else {
                let mut null_window_reply = minimax(pos, depth - 1, a, a + 1, 0, deadline, ctx);
                if let Some((child_eval, _)) = null_window_reply {
                    if child_eval > a {
                        // null window found a better option (hopefully rare). Re-search with full window.
                        null_window_reply = minimax(pos, depth - 1, a, b, 0, deadline, ctx)
                    }
                }
                null_window_reply
            };
            ctx.ply -= 1;
            pos.undo_move();

            match reply {
                None => return None, // Hard-deadline on time reached.
                Some((child_eval, _)) => {

                    // Maximizing player updates alpha
                    if child_eval > a {
                        a = child_eval;
                        top_mv = mv;
                        ctx.pv.adopt(ctx.ply, mv);
                    }

                    if a >= b {
                        // Cutoff achieved.
                        break;
                    }
                }
            }
        }

        let bound = if a <= a0 { Bound::Upper } else if a >= b0 { Bound::Lower } else { Bound::Exact };
        ctx.tt.store(pos.zobrist(), depth, bound, a, top_mv, ctx.generation);

        Some((a, top_mv))
    } else {

        // Minimizing player

        for i in 0..num_mvs {

            // pick next move from the move ordering scheme
            let mv = mv_picker.next_move(pos);

            pos.do_move(mv);
            ctx.ply += 1;
            let reply = if i == 0 {
                // first move in the ordering is searched with full window
                minimax(pos, depth - 1, a, b, 0, deadline, ctx)
            } else {
                let mut null_window_reply = minimax(pos, depth - 1, b-1, b, 0, deadline, ctx);
                if let Some((child_eval, _)) = null_window_reply {
                    if child_eval < b {
                        // null window found a better option (hopefully rare). Re-search with full window.
                        null_window_reply = minimax(pos, depth - 1, a, b, 0, deadline, ctx)
                    }
                }
                null_window_reply
            };
            ctx.ply -= 1;
            pos.undo_move();

            match reply {
                None => return None, // Hard-deadline on time reached.
                Some((child_eval, _)) => {

                    // Minimizing player updates beta
                    if child_eval < b {
                        b = child_eval;
                        top_mv = mv;
                        ctx.pv.adopt(ctx.ply, mv);
                    }

                    if a >= b {
                        // Cutoff achieved.
                        break;
                    }
                }
            }
        }

        let bound = if b <= a0 { Bound::Upper } else if b >= b0 { Bound::Lower } else { Bound::Exact };
        ctx.tt.store(pos.zobrist(), depth, bound, b, top_mv, ctx.generation);


        Some((b, top_mv))
    }
}