use bevy::prelude::*;

// Re-export templates for easier access
pub use crate::templates::monster_templates::MonsterTemplateRegistry;

// We can keep the GameData resource for future use, but the key part is the plugin.
#[derive(Resource, Default)]
pub struct GameData {}

pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameData>()
            .init_resource::<MonsterTemplateRegistry>()
            // This is the crucial part: load the data before the main app starts up.
            // PreStartup runs before any OnEnter states, guaranteeing the data is ready.
            .add_systems(PreStartup, load_monster_data);
    }
}

/// System that loads monster templates from JSON files into the registry.
fn load_monster_data(
    mut monster_registry: ResMut<MonsterTemplateRegistry>,
    asset_server: Res<AssetServer>, // Using AssetServer is more idiomatic in Bevy if files are in assets folder
) {
    // Make sure this path is correct relative to the src/data/mod.rs file.
    // Project/
    // |- data/
    // |  |- monsters/
    // |     |- goblins.json
    // |- src/
    // |  |- data/
    // |     |- mod.rs
    // ...
    let goblin_data = include_str!("../../data/monsters/goblins.json");

    match monster_registry.load_from_json(goblin_data) {
        Ok(_) => {
            info!(
                "Successfully loaded {} monster templates from goblins.json",
                monster_registry.count()
            );
        }
        Err(e) => {
            error!("Failed to load monster data from goblins.json: {}", e);
        }
    }

    // add more files here
    // let undead_data = include_str!("../../../data/monsters/undead.json");
    // monster_registry.load_from_json(undead_data).unwrap();
}