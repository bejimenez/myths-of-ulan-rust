// src/data/mod.rs
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub mod loader;
pub mod templates;

use loader::DataLoader;
use templates::*;

/// Main resource that holds all loaded game data
#[derive(Resource, Default)]
pub struct GameData {
    pub monsters: MonsterTemplateRegistry,
    pub items: ItemTemplateRegistry,
    pub npcs: NPCTemplateRegistry,
    pub loot_tables: LootTableRegistry,
}

/// Plugin that handles loading all game data
pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameData>()
            .add_systems(PreStartup, load_all_game_data);
    }
}

fn load_all_game_data(
    mut game_data: ResMut<GameData>,
) {
    let data_path = Path::new("data");
    
    // Load monsters
    if let Err(e) = DataLoader::load_monsters(&data_path.join("monsters"), &mut game_data.monsters) {
        error!("Failed to load monsters: {}", e);
    }
    
    // Load items
    if let Err(e) = DataLoader::load_items(&data_path.join("items"), &mut game_data.items) {
        error!("Failed to load items: {}", e);
    }
    
    // Load NPCs
    if let Err(e) = DataLoader::load_npcs(&data_path.join("npcs"), &mut game_data.npcs) {
        error!("Failed to load NPCs: {}", e);
    }
    
    // Load loot tables
    if let Err(e) = DataLoader::load_loot_tables(&data_path.join("loot_tables"), &mut game_data.loot_tables) {
        error!("Failed to load loot tables: {}", e);
    }
    
    info!("Game data loaded successfully!");
    info!("  - {} monster templates", game_data.monsters.count());
    info!("  - {} item templates", game_data.items.count());
    info!("  - {} NPC templates", game_data.npcs.count());
    info!("  - {} loot tables", game_data.loot_tables.count());
}