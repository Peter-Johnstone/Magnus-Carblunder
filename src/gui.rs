use std::collections::HashMap;
use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets, Skin};
use crate::mov::{Move, MoveList};
use crate::color::Color::{Black, White};
use crate::game_controller::GameMode;
use crate::position::{Position, Status};
use crate::position::Status::{Checkmate, Ongoing};

pub struct GuiState {
    textures: HashMap<String, Texture2D>,
    x_offset: f32,
    y_offset: f32,
    flipped:  bool,
    game_mode: GameMode,
    white_engine_name: String,
    black_engine_name: String,
    piece_style_idx: usize,
    flip_skin: Skin,
    restart_skin: Skin,
    flip_piece_style_skin: Skin,
    font: Font,
}

#[derive(Debug, Clone, Copy)]
pub enum UiEvent {
    FlipPressed,
    RestartPressed,
    FlipPieceStyle,
}

#[derive(Clone, Copy)]
pub struct AnimRender {
    pub from: u8,
    pub to: u8,
    pub t: f32, // 0..1
}




const WHITE_SQUARE_COLOR: macroquad::color::Color = macroquad::color::Color::from_rgba(240,217,181, 255);

const BLACK_SQUARE_COLOR: macroquad::color::Color = macroquad::color::Color::from_rgba(181,136,99, 255);

const BACKGROUND_COLOR: macroquad::color::Color = macroquad::color::Color::from_rgba(36,36,36, 255);
const SELECTED_PIECE_SQUARE_COLOR: macroquad::color::Color = macroquad::color::Color::from_rgba(194,231,255, 255);
const STARTING_PIECE_STYLE_INDEX: usize = 18;
const PIECE_STYLES: [&str; 36] = [
    "3d_chesskid",
    "3d_plastic",
    "3d_staunton",
    "3d_wood",
    "8_bit",
    "alpha",
    "bases",
    "book",
    "bubblegum",
    "cases",
    "classic",
    "club",
    "condal",
    "dash",
    "game_room",
    "glass",
    "gothic",
    "graffiti",
    "icy_sea",
    "light",
    "lolz",
    "marble",
    "maya",
    "metal",
    "modern",
    "nature",
    "neo",
    "neo_wood",
    "newspaper",
    "ocean",
    "sky",
    "space",
    "tigers",
    "tournament",
    "vintage",
    "wood",
];



const FRAME_COLOR: macroquad::color::Color = macroquad::color::Color::from_rgba(51,51,51, 255);

const DEPTH_TEXT_COLOR: macroquad::color::Color = macroquad::color::Color::from_rgba(233,102,102, 255);

const SELECTED_MOVE_COLOR: macroquad::color::Color = YELLOW;
const LAST_MOVE_COLOR: macroquad::color::Color = macroquad::color::Color::from_rgba(221,207,124, 255);



const BOARD_PIXELS: f32 = 640.0;

impl GuiState {
    pub async fn new(flipped: bool, game_mode: GameMode, white_engine_name: String, black_engine_name: String) -> Self {
        let x_offset = (screen_width() - BOARD_PIXELS)/2.0;
        let y_offset = (screen_height() - BOARD_PIXELS)/2.0;


        let font: Font = load_ttf_font("res/fonts/arial.ttf")
            .await
            .unwrap();
        let (flip_skin, restart_skin, flip_piece_style_skin) = Self::build_skins().await;

        let mut gui = GuiState { textures: HashMap::new(), x_offset, y_offset, flipped, game_mode, white_engine_name, black_engine_name, piece_style_idx: STARTING_PIECE_STYLE_INDEX, flip_skin, restart_skin , flip_piece_style_skin, font};
        gui.update_piece_textures().await;
        gui
    }


    pub fn draw_position_animated(
        &mut self,
        position: &Position,
        highlights: &MoveList,
        selected_square: u8,
        last_move: Move,
        game_status: Status,
        captured_piece_list: &[u8; 10],
        anim: Option<AnimRender>,
    ) {
        let to_squares: Vec<u8> = highlights.iter().map(|m| m.to()).collect();
        let last_move_squares: [u8; 2] = if last_move.is_null() {
            [64, 64]
        } else {
            [last_move.to(), last_move.from()]
        };

        self.draw_board(&to_squares, selected_square, last_move_squares);
        self.draw_board_frame();

        // draw all pieces, but if animating, skip drawing the piece on the dest square
        let skip_mask = anim.map(|a| 1u64 << a.to).unwrap_or(0);
        self.draw_pieces_skip(position, &self.textures, skip_mask);

        // overlay the moving piece at tweened position
        if let Some(a) = anim {
            self.draw_moving_piece(position, a);
        }

        self.draw_captured_pieces(captured_piece_list);
        self.draw_game_status(game_status);
        self.draw_combatants_names();
    }

