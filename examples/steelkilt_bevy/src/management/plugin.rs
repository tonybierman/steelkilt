use bevy::prelude::*;

use super::input_handler::handle_management_input;
use super::ui::update_management_ui;
use crate::state::ManagementState;

/// Plugin that handles combatant management functionality
///
/// Registers:
/// - ManagementState resource for tracking management screen state
/// - handle_management_input system for processing management commands
/// - update_management_ui system for refreshing management display
pub struct ManagementPlugin;

impl Plugin for ManagementPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ManagementState>()
            .add_systems(
                Update,
                (
                    handle_management_input,
                    update_management_ui,
                ),
            );
    }
}
