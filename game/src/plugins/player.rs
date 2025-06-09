// src/plugins/player.rs - FIXED FOR MULTIPLE MONSTERS

use bevy::prelude::*;
use crate::components::{Monster, Player, Position, Name};
use crate::game_state::GameState;
use crate::resources::MessageLog;
use super::combat::StartCombatEvent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerMoveEvent>()
            .add_systems(
                Update,
                (
                    player_input_system,
                    movement_system,
                )
                .chain()
                .run_if(in_state(GameState::Exploring))
            );
    }
}

#[derive(Event)]
pub struct PlayerMoveEvent {
    pub dx: i32,
    pub dy: i32,
}

fn player_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_move_events: EventWriter<PlayerMoveEvent>,
) {
    if keyboard.just_pressed(KeyCode::KeyW) || keyboard.just_pressed(KeyCode::ArrowUp) {
        player_move_events.send(PlayerMoveEvent { dx: 0, dy: 1 });
    }
    if keyboard.just_pressed(KeyCode::KeyS) || keyboard.just_pressed(KeyCode::ArrowDown) {
        player_move_events.send(PlayerMoveEvent { dx: 0, dy: -1 });
    }
    if keyboard.just_pressed(KeyCode::KeyA) || keyboard.just_pressed(KeyCode::ArrowLeft) {
        player_move_events.send(PlayerMoveEvent { dx: -1, dy: 0 });
    }
    if keyboard.just_pressed(KeyCode::KeyD) || keyboard.just_pressed(KeyCode::ArrowRight) {
        player_move_events.send(PlayerMoveEvent { dx: 1, dy: 0 });
    }
}

fn movement_system(
    mut move_events: EventReader<PlayerMoveEvent>,
    mut player_query: Query<&mut Position, (With<Player>, Without<Monster>)>,
    monster_query: Query<(Entity, &Position, &Name), (With<Monster>, Without<Player>)>,
    mut message_log: ResMut<MessageLog>,
    mut next_state: ResMut<NextState<GameState>>,
    mut combat_events: EventWriter<StartCombatEvent>,
) {
    for event in move_events.read() {
        if let Ok(mut player_pos) = player_query.get_single_mut() {
            let new_x = player_pos.x + event.dx;
            let new_y = player_pos.y + event.dy;
            let mut blocked = false;

            // Check all monsters for collision
            for (monster_entity, monster_pos, monster_name) in monster_query.iter() {
                if monster_pos.x == new_x && 
                   monster_pos.y == new_y && 
                   monster_pos.level == player_pos.level {
                    message_log.add(
                        format!("You encounter {}! Press 'A' to attack.", monster_name.0),
                        Color::ORANGE_RED,
                    );
                    
                    // Send combat event with the specific monster
                    combat_events.send(StartCombatEvent {
                        monster: monster_entity,
                    });
                    
                    next_state.set(GameState::InCombat);
                    blocked = true;
                    break;
                }
            }

            if !blocked {
                player_pos.x = new_x;
                player_pos.y = new_y;
            }
        }
    }
}