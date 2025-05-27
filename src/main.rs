// define starting state

// take gamestate, calc new gamestate, output new gamestate
// while Result<GameState> != Err<> {
//     GameState.update()
// }
mod game_state;
use game_state::GameState;

fn main() {
    let mut game = GameState::init();
    for _ in 0..10 {
        game.print();
        game.update();
    }
}