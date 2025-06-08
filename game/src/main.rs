use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use std::collections::HashMap;

// Map display constants
const MAP_WIDTH: i32 = 35;
const MAP_HEIGHT: i32 = 25;

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
        .add_plugins(EguiPlugin)
        // Game States
        .init_state::<GameState>()
        // Resources
        .init_resource::<GameWorld>()
        .init_resource::<MessageLog>()
        // Events
        .add_event::<CombatEvent>()
        .add_event::<PlayerCommand>()
        // Startup Systems
        .add_systems(Startup, setup_game)
        // Update Systems - Menu State
        .add_systems(Update, main_menu_system.run_if(in_state(GameState::MainMenu)))
        // Update Systems - Playing State
        .add_systems(
            Update,
            (
                player_input_system,
                movement_system,
                combat_system,
                ui_system, // This system is now updated
            )
                .chain() // Ensures systems run in order
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}

// ===== GAME STATES =====
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

// ===== COMPONENTS =====
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
struct Health {
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

// ===== ENUMS & TYPES =====
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

// ===== RESOURCES =====
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

// ===== EVENTS =====
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

// ===== SYSTEMS =====
// Functions that operate on entities with specific components

fn setup_game(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut message_log: ResMut<MessageLog>,
) {
    // Spawn the player entity
    commands.spawn((
        Player,
        Name("Hero".to_string()),
        Position {
            x: 0,
            y: 0,
            level: 1,
        },
        Health {
            current: 100,
            max: 100,
        },
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
            capacity: 20,
        },
    ));

    // Spawn a test monster
    commands.spawn((
        Monster {
            ai_type: AIType::Aggressive,
        },
        Name("Goblin".to_string()),
        Position {
            x: 5,
            y: 5,
            level: 1,
        },
        Health {
            current: 30,
            max: 30,
        },
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

    // Add initial message
    message_log.add(
        "Welcome to Myths of Ulan! Use WASD or arrow keys to move.".to_string(),
        Color::LIME_GREEN,
    );
    message_log.add(
        "A goblin lurks at position (5, 5). Approach carefully!".to_string(),
        Color::YELLOW,
    );

    // Start at main menu
    next_state.set(GameState::MainMenu);
}

fn main_menu_system(mut contexts: EguiContexts, mut next_state: ResMut<NextState<GameState>>) {
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
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_commands: EventWriter<PlayerCommand>,
) {
    // Movement keys
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

    // Wait/Skip turn
    if keyboard.just_pressed(KeyCode::Space) {
        player_commands.send(PlayerCommand::Wait);
    }
}

fn movement_system(
    mut commands: Commands,
    mut player_commands: EventReader<PlayerCommand>,
    mut player_query: Query<(&mut Position, Entity, &Name), With<Player>>,
    monster_query: Query<(&Position, Entity, &Name), (With<Monster>, Without<Player>)>,
    mut combat_events: EventWriter<CombatEvent>,
    mut message_log: ResMut<MessageLog>,
    mut game_world: ResMut<GameWorld>,
) {
    for command in player_commands.read() {
        match command {
            PlayerCommand::Move { dx, dy } => {
                if let Ok((mut player_pos, player_entity, _player_name)) =
                    player_query.get_single_mut()
                {
                    let new_x = player_pos.x + dx;
                    let new_y = player_pos.y + dy;

                    // Check for collision with monsters
                    let mut blocked = false;
                    for (monster_pos, monster_entity, monster_name) in monster_query.iter() {
                        if monster_pos.x == new_x
                            && monster_pos.y == new_y
                            && monster_pos.level == player_pos.level
                        {
                            // Monster encountered!
                            message_log.add(
                                format!("You bump into the {}!", monster_name.0),
                                Color::YELLOW,
                            );

                            // Initiate combat instead of moving
                            combat_events.send(CombatEvent {
                                attacker: player_entity,
                                defender: monster_entity,
                                damage: 0, // Will be calculated in combat system
                            });
                            blocked = true;
                            break;
                        }
                    }

                    // TODO: Check for wall collisions when we have dungeon generation

                    // Move if not blocked
                    if !blocked {
                        player_pos.x = new_x;
                        player_pos.y = new_y;

                        // Only increment turn count on successful actions
                        game_world.turn_count += 1;
                    }
                }
            }
            PlayerCommand::Wait => {
                game_world.turn_count += 1;
                message_log.add(
                    format!("Turn {}: You wait.", game_world.turn_count),
                    Color::GRAY,
                );
            }
            _ => {} // Handle other commands as needed
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
                        let attacker_name = name_query
                            .get(event.attacker)
                            .map(|n| n.0.clone())
                            .unwrap_or("Unknown".to_string());
                        let defender_name = name_query
                            .get(event.defender)
                            .map(|n| n.0.clone())
                            .unwrap_or("Unknown".to_string());

                        message_log.add(
                            format!(
                                "{} hits {} for {} damage!",
                                attacker_name, defender_name, damage
                            ),
                            Color::RED,
                        );

                        // Check for death
                        if defender_health.current <= 0 {
                            message_log.add(
                                format!("{} has been slain!", defender_name),
                                Color::DARK_GRAY,
                            );
                            commands.entity(event.defender).despawn();
                        }
                    }
                } else {
                    let attacker_name = name_query
                        .get(event.attacker)
                        .map(|n| n.0.clone())
                        .unwrap_or("Unknown".to_string());
                    message_log.add(format!("{} misses!", attacker_name), Color::GRAY);
                }
            }
        }
    }
}


