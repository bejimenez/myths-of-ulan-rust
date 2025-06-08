// src/data/templates/item_templates.rs
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ItemTemplate {
    Weapon(WeaponTemplate),
    Armor(ArmorTemplate),
    Consumable(ConsumableTemplate),
    Misc(MiscItemTemplate),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub weapon_type: WeaponType,
    pub damage: DamageRange,
    pub attack_speed: f32,
    pub requirements: ItemRequirements,
    pub modifiers: Vec<StatModifier>,
    pub rarity: ItemRarity,
    pub value: u32,
    pub stack_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmorTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub armor_type: ArmorType,
    pub defense: i32,
    pub requirements: ItemRequirements,
    pub modifiers: Vec<StatModifier>,
    pub rarity: ItemRarity,
    pub value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub consumable_type: ConsumableType,
    pub effects: Vec<ConsumableEffect>,
    pub charges: u32,
    pub cooldown: f32,
    pub value: u32,
    pub stack_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscItemTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub value: u32,
    pub stack_size: u32,
    pub quest_item: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageRange {
    pub min: i32,
    pub max: i32,
    pub damage_type: DamageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Axe,
    Mace,
    Dagger,
    Staff,
    Bow,
    Crossbow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArmorType {
    Light,
    Medium,
    Heavy,
    Shield,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DamageType {
    Physical,
    Fire,
    Cold,
    Lightning,
    Poison,
    Holy,
    Shadow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsumableType {
    Potion,
    Scroll,
    Food,
    Elixir,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsumableEffect {
    Heal { amount: i32 },
    RestoreMana { amount: i32 },
    Buff { stat: String, amount: i32, duration: f32 },
    CurePoison,
    RemoveCurse,
    Teleport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRequirements {
    pub level: Option<u32>,
    pub strength: Option<i32>,
    pub dexterity: Option<i32>,
    pub intelligence: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatModifier {
    pub stat: String,
    pub modifier_type: ModifierType,
    pub value: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModifierType {
    Flat,
    Percentage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Resource, Default)]
pub struct ItemTemplateRegistry {
    items: HashMap<String, ItemTemplate>,
}

impl ItemTemplateRegistry {
    pub fn register(&mut self, template: ItemTemplate) {
        let id = match &template {
            ItemTemplate::Weapon(w) => w.id.clone(),
            ItemTemplate::Armor(a) => a.id.clone(),
            ItemTemplate::Consumable(c) => c.id.clone(),
            ItemTemplate::Misc(m) => m.id.clone(),
        };
        self.items.insert(id, template);
    }
    
    pub fn get(&self, id: &str) -> Option<&ItemTemplate> {
        self.items.get(id)
    }
    
    pub fn count(&self) -> usize {
        self.items.len()
    }
}