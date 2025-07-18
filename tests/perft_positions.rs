pub struct PerftEntry<'a> {
    pub fen: &'a str,
    pub expected: [u64; 6],
}
pub const PERFT_POSITIONS: &[PerftEntry] = &[
    PerftEntry {
        fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -",
        expected: [20, 400, 8902, 197281, 4865609, 119060324],
    },
    PerftEntry {
        fen: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
        expected: [48, 2039, 97862, 4085603, 193690690, 8031647685],
    },
    PerftEntry {
        fen: "4k3/8/8/8/8/8/8/4K2R w K -",
        expected: [15, 66, 1197, 7059, 133987, 764643],
    },
    PerftEntry {
        fen: "4k3/8/8/8/8/8/8/R3K3 w Q -",
        expected: [16, 71, 1287, 7626, 145232, 846648],
    },
    PerftEntry {
        fen: "4k2r/8/8/8/8/8/8/4K3 w k -",
        expected: [5, 75, 459, 8290, 47635, 899442],
    },
    PerftEntry {
        fen: "r3k3/8/8/8/8/8/8/4K3 w q -",
        expected: [5, 80, 493, 8897, 52710, 1001523],
    },
    PerftEntry {
        fen: "4k3/8/8/8/8/8/8/R3K2R w KQ -",
        expected: [26, 112, 3189, 17945, 532933, 2788982],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/4K3 w kq -",
        expected: [5, 130, 782, 22180, 118882, 3517770],
    },
    PerftEntry {
        fen: "8/8/8/8/8/8/6k1/4K2R w K -",
        expected: [12, 38, 564, 2219, 37735, 185867],
    },
    PerftEntry {
        fen: "8/8/8/8/8/8/1k6/R3K3 w Q -",
        expected: [15, 65, 1018, 4573, 80619, 413018],
    },
    PerftEntry {
        fen: "4k2r/6K1/8/8/8/8/8/8 w k -",
        expected: [3, 32, 134, 2073, 10485, 179869],
    },
    PerftEntry {
        fen: "r3k3/1K6/8/8/8/8/8/8 w q -",
        expected: [4, 49, 243, 3991, 20780, 367724],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/R3K2R w KQkq -",
        expected: [26, 568, 13744, 314346, 7594526, 179862938],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/1R2K2R w Kkq -",
        expected: [25, 567, 14095, 328965, 8153719, 195629489],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/2R1K2R w Kkq -",
        expected: [25, 548, 13502, 312835, 7736373, 184411439],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/R3K1R1 w Qkq -",
        expected: [25, 547, 13579, 316214, 7878456, 189224276],
    },
    PerftEntry {
        fen: "1r2k2r/8/8/8/8/8/8/R3K2R w KQk -",
        expected: [26, 583, 14252, 334705, 8198901, 198328929],
    },
    PerftEntry {
        fen: "2r1k2r/8/8/8/8/8/8/R3K2R w KQk -",
        expected: [25, 560, 13592, 317324, 7710115, 185959088],
    },
    PerftEntry {
        fen: "r3k1r1/8/8/8/8/8/8/R3K2R w KQq -",
        expected: [25, 560, 13607, 320792, 7848606, 190755813],
    },
    PerftEntry {
        fen: "4k3/8/8/8/8/8/8/4K2R b K -",
        expected: [5, 75, 459, 8290, 47635, 899442],
    },
    PerftEntry {
        fen: "4k3/8/8/8/8/8/8/R3K3 b Q -",
        expected: [5, 80, 493, 8897, 52710, 1001523],
    },
    PerftEntry {
        fen: "4k2r/8/8/8/8/8/8/4K3 b k -",
        expected: [15, 66, 1197, 7059, 133987, 764643],
    },
    PerftEntry {
        fen: "r3k3/8/8/8/8/8/8/4K3 b q -",
        expected: [16, 71, 1287, 7626, 145232, 846648],
    },
    PerftEntry {
        fen: "4k3/8/8/8/8/8/8/R3K2R b KQ -",
        expected: [5, 130, 782, 22180, 118882, 3517770],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/4K3 b kq -",
        expected: [26, 112, 3189, 17945, 532933, 2788982],
    },
    PerftEntry {
        fen: "8/8/8/8/8/8/6k1/4K2R b K -",
        expected: [3, 32, 134, 2073, 10485, 179869],
    },
    PerftEntry {
        fen: "8/8/8/8/8/8/1k6/R3K3 b Q -",
        expected: [4, 49, 243, 3991, 20780, 367724],
    },
    PerftEntry {
        fen: "4k2r/6K1/8/8/8/8/8/8 b k -",
        expected: [12, 38, 564, 2219, 37735, 185867],
    },
    PerftEntry {
        fen: "r3k3/1K6/8/8/8/8/8/8 b q -",
        expected: [15, 65, 1018, 4573, 80619, 413018],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/R3K2R b KQkq -",
        expected: [26, 568, 13744, 314346, 7594526, 179862938],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/1R2K2R b Kkq -",
        expected: [26, 583, 14252, 334705, 8198901, 198328929],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/2R1K2R b Kkq -",
        expected: [25, 560, 13592, 317324, 7710115, 185959088],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/R3K1R1 b Qkq -",
        expected: [25, 560, 13607, 320792, 7848606, 190755813],
    },
    PerftEntry {
        fen: "1r2k2r/8/8/8/8/8/8/R3K2R b KQk -",
        expected: [25, 567, 14095, 328965, 8153719, 195629489],
    },
    PerftEntry {
        fen: "2r1k2r/8/8/8/8/8/8/R3K2R b KQk -",
        expected: [25, 548, 13502, 312835, 7736373, 184411439],
    },
    PerftEntry {
        fen: "r3k1r1/8/8/8/8/8/8/R3K2R b KQq -",
        expected: [25, 547, 13579, 316214, 7878456, 189224276],
    },
    PerftEntry {
        fen: "8/1n4N1/2k5/8/8/5K2/1N4n1/8 w - -",
        expected: [14, 195, 2760, 38675, 570726, 8107539],
    },
    PerftEntry {
        fen: "8/1k6/8/5N2/8/4n3/8/2K5 w - -",
        expected: [11, 156, 1636, 20534, 223507, 2594412],
    },
    PerftEntry {
        fen: "8/8/4k3/3Nn3/3nN3/4K3/8/8 w - -",
        expected: [19, 289, 4442, 73584, 1198299, 19870403],
    },
    PerftEntry {
        fen: "K7/8/2n5/1n6/8/8/8/k6N w - -",
        expected: [3, 51, 345, 5301, 38348, 588695],
    },
    PerftEntry {
        fen: "k7/8/2N5/1N6/8/8/8/K6n w - -",
        expected: [17, 54, 835, 5910, 92250, 688780],
    },
    PerftEntry {
        fen: "8/1n4N1/2k5/8/8/5K2/1N4n1/8 b - -",
        expected: [15, 193, 2816, 40039, 582642, 8503277],
    },
    PerftEntry {
        fen: "8/1k6/8/5N2/8/4n3/8/2K5 b - -",
        expected: [16, 180, 2290, 24640, 288141, 3147566],
    },
    PerftEntry {
        fen: "8/8/3K4/3Nn3/3nN3/4k3/8/8 b - -",
        expected: [4, 68, 1118, 16199, 281190, 4405103],
    },
    PerftEntry {
        fen: "K7/8/2n5/1n6/8/8/8/k6N b - -",
        expected: [17, 54, 835, 5910, 92250, 688780],
    },
    PerftEntry {
        fen: "k7/8/2N5/1N6/8/8/8/K6n b - -",
        expected: [3, 51, 345, 5301, 38348, 588695],
    },
    PerftEntry {
            fen: "B6b/8/8/8/2K5/4k3/8/b6B w - -",
        expected: [17, 278, 4607, 76778, 1320507, 22823890],
    },
    PerftEntry {
        fen: "8/8/1B6/7b/7k/8/2B1b3/7K w - -",
        expected: [21, 316, 5744, 93338, 1713368, 28861171],
    },
    PerftEntry {
        fen: "k7/B7/1B6/1B6/8/8/8/K6b w - -",
        expected: [21, 144, 3242, 32955, 787524, 7881673],
    },
    PerftEntry {
        fen: "K7/b7/1b6/1b6/8/8/8/k6B w - -",
        expected: [7, 143, 1416, 31787, 310862, 7382896],
    },
    PerftEntry {
        fen: "B6b/8/8/8/2K5/5k2/8/b6B b - -",
        expected: [6, 106, 1829, 31151, 530585, 9250746],
    },
    PerftEntry {
        fen: "8/8/1B6/7b/7k/8/2B1b3/7K b - -",
        expected: [17, 309, 5133, 93603, 1591064, 29027891],
    },
    PerftEntry {
        fen: "k7/B7/1B6/1B6/8/8/8/K6b b - -",
        expected: [7, 143, 1416, 31787, 310862, 7382896],
    },
    PerftEntry {
        fen: "K7/b7/1b6/1b6/8/8/8/k6B b - -",
        expected: [21, 144, 3242, 32955, 787524, 7881673],
    },
    PerftEntry {
        fen: "7k/RR6/8/8/8/8/rr6/7K w - -",
        expected: [19, 275, 5300, 104342, 2161211, 44956585],
    },
    PerftEntry {
        fen: "R6r/8/8/2K5/5k2/8/8/r6R w - -",
        expected: [36, 1027, 29215, 771461, 20506480, 525169084],
    },
    PerftEntry {
        fen: "7k/RR6/8/8/8/8/rr6/7K b - -",
        expected: [19, 275, 5300, 104342, 2161211, 44956585],
    },
    PerftEntry {
        fen: "R6r/8/8/2K5/5k2/8/8/r6R b - -",
        expected: [36, 1027, 29227, 771368, 20521342, 524966748],
    },
    PerftEntry {
        fen: "6kq/8/8/8/8/8/8/7K w - -",
        expected: [2, 36, 143, 3637, 14893, 391507],
    },
    PerftEntry {
        fen: "6KQ/8/8/8/8/8/8/7k b - -",
        expected: [2, 36, 143, 3637, 14893, 391507],
    },
    PerftEntry {
        fen: "K7/8/8/3Q4/4q3/8/8/7k w - -",
        expected: [6, 35, 495, 8349, 166741, 3370175],
    },
    PerftEntry {
        fen: "6qk/8/8/8/8/8/8/7K b - -",
        expected: [22, 43, 1015, 4167, 105749, 419369],
    },
    PerftEntry {
        fen: "6KQ/8/8/8/8/8/8/7k b - -",
        expected: [2, 36, 143, 3637, 14893, 391507],
    },
    PerftEntry {
        fen: "K7/8/8/3Q4/4q3/8/8/7k b - -",
        expected: [6, 35, 495, 8349, 166741, 3370175],
    },
    PerftEntry {
        fen: "8/8/8/8/8/K7/P7/k7 w - -",
        expected: [3, 7, 43, 199, 1347, 6249],
    },
    PerftEntry {
        fen: "8/8/8/8/8/7K/7P/7k w - -",
        expected: [3, 7, 43, 199, 1347, 6249],
    },
    PerftEntry {
        fen: "K7/p7/k7/8/8/8/8/8 w - -",
        expected: [1, 3, 12, 80, 342, 2343],
    },
    PerftEntry {
        fen: "7K/7p/7k/8/8/8/8/8 w - -",
        expected: [1, 3, 12, 80, 342, 2343],
    },
    PerftEntry {
        fen: "8/2k1p3/3pP3/3P2K1/8/8/8/8 w - -",
        expected: [7, 35, 210, 1091, 7028, 34834],
    },
    PerftEntry {
        fen: "8/8/8/8/8/K7/P7/k7 b - -",
        expected: [1, 3, 12, 80, 342, 2343],
    },
    PerftEntry {
        fen: "8/8/8/8/8/7K/7P/7k b - -",
        expected: [1, 3, 12, 80, 342, 2343],
    },
    PerftEntry {
        fen: "K7/p7/k7/8/8/8/8/8 b - -",
        expected: [3, 7, 43, 199, 1347, 6249],
    },
    PerftEntry {
        fen: "7K/7p/7k/8/8/8/8/8 b - -",
        expected: [3, 7, 43, 199, 1347, 6249],
    },
    PerftEntry {
        fen: "8/2k1p3/3pP3/3P2K1/8/8/8/8 b - -",
        expected: [5, 35, 182, 1091, 5408, 34822],
    },
    PerftEntry {
        fen: "8/8/8/8/8/4k3/4P3/4K3 w - -",
        expected: [2, 8, 44, 282, 1814, 11848],
    },
    PerftEntry {
        fen: "4k3/4p3/4K3/8/8/8/8/8 b - -",
        expected: [2, 8, 44, 282, 1814, 11848],
    },
    PerftEntry {
        fen: "8/8/7k/7p/7P/7K/8/8 w - -",
        expected: [3, 9, 57, 360, 1969, 10724],
    },
    PerftEntry {
        fen: "8/8/k7/p7/P7/K7/8/8 w - -",
        expected: [3, 9, 57, 360, 1969, 10724],
    },
    PerftEntry {
        fen: "8/8/3k4/3p4/3P4/3K4/8/8 w - -",
        expected: [5, 25, 180, 1294, 8296, 53138],
    },
    PerftEntry {
        fen: "8/3k4/3p4/8/3P4/3K4/8/8 w - -",
        expected: [8, 61, 483, 3213, 23599, 157093],
    },
    PerftEntry {
        fen: "8/8/3k4/3p4/8/3P4/3K4/8 w - -",
        expected: [8, 61, 411, 3213, 21637, 158065],
    },
    PerftEntry {
        fen: "k7/8/3p4/8/3P4/8/8/7K w - -",
        expected: [4, 15, 90, 534, 3450, 20960],
    },
    PerftEntry {
        fen: "8/8/7k/7p/7P/7K/8/8 b - -",
        expected: [3, 9, 57, 360, 1969, 10724],
    },
    PerftEntry {
        fen: "8/8/k7/p7/P7/K7/8/8 b - -",
        expected: [3, 9, 57, 360, 1969, 10724],
    },
    PerftEntry {
        fen: "8/8/3k4/3p4/3P4/3K4/8/8 b - -",
        expected: [5, 25, 180, 1294, 8296, 53138],
    },
    PerftEntry {
        fen: "8/3k4/3p4/8/3P4/3K4/8/8 b - -",
        expected: [8, 61, 411, 3213, 21637, 158065],
    },
    PerftEntry {
        fen: "8/8/3k4/3p4/8/3P4/3K4/8 b - -",
        expected: [8, 61, 483, 3213, 23599, 157093],
    },
    PerftEntry {
        fen: "k7/8/3p4/8/3P4/8/8/7K b - -",
        expected: [4, 15, 89, 537, 3309, 21104],
    },
    PerftEntry {
        fen: "7k/3p4/8/8/3P4/8/8/K7 w - -",
        expected: [4, 19, 117, 720, 4661, 32191],
    },
    PerftEntry {
        fen: "7k/8/8/3p4/8/8/3P4/K7 w - -",
        expected: [5, 19, 116, 716, 4786, 30980],
    },
    PerftEntry {
        fen: "k7/8/8/7p/6P1/8/8/K7 w - -",
        expected: [5, 22, 139, 877, 6112, 41874],
    },
    PerftEntry {
        fen: "k7/8/7p/8/8/6P1/8/K7 w - -",
        expected: [4, 16, 101, 637, 4354, 29679],
    },
    PerftEntry {
        fen: "k7/8/8/6p1/7P/8/8/K7 w - -",
        expected: [5, 22, 139, 877, 6112, 41874],
    },
    PerftEntry {
        fen: "k7/8/6p1/8/8/7P/8/K7 w - -",
        expected: [4, 16, 101, 637, 4354, 29679],
    },
    PerftEntry {
        fen: "k7/8/8/3p4/4p3/8/8/7K w - -",
        expected: [3, 15, 84, 573, 3013, 22886],
    },
    PerftEntry {
        fen: "k7/8/3p4/8/8/4P3/8/7K w - -",
        expected: [4, 16, 101, 637, 4271, 28662],
    },
    PerftEntry {
        fen: "7k/3p4/8/8/3P4/8/8/K7 b - -",
        expected: [5, 19, 117, 720, 5014, 32167],
    },
    PerftEntry {
        fen: "7k/8/8/3p4/8/8/3P4/K7 b - -",
        expected: [4, 19, 117, 712, 4658, 30749],
    },
    PerftEntry {
        fen: "k7/8/8/7p/6P1/8/8/K7 b - -",
        expected: [5, 22, 139, 877, 6112, 41874],
    },
    PerftEntry {
        fen: "k7/8/7p/8/8/6P1/8/K7 b - -",
        expected: [4, 16, 101, 637, 4354, 29679],
    },
    PerftEntry {
        fen: "k7/8/8/6p1/7P/8/8/K7 b - -",
        expected: [5, 22, 139, 877, 6112, 41874],
    },
    PerftEntry {
        fen: "k7/8/6p1/8/8/7P/8/K7 b - -",
        expected: [4, 16, 101, 637, 4354, 29679],
    },
    PerftEntry {
        fen: "k7/8/8/3p4/4p3/8/8/7K b - -",
        expected: [5, 15, 102, 569, 4337, 22579],
    },
    PerftEntry {
        fen: "k7/8/3p4/8/8/4P3/8/7K b - -",
        expected: [4, 16, 101, 637, 4271, 28662],
    },
    PerftEntry {
        fen: "7k/8/8/p7/1P6/8/8/7K w - -",
        expected: [5, 22, 139, 877, 6112, 41874],
    },
    PerftEntry {
        fen: "7k/8/p7/8/8/1P6/8/7K w - -",
        expected: [4, 16, 101, 637, 4354, 29679],
    },
    PerftEntry {
        fen: "7k/8/8/1p6/P7/8/8/7K w - -",
        expected: [5, 22, 139, 877, 6112, 41874],
    },
    PerftEntry {
        fen: "7k/8/1p6/8/8/P7/8/7K w - -",
        expected: [4, 16, 101, 637, 4354, 29679],
    },
    PerftEntry {
        fen: "k7/7p/8/8/8/8/6P1/K7 w - -",
        expected: [5, 25, 161, 1035, 7574, 55338],
    },
    PerftEntry {
        fen: "k7/6p1/8/8/8/8/7P/K7 w - -",
        expected: [5, 25, 161, 1035, 7574, 55338],
    },
    PerftEntry {
        fen: "3k4/3pp3/8/8/8/8/3PP3/3K4 w - -",
        expected: [7, 49, 378, 2902, 24122, 199002],
    },
    PerftEntry {
        fen: "7k/8/8/p7/1P6/8/8/7K b - -",
        expected: [5, 22, 139, 877, 6112, 41874],
    },
    PerftEntry {
        fen: "7k/8/p7/8/8/1P6/8/7K b - -",
        expected: [4, 16, 101, 637, 4354, 29679],
    },
    PerftEntry {
        fen: "7k/8/8/1p6/P7/8/8/7K b - -",
        expected: [5, 22, 139, 877, 6112, 41874],
    },
    PerftEntry {
        fen: "7k/8/1p6/8/8/P7/8/7K b - -",
        expected: [4, 16, 101, 637, 4354, 29679],
    },
    PerftEntry {
        fen: "k7/7p/8/8/8/8/6P1/K7 b - -",
        expected: [5, 25, 161, 1035, 7574, 55338],
    },
    PerftEntry {
        fen: "k7/6p1/8/8/8/8/7P/K7 b - -",
        expected: [5, 25, 161, 1035, 7574, 55338],
    },
    PerftEntry {
        fen: "3k4/3pp3/8/8/8/8/3PP3/3K4 b - -",
        expected: [7, 49, 378, 2902, 24122, 199002],
    },
    PerftEntry {
        fen: "8/Pk6/8/8/8/8/6Kp/8 w - -",
        expected: [11, 97, 887, 8048, 90606, 1030499],
    },
    PerftEntry {
        fen: "n1n5/1Pk5/8/8/8/8/5Kp1/5N1N w - -",
        expected: [24, 421, 7421, 124608, 2193768, 37665329],
    },
    PerftEntry {
        fen: "8/PPPk4/8/8/8/8/4Kppp/8 w - -",
        expected: [18, 270, 4699, 79355, 1533145, 28859283],
    },
    PerftEntry {
        fen: "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N w - -",
        expected: [24, 496, 9483, 182838, 3605103, 71179139],
    },
    PerftEntry {
        fen: "8/Pk6/8/8/8/8/6Kp/8 b - -",
        expected: [11, 97, 887, 8048, 90606, 1030499],
    },
    PerftEntry {
        fen: "n1n5/1Pk5/8/8/8/8/5Kp1/5N1N b - -",
        expected: [24, 421, 7421, 124608, 2193768, 37665329],
    },
    PerftEntry {
        fen: "8/PPPk4/8/8/8/8/4Kppp/8 b - -",
        expected: [18, 270, 4699, 79355, 1533145, 28859283],
    },
    PerftEntry {
        fen: "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - -",
        expected: [24, 496, 9483, 182838, 3605103, 71179139],
    },
    PerftEntry {
        fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -",
        expected: [20, 400, 8902, 197281, 4865609, 119060324],
    },
    PerftEntry {
        fen: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
        expected: [48, 2039, 97862, 4085603, 193690690, 8031647685],
    },
    PerftEntry {
        fen: "4k3/8/8/8/8/8/8/4K2R w K -",
        expected: [15, 66, 1197, 7059, 133987, 764643],
    },
    PerftEntry {
        fen: "4k3/8/8/8/8/8/8/R3K3 w Q -",
        expected: [16, 71, 1287, 7626, 145232, 846648],
    },
    PerftEntry {
        fen: "4k2r/8/8/8/8/8/8/4K3 w k -",
        expected: [5, 75, 459, 8290, 47635, 899442],
    },
    PerftEntry {
        fen: "r3k3/8/8/8/8/8/8/4K3 w q -",
        expected: [5, 80, 493, 8897, 52710, 1001523],
    },
    PerftEntry {
        fen: "4k3/8/8/8/8/8/8/R3K2R w KQ -",
        expected: [26, 112, 3189, 17945, 532933, 2788982],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/4K3 w kq -",
        expected: [5, 130, 782, 22180, 118882, 3517770],
    },
    PerftEntry {
        fen: "8/8/8/8/8/8/6k1/4K2R w K -",
        expected: [12, 38, 564, 2219, 37735, 185867],
    },
    PerftEntry {
        fen: "8/8/8/8/8/8/1k6/R3K3 w Q -",
        expected: [15, 65, 1018, 4573, 80619, 413018],
    },
    PerftEntry {
        fen: "4k2r/6K1/8/8/8/8/8/8 w k -",
        expected: [3, 32, 134, 2073, 10485, 179869],
    },
    PerftEntry {
        fen: "r3k3/1K6/8/8/8/8/8/8 w q -",
        expected: [4, 49, 243, 3991, 20780, 367724],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/R3K2R w KQkq -",
        expected: [26, 568, 13744, 314346, 7594526, 179862938],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/1R2K2R w Kkq -",
        expected: [25, 567, 14095, 328965, 8153719, 195629489],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/2R1K2R w Kkq -",
        expected: [25, 548, 13502, 312835, 7736373, 184411439],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/R3K1R1 w Qkq -",
        expected: [25, 547, 13579, 316214, 7878456, 189224276],
    },
    PerftEntry {
        fen: "1r2k2r/8/8/8/8/8/8/R3K2R w KQk -",
        expected: [26, 583, 14252, 334705, 8198901, 198328929],
    },
    PerftEntry {
        fen: "2r1k2r/8/8/8/8/8/8/R3K2R w KQk -",
        expected: [25, 560, 13592, 317324, 7710115, 185959088],
    },
    PerftEntry {
        fen: "r3k1r1/8/8/8/8/8/8/R3K2R w KQq -",
        expected: [25, 560, 13607, 320792, 7848606, 190755813],
    },
    PerftEntry {
        fen: "4k3/8/8/8/8/8/8/4K2R b K -",
        expected: [5, 75, 459, 8290, 47635, 899442],
    },
    PerftEntry {
        fen: "4k3/8/8/8/8/8/8/R3K3 b Q -",
        expected: [5, 80, 493, 8897, 52710, 1001523],
    },
    PerftEntry {
        fen: "4k2r/8/8/8/8/8/8/4K3 b k -",
        expected: [15, 66, 1197, 7059, 133987, 764643],
    },
    PerftEntry {
        fen: "r3k3/8/8/8/8/8/8/4K3 b q -",
        expected: [16, 71, 1287, 7626, 145232, 846648],
    },
    PerftEntry {
        fen: "4k3/8/8/8/8/8/8/R3K2R b KQ -",
        expected: [5, 130, 782, 22180, 118882, 3517770],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/4K3 b kq -",
        expected: [26, 112, 3189, 17945, 532933, 2788982],
    },
    PerftEntry {
        fen: "8/8/8/8/8/8/6k1/4K2R b K -",
        expected: [3, 32, 134, 2073, 10485, 179869],
    },
    PerftEntry {
        fen: "8/8/8/8/8/8/1k6/R3K3 b Q -",
        expected: [4, 49, 243, 3991, 20780, 367724],
    },
    PerftEntry {
        fen: "4k2r/6K1/8/8/8/8/8/8 b k -",
        expected: [12, 38, 564, 2219, 37735, 185867],
    },
    PerftEntry {
        fen: "r3k3/1K6/8/8/8/8/8/8 b q -",
        expected: [15, 65, 1018, 4573, 80619, 413018],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/R3K2R b KQkq -",
        expected: [26, 568, 13744, 314346, 7594526, 179862938],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/1R2K2R b Kkq -",
        expected: [26, 583, 14252, 334705, 8198901, 198328929],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/2R1K2R b Kkq -",
        expected: [25, 560, 13592, 317324, 7710115, 185959088],
    },
    PerftEntry {
        fen: "r3k2r/8/8/8/8/8/8/R3K1R1 b Qkq -",
        expected: [25, 560, 13607, 320792, 7848606, 190755813],
    },
    PerftEntry {
        fen: "1r2k2r/8/8/8/8/8/8/R3K2R b KQk -",
        expected: [25, 567, 14095, 328965, 8153719, 195629489],
    },
    PerftEntry {
        fen: "2r1k2r/8/8/8/8/8/8/R3K2R b KQk -",
        expected: [25, 548, 13502, 312835, 7736373, 184411439],
    },
    PerftEntry {
        fen: "r3k1r1/8/8/8/8/8/8/R3K2R b KQq -",
        expected: [25, 547, 13579, 316214, 7878456, 189224276],
    },
    PerftEntry {
        fen: "8/1n4N1/2k5/8/8/5K2/1N4n1/8 w - -",
        expected: [14, 195, 2760, 38675, 570726, 8107539],
    },
    PerftEntry {
        fen: "8/1k6/8/5N2/8/4n3/8/2K5 w - -",
        expected: [11, 156, 1636, 20534, 223507, 2594412],
    },
    PerftEntry {
        fen: "8/8/4k3/3Nn3/3nN3/4K3/8/8 w - -",
        expected: [19, 289, 4442, 73584, 1198299, 19870403],
    },
    PerftEntry {
        fen: "K7/8/2n5/1n6/8/8/8/k6N w - -",
        expected: [3, 51, 345, 5301, 38348, 588695],
    },
    PerftEntry {
        fen: "k7/8/2N5/1N6/8/8/8/K6n w - -",
        expected: [17, 54, 835, 5910, 92250, 688780],
    },
    PerftEntry {
        fen: "8/1n4N1/2k5/8/8/5K2/1N4n1/8 b - -",
        expected: [15, 193, 2816, 40039, 582642, 8503277],
    },
    PerftEntry {
        fen: "8/1k6/8/5N2/8/4n3/8/2K5 b - -",
        expected: [16, 180, 2290, 24640, 288141, 3147566],
    },
    PerftEntry {
        fen: "8/8/3K4/3Nn3/3nN3/4k3/8/8 b - -",
        expected: [4, 68, 1118, 16199, 281190, 4405103],
    },
    PerftEntry {
        fen: "K7/8/2n5/1n6/8/8/8/k6N b - -",
        expected: [17, 54, 835, 5910, 92250, 688780],
    },
    PerftEntry {
        fen: "k7/8/2N5/1N6/8/8/8/K6n b - -",
        expected: [3, 51, 345, 5301, 38348, 588695],
    },
    PerftEntry {
        fen: "B6b/8/8/8/2K5/4k3/8/b6B w - -",
        expected: [17, 278, 4607, 76778, 1320507, 22823890],
    },
    PerftEntry {
        fen: "8/8/1B6/7b/7k/8/2B1b3/7K w - -",
        expected: [21, 316, 5744, 93338, 1713368, 28861171],
    },
    PerftEntry {
        fen: "k7/B7/1B6/1B6/8/8/8/K6b w - -",
        expected: [21, 144, 3242, 32955, 787524, 7881673],
    },
    PerftEntry {
        fen: "K7/b7/1b6/1b6/8/8/8/k6B w - -",
        expected: [7, 143, 1416, 31787, 310862, 7382896],
    },
    PerftEntry {
        fen: "B6b/8/8/8/2K5/5k2/8/b6B b - -",
        expected: [6, 106, 1829, 31151, 530585, 9250746],
    },
    PerftEntry {
        fen: "8/8/1B6/7b/7k/8/2B1b3/7K b - -",
        expected: [17, 309, 5133, 93603, 1591064, 29027891],
    },
    PerftEntry {
        fen: "k7/B7/1B6/1B6/8/8/8/K6b b - -",
        expected: [7, 143, 1416, 31787, 310862, 7382896],
    },
    PerftEntry {
        fen: "K7/b7/1b6/1b6/8/8/8/k6B b - -",
        expected: [21, 144, 3242, 32955, 787524, 7881673],
    },
    PerftEntry {
        fen: "7k/RR6/8/8/8/8/rr6/7K w - -",
        expected: [19, 275, 5300, 104342, 2161211, 44956585],
    },
    PerftEntry {
        fen: "R6r/8/8/2K5/5k2/8/8/r6R w - -",
        expected: [36, 1027, 29215, 771461, 20506480, 525169084],
    },
    PerftEntry {
        fen: "7k/RR6/8/8/8/8/rr6/7K b - -",
        expected: [19, 275, 5300, 104342, 2161211, 44956585],
    },
    PerftEntry {
        fen: "R6r/8/8/2K5/5k2/8/8/r6R b - -",
        expected: [36, 1027, 29227, 771368, 20521342, 524966748],
    },
    PerftEntry {
        fen: "6kq/8/8/8/8/8/8/7K w - -",
        expected: [2, 36, 143, 3637, 14893, 391507],
    },
    PerftEntry {
        fen: "6KQ/8/8/8/8/8/8/7k b - -",
        expected: [2, 36, 143, 3637, 14893, 391507],
    },
    PerftEntry {
        fen: "K7/8/8/3Q4/4q3/8/8/7k w - -",
        expected: [6, 35, 495, 8349, 166741, 3370175],
    },
    PerftEntry {
        fen: "6qk/8/8/8/8/8/8/7K b - -",
        expected: [22, 43, 1015, 4167, 105749, 419369],
    },
    PerftEntry {
        fen: "6KQ/8/8/8/8/8/8/7k b - -",
        expected: [2, 36, 143, 3637, 14893, 391507],
    },
    PerftEntry {
        fen: "K7/8/8/3Q4/4q3/8/8/7k b - -",
        expected: [6, 35, 495, 8349, 166741, 3370175],
    },
    PerftEntry {
        fen: "8/8/8/8/8/K7/P7/k7 w - -",
        expected: [3, 7, 43, 199, 1347, 6249],
    },
    PerftEntry {
        fen: "8/8/8/8/8/7K/7P/7k w - -",
        expected: [3, 7, 43, 199, 1347, 6249],
    },
    PerftEntry {
        fen: "K7/p7/k7/8/8/8/8/8 w - -",
        expected: [1, 3, 12, 80, 342, 2343],
    },
    PerftEntry {
        fen: "7K/7p/7k/8/8/8/8/8 w - -",
        expected: [1, 3, 12, 80, 342, 2343],
    },
    PerftEntry {
        fen: "8/2k1p3/3pP3/3P2K1/8/8/8/8 w - -",
        expected: [7, 35, 210, 1091, 7028, 34834],
    },
    PerftEntry {
        fen: "8/8/8/8/8/K7/P7/k7 b - -",
        expected: [1, 3, 12, 80, 342, 2343],
    },
    PerftEntry {
        fen: "8/8/8/8/8/7K/7P/7k b - -",
        expected: [1, 3, 12, 80, 342, 2343],
    },
    PerftEntry {
        fen: "K7/p7/k7/8/8/8/8/8 b - -",
        expected: [3, 7, 43, 199, 1347, 6249],
    },
    PerftEntry {
        fen: "7K/7p/7k/8/8/8/8/8 b - -",
        expected: [3, 7, 43, 199, 1347, 6249],
    },
    PerftEntry {
        fen: "8/2k1p3/3pP3/3P2K1/8/8/8/8 b - -",
        expected: [5, 35, 182, 1091, 5408, 34822],
    },
    PerftEntry {
        fen: "8/8/8/8/8/4k3/4P3/4K3 w - -",
        expected: [2, 8, 44, 282, 1814, 11848],
    },
    PerftEntry {
        fen: "4k3/4p3/4K3/8/8/8/8/8 b - -",
        expected: [2, 8, 44, 282, 1814, 11848],
    },
    PerftEntry {
        fen: "8/8/7k/7p/7P/7K/8/8 w - -",
        expected: [3, 9, 57, 360, 1969, 10724],
    },
    PerftEntry {
        fen: "8/8/k7/p7/P7/K7/8/8 w - -",
        expected: [3, 9, 57, 360, 1969, 10724],
    },
    PerftEntry {
        fen: "8/8/3k4/3p4/3P4/3K4/8/8 w - -",
        expected: [5, 25, 180, 1294, 8296, 53138],
    },
    PerftEntry {
        fen: "8/3k4/3p4/8/3P4/3K4/8/8 w - -",
        expected: [8, 61, 483, 3213, 23599, 157093],
    },
    PerftEntry {
        fen: "8/8/3k4/3p4/8/3P4/3K4/8 w - -",
        expected: [8, 61, 411, 3213, 21637, 158065],
    },
    PerftEntry {
        fen: "k7/8/3p4/8/3P4/8/8/7K w - -",
        expected: [4, 15, 90, 534, 3450, 20960],
    },
    PerftEntry {
        fen: "8/8/7k/7p/7P/7K/8/8 b - -",
        expected: [3, 9, 57, 360, 1969, 10724],
    },
    PerftEntry {
        fen: "8/8/k7/p7/P7/K7/8/8 b - -",
        expected: [3, 9, 57, 360, 1969, 10724],
    },
    PerftEntry {
        fen: "8/8/3k4/3p4/3P4/3K4/8/8 b - -",
        expected: [5, 25, 180, 1294, 8296, 53138],
    },
    PerftEntry {
        fen: "8/3k4/3p4/8/3P4/3K4/8/8 b - -",
        expected: [8, 61, 411, 3213, 21637, 158065],
    },
    PerftEntry {
        fen: "8/8/3k4/3p4/8/3P4/3K4/8 b - -",
        expected: [8, 61, 483, 3213, 23599, 157093],
    },
    PerftEntry {
        fen: "k7/8/3p4/8/3P4/8/8/7K b - -",
        expected: [4, 15, 89, 537, 3309, 21104],
    },
    PerftEntry {
        fen: "7k/3p4/8/8/3P4/8/8/K7 w - -",
        expected: [4, 19, 117, 720, 4661, 32191],
    },
    PerftEntry {
        fen: "7k/8/8/3p4/8/8/3P4/K7 w - -",
        expected: [5, 19, 116, 716, 4786, 30980],
    },
    PerftEntry {
        fen: "k7/8/8/7p/6P1/8/8/K7 w - -",
        expected: [5, 22, 139, 877, 6112, 41874],
    },
    PerftEntry {
        fen: "k7/8/7p/8/8/6P1/8/K7 w - -",
        expected: [4, 16, 101, 637, 4354, 29679],
    },
    PerftEntry {
        fen: "k7/8/8/6p1/7P/8/8/K7 w - -",
        expected: [5, 22, 139, 877, 6112, 41874],
    },
    PerftEntry {
        fen: "k7/8/6p1/8/8/7P/8/K7 w - -",
        expected: [4, 16, 101, 637, 4354, 29679],
    },
    PerftEntry {
        fen: "k7/8/8/3p4/4p3/8/8/7K w - -",
        expected: [3, 15, 84, 573, 3013, 22886],
    },
    PerftEntry {
        fen: "k7/8/3p4/8/8/4P3/8/7K w - -",
        expected: [4, 16, 101, 637, 4271, 28662],
    },
    PerftEntry {
        fen: "7k/3p4/8/8/3P4/8/8/K7 b - -",
        expected: [5, 19, 117, 720, 5014, 32167],
    },
    PerftEntry {
        fen: "7k/8/8/3p4/8/8/3P4/K7 b - -",
        expected: [4, 19, 117, 712, 4658, 30749],
    },
    PerftEntry {
        fen: "k7/8/8/7p/6P1/8/8/K7 b - -",
        expected: [5, 22, 139, 877, 6112, 41874],
    },
    PerftEntry {
        fen: "k7/8/7p/8/8/6P1/8/K7 b - -",
        expected: [4, 16, 101, 637, 4354, 29679],
    },
    PerftEntry {
        fen: "k7/8/8/6p1/7P/8/8/K7 b - -",
        expected: [5, 22, 139, 877, 6112, 41874],
    },
    PerftEntry {
        fen: "k7/8/6p1/8/8/7P/8/K7 b - -",
        expected: [4, 16, 101, 637, 4354, 29679],
    },
    PerftEntry {
        fen: "k7/8/8/3p4/4p3/8/8/7K b - -",
        expected: [5, 15, 102, 569, 4337, 22579],
    },
    PerftEntry {
        fen: "k7/8/3p4/8/8/4P3/8/7K b - -",
        expected: [4, 16, 101, 637, 4271, 28662],
    },
    PerftEntry {
        fen: "7k/8/8/p7/1P6/8/8/7K w - -",
        expected: [5, 22, 139, 877, 6112, 41874],
    },
    PerftEntry {
        fen: "7k/8/p7/8/8/1P6/8/7K w - -",
        expected: [4, 16, 101, 637, 4354, 29679],
    },
    PerftEntry {
        fen: "7k/8/8/1p6/P7/8/8/7K w - -",
        expected: [5, 22, 139, 877, 6112, 41874],
    },
    PerftEntry {
        fen: "7k/8/1p6/8/8/P7/8/7K w - -",
        expected: [4, 16, 101, 637, 4354, 29679],
    },
    PerftEntry {
        fen: "k7/7p/8/8/8/8/6P1/K7 w - -",
        expected: [5, 25, 161, 1035, 7574, 55338],
    },
    PerftEntry {
        fen: "k7/6p1/8/8/8/8/7P/K7 w - -",
        expected: [5, 25, 161, 1035, 7574, 55338],
    },
    PerftEntry {
        fen: "3k4/3pp3/8/8/8/8/3PP3/3K4 w - -",
        expected: [7, 49, 378, 2902, 24122, 199002],
    },
    PerftEntry {
        fen: "7k/8/8/p7/1P6/8/8/7K b - -",
        expected: [5, 22, 139, 877, 6112, 41874],
    },
    PerftEntry {
        fen: "7k/8/p7/8/8/1P6/8/7K b - -",
        expected: [4, 16, 101, 637, 4354, 29679],
    },
    PerftEntry {
        fen: "7k/8/8/1p6/P7/8/8/7K b - -",
        expected: [5, 22, 139, 877, 6112, 41874],
    },
    PerftEntry {
        fen: "7k/8/1p6/8/8/P7/8/7K b - -",
        expected: [4, 16, 101, 637, 4354, 29679],
    },
    PerftEntry {
        fen: "k7/7p/8/8/8/8/6P1/K7 b - -",
        expected: [5, 25, 161, 1035, 7574, 55338],
    },
    PerftEntry {
        fen: "k7/6p1/8/8/8/8/7P/K7 b - -",
        expected: [5, 25, 161, 1035, 7574, 55338],
    },
    PerftEntry {
        fen: "3k4/3pp3/8/8/8/8/3PP3/3K4 b - -",
        expected: [7, 49, 378, 2902, 24122, 199002],
    },
    PerftEntry {
        fen: "8/Pk6/8/8/8/8/6Kp/8 w - -",
        expected: [11, 97, 887, 8048, 90606, 1030499],
    },
    PerftEntry {
        fen: "n1n5/1Pk5/8/8/8/8/5Kp1/5N1N w - -",
        expected: [24, 421, 7421, 124608, 2193768, 37665329],
    },
    PerftEntry {
        fen: "8/PPPk4/8/8/8/8/4Kppp/8 w - -",
        expected: [18, 270, 4699, 79355, 1533145, 28859283],
    },
    PerftEntry {
        fen: "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N w - -",
        expected: [24, 496, 9483, 182838, 3605103, 71179139],
    },
    PerftEntry {
        fen: "8/Pk6/8/8/8/8/6Kp/8 b - -",
        expected: [11, 97, 887, 8048, 90606, 1030499],
    },
    PerftEntry {
        fen: "n1n5/1Pk5/8/8/8/8/5Kp1/5N1N b - -",
        expected: [24, 421, 7421, 124608, 2193768, 37665329],
    },
    PerftEntry {
        fen: "8/PPPk4/8/8/8/8/4Kppp/8 b - -",
        expected: [18, 270, 4699, 79355, 1533145, 28859283],
    },
    PerftEntry {
        fen: "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - -",
        expected: [24, 496, 9483, 182838, 3605103, 71179139],
    },
];