pub(crate) const fn king_table() -> [u64; 64] {
    const fn get_movement(sq: usize) -> u64 {
        const FILE_A: u64 = 0x0101_0101_0101_0101;
        const FILE_H: u64 = 0x8080_8080_8080_8080;
        const RANK_1: u64 = 0x0000_0000_0000_00FF;
        const RANK_8: u64 = 0xFF00_0000_0000_0000;

        let bb = 1u64 << sq;

        let n  = (bb & !RANK_8) << 8;
        let s  = (bb & !RANK_1) >> 8;
        let e  = (bb & !FILE_H) << 1;
        let w  = (bb & !FILE_A) >> 1;
        let ne = (bb & !(RANK_8 | FILE_H)) << 9;
        let nw = (bb & !(RANK_8 | FILE_A)) << 7;
        let se = (bb & !(RANK_1 | FILE_H)) >> 7;
        let sw = (bb & !(RANK_1 | FILE_A)) >> 9;

        n | s | e | w | ne | nw | se | sw
    }

    let mut king_movement: [u64; 64] = [0; 64];
    let mut sq: usize = 0;
    while sq < 64 {
        king_movement[sq] = get_movement(sq);
        sq += 1;
    }

    king_movement
}
