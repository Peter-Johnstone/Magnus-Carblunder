pub(crate) const fn knight_table() -> [u64; 64] {

    const fn generate_knight_attacks(sq: usize) -> u64 {
        const FILE_A: u64 = 0x0101_0101_0101_0101;
        const FILE_B: u64 = 0x0202_0202_0202_0202;
        const FILE_G: u64 = 0x4040_4040_4040_4040;
        const FILE_H: u64 = 0x8080_8080_8080_8080;
        const RANK_1: u64 = 0x0000_0000_0000_00FF;
        const RANK_2: u64 = 0x0000_0000_0000_FF00;
        const RANK_7: u64 = 0x00FF_0000_0000_0000;
        const RANK_8: u64 = 0xFF00_0000_0000_0000;
        let bb = 1u64 << sq;

        let nne = (bb & !(RANK_8|RANK_7|FILE_H)) << (8*2 + 1);
        let nee = (bb & !(RANK_8|FILE_G|FILE_H)) << (8*1 + 2);
        let see = (bb & !(RANK_1|FILE_G|FILE_H)) >> (8*1 - 2);
        let sse = (bb & !(RANK_1|RANK_2|FILE_H)) >> (8*2 - 1);
        let ssw = (bb & !(RANK_1|RANK_2|FILE_A)) >> (8*2 + 1);
        let sww = (bb & !(RANK_1|FILE_B|FILE_A)) >> (8*1 + 2);
        let nww = (bb & !(RANK_8|FILE_B|FILE_A)) << (8*1 - 2);
        let nnw = (bb & !(RANK_8|RANK_7|FILE_A)) << (8*2 - 1);

        nne | nee | see | sse | ssw | sww | nww | nnw
    }

    let mut table = [0u64; 64];
    let mut i = 0;
    while i < 64 {
        table[i] = generate_knight_attacks(i);
        i += 1;
    }
    table
}
