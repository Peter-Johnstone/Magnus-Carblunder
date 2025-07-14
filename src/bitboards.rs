#[inline(always)]
pub(crate) fn pop_lsb(mut bb: u64, mut f: impl FnMut(u8)) {
    while bb != 0 {
        let sq = bb.trailing_zeros() as u8;
        bb &= bb - 1;
        f(sq);
    }
}

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
