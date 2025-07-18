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
            Piece::Pawn => 'p',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Rook   => 'r',
            Piece::Queen  => 'q',
            Piece::King   => 'k',
        }
    }
}



pub type EncodedPiece = u8;
pub const EMPTY_PIECE: EncodedPiece = 0;



// Piece encoded as 0b0000CPPP
pub fn encode_piece(piece: Piece, color: Color) -> EncodedPiece {
    ((color as u8) << 3) | (piece as u8 + 1) // piece = 0 and color = 0 reserved for empty
}

pub fn decode_piece(encoded: EncodedPiece) -> Option<(Piece, Color)> {
    if encoded == EMPTY_PIECE {
        return None;
    }

    let color = unsafe { std::mem::transmute::<u8, Color>((encoded >> 3) & 1) };
    let piece = unsafe { std::mem::transmute::<u8, Piece>((encoded & 0b111) - 1) };

    Some((piece, color))
}

