use crate::entities::Player;
use std::io::{self, Write};

pub struct GameState {
    pub player: Option<Player>,
    pub running: bool,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            player: None,
            running: true,
        }
    }

    pub fn create_player(&mut self) {
        println!("Myths of Ulan - Realm of 1000 Gods");
        print!("Enter your character's name: ");
        io::stdout().flush().unwrap();

        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read name");
        let name = name.trim().to_string();

        if name.is_empty() {
            println!("Invalid name! Using default name 'Adventurer'");
            self.player = Some(Player::new("Adventurer".to_string()));
        } else {
            self.player = Some(Player::new(name.clone()));
            println!("\nWelcome, {}!", name);
        }
    }

    pub fn process_command(&mut self, command: &str) {
        let command = command.trim().to_lowercase();

        match command.as_str() {
            "help" | "h" => self.show_help(),
            "status" | "s" => self.show_status(),
            "quit" | "q" | "exit" => {
                println!("Thanks for playing!");
                self.running = false;
            }
            _ => println!("Unknown command: '{}'. Type 'help' for available commands", command),
        }
    }
    fn show_help(&self) {
        println!("\n=== Available Commands ===");
        println!("help (h)  - Show this help message");
        println!("status (s)    - Show your character's status");
        println!("quit (q)  - Exit the game");
    }

    fn show_status(&self) {
        if let Some(player) = &self.player {
            player.display_status();
        } else {
            println!("No player created yet!");
        }
    }
}
