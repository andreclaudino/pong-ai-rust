mod game_entry;
use game_entry::start_pong;

mod constants;
mod entity;

mod game_state;

fn main() -> tetra::Result {
    start_pong()
}
