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
use plugins::{ui, player, monster, combat};
use resources::MessageLog;
use templates::monster_templates::MonsterTemplateRegistry;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_state::<GameState>()
        .init_resource::<MessageLog>()
        .init_resource::<MonsterTemplateRegistry>()
        .add_plugins((
            setup::SetupPlugin,
            ui::UiPlugin,
            player::PlayerPlugin,
            monster::MonsterPlugin,
            combat::CombatPlugin,
        ))
        .run();
}