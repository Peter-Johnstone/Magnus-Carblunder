use std::fmt;
use std::fmt::Debug;
use crate::piece::Piece;
use crate::position;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MoveKind {
    Quiet       = 0,
    DoublePush  = 1,
    Capture     = 2,
    EnPassant   = 3,
    Castling    = 4,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct MoveFlags(u8);

impl MoveFlags {
    const REGULAR_BITS: u8 = 4;               // bits 0‑3
    const PROMO_FLAG:  u8 = 1 << Self::REGULAR_BITS;      // bit 4
    const PROMO_SHIFT: u8 = Self::REGULAR_BITS + 1;       // bits 5‑6

    /// Non‑promotion constructor
    pub const fn new(kind: MoveKind) -> Self {
        MoveFlags(kind as u8)                 // promo flag = 0
    }

    /// Promotion constructor
    pub const fn with_promote(kind: MoveKind, piece: Piece) -> Self {
        debug_assert!(matches!(
            piece,
            Piece::Knight | Piece::Bishop | Piece::Rook | Piece::Queen
        ));
        // Knight→0, Bishop→1, Rook→2, Queen→3
        let piece_bits = ((piece as u8 - 1) & 0b11) << Self::PROMO_SHIFT;
        MoveFlags((kind as u8) | Self::PROMO_FLAG | piece_bits)
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Move {
    from: u8,
    to: u8,
    flags: MoveFlags,
}

impl Default for Move {
    fn default() -> Self {
        Move {
            from: 64,
            to: 64,
            flags: MoveFlags::new(MoveKind::Quiet), // or some invalid combo
        }
    }
}

impl Move {
    pub fn encode(from: u8, to: u8, flags: MoveFlags) -> Move {
        Move { from, to, flags}
    }

    pub fn to(&self) -> u8{
        self.to
    }
    pub fn from(&self) -> u8{
        self.from
    }

    pub(crate) fn kind(&self) -> MoveKind {
        match self.flags.0 & 0b1111 {
            0 => MoveKind::Quiet,
            1 => MoveKind::DoublePush,
            2 => MoveKind::Capture,
            3 => MoveKind::EnPassant,
            4 => MoveKind::Castling,

            _ => unreachable!(),
        }
    }


    pub fn is_promotion(&self) -> bool {
        self.flags.0 & MoveFlags::PROMO_FLAG != 0
    }

    pub fn promotion_piece(&self) -> Option<Piece> {
        if !self.is_promotion() {
            return None;
        }
        match (self.flags.0 >> MoveFlags::PROMO_SHIFT) & 0b11 {
            0 => Some(Piece::Knight),
            1 => Some(Piece::Bishop),
            2 => Some(Piece::Rook),
            3 => Some(Piece::Queen),
            _ => unreachable!(),
        }
    }

    pub fn new_en_passant_square(&self) -> u8 {
        if self.is_double_push() {
            (self.to() + self.from()) / 2
        } else {
            64
        }
    }

    pub fn is_quiet(&self) -> bool {
        self.kind() == MoveKind::Quiet
    }

    pub fn is_double_push(&self) -> bool {
        self.kind() == MoveKind::DoublePush
    }

    pub fn is_capture(&self) -> bool {
        self.kind() == MoveKind::Capture
    }

    pub fn is_en_passant(&self) -> bool {
        self.kind() == MoveKind::EnPassant
    }
    
    pub fn is_castling(&self) -> bool {
        self.kind() == MoveKind::Castling
    }
}


pub const MAX_MOVES: usize = 256; // Safe upper bound


pub struct MoveList {
    pub moves: [Move; MAX_MOVES],
    pub len: usize,
}

impl MoveList {
    pub fn new() -> Self {
        MoveList {
            moves: [Move::default(); MAX_MOVES],
            len: 0,
        }
    }

    pub fn peek(&self) -> Move {
        if self.len > 1 {
            return self.moves[self.len - 1]
        }
        Move::default()
    }

    pub fn moves_from_square(&self, square: u8) -> MoveList {
        let mut square_moves = MoveList::new();
        for mov in self.moves {
            if mov.from == square {
                square_moves.push(mov);
            }
        }
        square_moves
    }

    pub fn push(&mut self, m: Move) {
        self.moves[self.len] = m;
        self.len += 1;
    }

    pub fn iter(&self) -> impl Iterator<Item = Move> {
        self.moves[..self.len].iter().copied()
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            position::square_name(self.from),
            position::square_name(self.to)
        )?;
        if let Some(promo) = self.promotion_piece() {
            write!(f, "{}", promo.piece_initial())?;
        }

        Ok(())
    }

}

impl fmt::Debug for MoveList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(&self.moves[..self.len])
            .finish()
    }
}

