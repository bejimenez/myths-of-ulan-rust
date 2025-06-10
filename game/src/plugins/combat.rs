// src/plugins/combat.rs - FIXED IMPORTS

use bevy::prelude::*;
use crate::components::{CombatStats, Health, Name, Player, Monster};
use crate::game_state::GameState;
use crate::resources::{MessageLog, TurnState};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<AttackEvent>()
            .add_event::<StartCombatEvent>()
            .init_resource::<CurrentCombat>()
            .add_systems(OnEnter(GameState::InCombat), setup_combat)
            .add_systems(OnExit(GameState::InCombat), teardown_combat)
            .add_systems(
                Update,
                (
                    handle_combat_start,
                    combat_input_system,
                    process_attacks,
                    monster_ai_system,
                    check_combat_end,
                )
                .chain()
                .run_if(in_state(GameState::InCombat))
            );
    }
}

#[derive(Event)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub defender: Entity,
}

#[derive(Event)]
pub struct StartCombatEvent {
    pub monster: Entity,
}

#[derive(Resource, Default)]
pub struct CurrentCombat {
    pub monster_entity: Option<Entity>,
}

fn setup_combat(
    mut commands: Commands, 
    mut message_log: ResMut<MessageLog>,
) {
    commands.insert_resource(TurnState::PlayerTurn);
    message_log.add(
        "Combat begins! Press 'A' to attack.".to_string(),
        Color::ORANGE_RED,
    );
}

fn teardown_combat(
    mut commands: Commands,
    mut current_combat: ResMut<CurrentCombat>,
) {
    commands.remove_resource::<TurnState>();
    current_combat.monster_entity = None;
}

fn handle_combat_start(
    mut events: EventReader<StartCombatEvent>,
    mut current_combat: ResMut<CurrentCombat>,
) {
    for event in events.read() {
        current_combat.monster_entity = Some(event.monster);
    }
}

fn combat_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    turn_state: Res<TurnState>,
    mut attack_events: EventWriter<AttackEvent>,
    player_query: Query<Entity, With<Player>>,
    current_combat: Res<CurrentCombat>,
    mut message_log: ResMut<MessageLog>,
) {
    if *turn_state != TurnState::PlayerTurn {
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyA) {
        let Ok(player_entity) = player_query.get_single() else {
            message_log.add("No player found!".to_string(), Color::RED);
            return;
        };
        
        let Some(monster_entity) = current_combat.monster_entity else {
            message_log.add("No monster in combat!".to_string(), Color::RED);
            return;
        };
        
        attack_events.send(AttackEvent {
            attacker: player_entity,
            defender: monster_entity,
        });
    }
}

fn process_attacks(
    mut commands: Commands,
    mut attack_events: EventReader<AttackEvent>,
    mut combatants: Query<(&mut Health, &CombatStats, &Name)>,
    player_query: Query<(), With<Player>>,
    mut message_log: ResMut<MessageLog>,
    mut turn_state: ResMut<TurnState>,
) {
    for event in attack_events.read() {
        let Ok([(mut _attacker_health, attacker_stats, attacker_name), (mut defender_health, defender_stats, defender_name)]) =
            combatants.get_many_mut([event.attacker, event.defender]) else { 
                continue;
            };

        let hit_chance = attacker_stats.accuracy - defender_stats.evasion;
        let hit_roll = rand::random::<i32>() % 100;

        if hit_roll < hit_chance {
            let damage = (attacker_stats.damage - defender_stats.defense).max(1);
            defender_health.current -= damage;

            message_log.add(
                format!("{} hits {} for {} damage!", attacker_name.0, defender_name.0, damage),
                Color::RED,
            );

            if defender_health.current <= 0 {
                message_log.add(
                    format!("{} has been slain!", defender_name.0),
                    Color::DARK_GRAY,
                );
                // Only despawn the defender if it's not the player
                if player_query.get(event.defender).is_err() {
                    commands.entity(event.defender).despawn();
                }
            }
        } else {
            message_log.add(
                format!("{} misses {}!", attacker_name.0, defender_name.0),
                Color::GRAY
            );
        }

        // Switch turns
        match *turn_state {
            TurnState::PlayerTurn => *turn_state = TurnState::MonsterTurn,
            TurnState::MonsterTurn => *turn_state = TurnState::PlayerTurn,
        }
    }
}

pub fn monster_ai_system(
    turn_state: Res<TurnState>,
    mut attack_events: EventWriter<AttackEvent>,
    player_query: Query<Entity, With<Player>>,
    current_combat: Res<CurrentCombat>,
    monster_query: Query<&Monster>,
) {
    if *turn_state != TurnState::MonsterTurn {
        return;
    }
    
    let Some(monster_entity) = current_combat.monster_entity else {
        return;
    };
    
    // Check if the monster still exists
    if monster_query.get(monster_entity).is_err() {
        return;
    }
    
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };
    
    attack_events.send(AttackEvent {
        attacker: monster_entity,
        defender: player_entity,
    });
}

fn check_combat_end(
    current_combat: Res<CurrentCombat>,
    monster_query: Query<&Monster>,
    player_query: Query<&Health, With<Player>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut message_log: ResMut<MessageLog>,
) {
    // Check if the current combat monster is dead
    if let Some(monster_entity) = current_combat.monster_entity {
        if monster_query.get(monster_entity).is_err() {
            message_log.add(
                "You are victorious! You can move again.".to_string(),
                Color::LIME_GREEN,
            );
            game_state.set(GameState::Exploring);
            return;
        }
    }
    
    // Check if player is dead
    if let Ok(player_health) = player_query.get_single() {
        if player_health.current <= 0 {
            message_log.add(
                "You have been slain! Game Over.".to_string(),
                Color::RED,
            );
            game_state.set(GameState::GameOver);
        }
    }
}