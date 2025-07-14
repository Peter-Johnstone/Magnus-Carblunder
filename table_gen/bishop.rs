use crate::table_gen::bitboards;

// ── static helpers ───────────────────────────────────────────────────────
pub(crate) const BISHOP_MASKS: [u64; 64] = generate_bishop_masks();

pub const fn generate_bishop_masks() -> [u64; 64] {
    const MAIN: u64  = 0x8040_2010_0804_0201; // A1–H8
    const ANTI: u64  = 0x0102_0408_1020_4080; // H1–A8

    const EDGES: u64 =
            0x0101_0101_0101_0101 | // file A
            0x8080_8080_8080_8080 | // file H
            0x0000_0000_0000_00FF | // rank 1
            0xFF00_0000_0000_0000;  // rank 8

    let mut boards = [0u64; 64];
    let mut sq = 0;

    while sq < 64 {
        let file = sq % 8;          // 0..7
        let rank = sq / 8;          // 0..7
        let sq_bb = 1u64 << sq;

        // main diagonal (↗︎ / ↙︎) → file - rank is constant
        let diff = (file as i8) - (rank as i8);
        let sum  = file + rank;         // 0..14


        let main_mask = if diff < 0 {
            MAIN >> ((-diff as u32) * 8)
        } else {
            MAIN << ((diff as u32) * 8)
        };

        let anti_mask = if sum < 8 {
            ANTI >> ((7 - sum) * 8)
        } else {
            ANTI << ((sum - 7) * 8)
        };


        boards[sq as usize] =
            (main_mask | anti_mask) & !EDGES & !sq_bb;

        sq += 1;
    }
    boards
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
        let mut pos = sq as i8 + step;
        while (0..64).contains(&pos) {
            let bit = 1u64 << pos;
            attacks |= bit;

            if blockers & bit != 0 {
                break;
            }
            if (pos as u8 % 8) == edge_file {
                break;
            }
            pos += step;
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
