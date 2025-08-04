use rand::{Rng};


pub fn zobrist_randoms_pieces() -> [[[u64; 2]; 6]; 64] {
    let mut rng = rand::rng();

    let mut table = [[[0; 2]; 6]; 64]; // Using u64 is more typical for Zobrist hashing
    for color in 0..2 {
        for piece in 0..6 {
            for sq in 0..64 {
                table[sq][piece][color] = rng.random(); // Fill with a random u64
            }
        }
    }
    table
}

pub fn zobrist_randoms_turn() -> u64 {
    let mut rng = rand::rng();
    rng.random()
}

pub fn zobrist_randoms_castling() -> [u64; 4] {
    let mut rng = rand::rng();

    let mut table = [0u64; 4];

    for castle in 0..4 {
        table[castle] = rng.random();
    }
    table
}

pub fn zobrist_randoms_en_passant() -> [u64; 8] {
    let mut rng = rand::rng();
    let mut table = [0u64; 8];

    for file in 0.. 8 {
        table[file] = rng.random();
    }
    table
}


