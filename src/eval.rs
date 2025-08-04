use crate::color::Color;
use crate::piece::Piece;
use crate::position::Position;

/// Mid-game / end-game scores kept incrementally.
#[derive(Clone, Copy, Default, Debug)]
pub struct EvalCache {
    pub(crate) mg: i32,        // White – Black, mid-game units
    pub(crate) eg: i32,        // same for end-game
    pub(crate) phase: i32,     // 24 → opening, … 0 → pure endings
}
impl EvalCache {
    #[inline(always)]
    pub(crate) fn tapered(self) -> i16 {
        // phase in [0,24]
        ((self.mg * self.phase + self.eg * (24 - self.phase)) / 24) as i16
    }
}

pub const PIECE_VALUE: [i16; 6] = [100, 320, 330, 500, 900, 0];

// Pawn, Knight, …, King PSTs (white’s view)
pub const PAWN_PST:   [i16; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
    5,  5, 10, 25, 25, 10,  5,  5,
    0,  0,  0, 20, 20,  0,  0,  0,
    5, -5,-10,  0,  0,-10, -5,  5,
    5, 10, 10,-20,-20, 10, 10,  5,
    0,  0,  0,  0,  0,  0,  0,  0,
];
pub const KNIGHT_PST: [i16; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];
pub const BISHOP_PST: [i16; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];
pub const ROOK_PST:   [i16; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0,
];
pub const QUEEN_PST:  [i16; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
    -5,  0,  5,  5,  5,  5,  0, -5,
    0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20,
];
pub const KING_MG_PST: [i16; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
    20, 20,  0,  0,  0,  0, 20, 20,
    20, 30, 10,  0,  0, 10, 30, 20,
];
pub const KING_EG_PST: [i16; 64] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -50,-30,-30,-30,-30,-30,-30,-50,
];

// 0‑to‑98    (dx²+dy² ≤ 49+49)
// value = 500 – 30·sqrt(d²)  (rounded to nearest int)
const KING_DIST_BONUS: [i16; 99] = [
    500, 470, 448, 427, 409, 394, 381, 370, 359, 349,
    341, 333, 326, 319, 313, 308, 302, 298, 293, 289,
    285, 281, 278, 274, 271, 268, 265, 262, 260, 257,
    255, 252, 250, 248, 246, 244, 242, 240, 238, 237,
    235, 233, 232, 230, 229, 227, 226, 224, 223, 221,
    220, 219, 217, 216, 215, 214, 212, 211, 210, 209,
    208, 207, 206, 205, 204, 203, 202, 201, 200, 199,
    198, 197, 196, 195, 194, 194, 193, 192, 191, 190,
    189, 189, 188, 187, 186, 186, 185, 184, 183, 183,
    182, 181, 181, 180, 179, 179, 178, 177, 177,
];


pub fn build_eval(pos: &Position) -> EvalCache {

    let pst_mg: [&[i16; 64]; 6] = [&PAWN_PST, &KNIGHT_PST, &BISHOP_PST,
        &ROOK_PST, &QUEEN_PST, &KING_MG_PST];
    let pst_eg: [&[i16; 64]; 6] = [&PAWN_PST, &KNIGHT_PST, &BISHOP_PST,
        &ROOK_PST, &QUEEN_PST, &KING_EG_PST];
    let phase_inc = [0, 1, 1, 2, 4, 0];   // Stockfish convention

    let mut cache = EvalCache::default();

    for p in 0..6 {
        for &c in &[Color::White, Color::Black] {
            let (list, cnt) = pos.piece_list(Piece::from(p), c);
            for i in 0..cnt {
                let sq   = list[i] as usize;
                let idx  = if c.is_white() { sq } else { sq ^ 56 };
                let sgn  = if c.is_white() { 1 } else { -1 };

                cache.mg += sgn * pst_mg[p][idx]  as i32;
                cache.eg += sgn * pst_eg[p][idx]  as i32;
                cache.mg += sgn * PIECE_VALUE[p]  as i32;
                cache.eg += sgn * PIECE_VALUE[p]  as i32;

                cache.phase += phase_inc[p];
            }
        }
    }
    cache
}


pub const PST_MG: [&[i16; 64]; 6] = [&PAWN_PST, &KNIGHT_PST, &BISHOP_PST,
    &ROOK_PST, &QUEEN_PST, &KING_MG_PST];
pub const PST_EG: [&[i16; 64]; 6] = [&PAWN_PST, &KNIGHT_PST, &BISHOP_PST,
    &ROOK_PST, &QUEEN_PST, &KING_EG_PST];
pub const PHASE_INC: [i32; 6]     = [0,1,1,2,4,0];

#[inline(always)]
pub const fn mirror(sq: usize) -> usize { sq ^ 56 }
