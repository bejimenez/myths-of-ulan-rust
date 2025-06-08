// src/plugins/combat.rs
use bevy::prelude::*;
use crate::components::{CombatStats, Health, Name};
use crate::game_state::GameState;
use crate::resources::MessageLog;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CombatEvent>()
            .add_systems(Update, combat_system.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Event)]
pub struct CombatEvent {
    pub attacker: Entity,
    pub defender: Entity,
}

fn combat_system(
    mut commands: Commands,
    mut combat_events: EventReader<CombatEvent>,
    mut combatants: Query<(&mut Health, &CombatStats, &Name)>,
    mut message_log: ResMut<MessageLog>,
) {
    for event in combat_events.read() {
        let Ok([(attacker_health, attacker_stats, attacker_name), (defender_health, defender_stats, defender_name)]) =
            combatants.get_many_mut([event.attacker, event.defender]) else { continue };

        let hit_chance = attacker_stats.accuracy - defender_stats.evasion;
        let hit_roll = rand::random::<i32>() % 100;

        if hit_roll < hit_chance {
            let damage = (attacker_stats.damage - defender_stats.defense).max(1);
            let mut defender_health = defender_health;
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
    }
}