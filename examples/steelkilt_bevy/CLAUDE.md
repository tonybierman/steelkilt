# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Bevy-based combat simulator implementing the Draft 0.4 RPG combat system using the `steelkilt` library. The application provides a graphical interface for turn-based combat between two fighters with the Draft RPG rule set.

## Development Commands

### Building
```bash
cargo build          # Build the project
cargo build --release # Build with optimizations
```

### Running
```bash
cargo run            # Build and run the project
cargo run --release  # Run optimized build
```

### Testing
```bash
cargo test           # Run all tests
cargo test <test_name> # Run a specific test
cargo test -- --nocapture # Run tests with output visible
```

### Code Quality
```bash
cargo check          # Quick syntax and type checking
cargo clippy         # Run linter
cargo fmt            # Format code
cargo fmt -- --check # Check if code is formatted
```

## Dependencies

- **Bevy 0.15**: Game engine framework
- **steelkilt**: Draft 0.4 RPG combat system library (local path with serde feature)
- **serde & serde_json**: For loading combatants from JSON files

## Architecture

The application follows Bevy's ECS (Entity Component System) pattern:

### Components
- `Fighter`: Wraps a `steelkilt::Character` and tracks which player (1 or 2)
- `CombatLogText`: Marker for the combat log UI element
- `StatusText`: Marker for fighter status displays (identified by fighter_id)
- `InstructionText`: Marker for instruction/prompt text
- `SelectionUI`: Marker for character selection UI container
- `SelectionText`: Marker for the selection screen text element

### Resources
- `GameState`: Application state machine:
  - Current state (MainMenu, Management, Selection, Combat)
  - Previous state for back navigation
  - Handles transitions between screens

- `CombatState`: Combat-specific state:
  - Current round number
  - Which fighter is attacking
  - Whether waiting for defense input
  - Combat log history
  - Game over/paused state
  - Selected fighter indices for character selection
  - Selection cursor position for arrow key navigation

- `ManagementState`: Combatant management state:
  - List of available combatants (loaded from JSON files)
  - Selected combatant index for navigation
  - Current mode (List or View)
  - Delete confirmation state

### Systems
- `setup`: Initializes the main menu UI
- `handle_main_menu_input`: Processes main menu selections (1, 2, Q)
- `handle_management_input`: Handles combatant management navigation and actions
- `handle_selection_input`: Processes arrow keys and Space for character selection
- `handle_combat_input`: Processes combat keyboard input (P/D/Space/Q)
- `update_combat`: Placeholder for future combat automation logic
- `update_main_menu_ui`: Static main menu (no updates needed)
- `update_management_ui`: Updates management screen based on state
- `update_selection_ui`: Updates selection screen with available combatants
- `update_combat_ui`: Refreshes combat UI (status, log, instructions)

### Helper Functions
- `load_available_combatants()`: Scans combatants directory and returns list of JSON files
- `load_character_from_file()`: Deserializes a Character from a JSON file
- `save_character_to_file()`: Serializes and saves a Character to JSON (for future edit feature)
- `delete_character_file()`: Removes a combatant JSON file
- `spawn_main_menu_ui()`: Creates the main menu UI hierarchy
- `spawn_management_ui()`: Creates the combatant management UI hierarchy
- `spawn_selection_ui()`: Creates the character selection UI hierarchy
- `spawn_combat_ui()`: Creates the combat UI hierarchy

### Combat Flow
1. Fighter 1 attacks, opponent chooses defense (Parry or Dodge)
2. Combat round resolves via `steelkilt::combat_round()`
3. Results displayed in combat log
4. Fighter 2 attacks, same process
5. After both fighters act, round completes
6. Press Space to continue to next round
7. Combat ends when a fighter dies or both are incapacitated

### Navigation Flow

The application follows this state flow:
1. **Main Menu**: Choose to start combat or manage combatants
2. **Combatant Management** (optional): View, delete, or refresh combatant list
3. **Character Selection**: Select two fighters for combat
4. **Combat**: Turn-based combat until victory or exit

### Controls

**Main Menu:**
- **1**: Start Combat (go to character selection)
- **2**: Manage Combatants
- **Q / Escape**: Quit application

**Combatant Management:**
- **↑/↓**: Navigate combatant list
- **V**: View selected combatant details
- **D**: Delete selected combatant (with confirmation)
- **R**: Refresh combatant list
- **ESC / B**: Back to main menu (or back to list if viewing)

**Delete Confirmation:**
- **Y**: Confirm delete
- **N / ESC**: Cancel delete

**Character Selection:**
- **↑/↓**: Navigate through available combatants
- **Space**: Select highlighted combatant (first = Fighter 1, second = Fighter 2)
- **Enter**: Start combat with selected fighters (or select if none selected)
- **Backspace**: Clear last selection
- **ESC**: Return to main menu

**Combat:**
- **P**: Choose Parry defense
- **D**: Choose Dodge defense
- **Space**: Continue to next round
- **Q / Escape**: End combat and return to main menu

## Combatant System

### Loading Combatants from JSON

The application loads combatant definitions from JSON files in the `combatants/` directory. On startup, you'll see a character selection screen showing all available combatants.

**Important**: Always run the application from the `examples/steelkilt_bevy` directory so it can find the `combatants/` folder:

```bash
cd examples/steelkilt_bevy
cargo run
```

### Creating Custom Combatants

1. Create a new `.json` file in the `combatants/` directory
2. Follow the format documented in `combatants/README.md`
3. The new combatant will automatically appear in the selection screen on next launch

### Included Combatants

The project includes 12 pre-made combatants:
- **Warriors**: Aldric, Grimwald, Thora, Kael, Ragnar, Garrick, Zephyr, Elara
- **Mages**: Mira (with Divination spells), Brother Aldwyn, Sylvana, Morgana

Each has unique stat distributions optimized for different fighting styles.

### Magic System

The application supports the Draft RPG magic system from Chapter 5. Characters can have:
- **Lores**: Knowledge in magic branches (Divination, Elementalism, Animation, etc.)
- **Spells**: Learned spells with skill levels
- **Exhaustion**: Magical exhaustion from casting spells

#### Magic in Combat

When a magic-using combatant is selected, their status display shows:
- Number of spells known
- Current exhaustion points
- Exhaustion level (None/Light/Severe/Critical)

#### Adding Magic to Combatants

To create a magic-using combatant, add a `magic` field to the JSON file:

```json
{
  "name": "Example Mage",
  ...other fields...
  "magic": {
    "empathy": 9,
    "exhaustion_points": 0,
    "lores": {
      "Divination": {
        "branch": "Divination",
        "level": 5,
        "empathy_attribute": 9
      }
    },
    "spells": {
      "Detect Magic": {
        "spell": {
          "name": "Detect Magic",
          "branch": "Divination",
          "difficulty": "Easy",
          "preparation_time": 5,
          "casting_time": 1,
          "range": {"Short": 10},
          "duration": {"Minutes": 10}
        },
        "skill_level": 4
      }
    }
  }
}
```

See `combatants/mira_starweaver.json` for a complete example.

**Note**: Spell casting during combat is not yet implemented. Currently, magic stats are displayed for informational purposes.
