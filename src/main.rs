// define starting state

// take gamestate, calc new gamestate, output new gamestate
// while Result<GameState> != Err<> {
//     GameState.update()
// }
mod game_state;
mod generators;
mod app;
use game_state::GameState;
use generators::generate_random;

fn main() -> eframe::Result<()> {
    let app = app::App {
        state: GameState::from_field(generate_random(200)), // size of field vector, not visual field
        paused:false,
    };

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Game of Life", native_options, Box::new(|_cc| Ok(Box::new(app))))?;
    Ok(())
}