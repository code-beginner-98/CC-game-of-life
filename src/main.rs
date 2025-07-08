// define starting state

// take gamestate, calc new gamestate, output new gamestate
// while Result<GameState> != Err<> {
//     GameState.update()
// }
mod game_state;
mod generators;
mod app;
use egui::{Vec2, ViewportBuilder};
use game_state::GameState;
use generators::generate_random;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> eframe::Result<()> {

    let game = Arc::new(Mutex::new(GameState::from_field(generate_random(200))));
    let shared_speed = Arc::new(Mutex::new(2));
    let shared_paused = Arc::new(Mutex::new(false));

    let app = app::App {
        shared_state: Arc::clone(&game), // size of field vector, not visual field
        shared_paused: Arc::clone(&shared_paused),
        pan_offset: Vec2 {x: 0f32, y:0f32,},
        zoom: 1.0,
        shared_speed: Arc::clone(&shared_speed),
        window_height: None
    };

    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };

    // Game State Update Thread
    let _computation_handle = tokio::task::spawn(async move {
        // println!("running comp.");
        let mut speed: u8 = 2;

        loop {
            let start = Instant::now();

            if let Ok(speed_guard) = Arc::clone(&shared_speed).lock() {
                speed = *speed_guard;
            }
            let refresh_rate = Duration::from_millis(match speed {
                1 => 500,
                2 => 250,
                3 => 100, 
                4 => 40,
                _ => 10,    
            });
            let passed = Instant::now() - start;
            if passed < refresh_rate {
                tokio::time::sleep(refresh_rate - passed).await;
            }
            // println!("updating game state.");
            let mut paused = false;
            if let Ok(paused_guard) = Arc::clone(&shared_paused).lock() {
                paused = *paused_guard;
            }
            if !paused {
                Arc::clone(&game).lock().unwrap().update();
            }
            
        }
    });

    eframe::run_native("Game of Life", native_options, Box::new(|_cc| Ok(Box::new(app)))).unwrap();
    
    Ok(())
}