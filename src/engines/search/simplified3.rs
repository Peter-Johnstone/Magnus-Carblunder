use std::time::Instant;
use crate::attacks::movegen::all_moves;
use crate::engines::engine_manager::Ctx;
use crate::mov::{Move, MoveList, MAX_MOVES};
use crate::position::{Position};



// Basic Minimax with alpha beta pruning and MVV_LVA move ordering


pub const MVV_LVA: [[u16; 6]; 5] = [
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


fn score_moves(pos: &Position, mvs: &MoveList) -> [u16; MAX_MOVES] {

    let mut scores = [0; MAX_MOVES];

    for (i, mv) in mvs.iter().enumerate() {
        scores[i] = score_move(pos, mv);
    }

    scores
}


fn next_move(mvs: &mut MoveList,
             scores: &[u16; MAX_MOVES],
             start: usize // indicates how many moves have been processed in the movelist so far (ie: where to skip to)
) -> Move
{
    let mut top = 0;
    let mut top_i = start;
    let mut top_mv = Move::null();

    // moves to consider: between start and mvs.len
    for i in start..mvs.len {

        let mv_score = scores[i];
        if mv_score >= top {
            // new top scorer found.
            top = mv_score;
            top_i = i;
            top_mv = mvs.get(i);
        }
    }

    // to make sure our MoveList is ordered by already picked moves, we need to swap.
    if top_i != start {
        mvs.swap(top_i, start);
    }

    debug_assert!(!top_mv.is_null());
    top_mv
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

    let mut top_mv: Move = Move::null();

    let scores = score_moves(pos, &mvs);

    if pos.side_to_move().is_white()

    {

        // Maximizing player

        for i in 0..mvs.len {

            // pick next move from the move ordering scheme
            let mv = next_move(&mut mvs, &scores, i);

            pos.do_move(mv);
            let reply = minimax(pos, depth - 1, a, b, 0, deadline, ctx);
            pos.undo_move();

            match reply {
                None => return None, // Hard-deadline on time reached.
                Some((child_eval, _)) => {

                    // Maximizing player updates alpha
                    if child_eval > a {
                        a = child_eval;
                        top_mv = mv;
                    }

                    if a >= b {
                        // Cutoff achieved.
                        break;
                    }
                }
            }
        }

        Some((a, top_mv))
    } else {

        // Minimizing player

        for i in 0..mvs.len {

            // pick next move from the move ordering scheme
            let mv = next_move(&mut mvs, &scores, i);

            pos.do_move(mv);
            let reply = minimax(pos, depth - 1, a, b, 0, deadline, ctx);
            pos.undo_move();

            match reply {
                None => return None, // Hard-deadline on time reached.
                Some((child_eval, _)) => {

                    // Minimizing player updates beta
                    if child_eval < b {
                        b = child_eval;
                        top_mv = mv;
                    }

                    if a >= b {
                        // Cutoff achieved.
                        break;
                    }
                }
            }
        }


        Some((b, top_mv))
    }
}