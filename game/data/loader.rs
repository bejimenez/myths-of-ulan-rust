// src/data/loader.rs
use super::templates::*;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub struct DataLoader;

impl DataLoader {
    /// Load all JSON files from a directory recursively
    pub fn load_json_files<T: for<'de> serde::Deserialize<'de>>(
        dir_path: &Path,
    ) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        let mut items = Vec::new();
        
        if !dir_path.exists() {
            return Ok(items);
        }
        
        for entry in WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "json"))
        {
            let contents = fs::read_to_string(entry.path())?;
            let file_items: Vec<T> = serde_json::from_str(&contents)?;
            items.extend(file_items);
            
            debug!("Loaded {} items from {:?}", file_items.len(), entry.path());
        }
        
        Ok(items)
    }
    
    pub fn load_monsters(
        dir_path: &Path,
        registry: &mut MonsterTemplateRegistry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let monsters: Vec<MonsterTemplate> = Self::load_json_files(dir_path)?;
        for monster in monsters {
            registry.register(monster);
        }
        Ok(())
    }
    
    pub fn load_items(
        dir_path: &Path,
        registry: &mut ItemTemplateRegistry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let items: Vec<ItemTemplate> = Self::load_json_files(dir_path)?;
        for item in items {
            registry.register(item);
        }
        Ok(())
    }
    
    pub fn load_npcs(
        dir_path: &Path,
        registry: &mut NPCTemplateRegistry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let npcs: Vec<NPCTemplate> = Self::load_json_files(dir_path)?;
        for npc in npcs {
            registry.register(npc);
        }
        Ok(())
    }
    
    pub fn load_loot_tables(
        dir_path: &Path,
        registry: &mut LootTableRegistry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tables: Vec<LootTable> = Self::load_json_files(dir_path)?;
        for table in tables {
            registry.register(table);
        }
        Ok(())
    }
}