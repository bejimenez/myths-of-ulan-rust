// src/plugins/combat.rs

use bevy::prelude::*;
use crate::components::{CombatStats, Health, Name, Player, Monster};
use crate::game_state::GameState;
use crate::resources::{MessageLog, TurnState};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<AttackEvent>()
            .add_systems(OnEnter(GameState::InCombat), setup_combat)
            .add_systems(OnExit(GameState::InCombat), teardown_combat)
            .add_systems(
                Update,
                (
                    combat_input_system,
                    process_attacks,
                    check_combat_end,
                    // The monster AI system should also be ordered correctly
                    // We can apply it here to make the turn order explicit.
                    super::monster::monster_ai_system.after(process_attacks),
                )
                // FIX: Remove the .chain() call. The logic within each system
                // and the event-driven nature handles the ordering correctly.
                .run_if(in_state(GameState::InCombat))
            );
    }
}

#[derive(Event)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub defender: Entity,
}

fn setup_combat(mut commands: Commands) {
    commands.insert_resource(TurnState::PlayerTurn);
    // You could also add a message log entry here if you want.
}

fn teardown_combat(mut commands: Commands) {
    commands.remove_resource::<TurnState>();
}

fn combat_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    turn_state: Res<TurnState>,
    mut attack_events: EventWriter<AttackEvent>,
    player_query: Query<Entity, With<Player>>,
    monster_query: Query<Entity, With<Monster>>,
) {
    if *turn_state != TurnState::PlayerTurn {
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyA) {
        if let (Ok(player_entity), Ok(monster_entity)) = (player_query.get_single(), monster_query.get_single()) {
            attack_events.send(AttackEvent {
                attacker: player_entity,
                defender: monster_entity,
            });
        }
    }
}

fn process_attacks(
    mut commands: Commands,
    mut attack_events: EventReader<AttackEvent>,
    mut combatants: Query<(&mut Health, &CombatStats, &Name)>,
    mut message_log: ResMut<MessageLog>,
    // FIX: Change from NextState<TurnState> to a direct mutable resource
    mut turn_state: ResMut<TurnState>,
) {
    // Only process one attack per frame to ensure turn-based flow
    if let Some(event) = attack_events.read().next() {
        let Ok([(mut _attacker_health, attacker_stats, attacker_name), (mut defender_health, defender_stats, defender_name)]) =
            combatants.get_many_mut([event.attacker, event.defender]) else { return };

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
                commands.entity(event.defender).despawn();
            }
        } else {
            message_log.add(format!("{} misses {}!", attacker_name.0, defender_name.0), Color::GRAY);
        }

        // --- FIX: Switch turns using direct assignment ---
        // This is what makes the combat turn-based.
        match *turn_state {
            TurnState::PlayerTurn => *turn_state = TurnState::MonsterTurn,
            TurnState::MonsterTurn => *turn_state = TurnState::PlayerTurn,
        }
    }
}

fn check_combat_end(
    monster_query: Query<&Health, With<Monster>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut message_log: ResMut<MessageLog>,
) {
    if monster_query.is_empty() {
        message_log.add(
            "You are victorious! You can move again.".to_string(),
            Color::LIME_GREEN,
        );
        game_state.set(GameState::Exploring);
    }
}