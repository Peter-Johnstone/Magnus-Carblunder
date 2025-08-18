use macroquad::prelude::Conf;

use chess::game_controller::{GameController, GameMode};
use chess::mov::init_sounds;

fn window_conf() -> Conf {
    Conf {
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    //battle_against_other_eval_algos(3, 3).await;
    //battle_against_other_search_algos(4, 3, 5, 100);
    init_sounds().await;
    let mut controller = GameController::new(GameMode::PlayerWhite).await;
    controller.run().await;
}
