// Combatant Management module organization
//
// This module is organized into focused submodules for maintainability:
// - plugin: Bevy plugin for management system integration
// - ui: UI spawning and updating (spawn_management_ui, update_management_ui)
// - input_handler: Management input system (handle_management_input)

mod input_handler;
mod plugin;
mod ui;

// Re-export the management plugin (preferred interface)
pub use plugin::ManagementPlugin;

// Re-export UI spawner for direct use (needed for navigation from other modules)
pub use ui::spawn_management_ui;
