


pub(crate) fn pop_lsb(mut bb: u64, mut f: impl FnMut(u8)) {
    while bb != 0 {
        let sq = bb.trailing_zeros() as u8;
        bb &= bb - 1;
        f(sq);
    }
}