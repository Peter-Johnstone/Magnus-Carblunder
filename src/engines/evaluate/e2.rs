// src/eval.rs
use crate::attacks::movegen::all_attacks;
use crate::color::Color::{White};
use crate::position::Position;

pub fn evaluate(pos: &Position) -> i16 {
    pos.evaluate_2()
}
