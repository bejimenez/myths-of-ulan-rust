use super::room::{Room, Direction};
use super::tile::TileType;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dungeon {
    pub rooms: HashMap<String, Room>,
    pub current_room_id: String,
    pub player_x: usize,
    pub player_y: usize,
}

impl Dungeon {
    pub fn new_test_dungeon() -> Self {
        let mut rooms = HashMap::new();

        // Create entrance room
        let must entrance = Room::new(
            "entrance".to_string(),
            "Dungeon Entrance".to_string(),
            "A dimly lit stone chamber. The air is damp and cold.".to_string(),
            10,
            8
        );
        entrance.add_door(Direction::North, 5, 0);
        entrance.connections.insert(Direction::North, "hallway".to_string());

        // Create hallway
        let mut hallway = Room::new(
            "hallway".to_string(),
            "Stone Hallway".to_string(),
            "A narror corridor stretches before you. Torches flicker on the walls.".to_string(),
            12,
            6
        );
        hallway.add_door(Direction::South, 6, 5);
        hallway.add_door(Direction::East, 11, 3);
        hallway.connections.insert(Direction::South, "entrance".to_string());
        hallway.connections.insert(Direction::East, "chamber".to_string());

        // create chamber
        let mute chamber = Room::new(
            "chamber".to_string(),
            "Ancient Chamber".to_string(),
            "The walls of this room are covered in mysterious symbols.".to_string(),
            15,
            10
        );
        chamber.add_door(Direction::West, 0, 5);
        chamber.connections(Direction::West, "hallway".to_string());

        // Add stairs to chamber
        chamber.tiles[5][7] = TileType::Stairs;

        rooms.insert("entrance".to_string(), entrance);
        rooms.insert("hallway".to_string(), hallway);
        rooms.insert("chamber".to_string(), chamber);

        Dungeon {
            rooms,
            current_room_id: "entrance".to_string(),
            player_x: 5,
            player_y: 4,
        }
    }
    
    pub fn get_current_room(&self) -> Option<&Room> {
        self.rooms.get(&self.current_room_id)
    }

    pub fn get_current_room_mut(&mut self) -> Option<&mut Room> {
        self.rooms.get_mut(&self.current_room_id)
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) -> Result<String, String> {
        let new_x = (self.player_x as i32 + dx) as usize;
        let new_y = (self.player_y as i32 + dy) as usize;

        if let Some(room) = self.get_current_room() {
            // Check bounds
            if new_x >= room.width || new_y >= room.height {
                return Err("You can't move there!".to_string());
            }

            // Check if tile is walkable
            let tile = room.tiles[new_y][new_x];
            if !tile.is_walkable() {
                return Err("You bump into a wall!".to_string());
            }

            // Update Position
            self.player_x = new_x;
            self.player_y = new_y;

            // check for special tiles
            match tile {
                TileType::Stairs => Ok("You see stairs leading down...".to_string()),
                TileType::Door => Ok("You stand at a doorway.".to_string()),
                _ => Ok("You move.".to_string()),
            }
        } else {
            Err("Error: Current room not found!".to_string())
        }
    }

    pub fn change_room(&mut self, direction: Direction) -> Result<String, String> {
        if let Some(room) = self.get_current_room() {
            if let Some(new_room_id) = room.connections.get(&direction) {
                self.current_room_id = new_room_id.clone();

                // Place player at appropriate entrance
                match direction {
                    Direction::North => {
                        self.player_y = self.get_current_room().unwrap().height - 2;
                        self.player_x = self.get_current_room().unwrap().width / 2;
                    }
                    Direction::South => {
                        self.player_y = 1;
                        self.player_x = self.get_current_room().unwrap().width / 2;
                    }
                    Direction::East => {
                        self.player_x = 1;
                        self.player_y = self.get_current_room().unwrap().height / 2;
                    }
                    Direction::West => {
                        self.player_x = self.get_current_room().unwrap().width - 2;
                        self.player_y = self.get_current_room().unwrap().height / 2;
                    }
                }

                Ok(format!("You move {} into a new area.", direction.to_string()))
            } else {
                Err("There's no exit in that direction!".to_string())
            }
        } else {
            Err("Error: Current room not found!".to_string())
        }
    }
    
    pub fn display_current_room(&self) {
        if let Some(room) = self.get_current_room() {
            room.display(self.player_x, self.player_y);
        }
    }
}
