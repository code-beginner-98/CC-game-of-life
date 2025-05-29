// define starting state

// take gamestate, calc new gamestate, output new gamestate
// while Result<GameState> != Err<> {
//     GameState.update()
// }
mod game_state;
use std::{thread::sleep, time::Duration};

use game_state::GameState;

fn main() {
    print!("\x1B[2J\x1B[1;1H"); // clear screen once
    let mut game = GameState::init_rnd();
    for _ in 0..100 {    
        game.print();
        sleep(Duration::from_millis(400));
        game.update();
    }
}