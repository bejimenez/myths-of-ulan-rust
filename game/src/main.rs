// src/main.rs
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod components;
mod game_state;
mod plugins;
mod resources;
mod setup;
mod templates;

use game_state::GameState;
use plugins::GamePlugin;
use resources::{GameWorld, MessageLog};
use setup::setup_game;

fn main() {
    App::new()
        // Add Bevy's default plugins and Egui
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Myths of Ulan".to_string(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)

        // Add the game state
        .init_state::<GameState>()

        // Add our global resources
        .init_resource::<GameWorld>()
        .init_resource::<MessageLog>()

        // Add our custom game plugins
        .add_plugins(GamePlugin)

        // Add the startup system
        .add_systems(Startup, setup_game)

        .run();
}