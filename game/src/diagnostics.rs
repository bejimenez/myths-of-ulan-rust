use bevy::prelude::*;
use crate::components::*;

pub fn entity_diagnostics_system(
    query: Query<(Entity, &Name, Option<&Player>, Option<&Monster>, &Health, &CombatStats)>,
) {
    info!("=== ENTITY DIAGNOSTICS ===");
    for (entity, name, player, monster, health, combat) in query.iter() {
        let entity_type = if player.is_some() { 
            "PLAYER" 
        } else if monster.is_some() { 
            "MONSTER" 
        } else { 
            "UNKNOWN" 
        };
        
        info!(
            "{} Entity {:?}: {} | Health: {}/{} | Combat: dmg={}, def={}, acc={}, eva={}",
            entity_type,
            entity,
            name.0,
            health.current,
            health.max,
            combat.damage,
            combat.defense,
            combat.accuracy,
            combat.evasion
        );
    }
    info!("=========================");
}