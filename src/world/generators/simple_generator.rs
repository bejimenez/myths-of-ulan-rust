use super::generator_traits::{DungeonGenerator, GeneratorConfig};
use crate::world::{Dungeon, Room, Direction, TileType};
use rand::prelude::*;
use std::collections::HashMap;

pub struct SimpleGenerator;

impl SimpleGenerator {
    pub fn new() -> Self {
        SimpleGenerator
    }
    
    /// Generate a room at the specified position in the dungeon grid
    fn create_room_at(&self, 
        id: String, 
        name: String,
        x: usize, 
        y: usize, 
        width: usize, 
        height: usize,
        dungeon_grid: &mut Vec<Vec<bool>>
    ) -> Option<RoomData> {
        // Check if room fits and doesn't overlap
        if x + width >= dungeon_grid[0].len() || y + height >= dungeon_grid.len() {
            return None;
        }
        
        // Check for overlaps (with 1 tile padding)
        for dy in 0..height + 2 {
            for dx in 0..width + 2 {
                let check_y = y as i32 + dy as i32 - 1;
                let check_x = x as i32 + dx as i32 - 1;
                
                if check_y >= 0 && check_y < dungeon_grid.len() as i32 &&
                   check_x >= 0 && check_x < dungeon_grid[0].len() as i32 {
                    if dungeon_grid[check_y as usize][check_x as usize] {
                        return None;
                    }
                }
            }
        }
        
        // Mark area as occupied
        for dy in 0..height {
            for dx in 0..width {
                dungeon_grid[y + dy][x + dx] = true;
            }
        }
        
        Some(RoomData {
            id,
            name,
            x,
            y,
            width,
            height,
            center_x: x + width / 2,
            center_y: y + height / 2,
        })
    }
    
    /// Connect two rooms with a corridor
    fn create_corridor(&self, room1: &RoomData, room2: &RoomData, rng: &mut StdRng) -> Vec<CorridorSegment> {
        let mut segments = Vec::new();
        
        // Randomly choose to go horizontal first or vertical first
        if rng.gen_bool(0.5) {
            // Horizontal then vertical
            segments.push(CorridorSegment {
                start_x: room1.center_x,
                start_y: room1.center_y,
                end_x: room2.center_x,
                end_y: room1.center_y,
            });
            segments.push(CorridorSegment {
                start_x: room2.center_x,
                start_y: room1.center_y,
                end_x: room2.center_x,
                end_y: room2.center_y,
            });
        } else {
            // Vertical then horizontal
            segments.push(CorridorSegment {
                start_x: room1.center_x,
                start_y: room1.center_y,
                end_x: room1.center_x,
                end_y: room2.center_y,
            });
            segments.push(CorridorSegment {
                start_x: room1.center_x,
                start_y: room2.center_y,
                end_x: room2.center_x,
                end_y: room2.center_y,
            });
        }
        
        segments
    }
    
    /// Convert room data and corridors into actual Room objects
    fn build_rooms(&self, 
        room_data: &[RoomData], 
        _corridors: &[Vec<CorridorSegment>],
        room_connections: &HashMap<String, Vec<(String, Direction)>>
    ) -> HashMap<String, Room> {
        let mut rooms = HashMap::new();
        
        for (idx, data) in room_data.iter().enumerate() {
            let mut room = Room::new(
                data.id.clone(),
                data.name.clone(),
                self.generate_room_description(idx),
                data.width,
                data.height,
            );
            
            // Add connections
            if let Some(connections) = room_connections.get(&data.id) {
                for (target_id, direction) in connections {
                    room.connections.insert(*direction, target_id.clone());
                    
                    // Add door at appropriate position
                    match direction {
                        Direction::North => room.add_door(*direction, data.width / 2, 0),
                        Direction::South => room.add_door(*direction, data.width / 2, data.height - 1),
                        Direction::East => room.add_door(*direction, data.width - 1, data.height / 2),
                        Direction::West => room.add_door(*direction, 0, data.height / 2),
                    }
                }
            }
            
            // Add some random features
            if idx == room_data.len() - 1 {
                // Last room gets stairs
                room.tiles[data.height / 2][data.width / 2] = TileType::Stairs;
            }
            
            rooms.insert(data.id.clone(), room);
        }
        
        rooms
    }
    
    fn generate_room_description(&self, index: usize) -> String {
        let descriptions = vec![
            "A damp chamber with moss growing on the walls.",
            "The air is thick with dust in this ancient room.",
            "Strange symbols are carved into the stone floor here.",
            "Water drips steadily from cracks in the ceiling.",
            "Old bones are scattered across the floor.",
            "Tattered banners hang from rusty chains.",
            "The walls are covered in faded murals.",
            "A faint, eerie glow emanates from phosphorescent fungi.",
            "The room smells of decay and forgotten ages.",
            "Cobwebs fill every corner of this abandoned chamber.",
        ];
        
        descriptions[index % descriptions.len()].to_string()
    }
    
