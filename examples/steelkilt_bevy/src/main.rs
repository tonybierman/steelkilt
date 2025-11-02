mod combat;
mod components;
mod file_ops;
mod main_menu;
mod management;
mod selection;
mod state;

use bevy::prelude::*;

use combat::{handle_combat_input, update_combat, update_combat_ui};
use main_menu::{handle_main_menu_input, spawn_main_menu_ui};
use management::{handle_management_input, update_management_ui};
use selection::{handle_selection_input, update_selection_ui};
use state::{CombatState, GameState, ManagementState};

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
        .init_resource::<GameState>()
        .init_resource::<CombatState>()
        .init_resource::<ManagementState>()
        .add_systems(Startup, setup)
        .add_systems(Update, handle_main_menu_input)
        .add_systems(Update, handle_management_input)
        .add_systems(Update, handle_selection_input)
        .add_systems(Update, handle_combat_input)
        .add_systems(Update, update_combat)
        .add_systems(Update, update_management_ui)
        .add_systems(Update, update_selection_ui)
        .add_systems(Update, update_combat_ui)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    spawn_main_menu_ui(&mut commands);
}
