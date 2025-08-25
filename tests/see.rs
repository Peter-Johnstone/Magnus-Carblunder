use chess::mov::Move;
use chess::position::Position;

#[test]

fn see() {
    const FENS: [&str; 6] = [
        "rnbqkbnr/1ppppppp/8/p7/8/1PBP4/P1PQPPPP/RN2KBNR w KQkq a6 0",
        "r1bqkbnr/pppppppp/2n5/8/3P4/7P/PPP1PPP1/RNBQKBNR b KQkq - 0",
        "r1bqk1nr/ppp1ppbp/2n3p1/8/3P3P/1N3N2/PPP1BPP1/R1BQK2R b KQkq - 0",
        "r1bqk1nr/ppp1ppbp/6p1/8/3n3P/1N3N2/PPP1BPP1/R1BQK2R w KQkq - 0",
        "r1b1k1nr/pppqpp1p/8/8/3N2pP/2P5/PP2BPPR/R1BQK3 w Qkq - 1",
        "r1b1k1nr/1ppqpp1p/8/8/p2N2BP/2P3R1/PPQ2PP1/R1B1K3 b Qkq - 1"
    ];

    let moves: [Move; 6] = [
        Move::new(18450),
        Move::new(18154),
        Move::new(18154),
        Move::new(18129),
        Move::new(18316),
        Move::new(18355),

    ];
    const EXPECTED_SEE_VALUE: [bool; 6] = [
        true,
        false,
        false,
        true,
        false,
        false
    ];


    for (i, fen) in FENS.iter().enumerate() {
        let mut pos = Position::load_position_from_fen(fen);
        assert_eq!(pos.see(moves[i]), EXPECTED_SEE_VALUE[i]);
    }

}