    /// Determine which rooms are connected based on their positions
    fn calculate_connections(&self, room_data: &[RoomData]) -> HashMap<String, Vec<(String, Direction)>> {
        let mut connections = HashMap::new();
        
        for i in 0..room_data.len() {
            let room1 = &room_data[i];
            let mut room_connections = Vec::new();
            
            for j in 0..room_data.len() {
                if i == j { continue; }
                
                let room2 = &room_data[j];
                
                // Check if rooms are adjacent
                if let Some(direction) = self.get_adjacent_direction(room1, room2) {
                    room_connections.push((room2.id.clone(), direction));
                }
            }
            
            if !room_connections.is_empty() {
                connections.insert(room1.id.clone(), room_connections);
            }
        }
        
        connections
    }
    
    fn get_adjacent_direction(&self, room1: &RoomData, room2: &RoomData) -> Option<Direction> {
        // Check if room2 is to the north of room1
        if room2.y + room2.height < room1.y && 
           room1.center_x >= room2.x && room1.center_x < room2.x + room2.width {
            return Some(Direction::North);
        }
        
        // Check if room2 is to the south of room1
        if room1.y + room1.height < room2.y &&
           room1.center_x >= room2.x && room1.center_x < room2.x + room2.width {
            return Some(Direction::South);
        }
        
        // Check if room2 is to the east of room1
        if room1.x + room1.width < room2.x &&
           room1.center_y >= room2.y && room1.center_y < room2.y + room2.height {
            return Some(Direction::East);
        }
        
        // Check if room2 is to the west of room1
        if room2.x + room2.width < room1.x &&
           room1.center_y >= room2.y && room1.center_y < room2.y + room2.height {
            return Some(Direction::West);
        }
        
        None
    }
}

impl DungeonGenerator for SimpleGenerator {
    fn generate(&self, config: &GeneratorConfig, seed: u64) -> Result<Dungeon, String> {
        let mut rng = StdRng::seed_from_u64(seed);
        let num_rooms = rng.gen_range(config.min_rooms..=config.max_rooms);
        
        // Track occupied space
        let mut dungeon_grid = vec![vec![false; config.dungeon_width]; config.dungeon_height];
        let mut room_data = Vec::new();
        
        // Generate rooms
        let mut attempts = 0;
        while room_data.len() < num_rooms && attempts < 1000 {
            let width = rng.gen_range(config.min_room_size..=config.max_room_size);
            let height = rng.gen_range(config.min_room_size..=config.max_room_size);
            let x = rng.gen_range(1..config.dungeon_width.saturating_sub(width + 1));
            let y = rng.gen_range(1..config.dungeon_height.saturating_sub(height + 1));
            
            let room_names = vec![
                "Stone Chamber", "Dark Hall", "Ancient Vault", "Forgotten Crypt",
                "Mystic Sanctum", "Guard Room", "Treasury", "Library",
                "Throne Room", "Prison Cell", "Workshop", "Storage Room"
            ];
            
            let id = format!("room_{}", room_data.len());
            let name = room_names[rng.gen_range(0..room_names.len())].to_string();
            
            if let Some(room) = self.create_room_at(id, name, x, y, width, height, &mut dungeon_grid) {
                room_data.push(room);
            }
            
            attempts += 1;
        }
        
        if room_data.len() < config.min_rooms {
            return Err(format!("Could only generate {} rooms, minimum is {}", 
                room_data.len(), config.min_rooms));
        }
        
        // Connect rooms with corridors
        let mut corridors = Vec::new();
        for i in 0..room_data.len() - 1 {
            let corridor = self.create_corridor(&room_data[i], &room_data[i + 1], &mut rng);
            corridors.push(corridor);
        }
        
        // Calculate room connections based on layout
        let room_connections = self.calculate_connections(&room_data);
        
        // Build the actual rooms
        let rooms = self.build_rooms(&room_data, &corridors, &room_connections);
        
        // Set starting position
        let start_room = &room_data[0];
        let player_x = start_room.width / 2;
        let player_y = start_room.height / 2;
        
        Ok(Dungeon {
            rooms,
            current_room_id: start_room.id.clone(),
            player_x,
            player_y,
        })
    }
    
    fn name(&self) -> &'static str {
        "Simple Room Generator"
    }
}

#[derive(Debug)]
struct RoomData {
    id: String,
    name: String,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    center_x: usize,
    center_y: usize,
}

#[derive(Debug)]
struct CorridorSegment {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}
