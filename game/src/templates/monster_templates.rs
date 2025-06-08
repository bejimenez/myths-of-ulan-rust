// src/templates/monster_templates.rs
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::components::{AIType, CombatStats, Health, Stats, Monster, Name, Position};

/// A template that defines the base properties for a type of monster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterTemplate {
    /// Unique identifier for this monster type
    pub id: String,
    
    /// Display name for the monster
    pub name: String,
    
    /// Base health values
    pub health: HealthTemplate,
    
    /// Base stat values
    pub stats: StatsTemplate,
    
    /// Combat-related stats
    pub combat: CombatStatsTemplate,
    
    /// AI behavior type
    pub ai_type: AIType,
    
    /// Level range where this monster can spawn
    pub level_range: (i32, i32),
    
    /// Base experience reward
    pub experience_reward: u32,
    
    /// Loot table reference (for future implementation)
    pub loot_table_id: Option<String>,
    
    /// Visual representation
    pub display_char: char,
    pub display_color: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthTemplate {
    pub base_health: i32,
    pub health_per_level: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsTemplate {
    pub base_strength: i32,
    pub base_dexterity: i32,
    pub base_intelligence: i32,
    pub base_constitution: i32,
    
    // Stats gain per level
    pub strength_per_level: f32,
    pub dexterity_per_level: f32,
    pub intelligence_per_level: f32,
    pub constitution_per_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatStatsTemplate {
    pub base_damage: i32,
    pub base_defense: i32,
    pub base_accuracy: i32,
    pub base_evasion: i32,
    
    // Combat stats scaling
    pub damage_per_level: f32,
    pub defense_per_level: f32,
}

/// Resource that holds all monster templates
#[derive(Resource, Default)]
pub struct MonsterTemplateRegistry {
    templates: HashMap<String, MonsterTemplate>,
}

impl MonsterTemplateRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            templates: HashMap::new(),
        };
        
        // Register default templates
        registry.register_default_templates();
        registry
    }
    
    pub fn register(&mut self, template: MonsterTemplate) {
        self.templates.insert(template.id.clone(), template);
    }
    
    pub fn get(&self, id: &str) -> Option<&MonsterTemplate> {
        self.templates.get(id)
    }
    
    /// Get all templates that can spawn within the given level range
    pub fn get_templates_for_level_range(&self, min_level: i32, max_level: i32) -> Vec<&MonsterTemplate> {
        self.templates.values()
            .filter(|template| {
                template.level_range.0 <= max_level && template.level_range.1 >= min_level
            })
            .collect()
    }
    
    /// Get all template IDs
    pub fn get_all_template_ids(&self) -> Vec<&String> {
        self.templates.keys().collect()
    }
    
    /// Get all templates
    pub fn get_all_templates(&self) -> impl Iterator<Item = &MonsterTemplate> {
        self.templates.values()
    }
    
    /// Get the number of registered templates
    pub fn count(&self) -> usize {
        self.templates.len()
    }
    
    fn register_default_templates(&mut self) {
        // Basic Goblin
        self.register(MonsterTemplate {
            id: "goblin".to_string(),
            name: "Goblin".to_string(),
            health: HealthTemplate {
                base_health: 30,
                health_per_level: 5,
            },
            stats: StatsTemplate {
                base_strength: 6,
                base_dexterity: 8,
                base_intelligence: 4,
                base_constitution: 6,
                strength_per_level: 0.5,
                dexterity_per_level: 1.0,
                intelligence_per_level: 0.25,
                constitution_per_level: 0.5,
            },
            combat: CombatStatsTemplate {
                base_damage: 3,
                base_defense: 1,
                base_accuracy: 70,
                base_evasion: 15,
                damage_per_level: 0.5,
                defense_per_level: 0.25,
            },
            ai_type: AIType::Aggressive,
            level_range: (1, 5),
            experience_reward: 10,
            loot_table_id: Some("goblin_loot".to_string()),
            display_char: 'g',
            display_color: Color::rgb(0.0, 0.7, 0.0),
        });
        
        // Raging Goblin - Enhanced variant
        self.register(MonsterTemplate {
            id: "raging_goblin".to_string(),
            name: "Raging Goblin".to_string(),
            health: HealthTemplate {
                base_health: 45,
                health_per_level: 8,
            },
            stats: StatsTemplate {
                base_strength: 10,
                base_dexterity: 10,
                base_intelligence: 3,
                base_constitution: 8,
                strength_per_level: 1.0,
                dexterity_per_level: 0.75,
                intelligence_per_level: 0.1,
                constitution_per_level: 0.75,
            },
            combat: CombatStatsTemplate {
                base_damage: 6,
                base_defense: 2,
                base_accuracy: 80,
                base_evasion: 10,
                damage_per_level: 1.0,
                defense_per_level: 0.5,
            },
            ai_type: AIType::Aggressive,
            level_range: (3, 8),
            experience_reward: 25,
            loot_table_id: Some("goblin_loot".to_string()),
            display_char: 'G',
            display_color: Color::rgb(0.8, 0.2, 0.2),
        });
    }
}

/// System for spawning monsters from templates
pub fn spawn_monster_from_template(
    commands: &mut Commands,
    registry: &MonsterTemplateRegistry,
    template_id: &str,
    position: Position,
    level: Option<i32>,
) -> Option<Entity> {
    let template = registry.get(template_id)?;
    
    // Calculate the monster's level
    let monster_level = level.unwrap_or_else(|| {
        if template.level_range.0 == template.level_range.1 {
            template.level_range.0
        } else {
            rand::random::<i32>().abs() % (template.level_range.1 - template.level_range.0 + 1) + template.level_range.0
        }
    });
    
    // Calculate scaled stats
    let health = Health {
        current: template.health.base_health + (template.health.health_per_level * (monster_level - 1)),
        max: template.health.base_health + (template.health.health_per_level * (monster_level - 1)),
    };
    
    let stats = Stats {
        strength: template.stats.base_strength + ((template.stats.strength_per_level * (monster_level - 1) as f32) as i32),
        dexterity: template.stats.base_dexterity + ((template.stats.dexterity_per_level * (monster_level - 1) as f32) as i32),
        intelligence: template.stats.base_intelligence + ((template.stats.intelligence_per_level * (monster_level - 1) as f32) as i32),
        constitution: template.stats.base_constitution + ((template.stats.constitution_per_level * (monster_level - 1) as f32) as i32),
    };
    
    let combat_stats = CombatStats {
        damage: template.combat.base_damage + ((template.combat.damage_per_level * (monster_level - 1) as f32) as i32),
        defense: template.combat.base_defense + ((template.combat.defense_per_level * (monster_level - 1) as f32) as i32),
        accuracy: template.combat.base_accuracy,
        evasion: template.combat.base_evasion,
    };
    
    let entity = commands.spawn((
        Monster { ai_type: template.ai_type.clone() },
        Name(format!("{} (Lv.{})", template.name, monster_level)),
        position,
        health,
        stats,
        combat_stats,
        // Add a component to track the template this monster came from
        MonsterTemplateRef(template.id.clone()),
    )).id();
    
    Some(entity)
}

/// Component that references which template a monster was created from
#[derive(Component)]
pub struct MonsterTemplateRef(pub String);

// Optional: Load templates from JSON files
impl MonsterTemplateRegistry {
    pub fn load_from_json(&mut self, json_data: &str) -> Result<(), serde_json::Error> {
        let templates: Vec<MonsterTemplate> = serde_json::from_str(json_data)?;
        for template in templates {
            self.register(template);
        }
        Ok(())
    }
}