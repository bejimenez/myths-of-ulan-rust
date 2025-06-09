// src/plugins/ui.rs - ENHANCED WITH MONSTER DEBUG PANEL

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::collections::HashMap;

use crate::components::{Health, Monster, Player, Position, Stats, CombatStats, Name};
use crate::game_state::GameState;
use crate::resources::MessageLog;
use crate::plugins::combat::CurrentCombat;
use crate::templates::monster_templates::MonsterTemplateRef;

const MAP_WIDTH: i32 = 35;
const MAP_HEIGHT: i32 = 25;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, main_menu_system.run_if(in_state(GameState::MainMenu)))
            .add_systems(
                Update,
                ui_system.run_if(in_state(GameState::Exploring).or_else(in_state(GameState::InCombat)))
            );
    }
}

fn main_menu_system(mut contexts: EguiContexts, mut next_state: ResMut<NextState<GameState>>) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            ui.heading("MYTHS OF ULAN");
            ui.add_space(50.0);
            if ui.button("New Game").clicked() {
                next_state.set(GameState::Exploring);
            }
            if ui.button("Quit").clicked() {
                std::process::exit(0);
            }
        });
    });
}

fn ui_system(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    player_query: Query<(&Health, &Stats, &Position, &CombatStats), With<Player>>,
    entities_query: Query<(&Position, Option<&Player>, Option<&Monster>)>,
    message_log: Res<MessageLog>,
    game_state: Res<State<GameState>>,
    current_combat: Res<CurrentCombat>,
    monster_query: Query<(&Name, &Health, &Stats, &CombatStats, Option<&MonsterTemplateRef>), With<Monster>>,
) {
    // Top panel - Player stats
    egui::TopBottomPanel::top("stats_panel").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            if let Ok((health, stats, position, combat)) = player_query.get_single() {
                ui.label(format!("HP: {}/{}", health.current, health.max));
                ui.separator();
                ui.label(format!("STR: {} | DEX: {} | INT: {} | CON: {}", 
                    stats.strength, stats.dexterity, stats.intelligence, stats.constitution));
                ui.separator();
                ui.label(format!("DMG: {} | DEF: {} | ACC: {} | EVA: {}", 
                    combat.damage, combat.defense, combat.accuracy, combat.evasion));
                ui.separator();
                ui.label(format!("Pos: ({}, {})", position.x, position.y));
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Quit to Menu").clicked() {
                    next_state.set(GameState::MainMenu);
                }
            });
        });
    });

    // Bottom panel - Message log
    egui::TopBottomPanel::bottom("message_log")
        .resizable(true)
        .min_height(100.0)
        .default_height(150.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.label("Message Log");
                ui.separator();
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for (message, color) in message_log.messages.iter() {
                            let egui_color = egui::Color32::from_rgb(
                                (color.r() * 255.0) as u8,
                                (color.g() * 255.0) as u8,
                                (color.b() * 255.0) as u8
                            );
                            ui.add(egui::Label::new(egui::RichText::new(message).color(egui_color)).wrap(true));
                        }
                    });
            });
        });

    // Side panel - Monster Debug Info (only in combat)
    if game_state.get() == &GameState::InCombat {
        egui::SidePanel::right("monster_debug_panel")
            .default_width(300.0)
            .resizable(true)
            .show(contexts.ctx_mut(), |ui| {
                ui.heading("Monster Debug Info");
                ui.separator();
                
                if let Some(monster_entity) = current_combat.monster_entity {
                    if let Ok((name, health, stats, combat, template_ref)) = monster_query.get(monster_entity) {
                        // Basic info
                        ui.group(|ui| {
                            ui.label(egui::RichText::new(&name.0).size(16.0).strong());
                            if let Some(template_ref) = template_ref {
                                ui.label(format!("Template: {}", template_ref.0));
                            }
                            ui.label(format!("Entity ID: {:?}", monster_entity));
                        });
                        
                        ui.add_space(10.0);
                        
                        // Health
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Health").strong());
                            ui.horizontal(|ui| {
                                ui.label("Current:");
                                ui.label(egui::RichText::new(health.current.to_string()).color(egui::Color32::RED));
                                ui.label("/");
                                ui.label(egui::RichText::new(health.max.to_string()).color(egui::Color32::GREEN));
                            });
                            
                            // Health bar
                            let health_ratio = health.current as f32 / health.max as f32;
                            ui.add(egui::ProgressBar::new(health_ratio)
                                .text(format!("{:.0}%", health_ratio * 100.0))
                                .fill(egui::Color32::from_rgb(200, 50, 50)));
                        });
                        
                        ui.add_space(10.0);
                        
                        // Base Stats
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Base Stats").strong());
                            egui::Grid::new("monster_stats_grid")
                                .num_columns(2)
                                .spacing([20.0, 4.0])
                                .show(ui, |ui| {
                                    ui.label("Strength:");
                                    ui.label(stats.strength.to_string());
                                    ui.end_row();
                                    
                                    ui.label("Dexterity:");
                                    ui.label(stats.dexterity.to_string());
                                    ui.end_row();
                                    
                                    ui.label("Intelligence:");
                                    ui.label(stats.intelligence.to_string());
                                    ui.end_row();
                                    
                                    ui.label("Constitution:");
                                    ui.label(stats.constitution.to_string());
                                    ui.end_row();
                                });
                        });
                        
                        ui.add_space(10.0);
                        
                        // Combat Stats
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Combat Stats").strong());
                            egui::Grid::new("monster_combat_grid")
                                .num_columns(2)
                                .spacing([20.0, 4.0])
                                .show(ui, |ui| {
                                    ui.label("Damage:");
                                    ui.label(egui::RichText::new(combat.damage.to_string()).color(egui::Color32::LIGHT_RED));
                                    ui.end_row();
                                    
                                    ui.label("Defense:");
                                    ui.label(egui::RichText::new(combat.defense.to_string()).color(egui::Color32::LIGHT_BLUE));
                                    ui.end_row();
                                    
                                    ui.label("Accuracy:");
                                    ui.label(format!("{}%", combat.accuracy));
                                    ui.end_row();
                                    
                                    ui.label("Evasion:");
                                    ui.label(format!("{}%", combat.evasion));
                                    ui.end_row();
                                });
                        });
                        
                        ui.add_space(10.0);
                        
                        // Combat calculations helper
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Combat Calculations vs Player").strong());
                            if let Ok((player_health, player_stats, _, player_combat)) = player_query.get_single() {
                                ui.separator();
                                
                                // Monster attacking player
                                ui.label(egui::RichText::new("Monster → Player:").underline());
                                let m_hit_chance = combat.accuracy - player_combat.evasion;
                                let m_damage = (combat.damage - player_combat.defense).max(1);
                                ui.label(format!("Hit Chance: {}% - {}% = {}%", 
                                    combat.accuracy, player_combat.evasion, m_hit_chance));
                                ui.label(format!("Damage: {} - {} = {}", 
                                    combat.damage, player_combat.defense, m_damage));
                                ui.label(format!("Turns to kill: ~{}", 
                                    (player_health.current as f32 / m_damage as f32).ceil() as i32));
                                
                                ui.separator();
                                
                                // Player attacking monster
                                ui.label(egui::RichText::new("Player → Monster:").underline());
                                let p_hit_chance = player_combat.accuracy - combat.evasion;
                                let p_damage = (player_combat.damage - combat.defense).max(1);
                                ui.label(format!("Hit Chance: {}% - {}% = {}%", 
                                    player_combat.accuracy, combat.evasion, p_hit_chance));
                                ui.label(format!("Damage: {} - {} = {}", 
                                    player_combat.damage, combat.defense, p_damage));
                                ui.label(format!("Turns to kill: ~{}", 
                                    (health.current as f32 / p_damage as f32).ceil() as i32));
                            }
                        });
                    } else {
                        ui.label("Monster data not found!");
                    }
                } else {
                    ui.label("No monster in combat");
                }
            });
    }

    // Central panel - Map view
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        if let Ok((_, _, player_pos, _)) = player_query.get_single() {
            let mut map_chars: HashMap<(i32, i32), char> = HashMap::new();
            for (pos, is_player, is_monster) in entities_query.iter() {
                if pos.level == player_pos.level {
                    map_chars.insert(
                        (pos.x, pos.y),
                        if is_player.is_some() { '@' } 
                        else if is_monster.is_some() { 'g' } 
                        else { '?' }
                    );
                }
            }
            ui.vertical_centered(|ui| {
                let font = egui::FontId::monospace(14.0);
                for y in 0..MAP_HEIGHT {
                    let mut row_string = String::new();
                    for x in 0..MAP_WIDTH {
                        let world_x = player_pos.x + x - MAP_WIDTH / 2;
                        let world_y = player_pos.y - (y - MAP_HEIGHT / 2);
                        row_string.push(*map_chars.get(&(world_x, world_y)).unwrap_or(&'.'));
                        row_string.push(' ');
                    }
                    ui.label(egui::RichText::new(row_string).font(font.clone()));
                }
            });
        }
    });
}