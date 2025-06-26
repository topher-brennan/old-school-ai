use bevy::prelude::*;
use serde::{Deserialize, Serialize};

mod game_state;
mod character;
mod combat;
mod ui;
mod ai_client;

use game_state::GameStatePlugin;
use character::CharacterPlugin;
use combat::CombatPlugin;
use ui::UIPlugin;
use ai_client::AIClientPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Old School AI RPG".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            GameStatePlugin,
            CharacterPlugin,
            CombatPlugin,
            UIPlugin,
            AIClientPlugin,
        ))
        .run();
}

// Core game data structures
#[derive(Resource, Clone, Debug)]
pub struct GameConfig {
    pub ai_service_url: String,
    pub save_file_path: String,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            ai_service_url: "http://localhost:8000".to_string(),
            save_file_path: "save_game.json".to_string(),
        }
    }
}

// Game states
#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    CharacterCreation,
    InGame,
    Combat,
    Inventory,
    Settings,
} 