use std::cmp::PartialEq;
use std::fmt;
use std::time::{Duration, Instant};
use crate::attacks::movegen::all_moves;
use crate::color::Color;
use crate::engines::constants::MAX_DEPTH;
use crate::engines::engine_manager::Eval::{Basic, WithTradingBonus};
use crate::engines::engine_manager::Search::{AlphaBeta, Minimax, Random, CaptureLastPieceMO, WithHashMoveOrdering, WithMVVLVAMoveOrdering, WithNullMovePruning, WithQuiescenceSearch, WithRootPVOrdering, WithTranspositionTable, WithHistoryHeuristic, WithKillerMoves, WithLMR, WithInCheckQuiescence, Simplified1, Simplified2, Simplified3, Testing, Simplified4, Simplified5, Simplified6};
use crate::engines::evaluate::{e1, e2};
use crate::engines::search::{s1, s10, s11, s12, s13, s14, s2, s3, s4, s5, s6, s7, s8, s9, simplified1, simplified2, simplified3, simplified4, simplified5, simplified6, testing_only};
use crate::engines::transposition_table::TransTable;
use crate::mov::Move;
use crate::position::Position;
use crate::engines::history::History;
use crate::engines::pv::PV;
use crate::engines::stats::Stats;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Search {
    Random, // Random moves
    Minimax, // Basic Minimax
    AlphaBeta, // Minimax alpha beta pruning
    WithTranspositionTable, // Minimax alpha beta pruning, Transposition table
    WithRootPVOrdering, // uses the same first move of the principal variation for iterative deepening.
    WithHashMoveOrdering, // uses the hashed best move from the transposition table first.
    WithMVVLVAMoveOrdering, // uses most-valuable victim least-valuable aggressor move ordering
    WithQuiescenceSearch, // Searches until it reaches quiet positions
    WithNullMovePruning, // Incorporates null move pruning into the search
    CaptureLastPieceMO, // adds a bonus to try capturing the last moved piece
    WithHistoryHeuristic, // Shrinks the scored move metric from i32 to u8
    WithKillerMoves, // Implement killers
    WithLMR, // Implements late move reductions
    WithInCheckQuiescence, // Implements the quiescence search for positions where we are in check

    Simplified1,
    Simplified2,
    Simplified3,
    Simplified4,
    Simplified5,
    Simplified6,


    Testing,

}

pub const NUMBER_OF_SEARCH_ALGORITHMS: u8 = 20;
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Eval {
    Basic,
    WithTradingBonus, // awards trading in winning positions, punishes in losing positions.
}

pub const NUMBER_OF_EVAL_ALGORITHMS: u8 = 2;

