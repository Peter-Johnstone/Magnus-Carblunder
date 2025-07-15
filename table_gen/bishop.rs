use crate::table_gen::bitboards;

// ── static helpers ───────────────────────────────────────────────────────
pub(crate) const BISHOP_MASKS: [u64; 64] = generate_bishop_masks().0;
pub(crate) const BISHOP_RAYS: [[u64; 64]; 4] = generate_bishop_masks().1;

pub const fn generate_bishop_masks() -> ([u64; 64], [[u64; 64]; 4]) {
    const DIRS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
    let mut masks = [0u64; 64];
    let mut rays = [[0u64; 64]; 4]; // 4 directional rays NE, SE, SW, NW


    let mut sq = 0u8;
    while sq < 64 {
        let file = (sq % 8) as i8;
        let rank = (sq / 8) as i8;

        let mut mask = 0u64;

        // loop over the four diagonal directions
        let mut d = 0;
        while d < 4 {
            let (df, dr) = DIRS[d];
            let mut f = file + df;
            let mut r = rank + dr;

            // stop one square before the edge (file 0/7 or rank 0/7)
            while f > 0 && f < 7 && r > 0 && r < 7 {
                rays[d][sq as usize] |= 1u64 << (r * 8 + f) as u64;
                mask |= 1u64 << (r * 8 + f) as u64;
                f += df;
                r += dr;
            }

            // final square gets added to the rays, but not mask
            if f >= 0 && f <= 7 && r >= 0 && r <= 7 {
                rays[d][sq as usize] |= 1u64 << (r * 8 + f) as u64;
            }
            d += 1;
        }

        masks[sq as usize] = mask;
        sq += 1;
    }
    (masks, rays)
}




pub fn compute_bishop_attack(sq: u8, blockers: u64) -> u64 {
    let mut attacks = 0u64;

    // (direction step, file that would *wrap* if we take another step)
    const DIRS: [(i8, u8); 4] = [
        (  9, 7),  // NE: stop when we’re already on h-file
        (  7, 0),  // NW: stop when we’re already on a-file
        ( -7, 7),  // SE: stop when we’re on h-file
        ( -9, 0),  // SW: stop when we’re on a-file
    ];

    for &(step, edge_file) in &DIRS {
        // ---------- compute_bishop_attack ---------------------------------------
        let mut pos = sq as i8;
        loop {
            let file = pos % 8;
            pos += step;

            if pos < 0 || pos >= 64 { break; }

            let next_file = pos % 8;
            if (step == 9 || step == -7) && next_file <= file { break; } // NE / SW wrap
            if (step == 7 || step == -9) && next_file >= file { break; } // NW / SE wrap

            let bit = 1u64 << pos;
            attacks |= bit;

            if blockers & bit != 0 { break; }
        }
    }

    attacks
}

fn index_to_blockers(mut idx: u32, mask: u64) -> u64 {
    let mut result: u64 = 0;
    bitboards::pop_lsb(mask, |sq:u8| {
        if idx & 1 != 0 {
            result |= 1u64 << sq;
        }
        idx >>= 1
    });
    result
}


pub const BISHOP_MAGICS: [u64; 64] = [
    0x0002020202020200, 0x0002020202020000, 0x0004010202000000,
    0x0004040080000000, 0x0001104000000000, 0x0000821040000000,
    0x0000410410400000, 0x0000104104104000, 0x0000040404040400,
    0x0000020202020200, 0x0000040102020000, 0x0000040400800000,
    0x0000011040000000, 0x0000008210400000, 0x0000004104104000,
    0x0000002082082000, 0x0004000808080800, 0x0002000404040400,
    0x0001000202020200, 0x0000800802004000, 0x0000800400A00000,
    0x0000200100884000, 0x0000400082082000, 0x0000200041041000,
    0x0002080010101000, 0x0001040008080800, 0x0000208004010400,
    0x0000404004010200, 0x0000840000802000, 0x0000404002011000,
    0x0000808001041000, 0x0000404000820800, 0x0001041000202000,
    0x0000820800101000, 0x0000104400080800, 0x0000020080080080,
    0x0000404040040100, 0x0000808100020100, 0x0001010100020800,
    0x0000808080010400, 0x0000820820004000, 0x0000410410002000,
    0x0000082088001000, 0x0000002011000800, 0x0000080100400400,
    0x0001010101000200, 0x0002020202000400, 0x0001010101000200,
    0x0000410410400000, 0x0000208208200000, 0x0000002084100000,
    0x0000000020880000, 0x0000001002020000, 0x0000040408020000,
    0x0004040404040000, 0x0002020202020000, 0x0000104104104000,
    0x0000002082082000, 0x0000000020841000, 0x0000000000208800,
    0x0000000010020200, 0x0000000404080200, 0x0000040404040400,
    0x0002020202020200,
];
pub const BISHOP_SHIFTS: [u8; 64] = [
    58, 59, 59, 59, 59, 59, 59, 58,
    59, 59, 59, 59, 59, 59, 59, 59,
    59, 59, 57, 57, 57, 57, 59, 59,
    59, 59, 57, 55, 55, 57, 59, 59,
    59, 59, 57, 55, 55, 57, 59, 59,
    59, 59, 57, 57, 57, 57, 59, 59,
    59, 59, 59, 59, 59, 59, 59, 59,
    59, 59, 58, 59, 59, 59, 59, 59,
];
pub(crate) fn build_bishop_table() -> Vec<[u64; 512]> {
    let masks = BISHOP_MASKS;                    // 64 × u64, tiny
    let mut table: Vec<[u64; 512]> = Vec::with_capacity(64);

    for sq in 0..64 {
        let mask         = masks[sq];
        let permutations = 1u32 << mask.count_ones();
        let mut row      = [0u64; 512];

        for idx in 0..permutations {
            let blockers = index_to_blockers(idx, mask);
            let key      = ((blockers.wrapping_mul(BISHOP_MAGICS[sq]))
                >> BISHOP_SHIFTS[sq]) as usize;
            row[key] = compute_bishop_attack(sq as u8, blockers);
        }

        table.push(row);                       // each row copied once
    }
    table
}
