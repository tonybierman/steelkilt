// Main Menu module organization
//
// This module is organized into focused submodules for maintainability:
// - plugin: Bevy plugin for main menu system integration
// - ui: UI spawning and updating (spawn_main_menu_ui, update_main_menu_ui)
// - input_handler: Main menu input system (handle_main_menu_input)

mod input_handler;
mod plugin;
mod ui;

// Re-export the main menu plugin (preferred interface)
pub use plugin::MainMenuPlugin;

// Re-export UI spawner for direct use (needed for setup)
pub use ui::spawn_main_menu_ui;
