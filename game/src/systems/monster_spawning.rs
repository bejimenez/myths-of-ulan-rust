// src/systems/monster_spawning.rs
use bevy::prelude::*;
use rand::prelude::*;
use crate::components::{Position, Monster};
use crate::templates::monster_templates::{MonsterTemplateRegistry, spawn_monster_from_template};

/// Event for requesting a monster spawn
#[derive(Event)]
pub struct SpawnMonsterEvent {
    pub template_id: String,
    pub position: Position,
    pub level: Option<i32>,
}

/// Event for requesting random monsters in an area
#[derive(Event)]
pub struct SpawnRandomMonstersEvent {
    pub area_center: Position,
    pub area_size: (i32, i32),
    pub count: usize,
    pub level_range: (i32, i32),
    pub template_filter: Option<Vec<String>>, // If None, use all templates
}

/// System to handle monster spawn events
pub fn monster_spawn_system(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnMonsterEvent>,
    registry: Res<MonsterTemplateRegistry>,
) {
    for event in spawn_events.read() {
        spawn_monster_from_template(
            &mut commands,
            &registry,
            &event.template_id,
            event.position.clone(),
            event.level,
        );
    }
}

/// System to handle random monster spawning
pub fn random_monster_spawn_system(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnRandomMonstersEvent>,
    registry: Res<MonsterTemplateRegistry>,
    existing_positions: Query<&Position>,
) {
    let mut rng = thread_rng();
    
    for event in spawn_events.read() {
        // Get all valid templates for this spawn event
        let valid_templates: Vec<&str> = if let Some(filter) = &event.template_filter {
            filter.iter().map(|s| s.as_str()).collect()
        } else {
            // Get all templates that can spawn in the requested level range
            registry.templates.values()
                .filter(|t| {
                    t.level_range.0 <= event.level_range.1 && 
                    t.level_range.1 >= event.level_range.0
                })
                .map(|t| t.id.as_str())
                .collect()
        };
        
        if valid_templates.is_empty() {
            warn!("No valid templates found for spawn event");
            continue;
        }
        
        // Collect occupied positions to avoid spawning on top of entities
        let occupied: Vec<(i32, i32, i32)> = existing_positions.iter()
            .map(|p| (p.x, p.y, p.level))
            .collect();
        
        let mut spawned = 0;
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 1000;
        
        while spawned < event.count && attempts < MAX_ATTEMPTS {
            attempts += 1;
            
            // Generate random position within the area
            let x = event.area_center.x + rng.gen_range(-event.area_size.0..=event.area_size.0);
            let y = event.area_center.y + rng.gen_range(-event.area_size.1..=event.area_size.1);
            let level = event.area_center.level;
            
            // Check if position is occupied
            if occupied.contains(&(x, y, level)) {
                continue;
            }
            
            // Select random template
            let template_id = valid_templates.choose(&mut rng).unwrap();
            
            // Generate level within the event's range
            let monster_level = if event.level_range.0 == event.level_range.1 {
                event.level_range.0
            } else {
                rng.gen_range(event.level_range.0..=event.level_range.1)
            };
            
            // Spawn the monster
            spawn_monster_from_template(
                &mut commands,
                &registry,
                template_id,
                Position { x, y, level },
                Some(monster_level),
            );
            
            spawned += 1;
        }
        
        if spawned < event.count {
            warn!("Could only spawn {} out of {} requested monsters", spawned, event.count);
        }
    }
}

/// Helper function to get monsters appropriate for a dungeon level
pub fn get_appropriate_monsters_for_level(
    registry: &MonsterTemplateRegistry,
    dungeon_level: i32,
) -> Vec<String> {
    registry.templates.values()
        .filter(|template| {
            dungeon_level >= template.level_range.0 && 
            dungeon_level <= template.level_range.1
        })
        .map(|template| template.id.clone())
        .collect()
}

/// Monster density configuration
#[derive(Resource)]
pub struct MonsterDensityConfig {
    pub monsters_per_room_base: f32,
    pub monsters_per_room_per_level: f32,
    pub elite_chance: f32, // Chance for a monster to be an elite variant
    pub pack_spawn_chance: f32, // Chance for monsters to spawn in groups
    pub pack_size_min: usize,
    pub pack_size_max: usize,
}

impl Default for MonsterDensityConfig {
    fn default() -> Self {
        Self {
            monsters_per_room_base: 1.5,
            monsters_per_room_per_level: 0.2,
            elite_chance: 0.1,
            pack_spawn_chance: 0.3,
            pack_size_min: 2,
            pack_size_max: 5,
        }
    }
}

// Add to your plugin
pub struct MonsterSpawningPlugin;

impl Plugin for MonsterSpawningPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MonsterDensityConfig>()
            .add_event::<SpawnMonsterEvent>()
            .add_event::<SpawnRandomMonstersEvent>()
            .add_systems(Update, (
                monster_spawn_system,
                random_monster_spawn_system,
            ));
    }
}