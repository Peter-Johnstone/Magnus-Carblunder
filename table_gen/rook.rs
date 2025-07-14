use crate::table_gen::bitboards;

// ── static helpers ───────────────────────────────────────────────────────
pub(crate) const ROOK_MASKS: [u64; 64] = generate_rook_masks();

const fn generate_rook_masks() -> [u64; 64] {
    let mut boards = [0u64; 64];
    let mut sq = 0;
    while sq < 64 {
        let file = sq % 8;
        let rank = sq / 8;
        let sq_bb = 1u64 << sq;

        let rank_mask = 0xFFu64 << (rank * 8);
        let file_mask = 0x0101_0101_0101_0101u64 << file;

        const FILE_EDGES: u64 = 0x0101_0101_0101_0101 | 0x8080_8080_8080_8080;
        const RANK_EDGES: u64 = 0x0000_0000_0000_00FF | 0xFF00_0000_0000_0000;

        let rank_bb = rank_mask & !FILE_EDGES & !sq_bb;
        let file_bb = file_mask & !RANK_EDGES & !sq_bb;

        boards[sq as usize] = rank_bb | file_bb;
        sq += 1;
    }
    boards
}

pub(crate) fn compute_rook_attack(sq: u8, blockers: u64) -> u64 {
    let mut attacks = 0u64;
    let directions = [1i8, -1, 8, -8]; // East, West, North, South

    for &dir in &directions {
        let mut pos = sq as i8;
        loop {
            pos += dir;
            if pos < 0 || pos > 63 {
                break;
            }
            // Prevent wrapping around the board when moving horizontally
            let from_rank = sq / 8;
            let to_rank = (pos as u8) / 8;

            if (dir == 1 || dir == -1) && to_rank != from_rank {
                break;
            }
            let bit = 1u64 << pos;
            attacks |= bit;

            if blockers & bit != 0 {
                break;
            }
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


const ROOK_MAGICS: [u64; 64] =
    [0x0080001020400080, 0x0040001000200040, 0x0080081000200080,
        0x0080040800100080, 0x0080020400080080, 0x0080010200040080,
        0x0080008001000200, 0x0080002040800100, 0x0000800020400080,
        0x0000400020005000, 0x0000801000200080, 0x0000800800100080,
        0x0000800400080080, 0x0000800200040080, 0x0000800100020080,
        0x0000800040800100, 0x0000208000400080, 0x0000404000201000,
        0x0000808010002000, 0x0000808008001000, 0x0000808004000800,
        0x0000808002000400, 0x0000010100020004, 0x0000020000408104,
        0x0000208080004000, 0x0000200040005000, 0x0000100080200080,
        0x0000080080100080, 0x0000040080080080, 0x0000020080040080,
        0x0000010080800200, 0x0000800080004100, 0x0000204000800080,
        0x0000200040401000, 0x0000100080802000, 0x0000080080801000,
        0x0000040080800800, 0x0000020080800400, 0x0000020001010004,
        0x0000800040800100, 0x0000204000808000, 0x0000200040008080,
        0x0000100020008080, 0x0000080010008080, 0x0000040008008080,
        0x0000020004008080, 0x0000010002008080, 0x0000004081020004,
        0x0000204000800080, 0x0000200040008080, 0x0000100020008080,
        0x0000080010008080, 0x0000040008008080, 0x0000020004008080,
        0x0000800100020080, 0x0000800041000080, 0x00FFFCDDFCED714A,
        0x007FFCDDFCED714A, 0x003FFFCDFFD88096, 0x0000040810002101,
        0x0001000204080011, 0x0001000204000801, 0x0001000082000401,
        0x0001FFFAABFAD1A2];
pub const ROOK_SHIFTS: [u8; 64] = [
    52, 53, 53, 53, 53, 53, 53, 52,
    53, 54, 54, 54, 54, 54, 54, 53,
    53, 54, 54, 54, 54, 54, 54, 53,
    53, 54, 54, 54, 54, 54, 54, 53,
    53, 54, 54, 54, 54, 54, 54, 53,
    53, 54, 54, 54, 54, 54, 54, 53,
    52, 53, 53, 53, 53, 53, 53, 52,
    52, 53, 53, 53, 53, 53, 53, 52,
];
pub(crate) fn build_rook_table() -> Vec<[u64; 4096]> {
    let masks = ROOK_MASKS;                    // 64 × u64, tiny
    let mut table: Vec<[u64; 4096]> = Vec::with_capacity(64);

    for sq in 0..64 {
        let mask         = masks[sq];
        let permutations = 1u32 << mask.count_ones();
        let mut row      = [0u64; 4096];

        for idx in 0..permutations {
            let blockers = index_to_blockers(idx, mask);
            let key      = ((blockers.wrapping_mul(ROOK_MAGICS[sq]))
                >> ROOK_SHIFTS[sq]) as usize;
            row[key] = compute_rook_attack(sq as u8, blockers);
        }

        table.push(row);                       // each row copied once
    }
    table
}
