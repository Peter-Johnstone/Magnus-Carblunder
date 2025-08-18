// src/eval_weight.rs
use std::sync::atomic::{AtomicI32, Ordering::Relaxed};

/// Mobility multiplier in centi-units: 900 == 9.00
pub static WHITE_MOBILITY_MUL_CENTI: AtomicI32 = AtomicI32::new(900);
pub static BLACK_MOBILITY_MUL_CENTI: AtomicI32 = AtomicI32::new(900);

#[inline]
pub fn set_mobility_muls_centi(white: i32, black: i32) {
    WHITE_MOBILITY_MUL_CENTI.store(white, Relaxed);
    BLACK_MOBILITY_MUL_CENTI.store(black, Relaxed);
}

#[inline]
pub fn mobility_muls_centi() -> (i32, i32) {
    (
        WHITE_MOBILITY_MUL_CENTI.load(Relaxed),
        BLACK_MOBILITY_MUL_CENTI.load(Relaxed),
    )
}
