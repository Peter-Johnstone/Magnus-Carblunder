
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
}