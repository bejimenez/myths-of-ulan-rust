// src/components.rs
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Monster {
    pub ai_type: AIType,
}

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component, Clone, PartialEq, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub level: i32,
}

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct Stats {
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub constitution: i32,
}

#[derive(Component)]
pub struct CombatStats {
    pub damage: i32,
    pub defense: i32,
    pub accuracy: i32,
    pub evasion: i32,
}

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<Entity>,
    pub capacity: usize,
}

#[derive(Component)]
pub struct Item {
    pub item_type: ItemType,
    pub stack_size: u32,
}

// --- Helper Enums ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIType {
    Aggressive,
    Defensive,
    Passive,
}

#[derive(Debug, Clone)]
pub enum ItemType {
    Weapon { damage: i32 },
    Armor { defense: i32 },
    Potion { heal_amount: i32 },
    Gold { amount: u32 },
}