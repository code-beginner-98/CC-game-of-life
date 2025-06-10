// define starting state

// take gamestate, calc new gamestate, output new gamestate
// while Result<GameState> != Err<> {
//     GameState.update()
// }
mod game_state;
mod generators;
mod app;
use egui::Vec2;
use game_state::GameState;
use generators::generate_random;

fn main() -> eframe::Result<()> {
    let app = app::App {
        state: GameState::from_field(generate_random(200)), // size of field vector, not visual field
        paused: false,
        pan_offset: Vec2 {x: 0f32, y:0f32,},
        zoom: 1.0,
    };

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Game of Life", native_options, Box::new(|_cc| Ok(Box::new(app))))?;
    Ok(())
}