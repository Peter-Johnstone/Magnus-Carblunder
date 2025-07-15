//! 64 × 64 table: all squares on the infinite line passing through
//! `sq_a` and `sq_b` (inclusive). Returns 0 if the two squares
//! are not aligned along rank, file, or diagonal.

pub(crate) const LINE_BB: [[u64; 64]; 64] = generate_line_bb();

const fn generate_line_bb() -> [[u64; 64]; 64] {
    let mut line = [[0u64; 64]; 64];

    let mut a = 0;
    while a < 64 {
        let (r1, c1) = (a / 8, a % 8);

        let mut b = 0;
        while b < 64 {
            let (r2, c2) = (b / 8, b % 8);

            // direction steps (−1, 0, 1)
            let dr = (r2 as isize - r1 as isize).signum();
            let dc = (c2 as isize - c1 as isize).signum();

            // aligned AND not the same square  ← fixed guard
            let row_diff = if r1 > r2 { r1 - r2 } else { r2 - r1 };
            let col_diff = if c1 > c2 { c1 - c2 } else { c2 - c1 };
            if  a != b &&
                ((dr == 0 && dc != 0) ||               // same rank
                    (dc == 0 && dr != 0) ||               // same file
                    (row_diff == col_diff))               // diagonal
            {
                // walk from a towards one edge
                let mut r = r1 as isize;
                let mut c = c1 as isize;
                while r >= 0 && r < 8 && c >= 0 && c < 8 {
                    line[a][b] |= 1u64 << (r * 8 + c);
                    r -= dr;
                    c -= dc;
                }

                // walk from a towards the other edge (skip a itself once)
                let mut r = r1 as isize + dr;
                let mut c = c1 as isize + dc;
                while r >= 0 && r < 8 && c >= 0 && c < 8 {
                    line[a][b] |= 1u64 << (r * 8 + c);
                    r += dr;
                    c += dc;
                }
            }

            b += 1;
        }
        a += 1;
    }
    line
}
