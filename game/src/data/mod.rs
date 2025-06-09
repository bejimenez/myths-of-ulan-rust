// src/data/mod.rs
// This is a placeholder for the data loading system
// For now, we'll just re-export the templates module functionality

use bevy::prelude::*;

// Re-export templates for easier access
pub use crate::templates::monster_templates::MonsterTemplateRegistry;

// Placeholder for the comprehensive GameData resource
// This will eventually hold all loaded game data
#[derive(Resource, Default)]
pub struct GameData {
    // For now, we'll leave this empty
    // Later this will contain:
    // pub monsters: MonsterTemplateRegistry,
    // pub items: ItemTemplateRegistry,
    // pub npcs: NPCTemplateRegistry,
    // pub loot_tables: LootTableRegistry,
}

// Placeholder for the data plugin
pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameData>();
        // Data loading will be implemented here later
    }
}