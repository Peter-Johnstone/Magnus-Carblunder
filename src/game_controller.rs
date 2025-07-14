use crate::gui::{get_mouse_square, GuiState};
use crate::position::Position;
use macroquad::prelude::*;
use crate::mov::MoveList;
use crate::attacks::movegen::all_moves;

pub struct GameController {
    position: Position,
    gui: GuiState,
    selected_moves: MoveList,

    // Add more state here if needed (e.g. turn, input state, timers)
}

impl GameController {
    pub async fn new() -> Self {
        let position = Position::start();
        let gui = GuiState::new().await;
        Self { position, gui, selected_moves: MoveList::new() }
    }

    pub async fn run(&mut self) {
        loop {
            self.update().await;
            self.render();
            next_frame().await;
        }
    }

    async fn update(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let square: u8 = get_mouse_square(mouse_position());
            if (1u64 << square) & self.position.occupancy(self.position.turn()) != 0 {
                self.selected_moves = all_moves(&self.position).moves_from_square(square);
            } else {
                if let Some(&mov) = self.selected_moves.iter().find(|m| m.to() == square) {
                    self.position.do_move(mov);
                }
                self.selected_moves = MoveList::new(); // clear
            }
        }

        // handle inputs, update position, etc.
    }

    fn render(&self) {
        self.gui.draw_position(&self.position, &self.selected_moves);
    }
}
