use serde::{Deserialize, Serialize}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileType {
    Floor,
    Wall,
    Door,
    Stairs,
    Corridor,
    Empty,
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        matches!(self, TileType::Floor | TileType::Door | TileType::Stairs | TileType::Corridor)
    }

    pub fn to_char(&self) -> char {
        match self {
            TileType::Floor => '.',
            TileType::Wall => '#',
            TileType::Door => '+',
            TileType::Stairs => '>',
            TileType::Corridor => '=',
            TileType::Empty => ' ',
        }
    }
}
