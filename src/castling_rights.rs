use crate::color::Color;
use crate::tables::zobrist;

/// Bit positions (low nibble)
const WK: u8 = 0b0001;   // white king‑side
const WQ: u8 = 0b0010;   // white queen‑side
const BK: u8 = 0b0100;   // black king‑side
const BQ: u8 = 0b1000;   // black queen‑side

/// Pre‑computed mask that “keeps rights alive” if a move
/// touches the index square.  Any zero bit is a right that
/// must be cleared.
///
///   e1/e8  →  clear both rights of that colour
///   a1/h1  →  clear Q / K side for White
///   a8/h8  →  clear Q / K side for Black
pub const CASTLE_RIGHT_MASK: [u8; 64] = [
    0b1101,0b1111,0b1111,0b1111,0b1100,0b1111,0b1111,0b1110,
    0b1111,0b1111,0b1111,0b1111,0b1111,0b1111,0b1111,0b1111,
    0b1111,0b1111,0b1111,0b1111,0b1111,0b1111,0b1111,0b1111,
    0b1111,0b1111,0b1111,0b1111,0b1111,0b1111,0b1111,0b1111,
    0b1111,0b1111,0b1111,0b1111,0b1111,0b1111,0b1111,0b1111,
    0b1111,0b1111,0b1111,0b1111,0b1111,0b1111,0b1111,0b1111,
    0b1111,0b1111,0b1111,0b1111,0b1111,0b1111,0b1111,0b1111,
    0b0111,0b1111,0b1111,0b1111,0b0011,0b1111,0b1111,0b1011,
];


// Returns the "from" square of the rook, based on the "to" destination of the king, in castling moves.
pub static ROOK_START_FROM_KING_TO: [usize; 64] = [
    64,64, 0,64,64,64, 7,64,
    64,64,64,64,64,64,64,64,
    64,64,64,64,64,64,64,64,
    64,64,64,64,64,64,64,64,
    64,64,64,64,64,64,64,64,
    64,64,64,64,64,64,64,64,
    64,64,64,64,64,64,64,64,
    64,64,56,64,64,64,63,64,
];

// Returns the "to" destination square of the rook, based on the "to" destination of the king, in castling moves.
pub static ROOK_END_FROM_KING_TO: [usize; 64] = [
    64,64, 3,64,64,64, 5,64,
    64,64,64,64,64,64,64,64,
    64,64,64,64,64,64,64,64,
    64,64,64,64,64,64,64,64,
    64,64,64,64,64,64,64,64,
    64,64,64,64,64,64,64,64,
    64,64,64,64,64,64,64,64,
    64,64,59,64,64,64,61,64,
];


/// Compact, branch‑friendly castling rights.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct CastlingRights(pub u8);


// low nibble as above
impl Default for CastlingRights {
    fn default() -> Self { CastlingRights(WK | WQ | BK | BQ) }
}

impl core::fmt::Debug for CastlingRights {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl CastlingRights {
    /* ---------------------------------------------------------------- */
    /* Constructors                                                     */
    /* ---------------------------------------------------------------- */
    pub fn from_str(s: &str) -> Self {
        if s == "-" { return CastlingRights(0) }
        let mut rights = 0u8;
        for ch in s.bytes() {
            rights |= match ch {
                b'K' => WK, b'Q' => WQ,
                b'k' => BK, b'q' => BQ,
                _    => panic!("Unrecognised castling rights: {s}"),
            };
        }
        CastlingRights(rights)
    }

    #[inline(always)]
    pub(crate) fn castling_zobrist(&self) -> u64 {
        let mut zobrist = 0;
        if self.0 & WK != 0 {
            zobrist ^= zobrist::CASTLING[0];
        }
        if self.0 & WQ != 0 {
            zobrist ^= zobrist::CASTLING[1];
        }
        if self.0 & BK != 0 {
            zobrist ^= zobrist::CASTLING[2];
        }
        if self.0 & BQ != 0 {
            zobrist ^= zobrist::CASTLING[3];
        }
        zobrist
    }

    pub const fn default() -> Self {
        CastlingRights(0b1111)
    }

    pub fn to_str(self) -> String {
        let mut out = String::new();
        if self.0 & WK != 0 { out.push('K'); }
        if self.0 & WQ != 0 { out.push('Q'); }
        if self.0 & BK != 0 { out.push('k'); }
        if self.0 & BQ != 0 { out.push('q'); }
        if out.is_empty()   { out.push('-'); }
        out
    }

    /* ---------------------------------------------------------------- */
    /* Queries                                                          */
    /* ---------------------------------------------------------------- */
    #[inline(always)]
    pub fn kingside(self, colour: Color) -> bool {
        (self.0 & if colour.is_white() { WK } else { BK }) != 0
    }

    #[inline(always)]
    pub fn queenside(self, colour: Color) -> bool {
        (self.0 & if colour.is_white() { WQ } else { BQ }) != 0
    }
}
