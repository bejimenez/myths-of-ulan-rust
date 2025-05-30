use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    base_stats: HashMap<String, f64>,
}

impl Stats {
    pub fn new() -> Self {
        let mut base_stats = HashMap::new();
        // Starting Stats
        base_stats.insert("hp".to_string(), 100.0);
        base_stats.insert("mp".to_string(), 50.0);
        base_stats.insert("physical_attack".to_string(), 10.0);
        base_stats.insert("toughness".to_string(), 5.0);

        Stats { base_stats }
    }

    pub fn get_stat(&self, stat_name: &str) -> f64 {
        self.base_stats.get(stat_name).copied().unwrap_or(0.0)
    }

    pub fn set_stat(&mut self, stat_name: &str, value: f64) {
        self.base_stats.insert(stat_name.to_string(), value);
    }
}
