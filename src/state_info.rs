use crate::attacks::tables::{BETWEEN_EXCLUSIVE, BETWEEN_INCLUSIVE, LINE_BB};
use crate::bitboards::print_bitboard;

#[derive(Copy, Clone, Debug, Default)]
pub struct StateInfo {
    pub checkers: u64,
    pub blockers_for_king: u64,
    pub pinners: u64,
}
impl StateInfo {
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

        // Line between king and the candidate piece
        let line = LINE_BB[king_sq][from_sq];
        // If no pinner lies on that line => piece is not pinned
        let slider_on_line = pinners & line;
        if slider_on_line == 0 {
            return 0;
        }

        // Find the slider that lies on the same ray (there is exactly one)
        let pinner_sq = slider_on_line.trailing_zeros() as usize;
        print_bitboard(BETWEEN_INCLUSIVE[king_sq][pinner_sq]);
        // Full pin ray = king ↔ pinner (inclusive)
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