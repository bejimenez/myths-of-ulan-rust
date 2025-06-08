// src/plugins/player.rs
use bevy::prelude::*;
use crate::components::{Monster, Name, Player, Position};
use crate::game_state::GameState;
use crate::plugins::combat::CombatEvent;
use crate::resources::{GameWorld, MessageLog};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerCommand>()
            .add_systems(
                Update,
                (
                    player_input_system,
                    movement_system,
                )
                .chain()
                .run_if(in_state(GameState::Playing))
            );
    }
}

#[derive(Event)]
pub enum PlayerCommand {
    Move { dx: i32, dy: i32 },
    Wait,
}

fn player_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_commands: EventWriter<PlayerCommand>,
) {
    if keyboard.just_pressed(KeyCode::KeyW) || keyboard.just_pressed(KeyCode::ArrowUp) {
        player_commands.send(PlayerCommand::Move { dx: 0, dy: 1 });
    }
    if keyboard.just_pressed(KeyCode::KeyS) || keyboard.just_pressed(KeyCode::ArrowDown) {
        player_commands.send(PlayerCommand::Move { dx: 0, dy: -1 });
    }
    if keyboard.just_pressed(KeyCode::KeyA) || keyboard.just_pressed(KeyCode::ArrowLeft) {
        player_commands.send(PlayerCommand::Move { dx: -1, dy: 0 });
    }
    if keyboard.just_pressed(KeyCode::KeyD) || keyboard.just_pressed(KeyCode::ArrowRight) {
        player_commands.send(PlayerCommand::Move { dx: 1, dy: 0 });
    }
    if keyboard.just_pressed(KeyCode::Space) {
        player_commands.send(PlayerCommand::Wait);
    }
}

fn movement_system(
    mut player_commands: EventReader<PlayerCommand>,
    mut player_query: Query<(&mut Position, Entity), With<Player>>,
    monster_query: Query<(&Position, Entity, &Name), (With<Monster>, Without<Player>)>,
    mut combat_events: EventWriter<CombatEvent>,
    mut message_log: ResMut<MessageLog>,
    mut game_world: ResMut<GameWorld>,
) {
    for command in player_commands.read() {
        if let Ok((mut player_pos, player_entity)) = player_query.get_single_mut() {
             match command {
                PlayerCommand::Move { dx, dy } => {
                    let new_x = player_pos.x + dx;
                    let new_y = player_pos.y + dy;
                    let mut blocked = false;
                    for (monster_pos, monster_entity, monster_name) in monster_query.iter() {
                        if monster_pos.x == new_x && monster_pos.y == new_y && monster_pos.level == player_pos.level {
                            message_log.add(format!("You bump into the {}!", monster_name.0), Color::YELLOW);
                            combat_events.send(CombatEvent { attacker: player_entity, defender: monster_entity });
                            blocked = true;
                            break;
                        }
                    }
                    if !blocked {
                        player_pos.x = new_x;
                        player_pos.y = new_y;
                        game_world.turn_count += 1;
                    }
                }
                PlayerCommand::Wait => {
                    game_world.turn_count += 1;
                    message_log.add(format!("Turn {}: You wait.", game_world.turn_count), Color::GRAY);
                }
            }
        }
    }
}