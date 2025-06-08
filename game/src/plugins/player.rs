// src/plugins/player.rs

use bevy::prelude::*;
use crate::components::{Monster, Player, Position}; // Removed unused `Name` import
use crate::game_state::GameState;
use crate::resources::MessageLog;

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

// System with corrected Query parameters
fn movement_system(
    mut move_events: EventReader<PlayerMoveEvent>,
    // --- FIX IS HERE ---
    // We group multiple filters in a tuple and add `Without`.
    mut player_query: Query<&mut Position, (With<Player>, Without<Monster>)>,
    monster_query: Query<&Position, (With<Monster>, Without<Player>)>,
    // --- END FIX ---
    mut message_log: ResMut<MessageLog>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in move_events.read() {
        if let Ok(mut player_pos) = player_query.get_single_mut() {
            let new_x = player_pos.x + event.dx;
            let new_y = player_pos.y + event.dy;
            let mut blocked = false;

            for monster_pos in monster_query.iter() {
                if monster_pos.x == new_x && monster_pos.y == new_y && monster_pos.level == player_pos.level {
                    message_log.add(
                        "You encounter a goblin! Press 'A' to attack.".to_string(),
                        Color::ORANGE_RED,
                    );
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