    fn square_to_xy(&self, sq: u8) -> (f32, f32) {
        let tile = 80.0;
        let col = (sq % 8) as i32;
        let row = (sq / 8) as i32;
        let (file, rank_times8) = self.calc_row_col(col, row);
        let x = file as f32 * tile + self.x_offset;
        let y = (rank_times8 as f32 / 8.0) * tile + self.y_offset;
        (x, y)
    }

    fn texture_key_at(&self, position: &Position, sq: u8) -> Option<String> {
        let bb = 1u64 << sq;
        let color = if position.white() & bb != 0 { "w" }
        else if position.black() & bb != 0 { "b" }
        else { return None; };

        let pawns   = position.pawns(White)   | position.pawns(Black);
        let knights = position.knights(White) | position.knights(Black);
        let bishops = position.bishops(White) | position.bishops(Black);
        let rooks   = position.rooks(White)   | position.rooks(Black);
        let queens  = position.queens(White)  | position.queens(Black);
        let kings   = position.kings(White)   | position.kings(Black);

        let piece = if bb & pawns   != 0 { "p" }
        else if bb & knights != 0 { "n" }
        else if bb & bishops != 0 { "b" }
        else if bb & rooks   != 0 { "r" }
        else if bb & queens  != 0 { "q" }
        else if bb & kings   != 0 { "k" }
        else { return None; };

        Some(format!("{}{}", color, piece))
    }

    pub fn draw_game_status(&self, status: Status) {

        if status == Ongoing {
            return;
        }

        if status == Checkmate(White) || status == Checkmate(Black) {
            draw_centered_text("checkmate!", self.x_offset - 170.0, self.y_offset + BOARD_PIXELS/2.0, &self.font, 40, WHITE)
        } else {
            draw_centered_text("draw", self.x_offset - 170.0, self.y_offset + BOARD_PIXELS/2.0, &self.font, 40, WHITE)
        }
    }

    pub fn draw_eval_bar(&self, eval_cp: i16) {
        let bar_w = 20.0;
        let bar_h = BOARD_PIXELS;
        let x = self.x_offset + BOARD_PIXELS + 40.0; // right of the board
        let y = self.y_offset;

        // background + frame
        draw_rectangle(x, y, bar_w, bar_h, GRAY);
        draw_rectangle_lines(x, y, bar_w, bar_h, 2.0, FRAME_COLOR);

        // normalize eval to 0..1 with a tanh squish
        let scale = 600.0; // tweak: 100 = 1 pawn; 600 gives nice curve
        let p = 0.5 + 0.5 * ((eval_cp as f32) / scale).tanh();
        let white_h = bar_h * p;

        if !self.flipped {
            // white at bottom, black at top — no gap
            let black_h = bar_h - white_h;
            draw_rectangle(x, y, bar_w, black_h, BLACK);
            draw_rectangle(x, y + black_h, bar_w, white_h, WHITE);
        } else {
            // white at top, black at bottom — no gap
            let black_h = bar_h - white_h;
            draw_rectangle(x, y, bar_w, white_h, WHITE);
            draw_rectangle(x, y + white_h, bar_w, black_h, BLACK);
        }
    }

