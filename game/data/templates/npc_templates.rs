// src/data/templates/npc_templates.rs
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCTemplate {
    pub id: String,
    pub name: String,
    pub title: Option<String>,
    pub description: String,
    pub npc_type: NPCType,
    pub dialogue_personality: DialoguePersonality,
    pub services: Vec<NPCService>,
    pub faction: Option<String>,
    pub importance: NPCImportance,
    pub spawn_locations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NPCType {
    Merchant,
    QuestGiver,
    Trainer,
    Guard,
    Civilian,
    Noble,
    Innkeeper,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialoguePersonality {
    pub tone: String,           // "friendly", "gruff", "mysterious", etc.
    pub speaking_style: String, // "formal", "casual", "archaic", etc.
    pub interests: Vec<String>, // Topics they like to discuss
    pub knowledge_areas: Vec<String>, // What they know about
    pub personality_traits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NPCService {
    Shop { inventory_table: String },
    Quest { quest_ids: Vec<String> },
    Training { skills: Vec<String> },
    Inn { room_cost: u32 },
    Crafting { craft_types: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NPCImportance {
    Essential,  // Cannot be killed
    Important,  // Warns player before attacking
    Normal,     // Regular NPC
}

#[derive(Resource, Default)]
pub struct NPCTemplateRegistry {
    npcs: HashMap<String, NPCTemplate>,
}

impl NPCTemplateRegistry {
    pub fn register(&mut self, template: NPCTemplate) {
        self.npcs.insert(template.id.clone(), template);
    }
    
    pub fn get(&self, id: &str) -> Option<&NPCTemplate> {
        self.npcs.get(id)
    }
    
    pub fn count(&self) -> usize {
        self.npcs.len()
    }
}