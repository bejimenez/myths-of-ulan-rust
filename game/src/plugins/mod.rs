// src/plugins/mod.rs
use bevy::prelude::*;

pub mod combat;
pub mod monster;
pub mod player;
pub mod ui;

pub use combat::CombatPlugin;
pub use monster::MonsterPlugin;
pub use player::PlayerPlugin;
pub use ui::UiPlugin;

// This plugin will bundle all our game-specific plugins.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiPlugin,
            PlayerPlugin,
            CombatPlugin,
            MonsterPlugin,
        ));
    }
}