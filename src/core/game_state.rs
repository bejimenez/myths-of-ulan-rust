use crate::entities::Player;
use crate::world::{Dungeon, Direction};
use std::io::{self, Write};

pub struct GameState {
    pub player: Option<Player>,
    pub dungeon: Option<Dungeon>,
    pub running: bool,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            player: None,
            dungeon: None,
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

        // Create the test dungeon
        self.dungeon = Some(Dungeon::new_test_dungeon());
        println!("\nYou descend into the dungeon...\n");
    }

    pub fn process_command(&mut self, command: &str) {
        let parts: Vec<&str> = command.trim().split_whitespace().collect();
        if parts.is_empty() {
            return;
        }

        let cmd = parts[0].to_lowercase();

        match cmd.as_str() {
            "help" | "h" => self.show_help(),
            "status" | "c" => self.show_status(),
            "look" | "l" => self.look(),
            "north" | "n" => self.move_direction(Direction::North),
            "south" | "s" => self.move_direction(Direction::South),
            "east" | "e" => self.move_direction(Direction::East),
            "west" | "w" => self.move_direction(Direction::West),
            "go" => {
                if parts.len() > 1 {
                    if let Some(dir) = Direction::from_string(parts[1]) {
                        self.go_direction(dir);
                    } else {
                        println!("Invalid direction. Use: north, south, east, or west");
                    }
                } else {
                    println!("Go where? Specify a direction: north, south, east, or west");
                }
            }
            "quit" | "q" | "exit" => {
                println!("Thanks for playing!");
                self.running = false;
            }
            _ => println!("Unknown command: '{}'. Type 'help' for available commands.", cmd),
        }
    }
    fn show_help(&self) {
        println!("\n=== Available Commands ===");
        println!("help (h)  - Show this help message");
        println!("status (c)    - Show your character's status");
        println!("look (l)  - Look around the room");
        println!("north (n) - Move north");
        println!("south (s) - Move south");
        println!("east (e)  - Move east");
        println!("west (w)  - Move west");
        println!("go <dir>  - Travel to another room ('go north')");
        println!("quit (q)  - Exit the game");
    }

    fn show_status(&self) {
        if let Some(player) = &self.player {
            player.display_status();
        } else {
            println!("No player created yet!");
        }
    }
    pub fn look(&self) {
        if let Some(dungeon) = &self.dungeon {
            dungeon.display_current_room();
        }
    }

    fn move_direction(&mut self, direction: Direction) {
        if let Some(dungeon) = &mut self.dungeon {
            let (dx, dy) = match direction {
                Direction::North => (0, -1),
                Direction::South => (0, 1),
                Direction::East => (1, 0),
                Direction::West => (-1, 0),
            };

            match dungeon.move_player(dx, dy) {
                Ok(msg) => {
                    println!("{}", msg);
                    dungeon.display_current_room();
                }
                Err(msg) => println!("{}", msg),
            }
        }
    }
    
    fn go_direction(&mut self, direction: Direction) {
        if let Some(dungeon) = &mut self.dungeon {
            match dungeon.change_room(direction) {
                Ok(msg) => {
                    println!("{}", msg);
                    dungeon.display_current_room();
                }
                Err(msg) => println!("{}", msg),
            }
        }
    }
}
