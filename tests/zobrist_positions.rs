use indicatif::{ProgressBar, ProgressStyle};
use chess::attacks::movegen::all_moves;
use chess::mov::Move;
use chess::position::Position;
mod perft_positions;
use perft_positions::PERFT_POSITIONS;

#[test]
// Kasparov vs. Topalov, Wijk aan Zee 1999
fn zobrist_game_1() {
    let moves = [5900, 2803, 5835, 2942, 1153, 2998, 1282, 3517, 707, 2738, 1357, 6257, 774, 3321, 3028, 19446, 19403, 3194, 1032, 6452, 12420, 3387, 66, 2608, 140, 16060, 1090, 18148, 18115, 2218, 219, 2675, 1422, 3706, 2065, 3633, 1477, 2283, 1903, 3129, 263, 1763, 2258, 18665, 18652, 2804, 18115, 18146, 3332, 2672, 18141, 18473, 5705, 1568, 1179, 18667, 3124, 3192, 19568, 1699, 19282, 17432, 18989, 18000, 1162, 17561, 40, 722, 576, 203, 343, 763, 3313, 19659, 18053, 18081, 20425, 1267, 3647, 1178, 1592, 259, 1877, 6517, 129, 723, 3096, ];
    let fens = [
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        "rnbqkbnr/ppp1pppp/3p4/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2",
        "rnbqkbnr/ppp1pppp/3p4/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq d3 0 2",
        "rnbqkb1r/ppp1pppp/3p1n2/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 1 3",
        "rnbqkb1r/ppp1pppp/3p1n2/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 2 3",
        "rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 4",
        "rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/2N1B3/PPP2PPP/R2QKBNR b KQkq - 1 4",
        "rnbqk2r/ppp1ppbp/3p1np1/8/3PP3/2N1B3/PPP2PPP/R2QKBNR w KQkq - 2 5",
        "rnbqk2r/ppp1ppbp/3p1np1/8/3PP3/2N1B3/PPPQ1PPP/R3KBNR b KQkq - 3 5",
        "rnbqk2r/pp2ppbp/2pp1np1/8/3PP3/2N1B3/PPPQ1PPP/R3KBNR w KQkq - 0 6",
        "rnbqk2r/pp2ppbp/2pp1np1/8/3PP3/2N1BP2/PPPQ2PP/R3KBNR b KQkq - 0 6",
        "rnbqk2r/p3ppbp/2pp1np1/1p6/3PP3/2N1BP2/PPPQ2PP/R3KBNR w KQkq b6 0 7",
        "rnbqk2r/p3ppbp/2pp1np1/1p6/3PP3/2N1BP2/PPPQN1PP/R3KB1R b KQkq - 1 7",
        "r1bqk2r/p2nppbp/2pp1np1/1p6/3PP3/2N1BP2/PPPQN1PP/R3KB1R w KQkq - 2 8",
        "r1bqk2r/p2nppbp/2pp1npB/1p6/3PP3/2N2P2/PPPQN1PP/R3KB1R b KQkq - 3 8",
        "r1bqk2r/p2npp1p/2pp1npb/1p6/3PP3/2N2P2/PPPQN1PP/R3KB1R w KQkq - 0 9",
        "r1bqk2r/p2npp1p/2pp1npQ/1p6/3PP3/2N2P2/PPP1N1PP/R3KB1R b KQkq - 0 9",
        "r2qk2r/pb1npp1p/2pp1npQ/1p6/3PP3/2N2P2/PPP1N1PP/R3KB1R w KQkq - 1 10",
        "r2qk2r/pb1npp1p/2pp1npQ/1p6/3PP3/P1N2P2/1PP1N1PP/R3KB1R b KQkq - 0 10",
        "r2qk2r/pb1n1p1p/2pp1npQ/1p2p3/3PP3/P1N2P2/1PP1N1PP/R3KB1R w KQkq e6 0 11",
        "r2qk2r/pb1n1p1p/2pp1npQ/1p2p3/3PP3/P1N2P2/1PP1N1PP/2KR1B1R b kq - 1 11",
        "r3k2r/pb1nqp1p/2pp1npQ/1p2p3/3PP3/P1N2P2/1PP1N1PP/2KR1B1R w kq - 2 12",
        "r3k2r/pb1nqp1p/2pp1npQ/1p2p3/3PP3/P1N2P2/1PP1N1PP/1K1R1B1R b kq - 3 12",
        "r3k2r/1b1nqp1p/p1pp1npQ/1p2p3/3PP3/P1N2P2/1PP1N1PP/1K1R1B1R w kq - 0 13",
        "r3k2r/1b1nqp1p/p1pp1npQ/1p2p3/3PP3/P1N2P2/1PP3PP/1KNR1B1R b kq - 1 13",
        "2kr3r/1b1nqp1p/p1pp1npQ/1p2p3/3PP3/P1N2P2/1PP3PP/1KNR1B1R w - - 2 14",
        "2kr3r/1b1nqp1p/p1pp1npQ/1p2p3/3PP3/PNN2P2/1PP3PP/1K1R1B1R b - - 3 14",
        "2kr3r/1b1nqp1p/p1pp1npQ/1p6/3pP3/PNN2P2/1PP3PP/1K1R1B1R w - - 0 15",
        "2kr3r/1b1nqp1p/p1pp1npQ/1p6/3RP3/PNN2P2/1PP3PP/1K3B1R b - - 0 15",
        "2kr3r/1b1nqp1p/p2p1npQ/1pp5/3RP3/PNN2P2/1PP3PP/1K3B1R w - - 0 16",
        "2kr3r/1b1nqp1p/p2p1npQ/1pp5/4P3/PNN2P2/1PP3PP/1K1R1B1R b - - 1 16",
        "2kr3r/1b2qp1p/pn1p1npQ/1pp5/4P3/PNN2P2/1PP3PP/1K1R1B1R w - - 2 17",
        "2kr3r/1b2qp1p/pn1p1npQ/1pp5/4P3/PNN2PP1/1PP4P/1K1R1B1R b - - 0 17",
        "1k1r3r/1b2qp1p/pn1p1npQ/1pp5/4P3/PNN2PP1/1PP4P/1K1R1B1R w - - 1 18",
        "1k1r3r/1b2qp1p/pn1p1npQ/Npp5/4P3/P1N2PP1/1PP4P/1K1R1B1R b - - 2 18",
        "bk1r3r/4qp1p/pn1p1npQ/Npp5/4P3/P1N2PP1/1PP4P/1K1R1B1R w - - 3 19",
        "bk1r3r/4qp1p/pn1p1npQ/Npp5/4P3/P1N2PPB/1PP4P/1K1R3R b - - 4 19",
        "bk1r3r/4qp1p/pn3npQ/Nppp4/4P3/P1N2PPB/1PP4P/1K1R3R w - - 0 20",
        "bk1r3r/4qp1p/pn3np1/Nppp4/4PQ2/P1N2PPB/1PP4P/1K1R3R b - - 1 20",
        "b2r3r/k3qp1p/pn3np1/Nppp4/4PQ2/P1N2PPB/1PP4P/1K1R3R w - - 2 21",
        "b2r3r/k3qp1p/pn3np1/Nppp4/4PQ2/P1N2PPB/1PP4P/1K1RR3 b - - 3 21",
        "b2r3r/k3qp1p/pn3np1/Npp5/3pPQ2/P1N2PPB/1PP4P/1K1RR3 w - - 0 22",
        "b2r3r/k3qp1p/pn3np1/NppN4/3pPQ2/P4PPB/1PP4P/1K1RR3 b - - 1 22",
        "b2r3r/k3qp1p/p4np1/Nppn4/3pPQ2/P4PPB/1PP4P/1K1RR3 w - - 0 23",
        "b2r3r/k3qp1p/p4np1/NppP4/3p1Q2/P4PPB/1PP4P/1K1RR3 b - - 0 23",
        "b2r3r/k4p1p/p2q1np1/NppP4/3p1Q2/P4PPB/1PP4P/1K1RR3 w - - 1 24",
        "b2r3r/k4p1p/p2q1np1/NppP4/3R1Q2/P4PPB/1PP4P/1K2R3 b - - 0 24",
        "b2r3r/k4p1p/p2q1np1/Np1P4/3p1Q2/P4PPB/1PP4P/1K2R3 w - - 0 25",
        "b2r3r/k3Rp1p/p2q1np1/Np1P4/3p1Q2/P4PPB/1PP4P/1K6 b - - 1 25",
        "b2r3r/4Rp1p/pk1q1np1/Np1P4/3p1Q2/P4PPB/1PP4P/1K6 w - - 2 26",
        "b2r3r/4Rp1p/pk1q1np1/Np1P4/3Q4/P4PPB/1PP4P/1K6 b - - 0 26",
        "b2r3r/4Rp1p/p2q1np1/kp1P4/3Q4/P4PPB/1PP4P/1K6 w - - 0 27",
        "b2r3r/4Rp1p/p2q1np1/kp1P4/1P1Q4/P4PPB/2P4P/1K6 b - b3 0 27",
        "b2r3r/4Rp1p/p2q1np1/1p1P4/kP1Q4/P4PPB/2P4P/1K6 w - - 1 28",
        "b2r3r/4Rp1p/p2q1np1/1p1P4/kP6/P1Q2PPB/2P4P/1K6 b - - 2 28",
        "b2r3r/4Rp1p/p4np1/1p1q4/kP6/P1Q2PPB/2P4P/1K6 w - - 0 29",
        "b2r3r/R4p1p/p4np1/1p1q4/kP6/P1Q2PPB/2P4P/1K6 b - - 1 29",
        "3r3r/Rb3p1p/p4np1/1p1q4/kP6/P1Q2PPB/2P4P/1K6 w - - 2 30",
        "3r3r/1R3p1p/p4np1/1p1q4/kP6/P1Q2PPB/2P4P/1K6 b - - 0 30",
        "3r3r/1R3p1p/p4np1/1p6/kPq5/P1Q2PPB/2P4P/1K6 w - - 1 31",
        "3r3r/1R3p1p/p4Qp1/1p6/kPq5/P4PPB/2P4P/1K6 b - - 0 31",
        "3r3r/1R3p1p/p4Qp1/1p6/1Pq5/k4PPB/2P4P/1K6 w - - 0 32",
        "3r3r/1R3p1p/Q5p1/1p6/1Pq5/k4PPB/2P4P/1K6 b - - 0 32",
        "3r3r/1R3p1p/Q5p1/1p6/1kq5/5PPB/2P4P/1K6 w - - 0 33",
        "3r3r/1R3p1p/Q5p1/1p6/1kq5/2P2PPB/7P/1K6 b - - 0 33",
        "3r3r/1R3p1p/Q5p1/1p6/2q5/2k2PPB/7P/1K6 w - - 0 34",
        "3r3r/1R3p1p/6p1/1p6/2q5/2k2PPB/7P/QK6 b - - 1 34",
        "3r3r/1R3p1p/6p1/1p6/2q5/5PPB/3k3P/QK6 w - - 2 35",
        "3r3r/1R3p1p/6p1/1p6/2q5/5PPB/1Q1k3P/1K6 b - - 3 35",
        "3r3r/1R3p1p/6p1/1p6/2q5/5PPB/1Q5P/1K1k4 w - - 4 36",
        "3r3r/1R3p1p/6p1/1p6/2q5/5PP1/1Q5P/1K1k1B2 b - - 5 36",
        "7r/1R3p1p/6p1/1p6/2q5/5PP1/1Q1r3P/1K1k1B2 w - - 6 37",
        "7r/3R1p1p/6p1/1p6/2q5/5PP1/1Q1r3P/1K1k1B2 b - - 7 37",
        "7r/3r1p1p/6p1/1p6/2q5/5PP1/1Q5P/1K1k1B2 w - - 0 38",
        "7r/3r1p1p/6p1/1p6/2B5/5PP1/1Q5P/1K1k4 b - - 0 38",
        "7r/3r1p1p/6p1/8/2p5/5PP1/1Q5P/1K1k4 w - - 0 39",
        "7Q/3r1p1p/6p1/8/2p5/5PP1/7P/1K1k4 b - - 0 39",
        "7Q/5p1p/6p1/8/2p5/3r1PP1/7P/1K1k4 w - - 1 40",
        "Q7/5p1p/6p1/8/2p5/3r1PP1/7P/1K1k4 b - - 2 40",
        "Q7/5p1p/6p1/8/8/2pr1PP1/7P/1K1k4 w - - 0 41",
        "8/5p1p/6p1/8/Q7/2pr1PP1/7P/1K1k4 b - - 1 41",
        "8/5p1p/6p1/8/Q7/2pr1PP1/7P/1K2k3 w - - 2 42",
        "8/5p1p/6p1/8/Q4P2/2pr2P1/7P/1K2k3 b - - 0 42",
        "8/7p/6p1/5p2/Q4P2/2pr2P1/7P/1K2k3 w - f6 0 43",
        "8/7p/6p1/5p2/Q4P2/2pr2P1/7P/2K1k3 b - - 1 43",
        "8/7p/6p1/5p2/Q4P2/2p3P1/3r3P/2K1k3 w - - 2 44",
        "8/Q6p/6p1/5p2/5P2/2p3P1/3r3P/2K1k3 b - - 3 44",
    ];

    let mut move_position = Position::start();

    for i in 0..moves.len() {
        let mov = Move::new(moves[i]);
        move_position.do_move(mov);
        let fen_position = Position::load_position_from_fen(fens[i]);
        println!("Move number: {}, Move: {} ", i+1, mov);
        fen_position.print_board();
        assert_eq!(move_position.zobrist(), fen_position.zobrist());
    }


}



