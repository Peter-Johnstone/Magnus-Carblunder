use std::cmp::Ordering;
use std::fmt;
use rand::Rng;
use crate::piece::Piece;
use crate::position;
use macroquad::audio::{load_sound, play_sound, PlaySoundParams, Sound};
use std::sync::OnceLock;



#[repr(transparent)]
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Debug)]
pub struct Move(u16);



static CAPTURE_SOUND: OnceLock<Sound> = OnceLock::new();
static QUIET_SOUND:   OnceLock<Sound> = OnceLock::new();
static CHECK_SOUND:   OnceLock<Sound> = OnceLock::new();
static CASTLE_SOUND:   OnceLock<Sound> = OnceLock::new();
static PROMOTE_SOUND:   OnceLock<Sound> = OnceLock::new();





pub async fn init_sounds() {
    CAPTURE_SOUND
        .set(load_sound("res/capture.wav").await.unwrap())
        .expect("CAPTURE_SOUND already initialised");
    QUIET_SOUND
        .set(load_sound("res/quiet.wav").await.unwrap())
        .expect("QUIET_SOUND already initialised");
    CHECK_SOUND
        .set(load_sound("res/check.wav").await.unwrap())
        .expect("CHECK_SOUND already initialised");
    CASTLE_SOUND
        .set(load_sound("res/castle.wav").await.unwrap())
        .expect("CASTLE_SOUND already initialised");
    PROMOTE_SOUND
        .set(load_sound("res/promote.wav").await.unwrap())
        .expect("PROMOTE_SOUND already initialised");

}


pub mod flag {
    pub const QUIET:                u16 = 0;
    pub const DOUBLE_PAWN_PUSH:     u16 = 1;
    pub const KING_CASTLE:          u16 = 2;
    pub const QUEEN_CASTLE:         u16 = 3;
    pub const CAPTURE:              u16 = 4;
    pub const EN_PASSANT:           u16 = 5;
    pub const PROMO_KNIGHT:         u16 = 8;
    pub const PROMO_BISHOP:         u16 = 9;
    pub const PROMO_ROOK:           u16 = 10;
    pub const PROMO_QUEEN:          u16 = 11;
    pub const PROMO_KNIGHT_CAPTURE: u16 = 12;
    pub const PROMO_BISHOP_CAPTURE: u16 = 13;
    pub const PROMO_ROOK_CAPTURE:   u16 = 14;
    pub const PROMO_QUEEN_CAPTURE:  u16 = 15;
}



// Shifts
const FROM_SHIFT: u16 = 0;
const TO_SHIFT: u16 = 6;
const FLAG_SHIFT: u16 = 12;

// Masks
const FROM_MASK: u16 = 0x003F;
const TO_MASK:   u16 = 0x0FC0;
const FLAG_MASK: u16 = 0xF000;


impl Move {

    #[inline(always)]
    pub fn new(mov: u16) -> Move {
        Move(mov)
    }
    #[inline(always)]
    pub fn encode(from: u8, to: u8, flags: u16) -> Move {
        Move(
            (from as u16)   << FROM_SHIFT  |
            (to as u16)     << TO_SHIFT    |
            (flags & 0xF)   << FLAG_SHIFT
        )
    }

    pub fn encode_from_string(from: &str, to: &str, flags: &str) -> Move {
        let from_sq = algebraic_to_index(from);
        let to_sq = algebraic_to_index(to);
        let flag_val = match flags.to_uppercase().as_str() {
            "QUIET" => flag::QUIET,
            "DOUBLE_PAWN_PUSH" => flag::DOUBLE_PAWN_PUSH,
            "KING_CASTLE" => flag::KING_CASTLE,
            "QUEEN_CASTLE" => flag::QUEEN_CASTLE,
            "CAPTURE" => flag::CAPTURE,
            "EN_PASSANT" => flag::EN_PASSANT,
            "PROMO_KNIGHT" => flag::PROMO_KNIGHT,
            "PROMO_BISHOP" => flag::PROMO_BISHOP,
            "PROMO_ROOK" => flag::PROMO_ROOK,
            "PROMO_QUEEN" => flag::PROMO_QUEEN,
            "PROMO_KNIGHT_CAPTURE" => flag::PROMO_KNIGHT_CAPTURE,
            "PROMO_BISHOP_CAPTURE" => flag::PROMO_BISHOP_CAPTURE,
            "PROMO_ROOK_CAPTURE" => flag::PROMO_ROOK_CAPTURE,
            "PROMO_QUEEN_CAPTURE" => flag::PROMO_QUEEN_CAPTURE,
            _ => unreachable!("Please enter valid flags")
        };

        Move::encode(from_sq, to_sq, flag_val)
    }

