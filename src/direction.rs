#[derive(Copy, Clone, Debug)]
#[repr(u8)]                     // lets us cast to usize cheaply
pub(crate) enum Dir { N, E, S, W, NE, NW, SE, SW }

impl Dir {
    pub const ALL: [Dir; 8] = [
        Dir::N, Dir::E, Dir::S, Dir::W, Dir::NE, Dir::NW, Dir::SE, Dir::SW,
    ];

    #[inline(always)]
    pub fn idx(self) -> usize { self as usize }

    #[inline(always)]
    pub fn is_ortho(self) -> bool { self as u8 <= Dir::W as u8 } // first 4 are orthogonal
}