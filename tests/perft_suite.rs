use chess::position::Position;
use std::time::Instant;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use chess::attacks::movegen::all_moves;

mod perft_positions;
use perft_positions::PERFT_POSITIONS;



// PerftEntry {
// fen: "B6b/8/8/8/2K5/4k3/8/b6B w - -",
// expected: [17, 278, 4607, 76778, 1320507, 22823890],
// },

// c4d5 e3d2 a8e4
#[test]
fn perft_test_temp(){
    //let mut position = Position::load_position_from_fen("4rr2/1ppn1qpk/p1npp2p/P3p3/3PP3/2P1RNNP/QP3PP1/5RK1 w - - 1 22");

    let mut position = Position::start();
    let result = perft(&mut position, 6);
    assert_eq!(result, 119_060_324);
}


static PRINT_MODE: bool = false;


#[test]
fn run_perft_suite() {
    let bar = ProgressBar::new(PERFT_POSITIONS.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut total_nodes: u128 = 0;

    for (index, entry) in PERFT_POSITIONS.iter().enumerate() {
        bar.set_message(format!("Position {}", index + 1));
        let mut position = Position::load_position_from_fen(entry.fen);
        for (depth, &expected_nodes) in entry.expected.iter().enumerate() {
            // if depth == 5 {
            //     break;
            // }
            let depth = (depth + 1) as u8;
            let nodes = perft(&mut position, depth);
            total_nodes += nodes as u128;
            assert_eq!(
                nodes, expected_nodes,
                "Mismatch at depth {} in position {}", depth, index + 1
            );
        }
        bar.inc(1);
    }
    bar.finish_with_message("All positions completed");
    println!("Total Nodes: {}", total_nodes);
}



fn perft(position: &mut Position, max_depth: u8) -> u64 {
    if PRINT_MODE {
        position.print_board();
    }
    let start = Instant::now();
    let result = perft_recursive(position, max_depth, max_depth) as u64;
    let duration = start.elapsed();
    if PRINT_MODE {
        println!("\nNodes searched: {}", result);
        println!("Time: {:.2?}\n-----------------------------------------\n", duration);
    }
    result
}

fn perft_recursive(position: &mut Position, depth: u8, max_depth: u8) -> usize {
    let mut nodes = 0;
    match depth {
        0 => nodes += 1,
        1 => {
            nodes += all_moves(position).len
        }
        _ => {
            for mov in all_moves(position).iter() {
                position.do_move(mov);
                let child_nodes = perft_recursive(position, depth - 1, max_depth);
                position.undo_move();
                nodes += child_nodes;
                if PRINT_MODE && depth == max_depth {
                    println!("{}: {}", mov, child_nodes);
                }
            };
        }
    }
    nodes
}
