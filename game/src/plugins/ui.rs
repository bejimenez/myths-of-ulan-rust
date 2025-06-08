// src/plugins/ui.rs
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::collections::HashMap;

use crate::components::{Health, Monster, Player, Position, Stats};
use crate::game_state::GameState;
use crate::resources::MessageLog;

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
    player_query: Query<(&Health, &Stats, &Position), With<Player>>,
    entities_query: Query<(&Position, Option<&Player>, Option<&Monster>)>,
    message_log: Res<MessageLog>,
) {
    egui::TopBottomPanel::top("stats_panel").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            if let Ok((health, stats, position)) = player_query.get_single() {
                ui.label(format!("HP: {}/{}", health.current, health.max));
                ui.separator();
                ui.label(format!("STR: {} | DEX: {} | INT: {} | CON: {}", stats.strength, stats.dexterity, stats.intelligence, stats.constitution));
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

    egui::TopBottomPanel::bottom("message_log").resizable(true).min_height(100.0).default_height(150.0).show(contexts.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            ui.label("Message Log");
            ui.separator();
            egui::ScrollArea::vertical().auto_shrink([false; 2]).stick_to_bottom(true).show(ui, |ui| {
                for (message, color) in message_log.messages.iter() {
                    let egui_color = egui::Color32::from_rgb((color.r() * 255.0) as u8, (color.g() * 255.0) as u8, (color.b() * 255.0) as u8);
                    ui.add(egui::Label::new(egui::RichText::new(message).color(egui_color)).wrap(true));
                }
            });
        });
    });

    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        if let Ok((_, _, player_pos)) = player_query.get_single() {
            let mut map_chars: HashMap<(i32, i32), char> = HashMap::new();
            for (pos, is_player, is_monster) in entities_query.iter() {
                if pos.level == player_pos.level {
                    map_chars.insert((pos.x, pos.y), if is_player.is_some() { '@' } else if is_monster.is_some() { 'g' } else { '?' });
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