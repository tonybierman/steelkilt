# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Steelkilt is a Rust implementation of the Draft 0.4 RPG rule set (1993-1998 by Pitt Murmann), providing a reusable combat system library for tabletop role-playing games. The codebase implements both core combat mechanics and advanced features from the Draft RPG rulebook.

## Common Commands

### Building and Testing
```bash
# Build the project
cargo build --release

# Run all tests (39 comprehensive tests)
cargo test

# Run tests with output
cargo test -- --nocapture

# Run tests for a specific module
cargo test --lib skills  # Test skill system
cargo test --lib magic   # Test magic system
```

### Running Examples
```bash
# Interactive combat simulator (binary)
cargo run --bin combat-sim

# Quick combat example (basic melee combat)
cargo run --example quick_combat

# Advanced features showcase (all optional systems)
cargo run --example advanced_features

# Magic combat simulation (wizard's duel)
cargo run --example magic_combat
```

## Architecture

### Core vs Modules Design Pattern

The codebase follows a **two-tier architecture**:

1. **Core Library (`src/lib.rs`)**: Essential combat mechanics required for basic functionality
   - Exposed at the top level: `use steelkilt::*`
   - Contains: Attributes, Character, Weapon, Armor, Wounds, combat_round()
   - Always compiled and available

2. **Optional Modules (`src/modules/`)**: Advanced features from Draft RPG rulebook
   - Exposed via submodule: `use steelkilt::modules::*`
   - Contains: Skills, Exhaustion, Maneuvers, HitLocation, RangedCombat, Magic
   - Can be used selectively without bloating basic implementations

**Why this matters**: When adding features, determine whether they belong in the core combat loop (lib.rs) or are optional enhancements (modules/). Core features should be dependency-free and minimal.

### Module Responsibilities

Each module in `src/modules/` implements a distinct Draft RPG chapter/section:

- **skills.rs** (Section 3.13): Skill progression, difficulty classes, prerequisites, advancement costs
- **exhaustion.rs** (Section 4.24.1): Stamina tracking, fatigue penalties, recovery mechanics
- **maneuvers.rs** (Section 4.22): Combat stances (Charge, Defensive Position, All-Out Attack, Aimed Attack)
- **hit_location.rs** (Section 4.24.3): Body part targeting, damage multipliers, limb disabling
- **ranged_combat.rs** (Section 4.21): Bows, crossbows, firearms, range/cover modifiers, aiming
- **magic.rs** (Chapter 5): 9 magic branches, spell casting, lore system, magical exhaustion

### Integration Pattern

Advanced modules integrate with core types through these mechanisms:

1. **Attribute Extensions**: Modules use `Character.attributes` directly (e.g., magic uses empathy, exhaustion uses stamina)
2. **Wound System Integration**: Modules call `Character.wounds.add_wound()` for consistency
3. **Modifier Pattern**: Modules provide penalty/bonus values that stack with core rolls (e.g., `exhaustion.penalty()`, `stance.total_attack_modifier()`)
4. **State Separation**: Module state lives in separate structs (e.g., `SkillSet`, `Exhaustion`, `MagicUser`) rather than bloating `Character`

**Example**: A character with magic doesn't modify `Character` struct. Instead, create a separate `MagicUser` that references the character's empathy attribute and integrates wounds when spell damage occurs.

### Draft RPG Rule Set Fidelity

This library implements **specific sections** of the Draft 0.4 rulebook. When modifying combat mechanics:

- Core combat follows **Section 4.17-4.24**: d10-based skill checks, impact damage, CON-based wound thresholds
- Wound stacking is **exact**: 4 Light → 1 Severe, 3 Severe → 1 Critical, 2 Critical → Death
- Attributes use **9-attribute system** (Section 2.4-2.9): STR/DEX/CON, REA/INT/WIL, CHA/PER/EMP
- Skills are **attribute-capped**: costs increase beyond attribute score (Section 3.13)
- Magic requires **lore prerequisites**: can't learn spells without branch lore (Chapter 5)

The README.md documents which Draft RPG sections each feature implements. Maintain this mapping when adding features.

### Combat Damage Flow

Understanding the damage calculation pipeline is critical:

```
1. Attacker rolls: weapon_skill + d10 + modifiers (armor/wounds/maneuvers)
2. Defender rolls: weapon_skill/dodge_skill + d10 + modifiers
3. If attack > defense:
   damage = (attack - defense) + STR_bonus + weapon.damage - armor.protection
4. Compare damage to defender's CON:
   - damage ≤ CON/2: Light wound (-1 penalty)
   - damage ≤ CON: Severe wound (-2 penalty)
   - damage ≤ 2×CON: Critical wound (-4 penalty)
   - damage > 2×CON: Instant death
5. Apply wound stacking rules
6. Check for death (2+ Critical wounds)
```

**Key insight**: Wounds create a death spiral—each wound applies penalties to future rolls, making characters more vulnerable. This is intentional Draft RPG design.

### Testing Strategy

The codebase has **39 tests** covering:
- Core mechanics (4 tests): dice, attributes, wound stacking, death
- Each module (6-7 tests each): comprehensive coverage of Draft RPG rules

When adding features:
1. Add tests in the same module file (see `#[cfg(test)] mod tests` pattern)
2. Test **Draft RPG rule compliance**, not just code correctness
3. Test edge cases: wound stacking boundaries, attribute caps (1-10), death conditions
4. Use deterministic scenarios where possible (avoid relying on random d10 rolls in assertions)

## Bevy Integration Example

The `examples/steelkilt_bevy/` directory contains a separate Cargo workspace demonstrating Bevy game engine integration. It's maintained as a standalone example rather than part of the main library to avoid forcing Bevy as a dependency.
