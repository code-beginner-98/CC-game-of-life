// define starting state

// take gamestate, calc new gamestate, output new gamestate
// while Result<GameState> != Err<> {
//     GameState.update()
// }
mod game_state;
use std::{thread::sleep, time::Duration};

use game_state::GameState;

fn main() {
    let mut game = GameState::init_glider();
    for _ in 0..25 {
        game.print();
        sleep(Duration::from_millis(200));
        game.update();
    }
}