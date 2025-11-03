use bevy::prelude::*;

use crate::components::MainMenuUI;

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

/// Updates the main menu UI (currently static, no updates needed).
#[allow(dead_code)]
pub fn update_main_menu_ui() {
    // Main menu is static, no updates needed
}
