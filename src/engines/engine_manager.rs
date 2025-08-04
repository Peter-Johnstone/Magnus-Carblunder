use std::cmp::PartialEq;
use std::fmt;
use std::time::{Duration, Instant};
use crate::color::Color;
use crate::engines::constants::MAX_DEPTH;
use crate::engines::engine_manager::Eval::{Basic, PSTKingEgBonus, PST};
use crate::engines::engine_manager::Search::{AlphaBeta, Minimax, Random, WithCheckInQuiescenceSearch, WithHashMoveOrdering, WithMVVLVAMoveOrdering, WithNullMovePruning, WithQuiescenceSearch, WithRootPVOrdering, WithTranspositionTable};
use crate::engines::evaluate::{e1, e2, e3};
use crate::engines::search::{s1, s10, s2, s3, s4, s5, s6, s7, s8, s9};
use crate::engines::transposition_table::TransTable;
use crate::mov::Move;
use crate::position::Position;


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
    WithCheckInQuiescenceSearch, // adds check positions to quiescence search
}

pub const NUMBER_OF_SEARCH_ALGORITHMS: u8 = 10;
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Eval {
    Basic, // Just pieces
    PST, // PST
    PSTKingEgBonus, // PST and endgame king bonus
}

pub const NUMBER_OF_EVAL_ALGORITHMS: u8 = 3;

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
            10 => Ok(WithCheckInQuiescenceSearch),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Eval {
    type Error = ();
    fn try_from(x: u8) -> Result<Self, ()> {
        match x {
            1 => Ok(Basic),
            2 => Ok(PST),
            3 => Ok(PSTKingEgBonus),
            _ => Err(()),
        }
    }
}


const PV_ARRAY_LENGTH: usize = ((MAX_DEPTH*MAX_DEPTH + MAX_DEPTH)/2) as usize; // triangular number of max depth


pub struct Ctx {
    pub tt:           TransTable,
    pub eval_fn: fn(&Position) -> i16,

    pub nodes:          u64,
    pub hash_first:     u64,
    pub hash_miss:      u64,
    pub total_nodes:    u64,
    pub tt_probes:      u64,
    pub tt_hits:        u64,
    pub ply:            u16,
    pub generation:     u16,
    pub pv_index:       usize,
    pub tt_played:      u64,
    pub tt_played_cut:  u64,
    pub pv_played_cut:  u64,
    pub pv_played:      u64,
    pub pv_array:     [Move; PV_ARRAY_LENGTH],
}

impl Ctx {
    pub fn new(eval_fn: fn(&Position) -> i16) -> Self {
        Ctx {
            tt:             TransTable::new(128),
            eval_fn,
            nodes:          0,
            hash_first:     0,
            hash_miss:      0,
            total_nodes:    0,
            tt_probes:      0,
            tt_hits:        0,
            ply:            0,
            generation:     0,
            pv_index:       0,
            tt_played:      0,
            tt_played_cut:  0,
            pv_played_cut:  0,
            pv_played:      0,
            pv_array:       [Move::null(); PV_ARRAY_LENGTH],
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
            PST => e2::evaluate,
            PSTKingEgBonus => e3::evaluate
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
            WithCheckInQuiescenceSearch => s10::negamax,
        }
    }

    pub fn total_nodes(&self) -> u64 {
        self.search_ctx.total_nodes
    }




