pub(crate) const fn pawn_attacks() -> [[u64; 64]; 2] {
    let mut table = [[0u64; 64]; 2];

    const FILE_A: u64 = 0x0101_0101_0101_0101;
    const FILE_H: u64 = 0x8080_8080_8080_8080;

    let mut sq = 0;
    while sq < 64 {
        let bb = 1u64 << sq;

        // ------- white pawn attacks (north-west / north-east)
        let mut white = 0u64;
        if bb & FILE_A == 0 { white |= bb << 7; }   // no wrap from file A
        if bb & FILE_H == 0 { white |= bb << 9; }   // no wrap from file H
        table[0][sq] = white;

        // ------- black pawn attacks (south-west / south-east)
        let mut black = 0u64;
        if bb & FILE_A == 0 { black |= bb >> 9; }
        if bb & FILE_H == 0 { black |= bb >> 7; }
        table[1][sq] = black;

        // Update index
        sq += 1;
    }

    table
}