    pub fn play_move_sound(&self, in_check: bool) {
        let is_capture = self.is_capture() || self.is_en_passant();
        
        let sound = if is_capture {
            CAPTURE_SOUND.get()
        } else if self.is_king_castle() || self.is_queen_castle() {
            CASTLE_SOUND.get()
        } else if in_check {
            CHECK_SOUND.get()
        } else if self.is_promotion() {
            PROMOTE_SOUND.get()
        } else {
            QUIET_SOUND.get()
        };

        if let Some(s) = sound {            // `Sound` is `Copy`
            play_sound(&s, PlaySoundParams {
                looped: false,
                volume: 0.8,
            });
        }
    }

    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
    pub const fn null() -> Self {
        Move(0)
    }
    #[inline(always)]
    pub fn to(&self) -> u8 {
        ((self.0 & TO_MASK) >> TO_SHIFT) as u8
    }
    #[inline(always)]
    pub fn from(&self) -> u8{
        ((self.0 & FROM_MASK) >> FROM_SHIFT) as u8
    }
    #[inline(always)]
    pub(crate) fn flag(&self) -> u16 {
        (self.0 & FLAG_MASK) >> FLAG_SHIFT
    }

    #[inline(always)]
    pub fn promotion_piece(self) -> Piece {
        debug_assert!(self.is_promotion());
        match self.flag() & 0b0011 {
            0 => Piece::Knight,
            1 => Piece::Bishop,
            2 => Piece::Rook,
            3 => Piece::Queen,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    pub fn is_quiet(&self) -> bool {
        self.flag() == flag::QUIET
    }

    #[inline(always)]
    pub fn is_double_push(self) -> bool {
        self.flag() == flag::DOUBLE_PAWN_PUSH
    }

    #[inline(always)]
    pub fn is_en_passant(self) -> bool {
        self.flag() == flag::EN_PASSANT
    }

    #[inline(always)]
    pub fn is_king_castle(self) -> bool {
        self.flag() == flag::KING_CASTLE
    }

    #[inline(always)]
    pub fn is_queen_castle(self) -> bool {
        self.flag() == flag::QUEEN_CASTLE
    }

    #[inline(always)]
    pub fn is_capture(self) -> bool {
        matches!(self.flag(),
        flag::CAPTURE
      | flag::PROMO_KNIGHT_CAPTURE
      | flag::PROMO_BISHOP_CAPTURE
      | flag::PROMO_ROOK_CAPTURE
      | flag::PROMO_QUEEN_CAPTURE
    )
    }

    #[inline(always)]
    pub fn is_castling(self) -> bool {
        matches!(self.flag(), flag::KING_CASTLE | flag::QUEEN_CASTLE)
    }
    #[inline(always)]
    pub fn is_promotion(self) -> bool {
        self.flag() & 0b1000 != 0          // bit 3 is set on every promotion code
    }


    pub fn print_raw(self) {
        println!("{},", self.0);
    }
}

pub const fn quiet_promo_flag(piece: Piece) -> u16 {
    match piece {
        Piece::Knight => flag::PROMO_KNIGHT,
        Piece::Bishop => flag::PROMO_BISHOP,
        Piece::Rook   => flag::PROMO_ROOK,
        Piece::Queen  => flag::PROMO_QUEEN,
        _ => panic!("invalid promotion piece"),
    }
}

pub const fn capture_promo_flag(piece: Piece) -> u16 {
    match piece {
        Piece::Knight => flag::PROMO_KNIGHT_CAPTURE,
        Piece::Bishop => flag::PROMO_BISHOP_CAPTURE,
        Piece::Rook   => flag::PROMO_ROOK_CAPTURE,
        Piece::Queen  => flag::PROMO_QUEEN_CAPTURE,
        _ => panic!("invalid promotion piece"),
    }
}

/// bit‑3 = promotion, bit‑2 = capture
#[inline(always)]
pub fn is_flag_quiet_promo(flag: u16) -> bool {
    flag & 0b1000 != 0 && flag & 0b0100 == 0       // 8‑11
}

#[inline(always)]
pub fn is_flag_capture_promo(flag: u16) -> bool {
    flag & 0b1100 == 0b1100                        // 12‑15
}

#[inline(always)]
fn algebraic_to_index(square: &str) -> u8 {
    if square.len() != 2 {
        panic!("invalid square length");
    }
    let bytes = square.as_bytes();
    let file = bytes[0].to_ascii_lowercase();
    let rank = bytes[1];

    if !(b'a'..=b'h').contains(&file) || !(b'1'..=b'8').contains(&rank) {
        panic!("invalid rank");
    }

    let file_idx = file - b'a';
    let rank_idx = rank - b'1';

    rank_idx * 8 + file_idx
}
#[inline(always)]
pub fn index_to_algebraic(square: u8) -> String {
    if square > 63 {
        panic!("invalid square {square}");
    }
    let file = square % 8;
    let rank = square / 8;

    let file_char = (b'a' + file) as char;
    let rank_char = (b'1' + rank) as char;

    format!("{file_char}{rank_char}")
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


    #[inline(always)]
    pub fn swap(&mut self, i: usize, j: usize) {
        if i < self.len && j < self.len {
            let tmp = self.moves[i];
            self.moves[i] = self.moves[j];
            self.moves[j] = tmp;
        }
    }


    #[inline(always)]
    pub fn get(&self, index: usize) -> Move {
        self.moves[index]
    }

    #[inline(always)]
    /// NOT CHECKED!!! Make sure mv.is_null() is false!
    pub (crate) fn index(&self, search_mv: Move) -> usize {
        for (i, mv) in self.moves.iter().enumerate() {
            if *mv == search_mv {
                return i;
            }
        }
        255
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline(always)]
    pub fn random(&self) -> Move {
        let num = rand::rng().random_range(0..self.len);
        self.moves[num]
    }

    #[inline(always)]
    pub fn sort_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(Move, Move) -> Ordering,
    {
        // only the first `len` entries are meaningful
        self.moves[..self.len].sort_by(|a, b| cmp(*a, *b));
    }

    #[inline(always)]
    pub fn peek(&self) -> Move {
        if self.len > 1 {
            return self.moves[self.len - 1]
        }
        Move::default()
    }

    #[inline(always)]
    pub fn moves_from_square(&self, square: u8) -> MoveList {
        let mut square_moves = MoveList::new();
        for mov in self.moves[..self.len].iter() {
            if mov.from() == square {
                square_moves.push(*mov);
            }
        }
        square_moves
    }

    #[inline(always)]
    pub fn push(&mut self, m: Move) {
        self.moves[self.len] = m;
        self.len += 1;
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = Move> {
        self.moves[..self.len].iter().copied()
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_null(){
            write!(f, "NULL")?;
            return Ok(())
        }
        write!(
            f,
            "{}{}",
            position::square_name(self.from()),
            position::square_name(self.to())
        )?;
        if self.is_promotion() {
            write!(f, "{}", self.promotion_piece().piece_initial())?;
        }

        Ok(())
    }

}

impl fmt::Debug for MoveList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(&self.moves[..self.len])
            .finish()
    }
}









const NEW_EN_PASSANT_FROM: [u8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    16,17,18,19,20,21,22,23,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    40,41,42,43,44,45,46,47,
    0, 0, 0, 0, 0, 0, 0, 0,
];

#[inline(always)]
pub fn new_en_passant_square(from: usize) -> u8 {
    NEW_EN_PASSANT_FROM[from]
}




// returns the square of the en_passant captured piece given the square the move goes to.
const CAPTURED_SQUARE_TO: [usize; 64] = [
     0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0,
    24,25,26,27,28,29,30,31,
     0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0,
    32,33,34,35,36,37,38,39,
    0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0,
];


#[inline(always)]
pub fn en_passant_capture_pawn(to: usize) -> usize {
    CAPTURED_SQUARE_TO[to]
}