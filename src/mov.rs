use crate::piece::Piece;


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
    pub const fn new(kind: MoveKind) -> Self {
        MoveFlags(kind as u8)   // promo bits are 0 (=None/Queen)
    }

    /// Convert this flag into a promotion flag.
    pub const fn with_promote(kind: MoveKind, piece: Piece) -> Self {
        debug_assert!(
            matches!(piece, Piece::Knight | Piece::Bishop | Piece::Rook | Piece::Queen),
            "Only N/B/R/Q are legal promotion targets"
        );
        const REGULAR_MOVE_BITS: u8 = 4;
        let kind_mask = (1 << REGULAR_MOVE_BITS) - 1; // dynamically compute 0b1111
        let promo_bits = (piece as u8 & 0b11) << REGULAR_MOVE_BITS;
        MoveFlags((kind as u8 & kind_mask) | promo_bits)
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
            from: 0,
            to: 0,
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
        (self.flags.0 >> 4) != 0
    }

    pub fn promotion_piece(&self) -> Option<Piece> {
        if !self.is_promotion() {
            None
        } else {
            match (self.flags.0 >> 4) & 0b11 {
                0 => Some(Piece::Knight),
                1 => Some(Piece::Bishop),
                2 => Some(Piece::Rook),
                3 => Some(Piece::Queen),
                _ => unreachable!(),
            }
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

    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.moves[..self.len].iter()
    }
}

impl std::fmt::Debug for MoveList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(&self.moves[..self.len])
            .finish()
    }
}
