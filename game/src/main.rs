use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use std::collections::HashMap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Myths of Ulan".to_string(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EquiPlugin)
        .add_state::<GameState>()
        .init_resource::<GameWorld>()
        .init_resource::<MessageLog>()
        .add_event::<CombatEvent>()
        .add_event::<PlayerCommand>()
        .add_systems(Startup, setup_game)
        .add_systems(Update, main_menu.run_if(in_state(GameState::MainMenu)))

        .add_systems(
            Update,
            (
                player_input_system,
                movement_system,
                combat_system,
                ui_system,
                message_log_system,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
            )
        .run();
}

// === GAME STATES ===
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

// === COMPONENTS ===
// These are the "data" attached to entities

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Monster {
    ai_type: AIType,
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
    level: i32, // Dungeon level
}

#[derive(Component)]
stuct Health {
    current: i32,
    max: i32,
}

#[derive(Component)]
struct Stats {
    strength: i32,
    dexterity: i32,
    intelligence: i32,
    constitution: i32,
}

#[derive(Component)]
struct CombatStats {
    damage: i32,
    defense: i32,
    accuracy: i32,
    evasion: i32,
}

#[derive(Component)]
struct Inventory {
    items: Vec<Entity>,
    capacity: usize,
}

#[derive(Component)]
struct Item {
    item_type: ItemType,
    stack_size: u32,
}

// === ENUMS & TYPES ===
#[derive(Debug, Clone)]
enum AIType {
    Aggressive,
    Defensive,
    Passive,
}

#[derive(Debug, Clone)]
enum ItemType {
    Weapon { damage: i32 },
    Armor { defense: i32 },
    Potion { heal_amount: i32 },
    Gold { amount: u32 },
}

// === RESOURCES ===
// Global data shared across systems    

#[derive(Resource, Default)]
struct GameWorld {
    current_dungeon: Option<String>,
    dungeon_level: i32,
    turn_count: u32,
}

#[derive(Resource, Default)]
struct MessageLog {
    messages: Vec<(String, Color)>,
}

impl MessageLog {
    fn add(&mut self, message: String, color: Color) {
        self.messages.push((message, color));
        // Keep only last 100 messages
        if self.messages.len() > 100 {
            self.messages.remove(0);
        }
    }
}

// === EVENTS ===
#[derive(Event)]
enum PlayerCommand {
    Move { dx: i32, dy: i32 },
    Attack { target: Entity },
    UseItem { item: Entity },
    Wait,
}

#[derive(Event)]
struct CombatEvent {
    attacker: Entity,
    defender: Entity,
    damage: i32,
}

// === SYSTEMS ===
// Functions that operate on entities with specific components

fn setup_game(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    // Spawn the player Entity
    commands.spawn((
            Player,
            Name("Hero".to_string()),
            Position { x: 0, y: 0, level: 1 },
            Health { current: 100, max: 100 },
            Stats {
                strength: 10,
                dexterity: 10,
                intelligence: 10,
                constitution: 10,
            },
            CombatStats {
                damage: 5,
                defense: 2,
                accuracy: 85,
                evasion: 10,
            },
            Inventory {
                items: Vec::new(),
                capactiy: 20,
            },
        ));

    // Spawn a test Monster
    commands.spawn((
            Monster {
                ai_type: AIType::Aggressive,
            },
            Name("Goblin".to_string()),
            Position { x: 5, y: 5, level: 1 },
            Health { current: 30, max: 30 },
            Stats {
                strength: 6,
                dexterity: 8,
                intelligence: 4,
                constitution: 6,
            },
            CombatStats {
                damage: 3,
                defense: 1,
                accuracy: 70,
                evasion: 15,
            },
        ));

    // Start at main menu   
    next_state.set(GameState::MainMenu);
}

fn main_menu_system(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);

            ui.heading("MYTHS OF ULAN");
            ui.add_space(20.0);
            ui.label("A Retro Fantasy RPG");

            ui.add_space(50.0);

            if ui.button("New Game").clicked() {
                next_state.set(GameState::Playing);
            }

            if ui.button("Load Game").clicked() {
                // TODO: Implement load game
            }

            if ui.button("Quit").clicked() {
                std::process::exit(0);
            }
        });
    });
}

fn player_input_system(
    keyboard: Res<Input<KeyCode>>,
    mut player_commands: EventWriter<PlayerCommand>,
) {
    // Movement keys    
    if keyboard.just_pressed(KeyCode::W) || keyboard.just_pressed(KeyCode::Up) {
        player_commands.send(PlayerCommand::Move { dx: 0, dy: -1 });
    }
    if keyboard.just_pressed(KeyCode::S) || keyboard.just_pressed(KeyCode::Down) {
        player_commands.send(PlayerCommand:: Move { dx: 0, dy: 1 });
    }
    if keyboard.just_pressed(KeyCode::A) || keyboard.just_pressed(KeyCode::Left) {
        player_commands.send(PlayerCommand:: Move { dx: -1, dy: 0 });
    }
    if keyboard.just_pressed(KeyCode::D) || keyboard.just_pressed(KeyCode::Right) {
        player_commands.send(PlayerCommand:: Move { dx: 1, dy: 0 });
    }

    // Wait/Skip turn   
    if keyboard.just_pressed(KeyCode::Space) {
        player_commands.send(PlayerCommand::Wait);
    }
}

