// tools/validate_data.rs
use std::fs;
use std::path::Path;
use myths_of_ulan::data::templates::*;

fn main() {
    println!("Validating game data files...\n");
    
    let data_path = Path::new("data");
    
    // Validate monsters
    validate_directory::<MonsterTemplate>(&data_path.join("monsters"), "Monster");
    
    // Validate items
    validate_directory::<ItemTemplate>(&data_path.join("items"), "Item");
    
    // Validate NPCs
    validate_directory::<NPCTemplate>(&data_path.join("npcs"), "NPC");
    
    // Validate loot tables
    validate_directory::<LootTable>(&data_path.join("loot_tables"), "Loot Table");
}

fn validate_directory<T: for<'de> serde::Deserialize<'de>>(
    dir: &Path,
    type_name: &str,
) {
    println!("Validating {} files in {:?}", type_name, dir);
    
    if !dir.exists() {
        println!("  Directory does not exist!\n");
        return;
    }
    
    let mut total = 0;
    let mut errors = 0;
    
    for entry in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "json"))
    {
        total += 1;
        let path = entry.path();
        
        match fs::read_to_string(path) {
            Ok(contents) => {
                match serde_json::from_str::<Vec<T>>(&contents) {
                    Ok(items) => {
                        println!("  ✓ {:?} - {} items", path, items.len());
                    }
                    Err(e) => {
                        println!("  ✗ {:?} - Parse error: {}", path, e);
                        errors += 1;
                    }
                }
            }
            Err(e) => {
                println!("  ✗ {:?} - Read error: {}", path, e);
                errors += 1;
            }
        }
    }
    
    println!("  Total: {} files, {} errors\n", total, errors);
}