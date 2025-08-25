use crate::attacks::movegen::all_moves;
use crate::color::Color;
use crate::color::Color::White;
use crate::engines::engine_manager::{Engine, NUMBER_OF_EVAL_ALGORITHMS};
use crate::gui::{GuiState, UiEvent};
use crate::mov::{Move, MoveList};
use crate::position::{Position, Status, NO_SQ};
use crate::undo::UndoStack;
use macroquad::prelude::{get_time, is_key_pressed, is_mouse_button_pressed, mouse_position, next_frame, KeyCode, MouseButton};
use std::cmp::PartialEq;
use std::sync::mpsc::{Receiver, Sender};
use crate::piece::{ColoredPiece};

struct MoveAnimation {
    from: u8,
    to: u8,
    start_time: f64, // seconds since app start
    duration: f32,   // seconds
}

impl MoveAnimation {
    fn t(&self) -> f32 {
        let now = get_time() as f32;
        ((now - self.start_time as f32) / self.duration).clamp(0.0, 1.0)
    }
    fn done(&self) -> bool { self.t() >= 1.0 }}


enum EvalRequest {
    NewPosition(Position),
    Quit,
}

#[derive(Clone, Copy)]
struct EvalUpdate {
    eval: i16,
    depth: u8,
    best: Move,
    nodes: u64,
}


pub struct GameController {
    white_engine: Engine,
    black_engine: Engine,
    position: Position,
    gui: GuiState,
    selected_moves: MoveList,
    selected_square: u8,
    game_mode: GameMode,
    game_status: Status,    // pos doesn't keep track of game status because it would be too much work during search.

    last_depth: u8,
    last_eval: i16,
    last_move: Move,

    eval_tx: Sender<EvalRequest>,
    eval_rx: Receiver<EvalUpdate>,
    live_eval: i16,
    live_eval_depth: u8,

    move_anim: Option<MoveAnimation>,
    anim_ms: f32,
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
        let white_engine = Engine::new(26,  NUMBER_OF_EVAL_ALGORITHMS, 1000);
        let black_engine = Engine::new(26, NUMBER_OF_EVAL_ALGORITHMS, 1000);
        let position = Position::start();
        //let position = Position::load_position_from_fen("1k6/8/8/8/8/2K5/5q3/8 b - - 0 0");
        let gui = GuiState::new(game_mode == GameMode::PlayerBlack, game_mode.clone(), white_engine.name(), black_engine.name()).await;
        let (eval_tx, eval_rx) = spawn_eval_worker();

        let me = Self {
            white_engine, black_engine, position, gui,
            selected_moves: MoveList::new(),
            selected_square: NO_SQ,
            game_mode,
            game_status: Status::Ongoing,
            last_depth: 0, last_eval: 0, last_move: Move::null(),
            eval_tx, eval_rx, live_eval: 0, live_eval_depth: 0,
            move_anim: None, anim_ms: 400.0,
        };