fn movement_system(
    mut commands: Commands,
    mut player_commands: EventReader<PlayerCommand>,
    mut player_query: Query<(&mut Position, Entity), With<Player>>,
    monster_query: Query<(&Position, Entity), (With<Monster>, Without<Player>)>,
    mut combat_events: EventWriter<CombatEvent>,
    mut message_log: ResMut<MessageLog>,
) {
    for command in player_commands.read() {
        if let PlayerCommand::Move { dx, dy } = command {
            if let Ok((mut player_pos, player_entity)) = player_query.get_single_mut() {
                let new_x = player_pos.x + dx;
                let new_y = player_pos.y + dy;

                // Check for collision with monsters
                let mut blocked = false;
                for (monster_pos, monster_entity) in monster_query.iter() {
                    if monster_pos.x == new_x && monster_pos.y == new_y && monster_pos.level == player_pos.level {
                        // Initiate combat instead of moving
                        combat_events.send(CombatEvent {
                            attacker: player_entity,
                            defender: monster_entity,
                            damage: 0, // Will be calculated in the combat system
                        });
                        blocked = true;
                        break;
                    }
                }
                // Move if not blocked  
                if !blocked {
                    player_pos.x = new_x;
                    player_pos.y = new_y;
                    message_log.add(format!("You move to ({}, {})", new_x, new_y), Color::WHITE);
                }
            }
        }
    }
}

fn combat_system(
    mut commands: Commands,
    mut combat_events: EventReader<CombatEvent>,
    mut health_query: Query<&mut Health>,
    combat_stats_query: Query<&CombatStats>,
    name_query: Query<&Name>,
    mut message_log: ResMut<MessageLog>,
) {
    for event in combat_events.read() {
        // Get attacker stats   
        if let Ok(attacker_stats) = combat_stats_query.get(event.attacker) {
            if let Ok(defender_stats) = combat_stats_query.get(event.defender) {
                // Simple combat calculation    
                let hit_chance = attacker_stats.accuracy - defender_stats.evasion;
                let hit_roll = rand::random::<i32>() % 100;

                if hit_roll < hit_chance {
                    // Calculate damage
                    let damage = (attacker_stats.damage - defender_stats.defense).max(1);

                    // Apply damage
                    if let Ok(mut defender_health) = health_query.get_mut(event.defender) {
                        defender_health.current -= damage;

                        // Get names for message
                        let attacker_name = name_query.get(event.attacker)
                            .map(|n| n.0.clone())
                            .unwrap_or("Unknown".to_string());
                        let defender_name = name_query.get(event.defender)
                            .map(|n| n.0.clone())
                            .unwrap_or("Unknown".to_string());

                        message_log.add(
                            format!("{} hits {} for {} damage!", attacker_name, defender_name, damage),
                            Color::RED
                        );

                        // Check for death  
                        if defneder_health_current <= 0 {
                            message_log.add(
                                format!("{} has been slain!", defender_name),
                                Color::DARK_GRAY
                            );
                            commands.entity(event.defender).despawn();
                        }
                    }
                } else {
                    let attacker_name = name_query.get(event.attacker)
                        .map(|n| n.0.clone())
                        .unwrap_or("Unknown".to_string());
                    message.add(format!("{} misses!", attacker_name), Color::GRAY);
                }
            }
        }
    }
}

fn ui_system(
    mut contexts: EguiContexts,
    player_query: Query<(&Health, &Stats, &Position), With<Player>>,
    game_world: Res<GameWorld>,
    message_log: Res<MessageLog>,
) {
    // Stats Panel  
    equi::TopBottomPanel::top("stats_panel").show(contexts.ctx_mut(), |ui| {
        if let Ok((health, stats, position)) = player_query.get_single() {
            ui.horizontal(|ui| {
                ui.label(format!("HP: {}/{}", health.current, health.max));
                ui.separator();
                ui.label(format!("STR: {}", stats.strength));
                ui.label(format!("DEX: {}", stats.dexterity));
                ui.label(format!("INT: {}", stats.intelligence));
                ui.label(format!("CON: {}", stats.constitution));
                ui.separator();
                ui.label(format!("Postion: ({}, {})", position.x, position.y));
                ui.label(format!("Level: {}", position.level));
            });
        }
    });

    // Message log  
    egui::TopBottomPanel::bottom("message_log").show(contexts.ctx_mut(), |ui| {
        ui.label("Message Log:");
        egui::ScrollArea::vertical()
            .max_height(150.0)
            .stick_to_bottom(true)
            .show(ui, |ui| {
                for (message, color) in message_log.messages.iter().rev().take(10) {
                    ui.colored_label(egui::Color32::from_rgb(
                            (color.r() * 255.0) as u8,
                            (color.g() * 255.0) as u8,
                            (color.b() * 255.0) as u8,
                            ), message);
                }
            });
    });
}

fn message_log_system(
    mut message_log: ResMut<MessageLog>,
    mut game_world: ResMut<GameWorld>,
) {
    // This system could handle turn counting and other per-frame Update
    game_world.turn_count += 1;
}
