// src/main.rs

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod components;
mod data;
mod game_state;
mod plugins;
mod resources;
mod setup;
mod templates;

use data::DataPlugin;
use game_state::GameState;
use plugins::{ui, player, monster, combat};
use resources::MessageLog;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_state::<GameState>()
        .init_resource::<MessageLog>()
        // Add the data plugin which loads all JSON data
        .add_plugins(DataPlugin)
        .add_plugins((
            setup::SetupPlugin,
            ui::UiPlugin,
            player::PlayerPlugin,
            monster::MonsterPlugin,
            combat::CombatPlugin,
        ))
        .run();
}