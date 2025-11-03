use bevy::prelude::*;

use crate::main_menu::handle_main_menu_input;

/// Plugin that handles main menu functionality
///
/// Registers:
/// - handle_main_menu_input system for processing menu selections
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_main_menu_input);
    }
}
