# Steelkilt Combat Simulator

An interactive combat simulator demonstrating the Steelkilt RPG combat system with advanced Draft RPG features.

## Overview

This simulator showcases the integration of multiple advanced combat systems from the Draft 0.4 RPG rulebook:

- **Skills System**: Weapon proficiency and skill progression
- **Combat Stances**: Charge, Defensive Position, All-Out Attack, Aimed Attack
- **Exhaustion Tracking**: Fatigue accumulation through prolonged combat
- **Hit Locations**: Locational damage with body part targeting
- **Wound System**: Progressive wound accumulation and death spiral mechanics

## Running the Simulator

```bash
cargo run
```

The simulator presents an interactive combat between two predefined fighters:
- **Sir Roland the Defender**: Heavily armored knight with longsword
- **Thorgar the Fierce**: Powerful barbarian with two-handed sword

Each round, you can choose the knight's combat stance by entering the corresponding letter:
- `[N]` Normal - Standard attack with no modifiers
- `[C]` Charge - +1 attack, +1 damage, -2 defense (must charge first)
- `[D]` Defensive Position - +2 defense, cannot attack
- `[A]` All-Out Attack - +2 attack, -4 defense

## Architecture

The codebase is organized into separate modules with clear separation of concerns:

### Module Structure

```
src/
├── main.rs       - Main game loop and orchestration
├── ui.rs         - User interface and display functions
├── fighters.rs   - Character creation and configuration
├── state.rs      - Combat state management
└── combat.rs     - Combat execution and resolution
```

### Module Responsibilities

#### `main.rs` - Orchestration
- Initializes the combat simulation
- Manages the main game loop
- Coordinates user input and combat flow
- Minimal business logic - delegates to other modules

#### `ui.rs` - User Interface
**Purpose**: All user interaction and display logic

Functions:
- `get_user_choice()` - Generic choice prompt with options
- `print_fighter_status()` - Initial character sheet display
- `print_round_status()` - Per-round status summary
- `print_final_status()` - Post-combat detailed report
- `print_combat_header()` - Welcome message
- `print_section_divider()` - Section headers

**Dependency**: None (leaf module)

#### `fighters.rs` - Character Factory
**Purpose**: Character and skill set creation

Functions:
- `create_knight()` - Create the armored knight character
- `create_barbarian()` - Create the barbarian character
- `create_knight_skills()` - Configure knight's skill set
- `create_barbarian_skills()` - Configure barbarian's skill set

**Dependency**: Steelkilt core types (`Character`, `SkillSet`, `Weapon`, `Armor`)

**Design Note**: Pure factory functions - no state mutation. Easy to extend with new character archetypes.

#### `state.rs` - State Management
**Purpose**: Encapsulate all mutable combat state

Types:
- `FighterState` - Encapsulates character, skills, stance, exhaustion, and locations
- `CombatState` - Manages overall combat flow and both fighters

Key Features:
- Centralized state mutation
- Convenient methods for state transitions
- Hides complexity of multi-system state tracking

**Dependency**: Steelkilt modules (`CombatStance`, `Exhaustion`, `LocationalDamage`)

**Design Note**: This is the heart of state management. All combat state lives here, making it easy to serialize/deserialize for save games or replay features.

#### `combat.rs` - Combat Resolution
**Purpose**: Execute attacks and resolve combat outcomes

Functions:
- `determine_hit_location()` - Calculate which body part is hit
- `perform_attack()` - Execute full attack sequence with modifiers
- `apply_hit_damage()` - Calculate and apply damage with multipliers
- `apply_locational_wound()` - Track wounds by body location

**Dependency**: `state::FighterState`, Steelkilt combat system

**Design Note**: Pure business logic for combat resolution. All the "how does combat work" rules live here.

## Design Principles

### Separation of Concerns
Each module has a single, well-defined responsibility:
- UI knows nothing about combat mechanics
- Combat logic knows nothing about user interaction
- State management is isolated from business logic
- Character creation is pure data construction

### Maintainability
- **Easy to modify UI**: Change display format without touching combat logic
- **Easy to add fighters**: Create new factory functions in `fighters.rs`
- **Easy to extend combat**: Add new mechanics to `combat.rs`
- **Easy to modify state**: All state transitions go through `state.rs`

### Testability
- Business logic in `combat.rs` can be unit tested independently
- Factory functions in `fighters.rs` produce consistent test fixtures
- State module can be tested without UI or combat logic

### Extensibility

Future features can be easily added:

**AI Opponents**: Add `ai.rs` module that uses `FighterState` to make decisions

**Save/Load**: Serialize `CombatState` to JSON using serde

**Multiple Fighters**: Extend `CombatState` to manage a `Vec<FighterState>`

**Replay System**: Log combat actions and replay from `CombatState` snapshots

**Custom Fighters**: Add UI to `ui.rs` for interactive character creation

## Code Flow

```
main.rs
  ├─> fighters.rs: create_knight(), create_barbarian()
  ├─> state.rs: FighterState::new(), CombatState::new()
  ├─> ui.rs: print_combat_header(), print_fighter_status()
  └─> [Game Loop]
       ├─> state.rs: combat.next_round()
       ├─> ui.rs: get_user_choice()
       ├─> state.rs: knight.set_maneuver()
       ├─> combat.rs: perform_attack()
       │    ├─> combat.rs: determine_hit_location()
       │    ├─> steelkilt: combat_round()
       │    └─> combat.rs: apply_hit_damage()
       │         └─> state.rs: defender.add_location_wound()
       ├─> state.rs: combat.end_round()
       └─> ui.rs: print_round_status()
```

## Dependencies

- **steelkilt**: Core combat library (path = "../..", features = ["serde"])
- **serde**: Serialization support (future use for save/load)
- **serde_json**: JSON support (future use)
- **rand**: Random number generation (future use for AI)

## Future Enhancements

Possible additions while maintaining the current architecture:

1. **AI System** (`ai.rs`):
   - Tactical decision making for NPC fighters
   - Difficulty levels (aggressive, defensive, adaptive)

2. **Character Builder** (`builder.rs`):
   - Interactive character creation
   - Point-buy attribute allocation
   - Equipment selection

3. **Persistence** (extend `state.rs`):
   - Save combat state to JSON
   - Load and resume previous battles
   - Combat replay/analysis

4. **Campaign Mode** (`campaign.rs`):
   - Multiple sequential battles
   - Character progression between fights
   - Wound carryover and healing

5. **Network Play** (`network.rs`):
   - Two-player combat over network
   - Turn submission and state synchronization
