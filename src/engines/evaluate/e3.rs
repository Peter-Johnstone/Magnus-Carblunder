use crate::color::Color;
use crate::piece::Piece;
use crate::position::Position;

/// Pure material values, index = `Piece as usize`
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

#[inline(always)]
const fn mirror(sq: usize) -> usize { sq ^ 56 }   // a1↔a8 … simple xor

// --- piece‑square tables omitted here (keep yours) ----------------------
// use PAWN_PST, KNIGHT_PST, …, KING_MG_PST, KING_EG_PST as before
// -----------------------------------------------------------------------

const ENDGAME_MATERIAL: i16 = 1300;
const AVOID_EDGE_PEN: i16 = 50;

/// Returns a score from White’s point of view.
pub fn evaluate(pos: &Position) -> i16 {
    /* quick draw shortcuts */
    if pos.is_repeat_towards_three_fold_repetition() || pos.half_move() >= 99 { return 0; }

    let mut score          = 0;
    let mut mat_w: i16     = 0;
    let mut mat_b: i16     = 0;

    /* 1. material + PST for non‑kings */
    for (piece_idx, pst) in [
        (Piece::Pawn,   &PAWN_PST   as &[i16]),
        (Piece::Knight, &KNIGHT_PST),
        (Piece::Bishop, &BISHOP_PST),
        (Piece::Rook,   &ROOK_PST),
        (Piece::Queen,  &QUEEN_PST),
    ] {
        let base = PIECE_VALUE[piece_idx as usize];

        // iterate both colours
        for &c in &[Color::White, Color::Black] {
            let (list, cnt) = pos.piece_list(piece_idx, c);
            for i in 0..cnt {
                let sq  = list[i] as usize;
                let idx = if c == Color::White { sq } else { mirror(sq) };
                let val = base + pst[idx];
                if c == Color::White { score += val; mat_w += base; }
                else          { score -= val; mat_b += base; }
            }
        }
    }

    /* 2. king PST */
    let eg = (mat_w + mat_b) <= ENDGAME_MATERIAL;
    for &c in &[Color::White, Color::Black] {
        let sq  = pos.king_square(c) as usize;
        let idx = if c == Color::White { sq } else { mirror(sq) };
        let pst = if eg { KING_EG_PST[idx] } else { KING_MG_PST[idx] };
        if c == Color::White { score += pst; } else { score -= pst; }
    }

    /* 3. lone‑king heuristics */
    let lone_w = mat_w == 0;
    let lone_b = mat_b == 0;

    if lone_w || lone_b {
        let w_sq = pos.king_square(Color::White) as usize;
        let b_sq = pos.king_square(Color::Black) as usize;

        let (w_r, w_c) = (w_sq / 8, w_sq & 7);
        let (b_r, b_c) = (b_sq / 8, b_sq & 7);

        // squared distance ≤ 98 (7²+7²)
        let d2 = (w_r as i16 - b_r as i16).pow(2) +
            (w_c as i16 - b_c as i16).pow(2);
        let kd_bonus = KING_DIST_BONUS[d2 as usize];

        if lone_b {                 // White has the material edge
            score += kd_bonus;

            let pen = edge_avoid_penalty(b_r, b_c);
            score += pen;           // (subtract from Black = add to White)
        } else if lone_w {          // Black is stronger
            score -= kd_bonus;

            let pen = edge_avoid_penalty(w_r, w_c);
            score -= pen;
        }
    }

    score
}

/// 50 × ( distance from edge in rows + distance from edge in cols )
#[inline(always)]
fn edge_avoid_penalty(row: usize, col: usize) -> i16 {
    let pen_rows = 7 - row.min(7 - row);   // (7 - min) = max(row, 7‑row)
    let pen_cols = 7 - col.min(7 - col);
    ((pen_rows + pen_cols) as i16) * AVOID_EDGE_PEN
}

