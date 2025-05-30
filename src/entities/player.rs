use super::stats::Stats;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub level: u32,
    pub experience: u32,
    pub stats: Stats,
    pub current_hp: f64,
    pub current_mp: f64,
}

impl Player {
    pub fn new(name: String) -> Self {
        let stats = Stats::new();
        let max_hp = stats.get_stat("hp");
        let max_mp = stats.get_stat("mp");

        Player {
            name,
            level: 1,
            experience: 0,
            stats,
            current_hp: max_hp,
            current_mp: max_mp,
        }
    }

    pub fn display_status(&self) {
        println!("\n=== {} - Level {} ===", self.name, self.level);
        println!("HP: {}/{}", self.current_hp, self.stats.get_stat("hp"));
        println!("MP: {}/{}", self.current_mp, self.stats.get_stat("mp"));
        println!("Attack Power: {}", self.stats.get_stat("physical_attack"));
        println!("Toughness: {}", self.stats.get_stat("Toughness"));
        println!("Experience: {}", self.experience);
    }
}
