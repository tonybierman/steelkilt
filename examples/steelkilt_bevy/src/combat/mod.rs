// Combat module organization
//
// This module is organized into focused submodules for maintainability:
// - plugin: Bevy plugin for combat system integration
// - ui: UI spawning and updating (spawn_combat_ui, update_combat_ui)
// - input_handler: Main combat input system (handle_combat_input)
// - helpers: Shared utilities (fighter queries, turn management, logging)
// - melee: Melee combat execution
// - ranged: Ranged combat execution

mod helpers;
mod input_handler;
mod melee;
mod plugin;
mod ranged;
mod ui;

// Re-export the combat plugin (preferred interface)
pub use plugin::CombatPlugin;

// Re-export UI spawner for direct use (needed for setup)
pub use ui::spawn_combat_ui;

// Keep private exports for internal use
use input_handler::handle_combat_input;
use ui::{update_combat, update_combat_ui};
