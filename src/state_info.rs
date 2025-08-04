use crate::tables::{BETWEEN_INCLUSIVE, LINE_BB};

#[derive(Copy, Clone, Debug, Default)]
pub struct StateInfo {
    pub checkers: u64,
    pub blockers_for_king: u64,
    pub pinners: u64,
}
impl StateInfo {

    pub const fn default() -> Self {
        StateInfo {checkers: 0, blockers_for_king: 0, pinners: 0}
    }

    pub fn new(checkers: u64, blockers_for_king: u64, pinners: u64) -> Self {
        Self { checkers, blockers_for_king, pinners }
    }

    pub fn pinners(&self) -> u64 {
        self.pinners
    }

    pub fn checkers(&self) -> u64 {
        self.checkers
    }

    #[inline]
    pub fn pin_ray(king_sq: usize, from_sq: usize, pinners: u64) -> u64 {
        // Bitboard of the whole infinite line through king and from_sq
        let line = LINE_BB[king_sq][from_sq];

        // Sliders that sit somewhere on that line
        let sliders_on_line = line & pinners;
        if sliders_on_line == 0 {
            return 0;                                     // ❶ no pin possible
        }

        // Does the ray go towards larger square numbers or smaller ones?
        let forward = from_sq > king_sq;

        // Keep only bits **beyond the piece** in the forward direction
        // ‒ if forward → greater square indices
        // ‒ else       → smaller square indices
        let mask_after_piece = if forward {
            // bits > from_sq
            sliders_on_line & (!0u64).wrapping_shl((from_sq + 1) as u32)
        } else {
            // bits < from_sq
            sliders_on_line & ((1u64 << from_sq) - 1)
        };

        if mask_after_piece == 0 {
            return 0;                                     // ❷ no slider between piece and edge
        }

        // First pinner in that direction: LS1B if forward, MS1B if backward
        let pinner_sq = if forward {
            mask_after_piece.trailing_zeros() as usize
        } else {
            63 - mask_after_piece.leading_zeros() as usize
        };

        // Return king‑to‑pinner ray (inclusive)
        BETWEEN_INCLUSIVE[king_sq][pinner_sq]
    }




    pub fn blockers_for_king(&self) -> u64 {
        self.blockers_for_king
    }

    pub fn is_check(&self) -> bool {
        self.checkers.count_ones() >= 1
    }

    pub fn is_double_check(&self) -> bool {
        self.checkers.count_ones() >= 2
    }
}