    fn draw_captured_pieces(&self, captured: &[u8; 10]) {
        let mut white_i: f32 = 0.0;
        let mut black_i: f32 = 0.0;


        if captured.iter().any(|&x| x > 0) {
            draw_rectangle(self.x_offset + BOARD_PIXELS + 100.0,
                           3.4 * self.y_offset,
                           300.0,
                           BOARD_PIXELS - 4.8 * self.y_offset, DARKGRAY);
        }

        const SIZE: f32 = 42.0;
        const GAP: f32 = 13.0;
        let x = self.x_offset + BOARD_PIXELS + 100.0;
        let (y_white, y_black) = if self.flipped {
            (self.y_offset - SIZE/2.0 + BOARD_PIXELS/ 1.85, self.y_offset - SIZE/2.0 + BOARD_PIXELS / 2.15)
        } else {
            (self.y_offset - SIZE/2.0 + BOARD_PIXELS / 2.15, self.y_offset - SIZE/2.0 + BOARD_PIXELS / 1.85)
        };

        // Small helper to draw N copies of a texture key in a row.
        let mut draw_group = |key: &str, count: u8, i: &mut f32, y: f32| {
            if count == 0 {
                return;
            }
            if let Some(tex) = self.textures.get(key) {
                // create more space between non-equivalent pieces:
                for _ in 0..count {
                    draw_texture_ex(
                        tex,
                        x + (*i) * GAP,
                        y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(SIZE, SIZE)),
                            ..Default::default()
                        },
                    );
                    *i += 1.0;
                }
                *i += 1.4;
            } else {
                println!("not found");
            }
        };

        // White captured: pawn, knight, bishop, rook, queen
        for (key, idx) in [("wp", 0), ("wn", 1), ("wb", 2), ("wr", 3), ("wq", 4)] {
            draw_group(key, captured[idx], &mut white_i, y_white);
        }
        // Black captured: pawn, knight, bishop, rook, queen
        for (key, idx) in [("bp", 5), ("bn", 6), ("bb", 7), ("br", 8), ("bq", 9)] {
            draw_group(key, captured[idx], &mut black_i, y_black);
        }
    }




    fn draw_moving_piece(&self, position: &Position, a: AnimRender) {
        let (fx, fy) = self.square_to_xy(a.from);
        let (tx, ty) = self.square_to_xy(a.to);

        let t = ease_smootherstep(a.t);
        let x = fx + (tx - fx) * t;
        let y = fy + (ty - fy) * t;

        // Choose the texture: use the piece from `from` during flight to avoid
        // “instant promotion morphing”, switch at t==1.0 automatically.
        let key = self.texture_key_at(position, a.to);
        if let Some(key) = key {
            if let Some(tex) = self.textures.get(&key) {
                draw_texture_ex(tex, x, y, WHITE, DrawTextureParams {
                    dest_size: Some(Vec2::new(80.0, 80.0)),
                    ..Default::default()
                });
            }
        }
    }


    fn draw_pieces_skip(&self, position: &Position, textures: &HashMap<String, Texture2D>, skip_mask: u64) {
        let all_pieces = position.white() | position.black();

        for square_index in 0..64 {
            let bb: u64 = 1 << square_index;
            if bb & all_pieces == 0 { continue; }
            if bb & skip_mask != 0 { continue; }

            if bb & position.white() != 0 {
                self.update_square(position, bb, "w", textures);
            } else if bb & position.black() != 0 {
                self.update_square(position, bb, "b", textures);
            }
        }
    }


    async fn build_skins() -> (Skin, Skin, Skin) {
        let flip_img = load_image("res/flip.png").await.unwrap();
        let restart_img = load_image("res/restart.png").await.unwrap();
        let flip_piece_style_img = load_image("res/flip_piece_style.png").await.unwrap();


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

        let flip_piece_style_style = root_ui()
            .style_builder()
            .background(flip_piece_style_img)
            .color_hovered(LIGHTGRAY)
            .color_clicked(DARKGRAY)
            .build();

        // Start from the default skin and override just the button style
        let flip_skin = Skin { button_style: flip_style, ..root_ui().default_skin() };
        let restart_skin = Skin { button_style: restart_style, ..root_ui().default_skin() };
        let flip_piece_style_skins = Skin { button_style: flip_piece_style_style, ..root_ui().default_skin() };

        (flip_skin, restart_skin, flip_piece_style_skins)
    }


    pub fn flip_board(&mut self) {
        self.flipped = !self.flipped;
    }



    async fn update_piece_textures(&mut self) {
        let mut textures = HashMap::new();
        let colors = ["w", "b"];
        let pieces = ["p", "n", "b", "r", "q", "k"];

        for color in colors {
            for piece in pieces {
                let key = format!("{}{}", color, piece);
                let path = format!("res/pieces/{}/{}{}.png", PIECE_STYLES[self.piece_style_idx], color, piece);
                let texture = load_texture(&path).await.unwrap();
                textures.insert(key, texture);
            }
        }

        self.textures = textures;
    }


    fn draw_eval(&self, eval: i16) {
        let txt = format!("Eval: {}", eval);
        let font_size = 25;
        // top-left corner, adjust as you like
        draw_text_ex(&txt,
                  self.x_offset/4.0,
                  self.y_offset + BOARD_PIXELS/2.5,
                     TextParams {
                         font: Some(&self.font),
                         font_size,
                         color: YELLOW,
                         ..Default::default()
                     }
        );
    }
    fn draw_depth(&self, depth: u8) {
        let txt = format!("Depth Searched: {}", depth);
        let font_size = 25;
        draw_text_ex(&txt,
                     self.x_offset/4.0,
                     self.y_offset + BOARD_PIXELS/2.5,
                     TextParams {
                         font: Some(&self.font),
                         font_size,
                         color: DEPTH_TEXT_COLOR,
                         ..Default::default()
                     }
        );
    }

    fn draw_board(&self, highlights: &[u8], selected_square: u8, last_move_squares: [u8; 2]) {
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

                let is_highlighted = highlights.contains(&(square_index));
                let is_last_move = last_move_squares.contains(&square_index);
                let color = if (row + col) % 2 == 0 { WHITE_SQUARE_COLOR } else { BLACK_SQUARE_COLOR };
                let color = if is_last_move { LAST_MOVE_COLOR } else { color };
                let color = if is_highlighted { SELECTED_MOVE_COLOR } else { color };
                let color = if square_index == selected_square { SELECTED_PIECE_SQUARE_COLOR} else {color};

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
        let font_size = 25;
        // Draw rank and file labels
        let x = self.x_offset - width - margin;
        for i in 1..9 {
            let y = self.y_offset + (i-1) as f32 * 80.0 + 40.0;

            let i = if self.flipped { i } else { 9-i };
            draw_text_ex(&*i.to_string(), x, y,
                         TextParams {
                             font: Some(&self.font),
                             font_size,
                             color: WHITE,
                             ..Default::default()
                         });
        }

        let y = self.y_offset + BOARD_PIXELS + width + margin;
        for i in 1..9 {
            let x = self.x_offset + (i-1) as f32 * 80.0 + 40.0;

            let i = if self.flipped { 9-i } else { i };

            let file_char = (b'a' + (i-1)) as char;
            draw_text_ex(&*file_char.to_string(), x, y, TextParams {
                font: Some(&self.font),
                font_size,
                color: WHITE,
                ..Default::default()
            });
        }
    }



    fn draw_combatants_names(&self) {
        let font_size = 30;

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

        draw_centered_text(black_name, center_x, y_black, &self.font, font_size, black_color);
        draw_centered_text(white_name, center_x, y_white, &self.font, font_size, white_color);
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

        // Restart button with a different skin
        root_ui().push_skin(&self.flip_piece_style_skin);
        let restart_button = widgets::Button::new("")
            .position(vec2(self.x_offset * 1.65 + BOARD_PIXELS, self.y_offset + BOARD_PIXELS + 20.0))
            .size(vec2(50.0, 50.0))
            .ui(&mut root_ui());
        root_ui().pop_skin();

        if restart_button {
            events.push(UiEvent::FlipPieceStyle);
        }

        events
    }

    pub async fn flip_piece_style(&mut self) {
        self.piece_style_idx += 1;
        if self.piece_style_idx >= PIECE_STYLES.len() {
            self.piece_style_idx = 0;
        };
        self.update_piece_textures().await;
    }


    fn update_square(&self, position: &Position, square: u64, color: &str, textures: &HashMap<String, Texture2D>) {
        let pawns = position.pawns(White) | position.pawns(Black);
        let knights = position.knights(White) | position.knights(Black);
        let bishops = position.bishops(White) | position.bishops(Black);
        let rooks = position.rooks(White) | position.rooks(Black);
        let queens = position.queens(White) | position.queens(Black);
        let kings = position.kings(White) | position.kings(Black);

        let piece_name: &str;
        if square & pawns != 0 {
            piece_name = "p";
        } else if square & knights != 0 {
            piece_name = "n";
        } else if square & bishops != 0 {
            piece_name = "b";
        } else if square & rooks != 0 {
            piece_name = "r";
        } else if square & queens != 0 {
            piece_name = "q";
        } else if square & kings != 0 {
            piece_name = "k";
        } else {
            panic!("Invalid piece!")
        }

        let key = format!("{}{}", color, piece_name);
        let texture = textures.get(&key).unwrap();

        let col = square.trailing_zeros() % 8;
        let row = square.trailing_zeros() / 8;
        let tile_size = 80.0;
        let (file, rank_times8) = self.calc_row_col(col as i32, row as i32);
        let x = file as f32 * tile_size;
        let y = (rank_times8 as f32 / 8.0) * tile_size;   // undo the “×8” so 0-7 → 0-560


        draw_texture_ex(&*texture, x + self.x_offset, y + self.y_offset, WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(80.0, 80.0)), // target size
            ..Default::default()
        }, );
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

fn draw_centered_text(text: &str, center_x: f32, y: f32, font: &Font, font_size: u16, color: macroquad::color::Color) {
    // scale = 1.0 if you’re not scaling the font
    let dims = measure_text(text, None, font_size, 1.0);
    let x = center_x - dims.width * 0.5;
    draw_text_ex(text, x, y, TextParams {
        font: Some(&font),
        font_size,
        color,
        ..Default::default()
    });


}


fn ease_smootherstep(t: f32) -> f32 {
    // 6t^5 - 15t^4 + 10t^3
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}



