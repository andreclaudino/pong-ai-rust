use tetra::ContextBuilder;

use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::game_state::GameState;

pub fn start_pong() -> tetra::Result {
  ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()
        .unwrap()
        .run(GameState::new)
}