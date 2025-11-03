mod combat;
mod components;
mod file_ops;
mod main_menu;
mod main_menu_plugin;
mod management;
mod management_plugin;
mod selection;
mod selection_plugin;
mod state;

use bevy::prelude::*;

use combat::{spawn_combat_ui, CombatPlugin};
use main_menu::spawn_main_menu_ui;
use main_menu_plugin::MainMenuPlugin;
use management_plugin::ManagementPlugin;
use selection_plugin::SelectionPlugin;
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
