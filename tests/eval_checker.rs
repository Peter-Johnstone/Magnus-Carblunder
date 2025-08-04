use indicatif::{ProgressBar, ProgressStyle};
use chess::attacks::movegen::all_moves;
use chess::position::Position;
mod perft_positions;
use perft_positions::PERFT_POSITIONS;

#[test]
fn run_dfs_eval_check_suite() {
    let bar = ProgressBar::new(PERFT_POSITIONS.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );


    for (index, entry) in PERFT_POSITIONS.iter().enumerate() {
        bar.set_message(format!("Position {}", index + 1));
        let mut position = Position::load_position_from_fen(entry.fen);
        dfs_eval_check(&mut position, 3);
        bar.inc(1);
    }
    bar.finish_with_message("All positions completed");
}


pub fn dfs_eval_check(position: &mut Position, depth: u8) {
    let old_eval = position.evaluate();

    assert_eq!(old_eval, Position::load_position_from_fen(&*position.to_fen()).evaluate());

    if depth == 0 {
        return;
    }

    for mov in all_moves(position).iter() {
        position.do_move(mov);
        dfs_eval_check(position, depth-1);
        position.undo_move();

        assert_eq!(position.evaluate(), old_eval);
    }
}
