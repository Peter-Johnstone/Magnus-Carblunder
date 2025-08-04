use std::ops::Not;
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum Color {
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

    pub fn to_str(self) -> String {
        if self.is_white() {
            "w".to_string()
        } else {
            "b".to_string()
        }
    }
    pub const fn is_white(self) -> bool {
        matches!(self, Color::White)
    }

    pub const fn is_black(self) -> bool {
        matches!(self, Color::Black)
    }

}
impl Not for Color {
    type Output = Color;

    fn not(self) -> Color {
        unsafe { std::mem::transmute(self as u8 ^ 1) }
    }
}