        // kick the worker with the initial position
        let _ = me.eval_tx.send(EvalRequest::NewPosition(me.position.clone()));
        me
    }

    pub fn load_fen(&mut self, fen: &str) {
        self.position = Position::load_position_from_fen(fen);
    }

    pub async fn run_review_game(&mut self, undo_stack: &mut UndoStack) {
        loop {
            self.review_game_update(undo_stack);
            self.render().await;
        }
    }


    fn review_game_update(&mut self, undo_stack: &mut UndoStack) {
        if is_key_pressed(KeyCode::F) {
            if let Some(undo) = undo_stack.pop_front() {
                self.position.do_move(undo.mov);
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) &&( (PLAYER_COLOR == self.position.side_to_move()) || PLAYERS_ONLY){
            let square = self.gui.get_mouse_square(mouse_position());
            match square {
                Some(square) => {
                    if (1u64 << square) & self.position.occupancy(self.position.side_to_move()) != 0 {
                        self.selected_moves = all_moves(&self.position).moves_from_square(square);
                        self.selected_square = square;
                    } else {
                        self.selected_moves = MoveList::new(); // clear
                        self.selected_square = NO_SQ;
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

    pub async fn handle_ui_event(&mut self, event: UiEvent) {
        match event {
            UiEvent::FlipPressed => {
                self.selected_moves = MoveList::new();
                self.selected_square = NO_SQ;

                self.gui.flip_board();
            },
            UiEvent::RestartPressed => {
                self.restart_game();
            }

            UiEvent::FlipPieceStyle =>  {
                self.gui.flip_piece_style().await;
            }
        }
    }


    pub fn restart_game(&mut self) {
        self.push_eval_position();
        self.selected_moves = MoveList::new();
        self.selected_square = NO_SQ;
        self.last_eval = 0;
        self.last_depth = 0;
        self.last_move = Move::null();
        self.position = Position::start();
        self.live_eval = 0;
        self.live_eval_depth = 0;
        self.push_eval_position();
    }

    pub async fn handle_ui_events(&mut self, events: impl IntoIterator<Item = UiEvent>) {
        for e in events {
            self.handle_ui_event(e).await;
        }
    }

    pub async fn run(&mut self) {
        loop {
            self.update().await;
            self.render().await;
            next_frame().await;
        }
    }

    fn handle_player_click(&mut self) -> bool {
        let mut move_executed = false;
        if let Some(square) = self.gui.get_mouse_square(mouse_position()) {
            if (1u64 << square) & self.position.occupancy(self.position.side_to_move()) != 0 {
                self.selected_moves = all_moves(&self.position).moves_from_square(square);
                self.selected_square = square;
            } else {
                if let Some(mov) = self.selected_moves.iter().find(|m| m.to() == square) {
                    self.position.do_move(mov);
                    self.push_eval_position();
                    mov.play_move_sound(self.position.in_check());
                    self.last_move = mov;
                    move_executed = true;
                    self.game_status = self.position.game_status();
                }
                self.selected_moves = MoveList::new();
                self.selected_square = NO_SQ;

            }
        }
        move_executed
    }

    fn push_eval_position(&self) {
        let _ = self.eval_tx.send(EvalRequest::NewPosition(self.position.clone()));
    }


    fn start_animation(&mut self, mov: Move) {
        self.move_anim = Some(MoveAnimation {
            from: mov.from(),
            to: mov.to(),
            start_time: get_time(),
            duration: (self.anim_ms / 1000.0).max(0.0001),
        });
    }


    #[inline(always)]
    async fn white_engine_move(&mut self) {
        self.render().await;
        next_frame().await;

        let (mov, depth, eval) = self.white_engine.pick_and_stats(&mut self.position);

        self.position.do_move(mov);
        self.push_eval_position();

        // mov.play_move_sound(self.position.in_check());

        self.last_depth = depth;
        self.last_eval = eval;
        self.last_move = mov;

        self.start_animation(mov);
        self.game_status = self.position.game_status();
    }

    #[inline(always)]
    async fn black_engine_move(&mut self) {
        self.render().await;
        next_frame().await;

        let (mov, depth, eval) = self.black_engine.pick_and_stats(&mut self.position);

        self.position.do_move(mov);
        self.push_eval_position();

        // mov.play_move_sound(self.position.in_check());

        self.last_depth = depth;
        self.last_eval = eval;
        self.last_move = mov;

        self.start_animation(mov);
        self.game_status = self.position.game_status();
    }

    async fn update(&mut self) {
        while let Ok(upd) = self.eval_rx.try_recv() {
            self.live_eval = upd.eval;
            self.live_eval_depth = upd.depth;
        }
        if let Some(anim) = &self.move_anim {
            if anim.done() {
                self.last_move.play_move_sound(self.position.in_check());
                self.move_anim = None;
            }
            return; // render the in-flight frame; no inputs while animating
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let turn_is_white = self.position.side_to_move().is_white();
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
                    if self.position.side_to_move().is_white() {
                        let next_turn = self.handle_player_click();
                        if next_turn {
                            self.black_engine_move().await;
                        }
                    } else {
                        self.black_engine_move().await;
                    }
                }
                GameMode::PlayerBlack => {
                    if self.position.side_to_move().is_black() {
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

        if is_key_pressed(KeyCode::H) {
            self.position.print_move_history();
        }
    }


    async fn render(&mut self) {
        let anim_for_gui = self.move_anim.as_ref().map(|a| {
            crate::gui::AnimRender { from: a.from, to: a.to, t: a.t() }
        });


        self.gui.draw_position_animated(
            &self.position,
            &self.selected_moves,
            self.selected_square,
            self.last_move,
            self.game_status,
            &self.position.captured_pieces(),
            anim_for_gui,
        );

        self.gui.draw_eval_bar(self.live_eval);

        let events = self.gui.draw_buttons();
        self.handle_ui_events(events).await;
    }
}

fn spawn_eval_worker() -> (Sender<EvalRequest>, Receiver<EvalUpdate>) {
    use std::sync::mpsc::{self};
    use std::thread;

    let (tx_req, rx_req) = mpsc::channel::<EvalRequest>();
    let (tx_upd, rx_upd) = mpsc::channel::<EvalUpdate>();

    thread::spawn(move || {
        const BASE_MS: u64 = 200;   // responsive when positions change
        const MAX_MS:  u64 = 3000;  // cap so it still reacts within ~3s
        let mut slice_ms = BASE_MS;

        // strongest search, best eval, initial slice
        let mut engine = Engine::new(25, 2, slice_ms);

        let mut current: Option<Position> = None;
        let mut last_depth: u8 = 0;
        let mut stale: u32 = 0;

        loop {
            if current.is_none() {
                match rx_req.recv() {
                    Ok(EvalRequest::NewPosition(p)) => {
                        current = Some(p);
                        last_depth = 0;
                        slice_ms = BASE_MS;
                        stale = 0;
                        engine.set_time_limit(slice_ms);
                    }
                    Ok(EvalRequest::Quit) | Err(_) => break,
                }
            }

            // run one slice
            engine.set_time_limit(slice_ms);
            let pos_ref = current.as_ref().unwrap();
            let stm_is_white = pos_ref.side_to_move().is_white();

            let mut pos = pos_ref.clone();
            let (best, depth, eval_stm) = engine.pick_and_stats(&mut pos);

            // flip to White perspective for the UI
            let eval_white = if stm_is_white { eval_stm } else { -eval_stm };

            let _ = tx_upd.send(EvalUpdate { eval: eval_white, depth, best, nodes: engine.total_nodes() });

            // adapt the slice length
            if depth > last_depth {
                last_depth = depth;
                stale = 0;
                // decay toward base to stay snappy
                if slice_ms > BASE_MS {
                    slice_ms = (slice_ms as f32 * 0.6).max(BASE_MS as f32) as u64;
                }
            } else {
                stale += 1;
                if stale >= 3 && slice_ms < MAX_MS {
                    slice_ms = (slice_ms * 2).min(MAX_MS);
                    stale = 0;
                }
            }

            // drain any new positions, keep only the latest
            let mut latest: Option<Position> = None;
            while let Ok(msg) = rx_req.try_recv() {
                match msg {
                    EvalRequest::NewPosition(p) => latest = Some(p),
                    EvalRequest::Quit => return,
                }
            }
            if let Some(p) = latest {
                current = Some(p);
                last_depth = 0;
                slice_ms = BASE_MS;
                stale = 0;
                engine.set_time_limit(slice_ms);
            }
        }
    });

    (tx_req, rx_upd)
}
