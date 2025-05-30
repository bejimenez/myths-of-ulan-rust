use std::io::{self, Write};
use myths_of_ulan_rust::core::GameState;

fn main() {
    let mut game = GameState::new();

    // Create Player
    game.create_player();

    // Show initial room
    game.look();

    println!("\n Type 'help' for available commands.");

    // Main game loop   
    while game.running {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                game.process_command(&input);
            }
            Err(error) => {
                println!("Error reading input: {}", error);
            }
        }
    }
}
