use std::collections::HashMap;
use macroquad::prelude::*;
use crate::mov::MoveList;
use crate::position::Position;

pub struct GuiState {
    textures: HashMap<String, Texture2D>,
}

impl GuiState {
    pub async fn new() -> Self {
        let textures = Self::load_piece_textures().await;
        GuiState { textures }
    }

    async fn load_piece_textures() -> HashMap<String, Texture2D> {
        let mut textures = HashMap::new();
        let colors = ["white", "black"];
        let pieces = ["pawn", "knight", "bishop", "rook", "queen", "king"];

        for color in colors {
            for piece in pieces {
                let key = format!("{}_{}", color, piece);
                let path = format!("res/{}_{}.png", color, piece);
                let texture = load_texture(&path).await.unwrap();
                textures.insert(key, texture);
            }
        }

        textures
    }

    pub fn draw_position(&self, position: &Position, highlights: &MoveList) {
        let to_squares: Vec<u8> = highlights.iter().map(|m| m.to()).collect();
        draw_board(&to_squares);
        draw_pieces(position, &self.textures);
    }

}


fn draw_board(highlights: &[u8]) {
    clear_background(WHITE);
    // draw 8x8 chessboard
    let tile_size = 80.0;
    for row in 0..8 {
        for col in 0..8 {
            let square_index = (7 - row) * 8 + col; // convert to 0-based square index with A1 at bottom-left
            let x = col as f32 * tile_size;
            let y = row as f32 * tile_size;

            let is_highlighted = highlights.contains(&(square_index as u8));
            let base_color = if (row + col) % 2 == 0 { LIGHTGRAY } else { DARKGRAY };
            let color = if is_highlighted { GREEN } else { base_color };

            draw_rectangle(x, y, tile_size, tile_size, color);
        }
    }

}

pub fn get_mouse_square(mouse: (f32, f32)) -> u8 {
    let tile_size = 80.0;
    let x = mouse.0 as i32/ tile_size as i32;
    let y = mouse.1 as i32/ tile_size as i32;
    (x + (7-y)*8) as u8
}


fn update_square(position: &Position, square: u64, color: &str, textures: &HashMap<String, Texture2D>) {
    let pawns = position.pawns();
    let knights = position.knights();
    let bishops = position.bishops();
    let rooks = position.rooks();
    let queens = position.queens();
    let kings = position.kings();

    let piece_name: &str;
    if square&pawns != 0 {
        piece_name = "pawn";
    } else if square&knights != 0 {
        piece_name = "knight";
    } else if square&bishops != 0 {
        piece_name = "bishop";
    } else if square&rooks != 0 {
        piece_name = "rook";
    } else if square&queens != 0 {
        piece_name = "queen";
    } else if square&kings != 0 {
        piece_name = "king";
    } else {
        panic!("Invalid piece!")
    }

    let key = format!("{}_{}", color, piece_name); // e.g. "black_queen"
    let texture = textures.get(&key).unwrap();

    let col = square.trailing_zeros() % 8;
    let row = square.trailing_zeros() / 8;
    let tile_size = 80.0;

    let x = col as f32 * tile_size;
    let y = 7f32 * tile_size - row as f32 * tile_size; // row one means we're at the bottom of the screen

    draw_texture_ex(&*texture, x, y, WHITE, DrawTextureParams {
        dest_size: Some(Vec2::new(80.0, 80.0)), // target size
        ..Default::default()
    },);

}
pub fn draw_pieces(position: &Position, textures: &HashMap<String, Texture2D>) {

    let all_pieces = position.white() | position.black();

    for square_index in 0..64 {
        let square: u64 = 1 << square_index;

        if square & all_pieces == 0 {
            continue;
        }
        if square & position.white() != 0 {
            update_square(position, square, "white", textures);
            continue;
        } else if square & position.black() != 0 {
            update_square(position, square, "black", textures);
            continue;
        }
    }
}

