use crate::castling_rights::CastlingRights;
use crate::mov::Move;
use crate::piece::EMPTY_PIECE;
use crate::position::{StateInfo, NO_SQ};

const MAX_PLY: usize = u8::MAX as usize;   // safe upper bound for any search depth

#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct Undo {
    pub(crate) captured_piece:      i8,
    pub(crate) captured_square:     u8,
    pub(crate) castling:            CastlingRights,
    pub(crate) en_passant:          u8,
    pub(crate) half_move:           u8,
    pub(crate) zobrist:             u64,
    pub(crate) state_info:          StateInfo,
    pub(crate) mov:                 Move,
    pub(crate) delta_raw_piece_diff:i32,
    pub(crate) delta_mg:            i32,
    pub(crate) delta_eg:            i32,
    pub(crate) delta_phase:         i32,
}

#[derive(Clone, Debug)]
pub struct UndoStack {
    pub data: [Undo; MAX_PLY],
    pub len:  u8,                  // 0‥=MAX_PLY  (fits in a register)
}


impl UndoStack {
    #[inline(always)]
    pub const fn new() -> Self {
        const ZERO_UNDO: Undo = Undo {
            captured_piece: EMPTY_PIECE,
            captured_square: NO_SQ,
            castling: CastlingRights::default(),
            en_passant: NO_SQ,
            half_move: 0,
            zobrist: 0,
            state_info: StateInfo::default(),
            mov: Move::null(),
            delta_raw_piece_diff: 0,
            delta_mg: 0,
            delta_eg: 0,
            delta_phase: 0,
        };
        UndoStack { data: [ZERO_UNDO; MAX_PLY], len: 0 }
    }
    /// Mutable reference to the last element (top of stack).
    /// # Safety
    /// Caller must guarantee that the stack is **not** empty.
    #[inline(always)]
    pub fn last_mut(&mut self) -> &mut Undo {
        debug_assert!(self.len > 0, "last_mut on empty UndoStack");
        // SAFETY: len > 0 ⇒ len-1 is a valid index
        unsafe { self.data.get_unchecked_mut(self.len as usize - 1) }
    }


    /// Push without capacity check in release; checked in debug.
    #[inline(always)]
    pub fn push(&mut self, u: Undo) {
        debug_assert!(self.len < MAX_PLY as u8, "undo stack overflow");
        unsafe {
            *self.data.get_unchecked_mut(self.len as usize) = u;
        }
        self.len += 1;
    }

    /// Pop and return the last `Undo`.
    /// Returns `None` if the stack is empty.
    #[inline(always)]
    pub fn pop(&mut self) -> Undo {
        //unsafe

        // Fast path: non‑empty is overwhelmingly common.
        self.len -= 1;
        // SAFETY: we just decremented len, so index is in‑bounds.
        unsafe { *self.data.get_unchecked(self.len as usize) }
    }
    #[inline(always)]
    pub fn peek_index(&self, index: usize) -> Undo {
        self.data[index]
    }

    #[inline(always)]
    pub fn is_near_full(&self) -> bool {
        self.len > 200
    }

    #[inline(always)]
    pub fn make_space(&mut self) {
        self.len = 0;
    }

    /// Removes and returns the first `Undo` (the bottom of the stack),
    /// shifting all others left. Returns `None` if empty.
    pub fn pop_front(&mut self) -> Option<Undo> {
        if self.is_empty() {
            return None;
        }

        let first = self.data[0];

        for i in 1..self.len as usize {
            self.data[i - 1] = self.data[i];
        }

        self.len -= 1;
        Some(first)
    }
    

    #[inline(always)]
    pub(crate) fn is_empty(&self) -> bool {
        self.len == 0
    }
}
