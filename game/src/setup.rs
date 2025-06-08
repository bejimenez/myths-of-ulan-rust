// src/setup.rs
use bevy::prelude::*;
use crate::components::*;
use crate::game_state::GameState;
use crate::resources::MessageLog;
use crate::templates::monster_templates::{MonsterTemplateRegistry, spawn_monster_from_template};

pub fn setup_game(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut message_log: ResMut<MessageLog>,
) {
    // Initialize the monster template MonsterTemplateRegistry
    let monster_registry = MonsterTemplateRegistry::new();
    commands.insert_resource(monster_registry);

    // Spawn the player
    commands.spawn((
        Player,
        Name("Hero".to_string()),
        Position { x: 0, y: 0, level: 1 },
        Health { current: 100, max: 100 },
        Stats { strength: 10, dexterity: 10, intelligence: 10, constitution: 10 },
        CombatStats { damage: 5, defense: 2, accuracy: 85, evasion: 10 },
        Inventory { items: Vec::new(), capacity: 20 },
    ));
    
    // Access the registry we just created 
    let registry = commands.get_resource::<MonsterTemplateRegistry>().unwrap();

    // Spawn a Raging Goblin using the template spawn_monster_from_template
    spawn_monster_from_template(
        &mut commands,
        &registry,
        "raging_goblin",
        Position { x: 5, y: 5, level: 1 },
        Some(4), // Spawn at level 4
    );

    // Spawn a regular goblin at a random level within its range 
    spawn_monster_from_template(
        &mut commands,
        &registry,
        "goblin",
        Position { x: -3, y: 2, level: 1 },
        None, //let the system choose a random level
    );

    message_log.add(
        "Welcom to Myths of Ulan! The template system is now active.".to_string(),
        Color::LIME_GREEN,
        );
    message_log.add(
        "A Raging Goblin lurks at (5,5) and a regular Goblin at (-3, 2).".to_string(),
        Color::YELLOW,
        );

    next_state.set(GameState::MainMenu);
}
