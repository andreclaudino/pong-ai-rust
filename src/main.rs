use tetra::ContextBuilder;

mod game_state;
use game_state::GameState;

mod constants;
use constants::{WINDOW_WIDTH, WINDOW_HEIGHT};

mod entity;


fn main() -> tetra::Result {
    ContextBuilder::new("Pong",WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
