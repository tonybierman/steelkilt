use bevy::prelude::*;

use crate::components::{ManagementText, ManagementUI};
use crate::file_ops::{delete_character_file, load_available_combatants, load_character_from_file};
use crate::main_menu::spawn_main_menu_ui;
use crate::state::{GameState, GameStateEnum, ManagementMode, ManagementState};

/// Spawns the combatant management UI hierarchy.
pub fn spawn_management_ui(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ManagementUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("=== COMBATANT MANAGEMENT ==="),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.1)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            parent.spawn((
                Text::new("Loading..."),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                ManagementText,
            ));
        });
}

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

/// Updates the combatant management UI based on current state.
pub fn update_management_ui(
    management_state: Res<ManagementState>,
    game_state: Res<GameState>,
    mut query: Query<&mut Text, With<ManagementText>>,
) {
    if !game_state.is_in(GameStateEnum::Management) {
        return;
    }

    for mut text in query.iter_mut() {
        if let Some(ref filename) = management_state.confirm_delete {
            **text = format!("Delete '{}'?\n\n[Y] Yes, delete\n[N] No, cancel", filename);
        } else if management_state.mode == ManagementMode::View
            && !management_state.combatants.is_empty()
        {
            // View mode - show character details
            let filename = &management_state.combatants[management_state.selected_index];
            if let Ok(character) = load_character_from_file(filename) {
                **text = format!(
                    "Viewing: {}\n{}\n\nName: {}\nSTR: {} | DEX: {} | CON: {}\nREA: {} | INT: {} | WIL: {}\nCHA: {} | PER: {} | EMP: {}\n\nWeapon Skill: {} | Dodge Skill: {}\nWeapon: {} (Dmg: {})\nArmor: {} (Prot: {})\n\n[ESC] Back to list",
                    filename,
                    "=".repeat(50),
                    character.name,
                    character.attributes.strength,
                    character.attributes.dexterity,
                    character.attributes.constitution,
                    character.attributes.reason,
                    character.attributes.intuition,
                    character.attributes.willpower,
                    character.attributes.charisma,
                    character.attributes.perception,
                    character.attributes.empathy,
                    character.weapon_skill,
                    character.dodge_skill,
                    character.weapon.name,
                    character.weapon.damage,
                    character.armor.name,
                    character.armor.protection,
                );
            } else {
                **text = "Error loading character\n\n[ESC] Back to list".to_string();
            }
        } else {
            // List mode
            let mut display = String::from("Combatants:\n\n");

            if management_state.combatants.is_empty() {
                display.push_str("No combatants found.\n\n");
            } else {
                for (i, name) in management_state.combatants.iter().enumerate() {
                    let marker = if i == management_state.selected_index {
                        ">"
                    } else {
                        " "
                    };
                    display.push_str(&format!("{} {}\n", marker, name));
                }
                display.push('\n');
            }

            display.push('\n');
            display.push_str("[↑/↓] Navigate | [V] View | [D] Delete | [R] Refresh\n");
            display.push_str("[ESC] Back to Main Menu");

            **text = display;
        }
    }
}
