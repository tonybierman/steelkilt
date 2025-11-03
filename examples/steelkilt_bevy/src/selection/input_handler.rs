use bevy::prelude::*;

use crate::combat::spawn_combat_ui;
use crate::components::{Fighter, SelectionUI};
use crate::file_ops::{load_available_combatants, load_character_from_file};
use crate::main_menu::spawn_main_menu_ui;
use crate::state::{CombatState, GameState, GameStateEnum};
use steelkilt::modules::ranged_combat::RangedAttackState;

/// Handles character selection keyboard input.
pub fn handle_selection_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut combat_state: ResMut<CombatState>,
    mut commands: Commands,
    selection_ui: Query<Entity, With<SelectionUI>>,
) {
    if !game_state.is_in(GameStateEnum::Selection) {
        return;
    }

    let combatants = load_available_combatants();

    if combatants.is_empty() {
        return;
    }

    // Arrow keys for navigation
    if keyboard.just_pressed(KeyCode::ArrowUp) && combat_state.selection_cursor > 0 {
        combat_state.selection_cursor -= 1;
    }

    if keyboard.just_pressed(KeyCode::ArrowDown)
        && combat_state.selection_cursor < combatants.len() - 1
    {
        combat_state.selection_cursor += 1;
    }

    // Space or Enter to select fighter
    if keyboard.just_pressed(KeyCode::Space) {
        let current_idx = combat_state.selection_cursor;
        if combat_state.selected_fighter1.is_none() {
            combat_state.selected_fighter1 = Some(current_idx);
        } else if combat_state.selected_fighter2.is_none()
            && Some(current_idx) != combat_state.selected_fighter1
        {
            combat_state.selected_fighter2 = Some(current_idx);
        }
    }

    // Enter to start combat when both fighters selected
    if keyboard.just_pressed(KeyCode::Enter) {
        if let (Some(idx1), Some(idx2)) = (
            combat_state.selected_fighter1,
            combat_state.selected_fighter2,
        ) {
            let name1 = &combatants[idx1];
            let name2 = &combatants[idx2];

            if let (Ok(char1), Ok(char2)) = (
                load_character_from_file(name1),
                load_character_from_file(name2),
            ) {
                // Despawn selection UI
                for entity in selection_ui.iter() {
                    commands.entity(entity).despawn_recursive();
                }

                // Spawn fighters with ranged state if they have ranged weapons
                let ranged_state1 = if char1.ranged_weapon.is_some() {
                    Some(RangedAttackState::new())
                } else {
                    None
                };
                let ranged_state2 = if char2.ranged_weapon.is_some() {
                    Some(RangedAttackState::new())
                } else {
                    None
                };

                commands.spawn(Fighter {
                    character: char1,
                    is_player_one: true,
                    ranged_state: ranged_state1,
                });
                commands.spawn(Fighter {
                    character: char2,
                    is_player_one: false,
                    ranged_state: ranged_state2,
                });

                // Transition to combat
                game_state.transition_to(GameStateEnum::Combat);
                spawn_combat_ui(&mut commands);
            }
        } else if combat_state.selected_fighter1.is_none() {
            // If no fighters selected, Enter works like Space
            combat_state.selected_fighter1 = Some(combat_state.selection_cursor);
        }
    }

    // Backspace to clear selections
    if keyboard.just_pressed(KeyCode::Backspace) {
        if combat_state.selected_fighter2.is_some() {
            combat_state.selected_fighter2 = None;
        } else if combat_state.selected_fighter1.is_some() {
            combat_state.selected_fighter1 = None;
        }
    }

    // Escape to go back to main menu
    if keyboard.just_pressed(KeyCode::Escape) {
        for entity in selection_ui.iter() {
            commands.entity(entity).despawn_recursive();
        }
        combat_state.selected_fighter1 = None;
        combat_state.selected_fighter2 = None;
        combat_state.selection_cursor = 0;
        game_state.transition_to(GameStateEnum::MainMenu);
        spawn_main_menu_ui(&mut commands);
    }
}
