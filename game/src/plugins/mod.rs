// src/plugins/mod.rs
use bevy::prelude::*;

mod combat;
mod monster;
mod player;
mod ui;

use combat::CombatPlugin;
use monster::MonsterPlugin;
use player::PlayerPlugin;
use ui::UiPlugin;

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