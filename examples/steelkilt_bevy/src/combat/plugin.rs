use bevy::prelude::*;

use crate::state::CombatState;

use super::{handle_combat_input, update_combat, update_combat_ui};

/// Plugin that handles all combat-related functionality
///
/// Registers:
/// - CombatState resource for tracking combat progression
/// - handle_combat_input system for processing player input
/// - update_combat system for combat automation (placeholder)
/// - update_combat_ui system for refreshing combat display
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CombatState>()
            .add_systems(
                Update,
                (
                    handle_combat_input,
                    update_combat,
                    update_combat_ui,
                ),
            );
    }
}
