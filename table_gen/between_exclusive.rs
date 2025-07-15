pub(crate) const BETWEEN_EXCLUSIVE: [[u64; 64]; 64] = generate_between_exclusive();

const fn generate_between_exclusive() -> [[u64; 64]; 64] {
    let mut between_exclusive = [[0u64; 64]; 64];

    let mut from = 0;
    while from < 64 {
        let (r1, c1) = (from / 8, from % 8);

        let mut to = 0;
        while to < 64 {
            let (r2, c2) = (to / 8, to % 8);
            between_exclusive[from][to] = bitboard_in_between_exclusive(r1, c1, r2, c2);
            to += 1;
        }
        from += 1;
    }
    between_exclusive
}

/// Bitboard with all squares *strictly* between (r1,c1) and (r2,c2)
/// along a rank, file, or diagonal.
/// Returns `0` if the two squares are not aligned.
const fn bitboard_in_between_exclusive(r1: usize, c1: usize,
                                       r2: usize, c2: usize) -> u64 {
    // --- compute row / column deltas -------------------------------
    let dr: isize = match (r2 as isize).wrapping_sub(r1 as isize) {
        d if d >  0 =>  1,
        d if d <  0 => -1,
        _           =>  0,
    };
    let dc: isize = match (c2 as isize).wrapping_sub(c1 as isize) {
        d if d >  0 =>  1,
        d if d <  0 => -1,
        _           =>  0,
    };

    // --- are the two squares aligned? ------------------------------
    let row_diff = if r1 > r2 { r1 - r2 } else { r2 - r1 };
    let col_diff = if c1 > c2 { c1 - c2 } else { c2 - c1 };

    if !((dr == 0 && dc != 0)          // same rank
        || (dc == 0 && dr != 0)        // same file
        || (row_diff == col_diff))     // strict diagonal
    {
        return 0;
    }

    // --- walk from (r1,c1) towards (r2,c2) --------------------------
    let mut bb = 0u64;
    let mut r  = r1 as isize + dr;
    let mut c  = c1 as isize + dc;

    // Maximum 7 interior squares on an 8×8 board
    let mut steps_left = 7u8;
    while steps_left > 0 && (r != r2 as isize || c != c2 as isize) {
        if r >= 0 && r < 8 && c >= 0 && c < 8 {
            let idx = (r * 8 + c) as u32;
            bb |= 1u64 << idx;
        }
        r += dr;
        c += dc;
        steps_left -= 1;
    }

    bb
}
