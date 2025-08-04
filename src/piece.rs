use crate::color::Color;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Piece {
    Pawn   = 0,   // 0b0000
    Knight = 1,   // 0b0001
    Bishop = 2,   // 0b0010
    Rook   = 3,   // 0b0011
    Queen  = 4,   // 0b0100
    King   = 5,   // 0b0101
}

impl Piece {
    pub fn piece_initial(self) -> char {
        match  { self } {
            Piece::Pawn   => 'p',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Rook   => 'r',
            Piece::Queen  => 'q',
            Piece::King   => 'k',
        }
    }

    pub fn from(i: usize) -> Piece {
        match i {
            0 => Piece::Pawn,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            5 => Piece::King,
            _ => {panic!("fucking christ!"); },// bug
        }
    }
}


pub const EMPTY_PIECE: i8 = 0;
pub type ColoredPiece = i8;              // signed board entry



#[inline(always)]
pub const fn to_str(v: ColoredPiece) -> &'static str {
    match v {
         6 => "K",
         5 => "Q",
         4 => "R",
         3 => "B",
         2 => "N",
         1 => "P",
        -1 => "p",
        -2 => "n",
        -3 => "b",
        -4 => "r",
        -5 => "q",
        -6 => "k",
        _ => unreachable!()
    }
}
#[inline(always)]
pub const fn is_empty(v: ColoredPiece) -> bool {
    v == EMPTY_PIECE
}

#[inline(always)]
pub const fn to_color(v: ColoredPiece) -> Color {
    // v != 0
    if v > 0 { Color::White } else { Color::Black }
}

const PIECE_LUT: [Piece; 13] = [
    Piece::King, Piece::Queen,Piece::Rook,
    Piece::Bishop, Piece::Knight, Piece::Pawn,
    Piece::Pawn,           // idx 0 unused
    Piece::Pawn, Piece::Knight, Piece::Bishop,
    Piece::Rook, Piece::Queen, Piece::King,
];

#[inline(always)]
pub const fn to_piece(v: ColoredPiece) -> Piece {
    PIECE_LUT[(v + 6) as usize]
}

#[inline(always)]
pub const fn is_slider_val(v: ColoredPiece) -> bool {
    matches!(v.abs(), 3 | 4 | 5)   // B,R,Q
}

#[inline(always)]
pub const fn piece_to_val(p: Piece, c: Color) -> ColoredPiece {
    let val = p as ColoredPiece + 1;          // 1..=6
    if c.is_white() {  val } else { -val }
}