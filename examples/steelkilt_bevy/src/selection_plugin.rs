use bevy::prelude::*;

use crate::selection::{handle_selection_input, update_selection_ui};

/// Plugin that handles character selection functionality
///
/// Registers:
/// - handle_selection_input system for processing selection commands
/// - update_selection_ui system for refreshing selection display
pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_selection_input,
                update_selection_ui,
            ),
        );
    }
}
