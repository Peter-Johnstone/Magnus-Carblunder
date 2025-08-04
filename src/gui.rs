use std::collections::HashMap;
use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets, Skin};
use crate::mov::MoveList;
use crate::color::Color;
use crate::game_controller::GameMode;
use crate::position::{Position};

pub struct GuiState {
    textures: HashMap<String, Texture2D>,
    x_offset: f32,
    y_offset: f32,
    flipped:  bool,
    game_mode: GameMode,
    white_engine_name: String,
    black_engine_name: String,
    flip_skin: Skin,
    restart_skin: Skin,
}

#[derive(Debug, Clone, Copy)]
pub enum UiEvent {
    FlipPressed,
    RestartPressed,
}



const WHITE_SQUARE_COLOR: macroquad::color::Color = macroquad::color::Color::from_rgba(240,217,181, 255);

const BLACK_SQUARE_COLOR: macroquad::color::Color = macroquad::color::Color::from_rgba(181,136,99, 255);

const BACKGROUND_COLOR: macroquad::color::Color = macroquad::color::Color::from_rgba(36,36,36, 255);

const FRAME_COLOR: macroquad::color::Color = macroquad::color::Color::from_rgba(51,51,51, 255);

const DEPTH_TEXT_COLOR: macroquad::color::Color = macroquad::color::Color::from_rgba(233,102,102, 255);




const BOARD_PIXELS: f32 = 640.0;

impl GuiState {
    pub async fn new(flipped: bool, game_mode: GameMode, white_engine_name: String, black_engine_name: String) -> Self {
        let textures = Self::load_piece_textures().await;
        let x_offset = (screen_width() - BOARD_PIXELS)/2.0;
        let y_offset = (screen_height() - BOARD_PIXELS)/2.0;



        let (flip_skin, restart_skin) = Self::build_skins().await;


        GuiState { textures, x_offset, y_offset, flipped, game_mode, white_engine_name, black_engine_name, flip_skin, restart_skin }
    }

    async fn build_skins() -> (Skin, Skin) {
        let flip_img = load_image("res/flip.png").await.unwrap();
        let restart_img = load_image("res/restart.png").await.unwrap();

        let flip_style = root_ui()
            .style_builder()
            .background(flip_img)
            .color_hovered(LIGHTGRAY)
            .color_clicked(DARKGRAY)
            .build();

        let restart_style = root_ui()
            .style_builder()
            .background(restart_img)
            .color_hovered(LIGHTGRAY)
            .color_clicked(DARKGRAY)
            .build();

        // Start from the default skin and override just the button style
        let flip_skin = Skin { button_style: flip_style, ..root_ui().default_skin() };
        let restart_skin = Skin { button_style: restart_style, ..root_ui().default_skin() };
        (flip_skin, restart_skin)
    }


    pub fn draw_buttons(&mut self) -> Vec<UiEvent> {
        let mut events = Vec::new();

        // Flip button with its own skin
        root_ui().push_skin(&self.flip_skin);
        let flip_button = widgets::Button::new("")
            .position(vec2(self.x_offset * 1.5 + BOARD_PIXELS, self.y_offset + BOARD_PIXELS + 20.0))
            .size(vec2(50.0, 50.0))
            .ui(&mut root_ui());
        root_ui().pop_skin();

        if flip_button {
            events.push(UiEvent::FlipPressed);
        }

        // Restart button with a different skin
        root_ui().push_skin(&self.restart_skin);
        let restart_button = widgets::Button::new("")
            .position(vec2(self.x_offset * 1.35 + BOARD_PIXELS, self.y_offset + BOARD_PIXELS + 20.0))
            .size(vec2(50.0, 50.0))
            .ui(&mut root_ui());
        root_ui().pop_skin();

        if restart_button {
            events.push(UiEvent::RestartPressed);
        }

        events
    }

