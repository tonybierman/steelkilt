// Character Selection module organization
//
// This module is organized into focused submodules for maintainability:
// - plugin: Bevy plugin for selection system integration
// - ui: UI spawning and updating (spawn_selection_ui, update_selection_ui)
// - input_handler: Selection input system (handle_selection_input)

mod input_handler;
mod plugin;
mod ui;

// Re-export the selection plugin (preferred interface)
pub use plugin::SelectionPlugin;

// Re-export UI spawner for direct use (needed for navigation from other modules)
pub use ui::spawn_selection_ui;
