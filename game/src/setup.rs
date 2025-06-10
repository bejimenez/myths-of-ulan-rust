// src/setup.rs

use bevy::prelude::*;
use crate::components::*;
use crate::resources::MessageLog;
use crate::game_state::GameState;
use crate::templates::monster_templates::{MonsterTemplateRegistry, spawn_monster_from_template};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Exploring), (
                cleanup_old_game,
                setup_new_game,
            ).chain());
    }
}

fn cleanup_old_game(
    mut commands: Commands,
    entities: Query<Entity, Or<(With<Player>, With<Monster>)>>,
    mut message_log: ResMut<MessageLog>,
) {
    // Clear all game entities
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Clear message log
    message_log.messages.clear();
}

fn setup_new_game(
    mut commands: Commands,
    mut message_log: ResMut<MessageLog>,
    template_registry: Res<MonsterTemplateRegistry>,
) {
    // Spawn the player at (0, 0)
    commands.spawn((
        Player,
        Position { x: 0, y: 0, level: 0 },
        Health { current: 30, max: 30 },
        Stats {
            strength: 10,
            dexterity: 10,
            intelligence: 10,
            constitution: 10,
        },
        CombatStats {
            damage: 5,
            defense: 2,
            accuracy: 75,
            evasion: 10,
        },
        Name("Player".to_string()),
    ));

    // Spawn monsters from templates
    let raging_goblin = spawn_monster_from_template(
        &mut commands,
        &template_registry,
        "goblin_raging",
        Position { x: 5, y: 5, level: 0 },
        Some(3), // level 3
    );
    
    if raging_goblin.is_none() {
        eprintln!("Failed to spawn raging goblin - template not found");
    }

    let goblin = spawn_monster_from_template(
        &mut commands,
        &template_registry,
        "goblin",
        Position { x: -3, y: 2, level: 0 },
        Some(1), // level 1
    );
    
    if goblin.is_none() {
        eprintln!("Failed to spawn goblin - template not found");
    }

    message_log.add(
        "Welcome to Myths of Ulan! Move with WASD or arrow keys.".to_string(),
        Color::LIME_GREEN,
    );
}