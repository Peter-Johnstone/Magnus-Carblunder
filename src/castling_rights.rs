use crate::color::Color;

#[derive(Default, Debug, Clone, Copy)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl CastlingRights {
    pub fn from_str(s: &str) -> CastlingRights {
        let mut castling_rights: CastlingRights = CastlingRights::default();
        for ch in s.chars() {
            match ch {
                'k' => castling_rights.black_kingside = true,
                'q' => castling_rights.black_queenside = true,
                'K' => castling_rights.white_kingside = true,
                'Q' => castling_rights.white_queenside = true,
                '-' => {}
                _ => {panic!("Unrecognized castling_rights {}", s);}
            }
        }
        castling_rights
    }

    pub fn remove_castling_right(&mut self, from: usize) {
        match from {
            0  => self.white_queenside = false,
            7 => self.white_kingside  = false,
            56  => self.black_queenside = false,
            63 => self.black_kingside  = false,
            _ => panic!("Castling right could not be removed!")
        }
    }

    pub fn remove_castling_rights(&mut self, color: Color) {
        match color {
            Color::White => {
                self.white_kingside = false;
                self.white_queenside = false;
            }
            Color::Black => {
                self.black_kingside = false;
                self.black_queenside = false;
            }
        }
    }


    pub fn kingside(&self, color: Color) -> bool {
        if color == Color::White {
            self.white_kingside
        } else {
            self.black_kingside
        }
    }

    pub fn queenside(&self, color: Color) -> bool {
        if color == Color::White {
            self.white_queenside
        } else {
            self.black_queenside
        }
    }
}