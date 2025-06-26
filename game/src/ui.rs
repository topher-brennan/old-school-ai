use bevy::prelude::*;
use crate::{GameState, GameConfig};
use crate::character::{Character, CharacterClass};

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct CharacterCreationUI;

#[derive(Component)]
pub struct InGameUI;

#[derive(Component)]
pub struct CombatUI;

#[derive(Component)]
pub struct InventoryUI;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_ui::<MainMenuUI>)
            .add_systems(OnEnter(GameState::CharacterCreation), spawn_character_creation)
            .add_systems(OnExit(GameState::CharacterCreation), despawn_ui::<CharacterCreationUI>)
            .add_systems(OnEnter(GameState::InGame), spawn_in_game_ui)
            .add_systems(OnExit(GameState::InGame), despawn_ui::<InGameUI>)
            .add_systems(OnEnter(GameState::Combat), spawn_combat_ui)
            .add_systems(OnExit(GameState::Combat), despawn_ui::<CombatUI>)
            .add_systems(OnEnter(GameState::Inventory), spawn_inventory_ui)
            .add_systems(OnExit(GameState::Inventory), despawn_ui::<InventoryUI>)
            .add_systems(Update, (
                update_character_display,
                update_combat_log,
            ));
    }
}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.1, 0.1, 0.2).into(),
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "Old School AI RPG",
                TextStyle {
                    font_size: 48.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));

            // Subtitle
            parent.spawn(TextBundle::from_section(
                "Press Enter to Start",
                TextStyle {
                    font_size: 24.0,
                    color: Color::rgb(0.7, 0.7, 0.7),
                    ..default()
                },
            ));

            // Version info
            parent.spawn(TextBundle::from_section(
                "v0.1.0 - Built with Rust + Bevy",
                TextStyle {
                    font_size: 16.0,
                    color: Color::rgb(0.5, 0.5, 0.5),
                    ..default()
                },
            ));
        });
}

fn spawn_character_creation(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.1, 0.1, 0.2).into(),
                ..default()
            },
            CharacterCreationUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "Character Creation",
                TextStyle {
                    font_size: 36.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));

            // Instructions
            parent.spawn(TextBundle::from_section(
                "Press 1-7 to select class, then Enter to confirm",
                TextStyle {
                    font_size: 20.0,
                    color: Color::rgb(0.7, 0.7, 0.7),
                    ..default()
                },
            ));

            // Class options
            let classes = [
                "1. Fighter",
                "2. Magic User", 
                "3. Cleric",
                "4. Thief",
                "5. Dwarf",
                "6. Elf",
                "7. Halfling",
            ];

            for class in classes {
                parent.spawn(TextBundle::from_section(
                    class,
                    TextStyle {
                        font_size: 18.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                        ..default()
                    },
                ));
            }
        });
}

fn spawn_in_game_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            InGameUI,
        ))
        .with_children(|parent| {
            // Top bar with character info
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(60.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::rgb(0.2, 0.2, 0.3).into(),
                ..default()
            })
            .with_children(|parent| {
                // Character name and level
                parent.spawn(TextBundle::from_section(
                    "Character: [Name] Level 1",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));

                // HP display
                parent.spawn(TextBundle::from_section(
                    "HP: 10/10",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::rgb(0.9, 0.3, 0.3),
                        ..default()
                    },
                ));

                // Controls hint
                parent.spawn(TextBundle::from_section(
                    "I: Inventory | ESC: Menu",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::rgb(0.6, 0.6, 0.6),
                        ..default()
                    },
                ));
            });

            // Main game area (placeholder)
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex: 1.0,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Game World\n\nUse WASD to move\nClick to interact",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                        ..default()
                    },
                ));
            });
        });
}

fn spawn_combat_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::rgb(0.2, 0.1, 0.1).into(),
                ..default()
            },
            CombatUI,
        ))
        .with_children(|parent| {
            // Combat header
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(80.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.3, 0.1, 0.1).into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "COMBAT - Round 1",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::rgb(0.9, 0.3, 0.3),
                        ..default()
                    },
                ));
            });

            // Combat log
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(70.0),
                    height: Val::Percent(60.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "Combat log will appear here...",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                            ..default()
                        },
                    ),
                    CombatLog,
                ));
            });

            // Action buttons
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    gap: Size::new(Val::Px(10.0), Val::Px(0.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                let actions = ["Attack", "Cast Spell", "Use Item", "Flee"];
                for action in actions {
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(100.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                            ..default()
                        },
                        CombatActionButton(action.to_string()),
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            action,
                            TextStyle {
                                font_size: 14.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ));
                    });
                }
            });
        });
}

fn spawn_inventory_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::rgb(0.1, 0.1, 0.2).into(),
                ..default()
            },
            InventoryUI,
        ))
        .with_children(|parent| {
            // Header
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.2, 0.2, 0.3).into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Inventory",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            });

            // Inventory grid (placeholder)
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex: 1.0,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Inventory items will be displayed here\n\nPress I or ESC to close",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::rgb(0.7, 0.7, 0.7),
                        ..default()
                    },
                ));
            });
        });
}

fn despawn_ui<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
pub struct CombatLog;

#[derive(Component)]
pub struct CombatActionButton(pub String);

fn update_character_display(
    characters: Query<&Character>,
    mut text_query: Query<&mut Text>,
) {
    // This would update character info in the UI
    // For now, it's a placeholder
}

fn update_combat_log(
    mut text_query: Query<&mut Text, With<CombatLog>>,
) {
    // This would update the combat log
    // For now, it's a placeholder
} 