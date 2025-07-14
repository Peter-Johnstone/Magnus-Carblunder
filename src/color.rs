use std::ops::Not;
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub(crate) enum Color {
    #[default]
    White = 0,
    Black = 1
}

impl Color {
    pub fn from_str(s: &str) -> Color {
        if s == "w" {
            Color::White
        } else if s == "b" {
            Color::Black
        } else {
            panic!("Unrecognized color {}", s);
        }
    }
    pub fn is_white(self) -> bool {
        matches!(self, Color::White)
    }

    pub fn is_black(self) -> bool {
        matches!(self, Color::Black)
    }

}
impl Not for Color {
    type Output = Color;
    fn not(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }
}