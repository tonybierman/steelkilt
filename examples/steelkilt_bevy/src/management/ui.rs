use bevy::prelude::*;

use crate::components::{ManagementText, ManagementUI};
use crate::file_ops::load_character_from_file;
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
