
pub(crate) const FILE_A: u64 = 0x0101_0101_0101_0101;
pub (crate) const FILE_B: u64 = 0x0202_0202_0202_0202;
pub (crate) const FILE_C: u64 = 0x0404_0404_0404_0404;
pub (crate) const FILE_D: u64 = 0x0808_0808_0808_0808;
pub (crate) const FILE_E: u64 = 0x1010_1010_1010_1010;
pub (crate) const FILE_F: u64 = 0x2020_2020_2020_2020;
pub (crate) const FILE_G: u64 = 0x4040_4040_4040_4040;
pub(crate) const FILE_H: u64 = 0x8080_8080_8080_8080;
pub(crate) const RANK_1: u64 = 0x0000_0000_0000_00FF;
pub(crate) const RANK_3: u64 = 0x0000_0000_00FF_0000;
pub(crate) const RANK_6: u64 = 0x0000_FF00_0000_0000;
pub(crate) const RANK_8: u64 = 0xFF00_0000_0000_0000;
pub(crate) const FULL_BB: u64 =   0xFFFF_FFFF_FFFF_FFFF;

pub(crate) const PROMO_RANKS: u64 = RANK_1 | RANK_8;



#[inline(always)]
pub(crate) fn pop_lsb(mut bb: u64, mut f: impl FnMut(u8)) {
    while bb != 0 {
        let sq = bb.trailing_zeros() as u8;
        bb &= bb - 1;
        f(sq);
    }
}

#[allow(dead_code)]
pub(crate) fn print_bitboard(bb: u64) {
    for rank in (0..8).rev() {

        println!();
        print!("{}  ", rank + 1);
        for file in 0..8 {
            let square = rank * 8 + file;
            let bit = (bb >> square) & 1;
            print!("{} ", if bit == 1 { "1" } else { "." });
        }
    }
    println!("\n   a b c d e f g h");
}
