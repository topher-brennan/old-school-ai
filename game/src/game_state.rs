use bevy::prelude::*;
use crate::GameState;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(Startup, setup_game)
            .add_systems(Update, (
                handle_main_menu,
                handle_character_creation,
                handle_in_game,
                handle_combat_state,
                handle_inventory_state,
                handle_settings_state,
            ));
    }
}

fn setup_game(mut commands: Commands) {
    // Initialize game with main menu state
    commands.insert_resource(GameConfig::default());
}

fn handle_main_menu(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        next_state.set(GameState::CharacterCreation);
    }
}

fn handle_character_creation(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
    // Character creation logic will be handled by UI systems
}

fn handle_in_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::I) {
        next_state.set(GameState::Inventory);
    } else if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
    // Combat will be triggered by game events
}

fn handle_combat_state(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::InGame);
    }
}

fn handle_inventory_state(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::I) || keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::InGame);
    }
}

fn handle_settings_state(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
} 