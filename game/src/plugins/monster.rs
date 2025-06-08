// src/plugins/monster.rs

use bevy::prelude::*;
use crate::components::{Monster, Player};
use crate::game_state::GameState;
use crate::plugins::combat::AttackEvent;
use crate::resources::TurnState;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        // We move the system registration to the CombatPlugin
        // to ensure its execution order is correct relative to other combat systems.
        // This file now just defines the system itself.
    }
}

// FIX: make the system public using `pub`
pub fn monster_ai_system(
    turn_state: Res<TurnState>,
    mut attack_events: EventWriter<AttackEvent>,
    player_query: Query<Entity, With<Player>>,
    monster_query: Query<Entity, With<Monster>>,
) {
    if *turn_state != TurnState::MonsterTurn {
        return;
    }
    
    // The monster takes its turn immediately.
    if let (Ok(player_entity), Ok(monster_entity)) =
        (player_query.get_single(), monster_query.get_single())
    {
        attack_events.send(AttackEvent {
            attacker: monster_entity,
            defender: player_entity,
        });
    }
}