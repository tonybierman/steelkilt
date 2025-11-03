use bevy::prelude::*;

use crate::components::ManagementUI;
use crate::file_ops::{delete_character_file, load_available_combatants};
use crate::main_menu::spawn_main_menu_ui;
use crate::state::{GameState, GameStateEnum, ManagementMode, ManagementState};

/// Handles combatant management keyboard input.
pub fn handle_management_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut management_state: ResMut<ManagementState>,
    mut commands: Commands,
    ui_query: Query<Entity, With<ManagementUI>>,
) {
    if !game_state.is_in(GameStateEnum::Management) {
        return;
    }

    // Handle delete confirmation
    if let Some(ref filename) = management_state.confirm_delete {
        if keyboard.just_pressed(KeyCode::KeyY) {
            // Confirm delete
            let filename_clone = filename.clone();
            if delete_character_file(&filename_clone).is_ok() {
                management_state.combatants = load_available_combatants();
                if management_state.selected_index >= management_state.combatants.len()
                    && management_state.selected_index > 0
                {
                    management_state.selected_index -= 1;
                }
            }
            management_state.confirm_delete = None;
        } else if keyboard.just_pressed(KeyCode::KeyN) || keyboard.just_pressed(KeyCode::Escape) {
            // Cancel delete
            management_state.confirm_delete = None;
        }
        return;
    }

    // Navigation
    if keyboard.just_pressed(KeyCode::ArrowUp) && management_state.selected_index > 0 {
        management_state.selected_index -= 1;
    } else if keyboard.just_pressed(KeyCode::ArrowDown)
        && management_state.selected_index < management_state.combatants.len().saturating_sub(1)
    {
        management_state.selected_index += 1;
    }

    // Actions
    if keyboard.just_pressed(KeyCode::KeyV) && !management_state.combatants.is_empty() {
        // View selected combatant
        management_state.mode = ManagementMode::View;
    } else if keyboard.just_pressed(KeyCode::KeyD) && !management_state.combatants.is_empty() {
        // Delete confirmation
        let filename = management_state.combatants[management_state.selected_index].clone();
        management_state.confirm_delete = Some(filename);
    } else if keyboard.just_pressed(KeyCode::KeyR) {
        // Refresh list
        management_state.combatants = load_available_combatants();
        management_state.selected_index = 0;
    } else if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::KeyB) {
        // Back to main menu
        if management_state.mode == ManagementMode::View {
            management_state.mode = ManagementMode::List;
        } else {
            game_state.transition_to(GameStateEnum::MainMenu);
            for entity in ui_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            spawn_main_menu_ui(&mut commands);
        }
    }
}
