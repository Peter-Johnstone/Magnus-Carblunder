use chess::engines::engine_manager::Engine;
use chess::position::Position;
use chess::simulator::engine_battle_simulator::{battle_against_other_eval_algos, battle_against_other_search_algos, print_bar_graph, simulate_many_battles};

use funtime;           // re-exported proc-macro



#[test]
fn eval_1() {
    battle_against_other_eval_algos(5, 1, 2, 1000);
}

#[test]
fn eval_2() {
    battle_against_other_eval_algos(5, 2, 20, 1000);
}

#[test]
fn eval_3() {
    battle_against_other_search_algos(5, 3, 2, 1000);
}


#[test]
fn search_3() {
    battle_against_other_search_algos(3, 3, 2, 1000);
}

#[test]
fn search_4() {
    battle_against_other_search_algos(4, 3, 2, 1000);
}
#[test]
fn search_5() {
    battle_against_other_search_algos(5, 3, 2, 1000);
}

#[test]
fn search_6() {
    battle_against_other_search_algos(6, 3, 2, 1000);
}

#[test]
fn search_7() {
    battle_against_other_search_algos(7, 1, 30, 60);
}

#[test]
fn search_8() {
    battle_against_other_search_algos(8, 1, 30, 60);
}


#[test]
fn search_9() {
    battle_against_other_search_algos(9, 1, 10, 1000);
}

#[test]
fn search_10() {
    battle_against_other_search_algos(10, 1, 7, 1000);
}



#[test]
fn engine_2() {
    let (challenger_s, challenger_e) = (3, 3);
    let (champion_s, champion_e) = (4, 3);
    let mut challenger = Engine::new(challenger_s, challenger_e, 5);
    let mut champion = Engine::new(champion_s, champion_e, 5);
    let (wins, losses, draws) = simulate_many_battles(&mut challenger, &mut champion, 100);


    print_bar_graph(wins, losses, draws, challenger_s, challenger_s);
}



#[funtime::timed]
fn nodes_at_depth(position: &mut Position, search_algo: u8, depth: u8) -> u64 {
    let mut engine = Engine::new(search_algo, 1, 0);
    engine.nodes_searched(position, depth)
}
#[test]
fn test_transposition_table_node_decrease() {
    let mut position = Position::load_position_from_fen("8/7p/6p1/5p2/8/2p3P1/8/2K1k3 b - - 1 43");
    let no_tt = nodes_at_depth(&mut position, 3, 10);
    let with_tt = nodes_at_depth(&mut position, 4, 10);
    println!("no tt:   {:?}", no_tt);
    println!("with tt: {:?}", with_tt);
    assert!(no_tt > with_tt);
}

#[test]
fn test_pv_move_ordering_node_decrease() {
    let mut position = Position::load_position_from_fen("8/7p/6p1/5p2/8/2p3P1/8/2K1k3 b - - 1 43");
    let no_pv = nodes_at_depth(&mut position, 4, 10);
    let with_pv = nodes_at_depth(&mut position, 5, 10);
    println!("no pv:   {:?}", no_pv);
    println!("with pv: {:?}", with_pv);
    assert!(no_pv > with_pv);
}

#[test]
fn test_hash_move_ordering_node_decrease() {
    let mut position = Position::load_position_from_fen("8/7p/6p1/5p2/8/2p3P1/8/2K1k3 b - - 1 43");
    let no_hash_move = nodes_at_depth(&mut position, 5, 8);
    let with_hash_move = nodes_at_depth(&mut position, 6, 8);
    println!("no hash move:   {:?}", no_hash_move);
    println!("with hash move: {:?}", with_hash_move);
    assert!(no_hash_move > with_hash_move);
}



#[test]
fn test_transposition_table_depth_increase() {
    let mut challenger = Engine::new(3, 3, 1000);
    let mut champion = Engine::new(4, 3, 1000);


    let mut position = Position::load_position_from_fen("8/7p/6p1/5p2/8/2p3P1/8/2K1k3 b - - 1 43");
    let (_, challenger_depth, _) = challenger.pick_and_stats(&mut position);
    let (_, champion_depth, _) = champion.pick_and_stats(&mut position);
    println!("Challenger depth: {}", challenger_depth);
    println!("Champion depth: {}", champion_depth);
    assert!(champion_depth > challenger_depth);
}


#[test]
fn test_pv_move_ordering_depth_increase() {
    let mut challenger = Engine::new(4, 3, 10000);
    let mut champion = Engine::new(5, 3, 10000);


    let mut position = Position::load_position_from_fen("8/7p/6p1/5p2/8/2p3P1/8/2K1k3 b - - 1 43");
    let (_, challenger_depth, _) = challenger.pick_and_stats(&mut position);
    let (_, champion_depth, _) = champion.pick_and_stats(&mut position);
    println!("Challenger depth: {}", challenger_depth);
    println!("Champion depth: {}", champion_depth);
    assert!(champion_depth > challenger_depth);
}

#[test]
fn test_hash_move_ordering_depth_increase() {
    let mut challenger = Engine::new(5, 3, 10000);
    let mut champion = Engine::new(6, 3, 10000);


    let mut position = Position::load_position_from_fen("8/7p/6p1/5p2/8/2p3P1/8/2K1k3 b - - 1 43");
    let (_, challenger_depth, _) = challenger.pick_and_stats(&mut position);
    let (_, champion_depth, _) = champion.pick_and_stats(&mut position);
    println!("Challenger depth: {}", challenger_depth);
    println!("Champion depth: {}", champion_depth);
    assert!(champion_depth > challenger_depth);
}

#[test]
fn test_MVVLVA_move_ordering_depth_increase() {
    let mut challenger = Engine::new(6, 3, 10000);
    let mut champion = Engine::new(7, 3, 10000);


    let mut position = Position::load_position_from_fen("8/7p/6p1/5p2/8/2p3P1/8/2K1k3 b - - 1 43");
    let (_, challenger_depth, _) = challenger.pick_and_stats(&mut position);
    let (_, champion_depth, _) = champion.pick_and_stats(&mut position);
    println!("Challenger depth: {}", challenger_depth);
    println!("Champion depth: {}", champion_depth);
    assert!(champion_depth > challenger_depth);
}