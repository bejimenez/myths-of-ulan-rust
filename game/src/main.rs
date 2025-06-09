// src/main.rs - Updated to include the data plugin
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod components;
mod data; // Add this
mod game_state;
mod plugins;
mod resources;
mod setup;
mod systems; // Add this

use data::DataPlugin;
use game_state::GameState;
use plugins::GamePlugin;
use resources::{GameWorld, MessageLog};
use setup::setup_game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Myths of Ulan".to_string(),
                mode: bevy::window::WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins(DataPlugin) // Add this before GamePlugin
        .init_state::<GameState>()
        .init_resource::<GameWorld>()
        .init_resource::<MessageLog>()
        .add_plugins(GamePlugin)
        .add_systems(Startup, setup_game)
        .run();
}