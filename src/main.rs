use macroquad::prelude::*;

use chess::game_controller::GameController;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Chess Board".to_string(),
        window_width: 640,
        window_height: 640,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut controller = GameController::new().await;
    controller.run().await;
}
