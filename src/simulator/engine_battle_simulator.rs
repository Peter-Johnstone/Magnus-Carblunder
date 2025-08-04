use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use crate::attacks::movegen::all_moves;
use crate::color::Color;
use crate::engines::engine_manager::{Engine, NUMBER_OF_EVAL_ALGORITHMS, NUMBER_OF_SEARCH_ALGORITHMS};
use crate::simulator::even_fens::EVEN_FENS;
use crate::position::{Status, Position};
use crate::position::Status::{Checkmate, Draw};



pub  fn battle_against_other_eval_algos(search_algo: u8, eval_algo: u8, time_per_move: u64, num_battles: u16) {
    let mut champion = Engine::new(search_algo, eval_algo, time_per_move);
    println!("\
    \n\n\n\n                      SIMULATING ENGINE\
    \n                     [search: {search_algo}, eval: {eval_algo}]
--------------------------------------------------------------");
    for i in 1..NUMBER_OF_EVAL_ALGORITHMS+1 {
        if i != eval_algo {
            let mut challenger = Engine::new(search_algo, i, time_per_move);
            let (wins, losses, draws) = simulate_many_battles(&mut challenger, &mut champion, num_battles);
            print_bar_graph(wins, losses, draws, search_algo, i);
        }
    }
}


pub fn battle_against_other_search_algos(search_algo: u8, eval_algo: u8, time_per_move: u64, num_battles: u16) {
    let mut champion = Engine::new(search_algo, eval_algo, time_per_move);
    println!("\
    \n\n\n\n                      SIMULATING ENGINE\
    \n                     [search: {search_algo}, eval: {eval_algo}]
--------------------------------------------------------------");
    for i in 9..NUMBER_OF_SEARCH_ALGORITHMS+1 {
        if i != search_algo {
            let mut challenger = Engine::new(i, eval_algo, time_per_move);
            let (wins, losses, draws) = simulate_many_battles(&mut challenger, &mut champion, num_battles);
            print_bar_graph(wins, losses, draws, i, eval_algo);
        }
    }
}

pub fn print_bar_graph(wins: usize, losses: usize, draws: usize, challenger_search_algo: u8, challenger_eval_algo: u8) {
    let total = wins + losses + draws;
    let max_bar = 50;

    let bar = |count: usize| {
        let len = count * max_bar / total;
        "█".repeat(len)
    };
    println!();
    println!("            v. Engine [search: {challenger_search_algo}, eval: {challenger_eval_algo}]");
    println!("WINS  : {:>3} {}", wins,   bar(wins));
    println!("LOSSES: {:>3} {}", losses, bar(losses));
    println!("DRAWS : {:>3} {}", draws,  bar(draws));
    println!();
}


pub fn simulate_many_battles(challenger: &mut Engine, champion:   &mut Engine, num_battles: u16) -> (usize, usize, usize) {
    let mut wins   = 0usize;
    let mut losses = 0usize;
    let mut draws  = 0usize;
    let mut rng    = rand::rng();

    // ── set up timing + progress ────────────────────────────────────────────────
    let start = Instant::now();
    let pb = ProgressBar::new(num_battles as u64)
        .with_style(
            ProgressStyle::with_template(
                "{spinner:.green} {elapsed_precise} [{wide_bar:.cyan/blue}] \
                 {pos}/{len} • ETA {eta_precise}",
            )
                .unwrap()
                .progress_chars("=>-"),
        );

    // ── main loop ───────────────────────────────────────────────────────────────
    for _ in 0..num_battles {
        let result = battle(challenger, champion, &mut rng);
        match result {
            Checkmate(Color::White) => wins   += 1,
            Checkmate(Color::Black) => losses += 1,
            Draw                    => draws  += 1,
            _                       => {}               // just in case
        }
        pb.inc(1);
    }

    pb.finish_with_message(format!("Done in {:?}", start.elapsed()));
    (wins, losses, draws)
}



 fn battle<R: Rng>(challenger: &mut Engine, champion: &mut Engine, rng: &mut R) -> Status {
     // champion moves on white
    let position_id = rng.random_range(0..5000);

    let fen = EVEN_FENS[position_id];
    //println!("\n\n\n\n FEN: {fen}");

    let mut position = Position::load_position_from_fen(fen);  // or Position::from_id(position_id)
    let mut moves = 0;
    while !all_moves(&position).is_empty() {
        if moves > 200 {

            // let mut controller = GameController::new().await;
            // controller.load_fen(fen);
            // controller.run_review_game(&mut position.undo_stack()).await;
            return Draw;
        }
        moves += 1;
        let mov = if position.turn() == Color::White {
            champion.pick(&mut position)
        } else {
            challenger.pick(&mut position)

        };
        position.do_move(mov);
        // safe_move_or_debug(&mut position, mov, fen).await;
        // println!("Move #{moves}: {mov}");
    }
    let result = position.get_game_result();
    //println!("\n{result:?}");
    result
}



//
// use crate::mov::Move;
//
//  fn safe_move_or_debug(pos: &mut Position, mov: Move, fen: &str) {
//     let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
//         pos.do_move(mov);
//     }));
//
//     if result.is_err() {
//         println!("\n[!] do_move panicked on move: {}\n", mov);
//         println!("FEN: {fen}");
//         pos.print_board();
//         pos.print_move_history();
//
//         // optional: show GUI or exit
//         let mut controller = GameController::new().await;
//         controller.load_fen(fen);
//         controller.run_review_game(&mut pos.undo_stack()).await;
//
//         std::process::exit(1);
//     }
// }
