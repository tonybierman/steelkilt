use bevy::prelude::*;

use crate::components::MainMenuUI;
use crate::management::spawn_management_ui;
use crate::selection::spawn_selection_ui;
use crate::state::{GameState, GameStateEnum};

/// Spawns the main menu UI hierarchy.
pub fn spawn_main_menu_ui(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("=== DRAFT RPG COMBAT SIMULATOR ==="),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.1)),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            // Menu options
            parent.spawn((
                Text::new("[1] Start Combat\n[2] Manage Combatants\n\n[Q] Quit"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
            ));
        });
}

/// Handles main menu keyboard input.
pub fn handle_main_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut app_exit_events: EventWriter<AppExit>,
    mut commands: Commands,
    ui_query: Query<Entity, With<MainMenuUI>>,
) {
    if !game_state.is_in(GameStateEnum::MainMenu) {
        return;
    }

    if keyboard.just_pressed(KeyCode::Digit1) {
        // Start Combat - go to character selection
        game_state.transition_to(GameStateEnum::Selection);
        for entity in ui_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        spawn_selection_ui(&mut commands);
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        // Manage Combatants
        game_state.transition_to(GameStateEnum::Management);
        for entity in ui_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        spawn_management_ui(&mut commands);
    } else if keyboard.just_pressed(KeyCode::KeyQ) {
        // Don't process quit if we just came from combat (prevents double-processing the same keypress)
        if game_state.previous != Some(GameStateEnum::Combat) {
            app_exit_events.send(AppExit::Success);
        } else {
            // Clear the previous state so Q works normally next time
            game_state.previous = None;
        }
    }
}

/// Updates the main menu UI (currently static, no updates needed).
#[allow(dead_code)]
pub fn update_main_menu_ui() {
    // Main menu is static, no updates needed
}
