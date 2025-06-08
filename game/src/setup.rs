// src/setup.rs
use bevy::prelude::*;
use crate::components::*;
use crate::game_state::GameState;
use crate::resources::MessageLog;

pub fn setup_game(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut message_log: ResMut<MessageLog>,
) {
    commands.spawn((
        Player,
        Name("Hero".to_string()),
        Position { x: 0, y: 0, level: 1 },
        Health { current: 100, max: 100 },
        Stats { strength: 10, dexterity: 10, intelligence: 10, constitution: 10 },
        CombatStats { damage: 5, defense: 2, accuracy: 85, evasion: 10 },
        Inventory { items: Vec::new(), capacity: 20 },
    ));

    commands.spawn((
        Monster { ai_type: AIType::Aggressive },
        Name("Goblin".to_string()),
        Position { x: 5, y: 5, level: 1 },
        Health { current: 30, max: 30 },
        Stats { strength: 6, dexterity: 8, intelligence: 4, constitution: 6 },
        CombatStats { damage: 3, defense: 1, accuracy: 70, evasion: 15 },
    ));

    message_log.add(
        "Welcome to Myths of Ulan! The game has been refactored.".to_string(),
        Color::LIME_GREEN,
    );
    message_log.add(
        "A goblin still lurks at (5, 5).".to_string(),
        Color::YELLOW,
    );

    next_state.set(GameState::MainMenu);
}