    pub fn flip_board(&mut self) {
        self.flipped = !self.flipped;
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

    pub fn draw_position(&mut self, position: &Position, highlights: &MoveList, depth: u8, eval: i16) {
        let to_squares: Vec<u8> = highlights.iter().map(|m| m.to()).collect();
        self.draw_board(&to_squares);
        self.draw_board_frame();
        self.draw_pieces(position, &self.textures);
        self.draw_depth(depth);
        self.draw_eval(eval);


        self.draw_combatants_names();
    }


    fn draw_eval(&self, eval: i16) {
        let txt = format!("Eval: {}", eval);
        let font = 30.0;
        // top-left corner, adjust as you like
        draw_text(&txt,
                  self.x_offset/4.0,
                  self.y_offset + BOARD_PIXELS/2.5,
                  font,
                  YELLOW);
    }
    fn draw_depth(&self, depth: u8) {
        let txt = format!("Depth Searched: {}", depth);
        let font = 30.0;
        // top-left corner, adjust as you like
        draw_text(&txt,
                  self.x_offset/4.0,
                  self.y_offset + BOARD_PIXELS/2.0,
                  font,
                  DEPTH_TEXT_COLOR);
    }

    fn draw_board(&self, highlights: &[u8]) {
        clear_background(BACKGROUND_COLOR);
        // draw 8x8 chessboard
        let tile_size = 80.0;
        for row in 0..8 {
            for col in 0..8 {
                let square_index = if self.flipped {
                    row * 8 + (7 - col)          // 180-degree view
                } else {
                    (7 - row) * 8 + col          // White at the bottom
                };
                let x = col as f32 * tile_size;
                let y = row as f32 * tile_size;

                let is_highlighted = highlights.contains(&(square_index as u8));
                let base_color = if (row + col) % 2 == 0 { WHITE_SQUARE_COLOR } else { BLACK_SQUARE_COLOR };
                let color = if is_highlighted { YELLOW } else { base_color };

                draw_rectangle(x+self.x_offset, y+self.y_offset, tile_size, tile_size, color);
            }
        }
    }

    fn draw_board_frame(&self) {
        let width = 10.0;

        // On the y
        draw_rectangle(self.x_offset - width, self.y_offset - width, width, BOARD_PIXELS + 2.0 * width, FRAME_COLOR);
        draw_rectangle(self.x_offset + BOARD_PIXELS, self.y_offset - width, width, BOARD_PIXELS + 2.0 * width, FRAME_COLOR);

        // on the x
        draw_rectangle(self.x_offset - width, self.y_offset - width, BOARD_PIXELS + 2.0 * width, width, FRAME_COLOR);
        draw_rectangle(self.x_offset - width, self.y_offset + BOARD_PIXELS, BOARD_PIXELS + 2.0 * width, width, FRAME_COLOR);

        let margin = 25.0;
        let font_size = 30.0;
        // Draw rank and file labels
        let x = self.x_offset - width - margin;
        for i in 1..9 {
            let y = self.y_offset + (i-1) as f32 * 80.0 + 40.0;

            let i = if self.flipped { i } else { 9-i };
            draw_text(&*i.to_string(), x, y, font_size, WHITE);
        }

        let y = self.y_offset + BOARD_PIXELS + width + margin;
        for i in 1..9 {
            let x = self.x_offset + (i-1) as f32 * 80.0 + 40.0;

            let i = if self.flipped { 9-i } else { i };

            let file_char = (b'a' + (i-1)) as char;
            draw_text(&*file_char.to_string(), x, y, font_size, WHITE);
        }
    }



    fn draw_combatants_names(&self) {
        let font_size = 40.0;

        // center line of your area; use /2.0 if you truly want the middle
        let center_x = self.x_offset + BOARD_PIXELS / 2.0;

        let y_top = self.y_offset / 1.2;
        let y_bot = 1.6 * self.y_offset + BOARD_PIXELS;
        let (y_white, y_black) = if self.flipped { (y_top, y_bot) } else { (y_bot, y_top) };

        let black_color = YELLOW;
        let white_color = GREEN;

        let (black_name, white_name) = match self.game_mode {
            GameMode::PlayerBlack => ("Player", &*self.white_engine_name),
            GameMode::PlayerWhite => (&*self.black_engine_name, "Player"),
            GameMode::PlayersOnly => ("Player 2", "Player 1"),
            GameMode::EnginesOnly => (&*self.black_engine_name, &*self.white_engine_name),
        };

        draw_centered_text(black_name, center_x, y_black, font_size, black_color);
        draw_centered_text(white_name, center_x, y_white, font_size, white_color);
    }


    fn update_square(&self, position: &Position, square: u64, color: &str, textures: &HashMap<String, Texture2D>) {
        let pawns = position.pawns(Color::White)     | position.pawns(Color::Black);
        let knights = position.knights(Color::White) | position.knights(Color::Black);
        let bishops = position.bishops(Color::White) | position.bishops(Color::Black);
        let rooks = position.rooks(Color::White) | position.rooks(Color::Black);
        let queens = position.queens(Color::White) | position.queens(Color::Black);
        let kings = position.kings(Color::White) | position.kings(Color::Black);

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
        let (file, rank_times8) = self.calc_row_col(col as i32, row as i32);
        let x = file         as f32 * tile_size;
        let y = (rank_times8 as f32 / 8.0) * tile_size;   // undo the “×8” so 0-7 → 0-560


        draw_texture_ex(&*texture, x+self.x_offset, y+self.y_offset, WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(80.0, 80.0)), // target size
            ..Default::default()
        },);

    }
    pub fn draw_pieces(&self, position: &Position, textures: &HashMap<String, Texture2D>) {

        let all_pieces = position.white() | position.black();

        for square_index in 0..64 {
            let square: u64 = 1 << square_index;

            if square & all_pieces == 0 {
                continue;
            }
            if square & position.white() != 0 {
                self.update_square(position, square, "white", textures);
                continue;
            } else if square & position.black() != 0 {
                self.update_square(position, square, "black", textures);
                continue;
            }
        }
    }

    fn calc_row_col(&self, x: i32, y: i32) -> (u8, u8) {
        if self.flipped {
            ( (7 - x) as u8,  (y as u8) * 8 )         // file = 7-x,  rank = y
        } else {
            (  x      as u8, ((7 - y) as u8) * 8 )   // file = x,   rank = 7-y
        }
    }


    pub fn get_mouse_square(&self, mouse: (f32, f32)) -> Option<u8> {
        let tile_size = 80.0;
        let x = (mouse.0 - self.x_offset) as i32/ tile_size as i32;
        let y = (mouse.1 - self.y_offset) as i32/ tile_size as i32;
        if x > 8 || y > 8 || x < 0 || y < 0 {
            return None;
        }
        let (x1, y1) = self.calc_row_col(x,y);
        Some(x1+y1)
    }
}

fn draw_centered_text(text: &str, center_x: f32, y: f32, font_size: f32, color: macroquad::color::Color) {
    // scale = 1.0 if you’re not scaling the font
    let dims = measure_text(text, None, font_size as u16, 1.0);
    let x = center_x - dims.width * 0.5;
    draw_text(text, x, y, font_size, color);
}









