// src/plugins/monster.rs - DEBUG VERSION

use bevy::prelude::*;
use crate::components::{Monster, Player};
use crate::plugins::combat::AttackEvent;
use crate::resources::TurnState;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, _app: &mut App) {
        // The monster AI system is registered in the CombatPlugin
    }
}

pub fn monster_ai_system(
    turn_state: Option<Res<TurnState>>,
    mut attack_events: EventWriter<AttackEvent>,
    player_query: Query<Entity, With<Player>>,
    monster_query: Query<Entity, With<Monster>>,
) {
    info!("Monster AI system running");
    
    // Check if we have turn state
    let Some(turn) = turn_state else {
        error!("No TurnState in monster_ai_system!");
        return;
    };
    
    info!("Monster AI checking turn: {:?}", *turn);
    
    // Only act during monster turn
    if *turn != TurnState::MonsterTurn {
        info!("Not monster turn, skipping");
        return;
    }
    
    info!("It's monster turn!");
    
    // Get entities
    match (player_query.get_single(), monster_query.get_single()) {
        (Ok(player_entity), Ok(monster_entity)) => {
            info!("Monster {:?} attacking Player {:?}", monster_entity, player_entity);
            attack_events.send(AttackEvent {
                attacker: monster_entity,
                defender: player_entity,
            });
        }
        (Err(e1), _) => {
            error!("Failed to get player for monster AI: {:?}", e1);
        }
        (_, Err(e2)) => {
            error!("Failed to get monster for monster AI: {:?}", e2);
        }
    }
}