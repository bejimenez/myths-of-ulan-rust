// src/data/templates/loot_tables.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootTable {
    pub id: String,
    pub name: String,
    pub rolls: LootRolls,
    pub entries: Vec<LootEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootRolls {
    pub min: u32,
    pub max: u32,
    pub bonus_rolls: Option<BonusRolls>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BonusRolls {
    pub per_level: f32,
    pub per_luck: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootEntry {
    pub weight: f32,
    pub item: LootItem,
    pub conditions: Vec<LootCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LootItem {
    Item { 
        id: String, 
        quantity: QuantityRange,
        #[serde(skip_serializing_if = "Option::is_none")]
        modifiers: Option<Vec<String>>,
    },
    Table { 
        id: String 
    },
    Gold { 
        amount: QuantityRange 
    },
    Experience { 
        amount: QuantityRange 
    },
    Nothing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantityRange {
    pub min: u32,
    pub max: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LootCondition {
    MinLevel { level: u32 },
    MaxLevel { level: u32 },
    Random { chance: f32 },
    KilledByElement { element: String },
    PlayerHasItem { item_id: String },
}

#[derive(Resource, Default)]
pub struct LootTableRegistry {
    tables: HashMap<String, LootTable>,
}

impl LootTableRegistry {
    pub fn register(&mut self, table: LootTable) {
        self.tables.insert(table.id.clone(), table);
    }
    
    pub fn get(&self, id: &str) -> Option<&LootTable> {
        self.tables.get(id)
    }
    
    pub fn count(&self) -> usize {
        self.tables.len()
    }
    
    pub fn roll_loot(&self, table_id: &str, level: u32, luck: f32) -> Vec<LootResult> {
        let mut results = Vec::new();
        
        if let Some(table) = self.get(table_id) {
            // Calculate number of rolls
            let base_rolls = rand::random::<u32>() % (table.rolls.max - table.rolls.min + 1) + table.rolls.min;
            let bonus_rolls = if let Some(bonus) = &table.rolls.bonus_rolls {
                let level_bonus = (level as f32 * bonus.per_level) as u32;
                let luck_bonus = (luck * bonus.per_luck) as u32;
                level_bonus + luck_bonus
            } else {
                0
            };
            
            let total_rolls = base_rolls + bonus_rolls;
            
            // Perform rolls
            for _ in 0..total_rolls {
                if let Some(entry) = self.weighted_random_select(&table.entries, level) {
                    match &entry.item {
                        LootItem::Item { id, quantity, .. } => {
                            let qty = rand::random::<u32>() % (quantity.max - quantity.min + 1) + quantity.min;
                            results.push(LootResult::Item { 
                                item_id: id.clone(), 
                                quantity: qty 
                            });
                        }
                        LootItem::Table { id } => {
                            // Recursive roll
                            results.extend(self.roll_loot(id, level, luck));
                        }
                        LootItem::Gold { amount } => {
                            let qty = rand::random::<u32>() % (amount.max - amount.min + 1) + amount.min;
                            results.push(LootResult::Gold { amount: qty });
                        }
                        LootItem::Experience { amount } => {
                            let qty = rand::random::<u32>() % (amount.max - amount.min + 1) + amount.min;
                            results.push(LootResult::Experience { amount: qty });
                        }
                        LootItem::Nothing => {}
                    }
                }
            }
        }
        
        results
    }
    
    fn weighted_random_select<'a>(&self, entries: &'a [LootEntry], level: u32) -> Option<&'a LootEntry> {
        let eligible_entries: Vec<(&LootEntry, f32)> = entries.iter()
            .filter_map(|entry| {
                // Check conditions
                for condition in &entry.conditions {
                    match condition {
                        LootCondition::MinLevel { level: min } => {
                            if level < *min { return None; }
                        }
                        LootCondition::MaxLevel { level: max } => {
                            if level > *max { return None; }
                        }
                        LootCondition::Random { chance } => {
                            if rand::random::<f32>() > *chance { return None; }
                        }
                        _ => {} // Other conditions need more context
                    }
                }
                Some((entry, entry.weight))
            })
            .collect();
        
        if eligible_entries.is_empty() {
            return None;
        }
        
        let total_weight: f32 = eligible_entries.iter().map(|(_, w)| w).sum();
        let mut roll = rand::random::<f32>() * total_weight;
        
        for (entry, weight) in eligible_entries {
            roll -= weight;
            if roll <= 0.0 {
                return Some(entry);
            }
        }
        
        None
    }
}

#[derive(Debug, Clone)]
pub enum LootResult {
    Item { item_id: String, quantity: u32 },
    Gold { amount: u32 },
    Experience { amount: u32 },
}