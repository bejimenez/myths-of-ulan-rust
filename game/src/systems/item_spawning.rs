// src/systems/item_spawning.rs  - example system for spawning items in a game
use bevy::prelude::*;
use crate::components::{Item, ItemType, Position};
use crate::data::GameData;
use crate::data::templates::{ItemTemplate, WeaponTemplate, ConsumableTemplate};

pub fn spawn_item_from_template(
    commands: &mut Commands,
    game_data: &GameData,
    item_id: &str,
    position: Position,
) -> Option<Entity> {
    let template = game_data.items.get(item_id)?;

    match template {
        ItemTemplate::Weapon(weapon) => {
            spawn_weapon(commands, weapon, position)
        }
        ItemTemplate::Consumable(consumable) => {
            spawn_consumable(commands, consumable, position)
        }
        // Add more item types as needed
        _ => None,
    }
}

fn spawn_weapon(
    commands: &mut Commands,
    template: &WeaponTemplate,
    position: Position,
) -> Option<Entity> {
    let entity = commands.spawn((
        Item {
            item_type: ItemType::Weapon {
                damage: (template.damage.min + template.damage.max) / 2,
            },
            stack_size: template.stack_size,
        },
        Name(template.name.clone()),
        position,
        // add visual components, etc...
    )).id();
}

fn spawn_consumable(
    commands: &mut Commands,
    template: &ConsumableTemplate,
    position: Position,
) -> Option<Entity> {
    // Convert template effects to actual item properties
    let heal_amount = template.effects.iter()
        .find_map(|effect| {
            if let ConsumableEffect::Heal { amount } = effect {
                Some(*amount)
            } else {
                None
            }
        })
        .unwrap_or(0);
    
    let entity = commands.spawn((
        Item {
            item_type: ItemType::Potion { heal_amount },
            stack_size: template.stack_size,
        },
        Name(template.name.clone()),
        position,
    )).id();
    
    Some(entity)
}