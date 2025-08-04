use std::env;
use std::process::exit;
use chess::simulator::engine_battle_simulator::battle_against_other_search_algos;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run --simulator battle -- <engine_number>");
        exit(1);
    }

    let engine_number: u8 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid engine number: {}", args[1]);
            exit(1);
        }
    };

    //battle_against_other_search_algos(engine_number, 1, 2);
}
