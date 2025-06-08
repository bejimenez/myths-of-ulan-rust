// src/resources.rs
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameWorld {
    pub current_dungeon: Option<String>,
    pub dungeon_level: i32,
    pub turn_count: u32,
}

#[derive(Resource, Default)]
pub struct MessageLog {
    pub messages: Vec<(String, Color)>,
}

impl MessageLog {
    pub fn add(&mut self, message: String, color: Color) {
        self.messages.push((message, color));
        if self.messages.len() > 100 {
            self.messages.remove(0);
        }
    }
}