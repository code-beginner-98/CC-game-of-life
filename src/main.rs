// define starting state

// take gamestate, calc new gamestate, output new gamestate
// while Result<GameState> != Err<> {
//     GameState.update()
// }
mod game_state;
mod generators;
use std::{io, thread::sleep, time::Duration};
use game_state::GameState;
use generators::{generate_empty, generate_glider, generate_random};

fn main() -> io::Result<()>{
    print!("\x1B[2J\x1B[1;1H"); // clear screen once
    print!("Welcome to the Game of Life, published by John Horton Conway in 1970.");
    println!("What Starting Pattern would you like to see?\n
        1: Simple Glider.\n
        2: Random Pattern.");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let buf_str: Vec<char> = buffer.chars().collect();
    let mut game = match buf_str[0] {
        '1' => GameState::from_field(generate_glider()),
        '2' => GameState::from_field(generate_random(10)),
        _ => {
            println!("Initiating Empty field...");
            GameState::from_field(generate_empty(10))
        }
    };
    print!("\x1B[2J\x1B[1;1H"); // clear screen once
    for _ in 0..100 {    
        game.print();
        sleep(Duration::from_millis(400));
        game.update();
    }
    Ok(())
}