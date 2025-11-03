mod combat;
mod components;
mod file_ops;
mod main_menu;
mod management;
mod selection;
mod state;

use bevy::prelude::*;

use combat::CombatPlugin;
use main_menu::{spawn_main_menu_ui, MainMenuPlugin};
use management::ManagementPlugin;
use selection::SelectionPlugin;
use state::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Draft RPG Combat Simulator".to_string(),
                resolution: (1400., 900.).into(),
                ..default()
            }),
            ..default()
        }))
        // Global game state
        .init_resource::<GameState>()
        // Feature plugins (each handles its own systems and resources)
        .add_plugins((
            MainMenuPlugin,
            ManagementPlugin,
            SelectionPlugin,
            CombatPlugin,
        ))
        // Startup system
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    spawn_main_menu_ui(&mut commands);
}
