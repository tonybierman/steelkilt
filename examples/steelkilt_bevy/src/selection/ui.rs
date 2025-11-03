use bevy::prelude::*;

use crate::components::{SelectionText, SelectionUI};
use crate::file_ops::load_available_combatants;
use crate::state::{CombatState, GameState, GameStateEnum};

/// Spawns the character selection UI hierarchy.
pub fn spawn_selection_ui(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            SelectionUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("=== CHARACTER SELECTION ==="),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.1)),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            parent.spawn((
                Text::new("Loading combatants..."),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                SelectionText,
            ));
        });
}

/// Updates the character selection UI based on current state.
pub fn update_selection_ui(
    combat_state: Res<CombatState>,
    game_state: Res<GameState>,
    mut query: Query<&mut Text, With<SelectionText>>,
) {
    if !game_state.is_in(GameStateEnum::Selection) {
        return;
    }

    let combatants = load_available_combatants();

    for mut text in query.iter_mut() {
        let mut display = String::from("=== CHARACTER SELECTION ===\n\n");
        display.push_str("Select two combatants to fight:\n\n");

        for (i, name) in combatants.iter().enumerate() {
            let cursor = if i == combat_state.selection_cursor {
                "> "
            } else {
                "  "
            };
            let marker = if Some(i) == combat_state.selected_fighter1 {
                " [FIGHTER 1] ✓"
            } else if Some(i) == combat_state.selected_fighter2 {
                " [FIGHTER 2] ✓"
            } else {
                ""
            };
            display.push_str(&format!("{}{}{}\n", cursor, name, marker));
        }

        display.push('\n');

        if combat_state.selected_fighter1.is_some() && combat_state.selected_fighter2.is_some() {
            display.push_str("Press [ENTER] to start combat\n");
        } else if combat_state.selected_fighter1.is_some() {
            display.push_str("Select second fighter with [SPACE]\n");
        } else {
            display.push_str("Select first fighter with [SPACE]\n");
        }

        display.push_str(
            "Press [↑/↓] to navigate | [BACKSPACE] to clear | [ESC] to return to main menu",
        );

        **text = display;
    }
}
