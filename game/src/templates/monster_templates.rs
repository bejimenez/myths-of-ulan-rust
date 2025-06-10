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
    
    /// Monster family (e.g., Humanoid, Beast, Undead, Elemental)
    pub family: MonsterFamily,
    
    /// Monster type (e.g., Goblin, Wolf, Skeleton)
    #[serde(rename = "type")]
    pub monster_type: String,
    
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
    pub display_color: [f32; 3],
}

/// Monster families for categorization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MonsterFamily {
    Humanoid,
    Beast,
    Undead,
    Elemental,
    Demon,
    Dragon,
    Construct,
    Aberration,
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
    // Index by monster type for efficient filtering
    by_type: HashMap<String, Vec<String>>,
    // Index by family for efficient filtering
    by_family: HashMap<MonsterFamily, Vec<String>>,
}

impl MonsterTemplateRegistry {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
            by_type: HashMap::new(),
            by_family: HashMap::new(),
        }
    }
    
    pub fn register(&mut self, template: MonsterTemplate) {
        // Add to type index
        self.by_type
            .entry(template.monster_type.clone())
            .or_insert_with(Vec::new)
            .push(template.id.clone());
            
        // Add to family index
        self.by_family
            .entry(template.family.clone())
            .or_insert_with(Vec::new)
            .push(template.id.clone());
            
        // Add to main registry
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
    
    /// Get all templates of a specific monster type
    pub fn get_templates_by_type(&self, monster_type: &str) -> Vec<&MonsterTemplate> {
        self.by_type
            .get(monster_type)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.templates.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Get all templates of a specific family
    pub fn get_templates_by_family(&self, family: &MonsterFamily) -> Vec<&MonsterTemplate> {
        self.by_family
            .get(family)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.templates.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Get templates by type within a level range
    pub fn get_templates_by_type_and_level(&self, monster_type: &str, min_level: i32, max_level: i32) -> Vec<&MonsterTemplate> {
        self.get_templates_by_type(monster_type)
            .into_iter()
            .filter(|template| {
                template.level_range.0 <= max_level && template.level_range.1 >= min_level
            })
            .collect()
    }
    
    /// Get a random template of a specific type within level range
    pub fn get_random_template_by_type(&self, monster_type: &str, min_level: i32, max_level: i32) -> Option<&MonsterTemplate> {
        let candidates = self.get_templates_by_type_and_level(monster_type, min_level, max_level);
        if candidates.is_empty() {
            None
        } else {
            let index = rand::random::<usize>() % candidates.len();
            Some(candidates[index])
        }
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