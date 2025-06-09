// src/setup.rs - Updated to use the data system
use bevy::prelude::*;
use crate::components::*;
use crate::data::GameData;
use crate::data::templates::spawn_monster_from_template;
use crate::game_state::GameState;
use crate::resources::MessageLog;
use crate::systems::DropLootEvent;

pub fn setup_game(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut message_log: ResMut<MessageLog>,
    game_data: Res<GameData>, // Now we can access loaded data
    mut loot_events: EventWriter<DropLootEvent>,
) {
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

    // Spawn monsters using the loaded templates
    if let Some(entity) = spawn_monster_from_template(
        &mut commands,
        &game_data.monsters,
        "goblin_warrior",
        Position { x: 5, y: 5, level: 1 },
        Some(3),
    ) {
        message_log.add(
            "A Goblin Warrior appears!".to_string(),
            Color::YELLOW,
        );
    }
    
    // Spawn some loot on the ground
    loot_events.send(DropLootEvent {
        loot_table_id: "goblin_warrior_loot".to_string(),
        position: Position { x: 2, y: 2, level: 1 },
        level: 5,
        luck: 1.0,
    });

    message_log.add(
        format!("Game data loaded: {} monsters, {} items, {} NPCs available", 
            game_data.monsters.count(),
            game_data.items.count(),
            game_data.npcs.count()
        ),
        Color::LIME_GREEN,
    );

    next_state.set(GameState::MainMenu);
}