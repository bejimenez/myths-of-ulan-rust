// src/systems/mod.rs
// This module will contain various game systems
// For now, it's a placeholder

use bevy::prelude::*;

// Future modules will be added here:
// pub mod item_spawning;
// pub mod loot_system;
// pub mod dungeon_generation;

// Placeholder for systems that aren't part of the plugin architecture yet
pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // Systems will be registered here
    }
}