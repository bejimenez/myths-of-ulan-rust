// src/setup.rs

use bevy::prelude::*;
use crate::components::*;
use crate::resources::MessageLog;
use crate::game_state::GameState;
use crate::templates::monster_templates::{MonsterTemplateRegistry, spawn_monster_from_template};
use rand::prelude::*;

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

    // Spawn random goblins from the loaded templates
    spawn_random_goblins(&mut commands, &template_registry);

    message_log.add(
        "Welcome to Myths of Ulan! Move with WASD or arrow keys.".to_string(),
        Color::LIME_GREEN,
    );
}

/// Spawns random goblins in the starting area
fn spawn_random_goblins(
    commands: &mut Commands,
    template_registry: &MonsterTemplateRegistry,
) {
    // Define spawn parameters
    const NUM_GOBLINS: usize = 2;
    const MIN_LEVEL: i32 = 1;
    const MAX_LEVEL: i32 = 3;

    // Possible spawn positions (avoiding the player's starting position at 0,0)
    let possible_positions = vec![
        Position { x: 5, y: 5, level: 0 },
        Position { x: -3, y: 2, level: 0 },
        Position { x: 3, y: -4, level: 0 },
        Position { x: -5, y: -2, level: 0 },
        Position { x: 7, y: 1, level: 0 },
    ];

    let mut rng = thread_rng();
    let mut used_positions: Vec<Position> = Vec::new();

    // Get all valid goblin templates in the desired level range *once*
    let goblin_templates = template_registry.get_templates_by_type_and_level("Goblin", MIN_LEVEL, MAX_LEVEL);

    if goblin_templates.is_empty() {
        warn!("No 'Goblin' type templates found for level range {}-{}", MIN_LEVEL, MAX_LEVEL);
        return;
    }

    for i in 0..NUM_GOBLINS {
        // Choose a random template from the pre-filtered list
        if let Some(goblin_template) = goblin_templates.choose(&mut rng) {
            // Choose a random position that hasn't been used
            let available_positions: Vec<&Position> = possible_positions
                .iter()
                .filter(|pos| !used_positions.contains(pos))
                .collect();

            if let Some(&position) = available_positions.choose(&mut rng) {
                used_positions.push(*position);

                // Generate a random level within the spawn parameters, clamped to the template's own limits
                let level = rng.gen_range(MIN_LEVEL..=MAX_LEVEL)
                    .clamp(goblin_template.level_range.0, goblin_template.level_range.1);

                // Spawn the monster using the chosen template and position
                if spawn_monster_from_template(
                    commands,
                    template_registry,
                    &goblin_template.id,
                    *position,
                    Some(level),
                ).is_some() {
                    info!("Spawned {} (Lvl {}) at ({}, {})", goblin_template.name, level, position.x, position.y);
                } else {
                    // This case should be rare now
                    warn!("Failed to spawn monster from template: {}", goblin_template.id);
                }
            } else {
                warn!("Could not find an available position for goblin spawn #{}", i + 1);
            }
        }
    }
}