impl TryFrom<u8> for Search {
    type Error = ();
    fn try_from(x: u8) -> Result<Self, ()> {
        match x {
            1  => Ok(Random),
            2  => Ok(Minimax),
            3  => Ok(AlphaBeta),
            4  => Ok(WithTranspositionTable),
            5  => Ok(WithRootPVOrdering),
            6  => Ok(WithHashMoveOrdering),
            7  => Ok(WithMVVLVAMoveOrdering),
            8  => Ok(WithQuiescenceSearch),
            9  => Ok(WithNullMovePruning),
            10 => Ok(CaptureLastPieceMO),
            11 => Ok(WithHistoryHeuristic),
            12 => Ok(WithKillerMoves),
            13 => Ok(WithLMR),
            14 => Ok(WithInCheckQuiescence),
            15 => Ok(Simplified1),
            16 => Ok(Simplified2),
            17 => Ok(Simplified3),
            18 => Ok(Simplified4),
            19 => Ok(Simplified5),
            20 => Ok(Simplified6),

            30 => Ok(Testing),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Eval {
    type Error = ();
    fn try_from(x: u8) -> Result<Self, ()> {
        match x {
            1 => Ok(Basic),
            2 => Ok(WithTradingBonus),
            _ => Err(()),
        }
    }
}


const PV_ARRAY_LENGTH: usize = ((MAX_DEPTH*MAX_DEPTH + MAX_DEPTH)/2) as usize; // triangular number of max depth


pub struct Ctx {
    pub tt:             TransTable,
    pub eval_fn:        fn(&Position) -> i16,
    pub nodes:          u64,
    pub total_nodes:    u64,
    pub generation:     u16,
    pub ply:            u16,
    pub pv_index:       usize,
    pub pv_array:       [Move; PV_ARRAY_LENGTH],
    pub pv:             PV,
    pub stats:          Stats,
    pub history:        History,
    pub killers:        [[Move; 2]; MAX_DEPTH as usize],
}

impl Ctx {
    pub fn new(eval_fn: fn(&Position) -> i16) -> Self {
        Ctx {
            tt:             TransTable::new(128),
            eval_fn,
            nodes:          0,
            total_nodes:    0,
            ply:            0,
            generation:     0,
            pv_index:       0,
            pv_array:       [Move::null(); PV_ARRAY_LENGTH],
            pv:             PV::new(),
            stats:          Stats::default(),
            history:        Default::default(),
            killers:        [[Move::null(); 2]; MAX_DEPTH as usize],
        }
    }
}


type SearchFn = fn(
    &mut Position,            // position to modify
    u8,                       // depth
    i16,                     // beta
    i16,                     // alpha
    i16,                      // color factor
    Instant,                  // deadline
    &mut Ctx)           // shared context
    -> Option<(i16, Move)>;   // score + best move


pub struct Engine {
    search: Search,
    eval: Eval,
    search_fn:  SearchFn,
    search_ctx: Ctx,
    time_ms:    u64,
}

impl Engine {
    pub fn new(search_algo: u8, eval_algo: u8, time_ms: u64) -> Engine {
        let search = Search::try_from(search_algo).unwrap();
        let search_fn = Self::search_fn(search);
        let eval = Eval::try_from(eval_algo).unwrap();
        let eval_fn   = Self::eval_fn  (eval);

        Engine {
            search,
            eval,
            search_fn,
            search_ctx: Ctx::new(eval_fn),
            time_ms,
        }
    }

    fn eval_fn(eval: Eval) -> fn(&Position) -> i16 {
        match eval {
            Basic => e1::evaluate,
            WithTradingBonus => e2::evaluate,
        }
    }

    fn search_fn(search: Search) -> SearchFn {
        match search {
            Random  => s1::pick,
            Minimax => s2::negamax,
            AlphaBeta => s3::negamax,
            WithTranspositionTable => s4::negamax,
            WithRootPVOrdering => s5::negamax,
            WithHashMoveOrdering => s6::negamax,
            WithMVVLVAMoveOrdering => s7::negamax,
            WithQuiescenceSearch => s8::negamax,
            WithNullMovePruning => s9::negamax,
            CaptureLastPieceMO => s10::negamax,
            WithHistoryHeuristic => s11::negamax,
            WithKillerMoves => s12::negamax,
            WithLMR => s13::negamax,
            WithInCheckQuiescence => s14::negamax,

            Simplified1 => simplified1::minimax,
            Simplified2 => simplified2::minimax,
            Simplified3 => simplified3::minimax,
            Simplified4 => simplified4::minimax,
            Simplified5 => simplified5::minimax,
            Simplified6 => simplified6::minimax,

            Testing => testing_only::minimax,

        }
    }

    pub fn total_nodes(&self) -> u64 {
        self.search_ctx.total_nodes
    }

    pub fn set_time_limit(&mut self, ms: u64) {
        self.time_ms = ms;
    }

    pub fn pick_and_stats(&mut self, pos: &mut Position) -> (Move, u8, i16) {
        // ───────────────────────────────────────────────────────────────
        // (0) fresh bookkeeping for this whole search
        // ───────────────────────────────────────────────────────────────

        if pos.undo_stack.is_near_full() {
            pos.undo_stack.make_space();
        }
        let ctx = &mut self.search_ctx;
        {
            ctx.history.clear();   // reset history to 0s
            ctx.generation = ctx.generation.wrapping_add(1);
            ctx.total_nodes += ctx.nodes;
            ctx.nodes     = 0;
            ctx.pv_index  = 0;
            ctx.ply       = 0;
            ctx.stats     = Stats::default();
            ctx.pv_array[0] = Move::null();
            ctx.killers.iter_mut().for_each(|slot| *slot = [Move::null(); 2]);
        }

        let deadline = Instant::now() + Duration::from_millis(self.time_ms);
        let color    = if pos.side_to_move() == Color::White { 1 } else { -1 };

        let mut best_eval = i16::MIN;
        let mut best      = Move::null();
        let mut depth     = 1u8;

        while Instant::now() < deadline {
            // ----- 1a. run search (mutable borrow inside this block)

            ctx.stats     = Stats::default();
            ctx.pv_index  = 0;
            ctx.ply       = 0;
            ctx.pv_array[0] = Move::null();
            ctx.pv.clear_node(ctx.ply);
            ctx.killers.iter_mut().for_each(|slot| *slot = [Move::null(); 2]);

            let search_result = {
                (self.search_fn)(
                    pos,
                    depth,
                    i16::MIN + 1,
                    i16::MAX,
                    color,
                    deadline,
                    ctx,
                )
            }; // ctx borrow ends here

            // ----- 1b. handle result  (now we can borrow ctx immutably)
            match search_result {
                Some((eval, mv)) if !mv.is_null() => {
                    best      = mv;
                    best_eval = eval;
                    //ctx.stats.print();

                    // prepare next iteration
                    if depth >= (MAX_DEPTH - 2) as u8 || Instant::now() >= deadline {
                        break;
                    }
                    depth = depth.saturating_add(1);

                    // reset root-level bookkeeping
                    ctx.pv_index = 0;
                    ctx.ply      = 0;
                }
                _ => break, // timeout or search returned no legal move
            }
        }

        if best.is_null() {
            best = all_moves(pos).get(0);
        }

        // depth now points one past the last *completed* ply
        (best, depth.saturating_sub(1), best_eval)
    }

    pub fn clone(&self) -> Engine {
        let eval_fn   = Self::eval_fn  (self.eval);
        let ctx = Ctx::new(eval_fn);
        Engine {
            search: self.search,
            eval: self.eval,
            search_fn: self.search_fn,
            search_ctx: ctx,
            time_ms: self.time_ms,
        }
    }



    #[allow(dead_code)]
    pub fn print_pv(&self, depth: u8) {
        let pv = &self.search_ctx.pv_array;
        for i in 0..depth as usize {
            if pv[i].is_null() { break; }
            print!("{} ", pv[i]);
        }
        println!();
    }


    #[allow(dead_code)]
    fn percent(part: u64, whole: u64) -> f64 {
        if whole == 0 { 0.0 } else { 100.0 * part as f64 / whole as f64 }
    }



    pub fn pick_fixed_depth(&mut self, pos: &mut Position, depth: u8) -> Move {
        // ───────────────────────────────────────────────────────────────
        // (0) fresh bookkeeping for this whole search
        // ───────────────────────────────────────────────────────────────
        let ctx = &mut self.search_ctx;
        {
            ctx.history.clear(); // reset history to 0
            ctx.generation = ctx.generation.wrapping_add(1);
            ctx.total_nodes += ctx.nodes;
            ctx.nodes     = 0;
            ctx.pv_index  = 0;
            ctx.ply       = 0;
            ctx.pv_array[0] = Move::null();
            ctx.pv.clear_node(ctx.ply);

            ctx.killers.iter_mut().for_each(|slot| *slot = [Move::null(); 2]);
        }

        const FAR_FUTURE_SECS: u64 = 1000 * 365 * 24 * 60 * 60; // ~1000 years
        let deadline = Instant::now() + Duration::from_secs(FAR_FUTURE_SECS); //no deadline
        let color    = if pos.side_to_move() == Color::White { 1 } else { -1 };

        if let Some(search_result) = {
            (self.search_fn)(
                pos,
                depth,
                i16::MIN + 1,
                i16::MAX,
                color,
                deadline,
                ctx,
            )
        } {
            return search_result.1
        };
        Move::null()
    }

    pub fn nodes_searched(&mut self, pos: &mut Position, depth: u8) -> u64 {
        // ───────────────────────────────────────────────────────────────
        // (0) fresh bookkeeping for this whole search
        // ───────────────────────────────────────────────────────────────
        if pos.undo_stack.is_near_full() {
            pos.undo_stack.make_space();
        }
        let ctx = &mut self.search_ctx;
        {
            ctx.history.clear();   // reset history to 0s
            ctx.generation = ctx.generation.wrapping_add(1);
            ctx.total_nodes += ctx.nodes;
            ctx.nodes     = 0;
            ctx.pv_index  = 0;
            ctx.ply       = 0;
            ctx.pv_array[0] = Move::null();
            ctx.pv.clear_node(ctx.ply);

            ctx.killers.iter_mut().for_each(|slot| *slot = [Move::null(); 2]);
        }

        const FAR_FUTURE_SECS: u64 = 1000 * 365 * 24 * 60 * 60; // ~1000 years
        let deadline = Instant::now() + Duration::from_secs(FAR_FUTURE_SECS); //no deadline
        let color    = if pos.side_to_move() == Color::White { 1 } else { -1 };

        (self.search_fn)(
            pos,
            depth,
            i16::MIN + 1,
            i16::MAX,
            color,
            deadline,
            ctx,);

        ctx.nodes
    }

    pub fn pick(&mut self, pos: &Position) -> Move {
        let mut position = pos.clone();
        self.pick_and_stats(&mut position).0
    }

    pub fn name(&self) -> String{
        let mut s = "[search: ".to_owned();
        s += &*self.search.to_string();
        s += ", eval: ";
        s += &*self.eval.to_string();
        s += "]";
        s
    }

}

impl fmt::Display for Search {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Eval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}