pub mod bishop_magics;
pub mod rook_magics;
pub mod bishop_shifts;
pub mod rook_shifts;
pub mod rook_attacks;
mod knight_moves;
mod king_moves;
mod rook_masks;

mod bishop_masks;
mod bishop_attacks;
pub mod pawn_attacks;

pub use pawn_attacks::PAWN_ATTACKS;
pub use king_moves::KING_MOVES;
pub use rook_masks::ROOK_MASKS;
pub use knight_moves::KNIGHT_MOVES;
pub use bishop_attacks::BISHOP_ATTACKS;
pub use bishop_masks::BISHOP_MASKS;
pub use rook_attacks::ROOK_ATTACKS;
pub use rook_shifts::ROOK_SHIFTS;
pub use bishop_shifts::BISHOP_SHIFTS;
pub use bishop_magics::BISHOP_MAGICS;
pub use rook_magics::ROOK_MAGICS;