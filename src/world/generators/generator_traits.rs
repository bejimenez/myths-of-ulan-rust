use crate::world::Dungeon;
use serde::{Deserialize, Serialize};

// Trait for all dungeon generators to implement
pub trait DungeonGenerator {
    fn generate(&self, config: &GeneratorConfig, seed: u64) -> Result<Dungeon, String>;
    fn name(&self) -> &'static str;
}

// Configuration for dungeon generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorConfig {
    pub min_rooms: usize, 
    pub max_rooms: usize,
    pub min_room_size: usize,
    pub max_room_size: usize,
    pub dungeon_width: usize,
    pub dungeon_height: usize,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        GeneratorConfig {
            min_rooms: 5,
            max_rooms: 10,
            min_room_size: 5,
            max_room_size: 12,
            dungeon_width: 80,
            dungeon_height: 50,
        }
    }
}
