// Combat module organization
//
// This module is organized into focused submodules for maintainability:
// - ui: UI spawning and updating (spawn_combat_ui, update_combat_ui)
// - input_handler: Main combat input system (handle_combat_input)
// - helpers: Shared utilities (fighter queries, turn management, logging)
// - melee: Melee combat execution
// - ranged: Ranged combat execution

mod helpers;
mod input_handler;
mod melee;
mod ranged;
mod ui;

// Re-export public systems for use in main.rs
pub use input_handler::handle_combat_input;
pub use ui::{spawn_combat_ui, update_combat, update_combat_ui};