    pub fn pick_and_stats(&mut self, pos: &mut Position) -> (Move, u8, i16) {
        // ───────────────────────────────────────────────────────────────
        // (0) fresh bookkeeping for this whole search
        // ───────────────────────────────────────────────────────────────
        {
            let ctx = &mut self.search_ctx;

            ctx.generation = ctx.generation.wrapping_add(1);
            ctx.total_nodes += ctx.nodes;
            ctx.nodes           = 0;
            ctx.hash_first      = 0;
            ctx.tt_probes       = 0;
            ctx.tt_hits         = 0;
            ctx.tt_played       = 0;
            ctx.tt_played_cut   = 0;
            ctx.pv_played       = 0;
            ctx.pv_played_cut   = 0;

            ctx.pv_index  = 0;
            ctx.ply       = 0;
            ctx.pv_array[0] = Move::null();          // blank root row
        }   // mutable borrow ends here

        // ───────────────────────────────────────────────────────────────
        // (1) iterative deepening
        // ───────────────────────────────────────────────────────────────
        let deadline = Instant::now() + Duration::from_millis(self.time_ms);
        let color    = if pos.turn() == Color::White { 1 } else { -1 };

        let mut best_eval = i16::MIN;
        let mut best  = Move::null();
        let mut depth = 1_u8;

        while Instant::now() < deadline {
            // -------- call the search (mutable borrow inside this block)
            let search_result = {
                let ctx = &mut self.search_ctx;      // short-lived &mut self
                (self.search_fn)(
                    pos,
                    depth,
                    i16::MIN + 1,
                    i16::MAX,
                    color,
                    deadline,
                    ctx,
                )
            };  // ctx borrow ends here

            // -------- handle the result (immutable borrow now allowed)
            match search_result {
                Some((eval, mv)) if !mv.is_null() => {
                    best = mv;                       //  ← store best move
                    best_eval = eval;

                    // print!("Depth {depth}: ");
                    // self.print_pv(depth);                 //  ← prints current PV
                    // eprintln!(
                    //     "Engine #{:?}, TT played: {:.0}%, cuts: {:.0}%, PV played {:.0}%, cuts {:.0}%",
                    //     self.search,
                    //
                    //     100.0 * ctx.tt_played as f64 / nodes,
                    //     percent(ctx.tt_played_cut, ctx.tt_played),
                    //     100.0 * ctx.pv_played as f64 / nodes,
                    //     percent(ctx.pv_played_cut, ctx.pv_played),
                    // );


                    // prepare for next iteration
                    if depth == (MAX_DEPTH-2) as u8 { break; }
                    depth = depth.saturating_add(1);

                    let ctx = &mut self.search_ctx;  // new short-lived borrow
                    ctx.pv_index   = 0;
                    ctx.ply        = 0;
                }
                _ => break,                          // timeout or no legal move
            }
        }

        // depth now points one past the last *completed* ply
        (best, depth.saturating_sub(1), best_eval)
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
        let ctx = &mut self.search_ctx;
        ctx.generation = ctx.generation.wrapping_add(1);
        ctx.total_nodes += ctx.nodes;
        ctx.nodes = 0;
        ctx.hash_first = 0;
        ctx.tt_probes = 0;
        ctx.tt_hits   = 0;

        /* 1. 100 % critical: reset PV position counters */
        ctx.pv_index = 0;    // root always starts at cell 0
        ctx.ply      = 0;    // root is ply 0

        /* optional: blank the first row for clean diagnostics */
        ctx.pv_array[0] = Move::null();

        const FAR_FUTURE_SECS: u64 = 1000 * 365 * 24 * 60 * 60; // ~1000 years
        let deadline = Instant::now() + Duration::from_secs(FAR_FUTURE_SECS); //no deadline
        let color    = if pos.turn() == Color::White { 1 } else { -1 };

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
        let ctx = &mut self.search_ctx;
        ctx.generation = ctx.generation.wrapping_add(1);
        ctx.total_nodes += ctx.nodes;
        ctx.nodes = 0;
        ctx.hash_first = 0;
        ctx.tt_probes = 0;
        ctx.tt_hits   = 0;

        /* 1. 100 % critical: reset PV position counters */
        ctx.pv_index = 0;    // root always starts at cell 0
        ctx.ply      = 0;    // root is ply 0

        /* optional: blank the first row for clean diagnostics */
        ctx.pv_array[0] = Move::null();

        const FAR_FUTURE_SECS: u64 = 1000 * 365 * 24 * 60 * 60; // ~1000 years
        let deadline = Instant::now() + Duration::from_secs(FAR_FUTURE_SECS); //no deadline
        let color    = if pos.turn() == Color::White { 1 } else { -1 };

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