// MODIFIED SYSTEM
fn ui_system(
    mut contexts: EguiContexts,
    // We now need two queries: one just for the player's position to center the map,
    // and another for all drawable entities on the map.
    player_query: Query<(&Health, &Stats, &Position), With<Player>>,
    entities_query: Query<(&Position, Option<&Player>, Option<&Monster>)>,
    game_world: Res<GameWorld>,
    message_log: Res<MessageLog>,
) {
    // --- Top Stats Panel (Unchanged) ---
    egui::TopBottomPanel::top("stats_panel").show(contexts.ctx_mut(), |ui| {
        if let Ok((health, stats, position)) = player_query.get_single() {
            ui.horizontal(|ui| {
                ui.label(format!("HP: {}/{}", health.current, health.max));
                ui.separator();
                ui.label(format!("STR: {}", stats.strength));
                ui.label(format!("DEX: {}", stats.dexterity));
                ui.label(format!("INT: {}", stats.intelligence));
                ui.label(format!("CON: {}", stats.constitution));
                ui.separator();
                ui.label(format!("Position: ({}, {})", position.x, position.y));
                ui.label(format!("Level: {}", position.level));
            });
        }
    });

    // --- Bottom Message Log (Unchanged) ---
    egui::TopBottomPanel::bottom("message_log")
        .resizable(true)
        .min_height(200.0)
        .default_height(250.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.label("Message Log");
                ui.separator();

                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        let message_count = message_log.messages.len();
                        let start_index = if message_count > 50 { message_count - 50 } else { 0 };

                        for (message, color) in message_log.messages[start_index..].iter() {
                            ui.colored_label(
                                egui::Color32::from_rgb(
                                    (color.r() * 255.0) as u8,
                                    (color.g() * 255.0) as u8,
                                    (color.b() * 255.0) as u8,
                                ),
                                message,
                            );
                        }
                    });
            });
        });

    // --- NEW: Central Map Panel ---
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        // We need the player's position to center the map. If the player doesn't exist,
        // we can't draw the map.
        if let Ok((_, _, player_pos)) = player_query.get_single() {
            // Use a HashMap to store the characters for our entities. This is more
            // efficient than a full 2D array if the map is sparse.
            // The key is the (x, y) world coordinate, and the value is the character to draw.
            let mut map_chars: HashMap<(i32, i32), char> = HashMap::new();

            // Populate the map with entities.
            for (pos, is_player, is_monster) in entities_query.iter() {
                // Only draw entities on the same dungeon level as the player.
                if pos.level == player_pos.level {
                    let char = if is_player.is_some() {
                        '@' // Player
                    } else if is_monster.is_some() {
                        'g' // Goblin
                    } else {
                        '?' // Unknown entity
                    };
                    map_chars.insert((pos.x, pos.y), char);
                }
            }

            // Center the map content.
            ui.vertical_centered(|ui| {
                ui.label(format!("-- Dungeon Level {} --", player_pos.level));
                ui.add_space(10.0);

                // Use a fixed-width font for proper alignment. This is CRITICAL.
                let font = egui::FontId::monospace(14.0);
                
                // Draw the map grid.
                for y in 0..MAP_HEIGHT {
                    let mut row_string = String::new();
                    for x in 0..MAP_WIDTH {
                        // Calculate the world coordinates for this screen cell.
                        // The center of our view (MAP_WIDTH/2, MAP_HEIGHT/2) corresponds
                        // to the player's current position.
                        // Y is inverted because in games, positive Y is usually "up",
                        // but in UI, positive Y is "down".
                        let world_x = player_pos.x + x - MAP_WIDTH / 2;
                        let world_y = player_pos.y - (y - MAP_HEIGHT / 2);

                        // Get the character from our map, or a floor tile '.' if nothing is there.
                        let char_to_draw = map_chars.get(&(world_x, world_y)).unwrap_or(&'.');
                        row_string.push(*char_to_draw);
                        row_string.push(' '); // Add a space for better readability
                    }
                    // Display the completed row.
                    ui.label(egui::RichText::new(row_string).font(font.clone()));
                }
            });
        }
    });
}
