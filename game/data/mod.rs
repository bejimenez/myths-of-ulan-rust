// src/data/mod.rs
use bevy::prelude::*;

// Re-export templates for easier access
pub use crate::templates::monster_templates::{MonsterTemplateRegistry, MonsterTemplate};

// Comprehensive GameData resource that holds all loaded game data
#[derive(Resource, Default)]
pub struct GameData {
    // We'll just focus on monsters for now
    // Later this will contain:
    // pub items: ItemTemplateRegistry,
    // pub npcs: NPCTemplateRegistry,
    // pub loot_tables: LootTableRegistry,
}

// Data loading plugin
pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameData>()
            .init_resource::<MonsterTemplateRegistry>()
            .add_systems(PreStartup, load_monster_data);
    }
}

fn load_monster_data(
    mut monster_registry: ResMut<MonsterTemplateRegistry>,
) {
    // For now, we'll load from embedded JSON data
    // In production, this would read from files
    let goblin_data = include_str!("../../data/monsters/goblins.json");
    
    match monster_registry.load_from_json(goblin_data) {
        Ok(_) => {
            info!("Successfully loaded {} monster templates", monster_registry.count());
            
            // Log some debug info
            for template in monster_registry.get_all_templates() {
                debug!("Loaded monster: {} ({})", template.name, template.id);
            }
        }
        Err(e) => {
            error!("Failed to load monster data: {}", e);
        }
    }
}