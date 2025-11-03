use bevy::prelude::*;

use crate::components::MainMenuUI;
use crate::management::spawn_management_ui;
use crate::selection::spawn_selection_ui;
use crate::state::{GameState, GameStateEnum};

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
