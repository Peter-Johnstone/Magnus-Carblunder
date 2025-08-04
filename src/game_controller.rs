use std::cmp::PartialEq;
use crate::gui::{GuiState, UiEvent};
use crate::position::Position;
use crate::color::Color;
use macroquad::prelude::{mouse_position, next_frame, is_mouse_button_pressed, MouseButton, is_key_pressed, KeyCode};
use crate::mov::MoveList;
use crate::attacks::movegen::all_moves;
use crate::color::Color::{White};
use crate::engines::engine_manager::{Engine, NUMBER_OF_SEARCH_ALGORITHMS};
use crate::undo::UndoStack;

pub struct GameController {
    white_engine: Engine,
    black_engine: Engine,
    position: Position,
    gui: GuiState,
    selected_moves: MoveList,
    game_mode: GameMode,
    last_depth: u8,
    last_eval: i16,
}



#[derive(PartialEq, Clone)]
pub enum GameMode {
    PlayersOnly,
    EnginesOnly,
    PlayerWhite,
    PlayerBlack,
}

const PLAYER_COLOR: Color = White;
const PLAYERS_ONLY: bool = false;

impl GameController {

    pub async fn new(game_mode: GameMode) -> Self {
        let white_engine = Engine::new(NUMBER_OF_SEARCH_ALGORITHMS, 1, 1000);
        let black_engine = Engine::new(NUMBER_OF_SEARCH_ALGORITHMS, 1, 1000);
        let position = Position::start();
        //let position = Position::load_position_from_fen("8/k7/3p4/p2P1p2/P2P1P2/8/8/K7 w - -");
        let gui = GuiState::new(game_mode == GameMode::PlayerBlack, game_mode.clone(), white_engine.name(), black_engine.name()).await;

        Self { white_engine, black_engine, position, gui, selected_moves: MoveList::new(), game_mode, last_depth: 0, last_eval: 0 }
    }

    pub fn load_fen(&mut self, fen: &str) {
        self.position = Position::load_position_from_fen(fen);
    }

    pub async fn run_review_game(&mut self, undo_stack: &mut UndoStack) {
        loop {
            self.review_game_update(undo_stack);
            self.render();
        }
    }

    fn review_game_update(&mut self, undo_stack: &mut UndoStack) {
        if is_key_pressed(KeyCode::F) {
            if let Some(undo) = undo_stack.pop_front() {
                self.position.do_move(undo.mov);
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) &&( (PLAYER_COLOR == self.position.turn()) || PLAYERS_ONLY){
            let square = self.gui.get_mouse_square(mouse_position());
            match square {
                Some(square) => {
                    if (1u64 << square) & self.position.occupancy(self.position.turn()) != 0 {
                        self.selected_moves = all_moves(&self.position).moves_from_square(square);
                    } else {
                        self.selected_moves = MoveList::new(); // clear
                    }
                }
                None => {}
            }
        }

        if is_key_pressed(KeyCode::U) {
            if !self.position.can_undo() {
                return;
            }
            self.position.undo_move();
        }
    }

    pub fn handle_ui_event(&mut self, event: UiEvent) {
        match event {

            UiEvent::FlipPressed => {
                self.selected_moves = MoveList::new();
                self.gui.flip_board()
            },
            UiEvent::RestartPressed => {
                self.selected_moves = MoveList::new();
                self.last_eval = 0;
                self.last_depth = 0;

                self.position = Position::start();
            }
        }
    }


    pub fn handle_ui_events(&mut self, events: impl IntoIterator<Item = UiEvent>) {
        for e in events {
            self.handle_ui_event(e);
        }
    }

    pub async fn run(&mut self) {
        loop {
            self.update().await;
            self.render();
            next_frame().await;
        }
    }

    fn handle_player_click(&mut self) -> bool {
        let mut move_executed = false;
        if let Some(square) = self.gui.get_mouse_square(mouse_position()) {
            if (1u64 << square) & self.position.occupancy(self.position.turn()) != 0 {
                self.selected_moves = all_moves(&self.position).moves_from_square(square);
            } else {
                if let Some(mov) = self.selected_moves.iter().find(|m| m.to() == square) {
                    self.position.do_move(mov);
                    move_executed = true;
                }
                self.selected_moves = MoveList::new();
            }
        }
        move_executed
    }



    #[inline(always)]
    async fn white_engine_move(&mut self) {
        self.render();
        next_frame().await;

        let (mov, depth, eval) = self.white_engine.pick_and_stats(&mut self.position);
        self.last_depth = depth;
        self.last_eval = eval;

        self.position.do_move(mov);
    }

    #[inline(always)]
    async fn black_engine_move(&mut self) {
        self.render();
        next_frame().await;

        let (mov, depth, eval) = self.black_engine.pick_and_stats(&mut self.position);
        self.last_depth = depth;

        self.last_eval = eval;
        self.position.do_move(mov);
    }
    async fn update(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let turn_is_white = self.position.turn().is_white();
            match self.game_mode {
                GameMode::PlayersOnly => {
                    self.handle_player_click();
                }
                GameMode::EnginesOnly => {
                    if turn_is_white {
                        self.white_engine_move().await;
                    } else {
                        self.black_engine_move().await;
                    }
                }
                GameMode::PlayerWhite => {
                    if self.position.turn().is_white() {
                        let next_turn = self.handle_player_click();
                        if next_turn {
                            self.black_engine_move().await;
                        }
                    } else {
                        self.black_engine_move().await;
                    }
                }
                GameMode::PlayerBlack => {
                    if self.position.turn().is_black() {
                        let next_turn = self.handle_player_click();
                        if next_turn {
                            self.white_engine_move().await;
                        }
                    } else {
                        self.white_engine_move().await;
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::R) {
            println!("is repeat? {}", self.position.is_repeat_towards_three_fold_repetition());
        }

        if is_key_pressed(KeyCode::H) {
            self.position.print_move_history();
        }
    }

    fn render(&mut self) {
        self.gui.draw_position(&self.position, &self.selected_moves, self.last_depth, self.last_eval);
        let events = self.gui.draw_buttons();

        self.handle_ui_events(events);
    }
}