#[test]
// random garbage
fn zobrist_game_2() {
    let moves = [5900, 2617, 2332, 6517, 23396, 19326, 1350, 2998, 1749, 2285, 2499, 3517, 19367, 19383, 1669, 12220, 8580, 18166, 1025,3899, 6095, 3452, 2527, 3518, 3047, 2934, 3567, 4021, 65463, 2477, 2878, 1894, 1162, 17563, 1357, 2478, 1422, 17821, 2988, 1878, 17547, 2333, 838, 1958, 781, 1438, 268, 918, 837, 45454, 333, 16710, 708, 1635, 1291, 17733, 724, 1241, 651, 789, 74, 16531, 656, 1218, 17626, ];
    let fens = [
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0",
        "r1bqkbnr/pppppppp/n7/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 1",
        "r1bqkbnr/pppppppp/n7/4P3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0",
        "r1bqkbnr/ppppp1pp/n7/4Pp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0",
        "r1bqkbnr/ppppp1pp/n4P2/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0",
        "r1bqkb1r/ppppp1pp/n4n2/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0",
        "r1bqkb1r/ppppp1pp/n4n2/8/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1",
        "r1bqkb1r/ppppp2p/n4np1/8/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0",
        "r1bqkb1r/ppppp2p/n4np1/8/3N4/8/PPPP1PPP/RNBQKB1R b KQkq - 1",
        "r1bqkb1r/ppppp2p/n5p1/3n4/3N4/8/PPPP1PPP/RNBQKB1R w KQkq - 2",
        "r1bqkb1r/ppppp2p/n5p1/3n3Q/3N4/8/PPPP1PPP/RNB1KB1R b KQkq - 3",
        "r1bqk2r/ppppp1bp/n5p1/3n3Q/3N4/8/PPPP1PPP/RNB1KB1R w KQkq - 4",
        "r1bqk2r/ppppp1bp/n5Q1/3n4/3N4/8/PPPP1PPP/RNB1KB1R b KQkq - 0",
        "r1bqk2r/ppppp1b1/n5p1/3n4/3N4/8/PPPP1PPP/RNB1KB1R w KQkq - 0",
        "r1bqk2r/ppppp1b1/n5p1/3n4/2BN4/8/PPPP1PPP/RNB1K2R b KQkq - 1",
        "r1bq1rk1/ppppp1b1/n5p1/3n4/2BN4/8/PPPP1PPP/RNB1K2R w KQ - 2",
        "r1bq1rk1/ppppp1b1/n5p1/3n4/2BN4/8/PPPP1PPP/RNB2RK1 b - - 3",
        "r1bq1rk1/ppppp3/n5p1/3n4/2Bb4/8/PPPP1PPP/RNB2RK1 w - - 0",
        "r1bq1rk1/ppppp3/n5p1/3n4/2Bb4/N7/PPPP1PPP/R1B2RK1 b - - 1",
        "r1b1qrk1/ppppp3/n5p1/3n4/2Bb4/N7/PPPP1PPP/R1B2RK1 w - - 2",
        "r1b1qrk1/ppppp3/n5p1/3n4/2Bb3P/N7/PPPP1PP1/R1B2RK1 b - h3 0",
        "r1b2rk1/pppppq2/n5p1/3n4/2Bb3P/N7/PPPP1PP1/R1B2RK1 w - - 1",
        "r1b2rk1/pppppq2/n5p1/3n3P/2Bb4/N7/PPPP1PP1/R1B2RK1 b - - 0",
        "r1b2r2/pppppqk1/n5p1/3n3P/2Bb4/N7/PPPP1PP1/R1B2RK1 w - - 1",
        "r1b2r2/pppppqk1/n5pP/3n4/2Bb4/N7/PPPP1PP1/R1B2RK1 b - - 0",
        "r1b2r2/pppppq2/n4kpP/3n4/2Bb4/N7/PPPP1PP1/R1B2RK1 w - - 1",
        "r1b2r2/pppppq1P/n4kp1/3n4/2Bb4/N7/PPPP1PP1/R1B2RK1 b - - 0",
        "r1b2rq1/ppppp2P/n4kp1/3n4/2Bb4/N7/PPPP1PP1/R1B2RK1 w - - 1",
        "r1b2rQ1/ppppp3/n4kp1/3n4/2Bb4/N7/PPPP1PP1/R1B2RK1 b - - 0",
        "r1b2rQ1/ppppp3/n5p1/3n2k1/2Bb4/N7/PPPP1PP1/R1B2RK1 w - - 1",
        "r1b2r2/ppppp3/n3Q1p1/3n2k1/2Bb4/N7/PPPP1PP1/R1B2RK1 b - - 2",
        "r1b2r2/ppppp3/n3Q1p1/3n4/2Bb1k2/N7/PPPP1PP1/R1B2RK1 w - - 3",
        "r1b2r2/ppppp3/n3Q1p1/3n4/2Bb1k2/N1P5/PP1P1PP1/R1B2RK1 b - - 0",
        "r1b2r2/ppppp3/n3Q1p1/3n4/2B2k2/N1b5/PP1P1PP1/R1B2RK1 w - - 0",
        "r1b2r2/ppppp3/n3Q1p1/3n4/2B2k2/N1b2P2/PP1P2P1/R1B2RK1 b - - 0",
        "r1b2r2/ppppp3/n3Q3/3n2p1/2B2k2/N1b2P2/PP1P2P1/R1B2RK1 w - - 0",
        "r1b2r2/ppppp3/n3Q3/3n2p1/2B2k2/N1b2PP1/PP1P4/R1B2RK1 b - - 0",
        "r1b2r2/ppppp3/n3Q3/3n2p1/2B5/N1b2Pk1/PP1P4/R1B2RK1 w - - 0",
        "r1b2r2/ppppp3/n5Q1/3n2p1/2B5/N1b2Pk1/PP1P4/R1B2RK1 b - - 1",
        "r1b2r2/ppppp3/n5Q1/3n2p1/2B2k2/N1b2P2/PP1P4/R1B2RK1 w - - 2",
        "r1b2r2/ppppp3/n5Q1/3n2p1/2B2k2/N1P2P2/PP6/R1B2RK1 b - - 0",
        "r1b2r2/ppppp3/n5Q1/3nk1p1/2B5/N1P2P2/PP6/R1B2RK1 w - - 1",
        "r1b2r2/ppppp3/n5Q1/3nk1p1/2B5/N1P2P2/PP3K2/R1B2R2 b - - 2",
        "r1b2r2/ppppp3/n5Q1/3nk3/2B3p1/N1P2P2/PP3K2/R1B2R2 w - - 0",
        "r1b2r2/ppppp3/n5Q1/3nk3/2B3p1/N1P2P2/PP2K3/R1B2R2 b - - 1",
        "r1b2r2/ppppp3/n5Q1/3nk3/2B5/N1P2Pp1/PP2K3/R1B2R2 w - - 0",
        "r1b2r2/ppppp3/n5Q1/3nk3/2B5/N1P2Pp1/PP6/R1B1KR2 b - - 1",
        "r1b2r2/ppppp3/n5Q1/3nk3/2B5/N1P2P2/PP4p1/R1B1KR2 w - - 0",
        "r1b2r2/ppppp3/n5Q1/3nk3/2B5/N1P2P2/PP3Rp1/R1B1K3 b - - 1",
        "r1b2r2/ppppp3/n5Q1/3nk3/2B5/N1P2P2/PP3R2/R1B1K1q1 w - - 0",
        "r1b2r2/ppppp3/n5Q1/3nk3/2B5/N1P2P2/PP6/R1B1KRq1 b - - 1",
        "r1b2r2/ppppp3/n5Q1/3nk3/2B5/N1P2P2/PP6/R1B1Kq2 w - - 0",
        "r1b2r2/ppppp3/n5Q1/3nk3/2B5/N1P2P2/PP1K4/R1B2q2 b - - 1",
        "r1b2r2/ppppp3/n5Q1/4k3/1nB5/N1P2P2/PP1K4/R1B2q2 w - - 2",
        "r1b2r2/ppppp3/n5Q1/4k3/1nB5/N1P1KP2/PP6/R1B2q2 b - - 3",
        "r1b2r2/ppppp3/n5Q1/4k3/1nB5/N1P1Kq2/PP6/R1B5 w - - 0",
        "r1b2r2/ppppp3/n5Q1/4k3/1nB5/N1P2q2/PP1K4/R1B5 b - - 1",
        "r1b2r2/ppppp3/n5Q1/4k3/2B5/N1Pn1q2/PP1K4/R1B5 w - - 2",
        "r1b2r2/ppppp3/n5Q1/4k3/2B5/N1Pn1q2/PPK5/R1B5 b - - 3",
        "r1b2r2/ppppp3/n5Q1/4k3/2B5/N1Pn4/PPK1q3/R1B5 w - - 4",
        "r1b2r2/ppppp3/n5Q1/4k3/2B5/N1Pn4/PP2q3/RKB5 b - - 5",
        "r1b2r2/ppppp3/n5Q1/4k3/2B5/N1P5/PP2q3/RKn5 w - - 0",
        "r1b2r2/ppppp3/n5Q1/4k3/2B5/2P5/PPN1q3/RKn5 b - - 1",
        "r1b2r2/ppppp3/n5Q1/4k3/2B5/2Pn4/PPN1q3/RK6 w - - 2",
        "r1b2r2/ppppp3/n5Q1/4k3/8/2PB4/PPN1q3/RK6 b - - 0",
    ];

    let mut move_position = Position::start();

    for i in 0..moves.len() {
        let mov = Move::new(moves[i]);
        move_position.do_move(mov);
        let fen_position = Position::load_position_from_fen(fens[i]);
        println!("Move number: {}, Move: {} ", i+1, mov);
        fen_position.print_board();
        assert_eq!(move_position.zobrist(), fen_position.zobrist());
    }
}






#[test]
fn run_dfs_zobrist_check_suite() {
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
        dfs_zobrist_check(&mut position, 4);
        bar.inc(1);
    }
    bar.finish_with_message("All positions completed");
}


pub fn dfs_zobrist_check(position: &mut Position, depth: u8) {
    let old_hash = position.zobrist();
    assert_eq!(old_hash, Position::load_position_from_fen(&*position.to_fen()).zobrist());

    if depth == 0 {
        return;
    }

    for mov in all_moves(position).iter() {
        position.do_move(mov);
        dfs_zobrist_check(position, depth-1);
        position.undo_move();

        assert_eq!(position.zobrist(), old_hash);
    }
}
