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
- **steelkilt**: Draft 0.4 RPG combat system library (from GitHub)

## Architecture

The application follows Bevy's ECS (Entity Component System) pattern:

### Components
- `Fighter`: Wraps a `steelkilt::Character` and tracks which player (1 or 2)
- `CombatLogText`: Marker for the combat log UI element
- `StatusText`: Marker for fighter status displays (identified by fighter_id)
- `InstructionText`: Marker for instruction/prompt text

### Resources
- `CombatState`: Global state tracking:
  - Current round number
  - Which fighter is attacking
  - Whether waiting for defense input
  - Combat log history
  - Game over/paused state

### Systems
- `setup`: Initializes fighters and UI (runs once at startup)
- `handle_input`: Processes keyboard input for defense actions (P/D) and game flow (Space/Q)
- `update_combat`: Placeholder for future combat automation logic
- `update_ui`: Refreshes all UI text based on current combat state

### Combat Flow
1. Fighter 1 attacks, opponent chooses defense (Parry or Dodge)
2. Combat round resolves via `steelkilt::combat_round()`
3. Results displayed in combat log
4. Fighter 2 attacks, same process
5. After both fighters act, round completes
6. Press Space to continue to next round
7. Combat ends when a fighter dies or both are incapacitated

### Controls
- **P**: Choose Parry defense
- **D**: Choose Dodge defense
- **Space**: Continue to next round
- **Q / Escape**: Quit combat
