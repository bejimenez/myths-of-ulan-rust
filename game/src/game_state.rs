// src/game_state.rs
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Exploring,
    InCombat,
    Paused,
    GameOver,
}