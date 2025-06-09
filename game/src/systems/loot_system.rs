// src/systems/loot_system.rs - Example of using loot tables
use bevy::prelude::*;
use crate::data::GameData;
use crate::data::templates::LootResult;
use crate::components::Position;

#[derive(Event)]
pub struct DropLootEvent {
    pub loot_table_id: String,
    pub position: Position,
    pub level: u32,
    pub luck: f32,
}

pub fn loot_drop_system(
    mut commands: Commands,
    mut events: EventReader<DropLootEvent>,
    game_data: Res<GameData>,
) {
    for event in events.read() {
        let loot_results = game_data.loot_tables.roll_loot(
            &event.loot_table_id,
            event.level,
            event.luck,
        );
        
        for (i, result) in loot_results.iter().enumerate() {
            let offset_pos = Position {
                x: event.position.x + (i as i32 % 3) - 1,
                y: event.position.y + (i as i32 / 3) - 1,
                level: event.position.level,
            };
            
            match result {
                LootResult::Item { item_id, quantity } => {
                    // Spawn the item
                    if let Some(mut entity_commands) = spawn_item_from_template(
                        &mut commands,
                        &game_data,
                        item_id,
                        offset_pos,
                    ).and_then(|e| commands.get_entity(e)) {
                        // Update quantity if needed
                        entity_commands.insert(StackSize(*quantity));
                    }
                }
                LootResult::Gold { amount } => {
                    // Spawn gold pile
                    commands.spawn((
                        Item {
                            item_type: ItemType::Gold { amount: *amount },
                            stack_size: 1,
                        },
                        Name(format!("{} Gold", amount)),
                        offset_pos,
                    ));
                }
                LootResult::Experience { amount } => {
                    // Handle experience (usually added directly to player)
                    info!("Dropped {} experience", amount);
                }
            }
        }
    }
}