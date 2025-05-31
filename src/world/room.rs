use super::tile::TileType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub description: String,
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<TileType>>,
    pub connections: std::collections::HashMap<Direction, String> // Direction -> Room ID
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Direction::North => "north",
            Direction::South => "south",
            Direction::East => "east",
            Direction::West => "West",
        }
    }
    pub fn from_string(s: &str) -> Option<Direction> {
        match s.to_lowercase().as_str() {
            "north" | "n" => Some(Direction::North),
            "south" | "s" => Some(Direction::South),
            "east" | "e" => Some(Direction::East),
            "west" | "w" => Some(Direction::West),
            _ => None,
        }
    }
}

impl Room {
    pub fn new(id: String, name: String, description: String, width: usize, height: usize) -> Self {
        // Create a simple room with walls and floor
        let mut tiles = vec![vec![TileType::Wall; width]; height];

        // Fill interior with floor
        for y in 1..height-1 {
            for x in 1..width-1 {
                tiles[y][x] = TileType::Floor;
            }
        }

        Room {
            id,
            name,
            description,
            width,
            height,
            tiles,
            connections: std::collections::HashMap::new(),
        }
    }

    pub fn add_door(&mut self, _direction: Direction, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.tiles[y][x] = TileType::Door;
        }
    }

    pub fn display(&self, player_x: usize, player_y: usize) {
        println!("\n=== {} ===", self.name);
        println!("{}", self.description);
        println!();

        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if x == player_x && y == player_y {
                    print!("@");
                } else {
                    print!("{}", tile.to_char());
                }
            }
            println!();
        }

        if !self.connections.is_empty() {
            print!("\nExits: ");
            let exits: Vec<String> = self.connections.keys()
                .map(|dir| dir.to_string().to_string())
                .collect();
            println!("{}", exits.join(", "));
        }
    }
}
