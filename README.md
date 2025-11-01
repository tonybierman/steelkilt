# Steelkilt - Draft RPG Combat System

A Rust implementation of the Draft 0.4 RPG rule set.

Based on the "Draft 0.4 RPG Rule Set" by Pitt Murmann (1993-1998), this library provides a reusable combat system for tabletop role-playing games.

## Features

### Core Systems Implemented

- **9 Attributes System**: Physical (STR, DEX, CON), Mental (REA, INT, WIL), and Interactive (CHA, PER, EMP)
- **Skill-Based Combat**: Attack, parry, and dodge mechanics using d10 rolls
- **Weapon System**: Impact-based damage with various weapon types (daggers, swords, etc.)
- **Armor System**: Protection values with movement penalties
- **Wound Tracking**: Light, Severe, and Critical wounds with stacking mechanics
- **Combat Rounds**: Turn-based combat resolution

### Combat Mechanics

The combat system follows the Draft RPG rules:

1. **Attack Roll** = Weapon Skill + d10 + Modifiers
2. **Defense Roll** = Weapon Skill (parry) or Dodge Skill + d10 + Modifiers
3. **Damage** = Attack Roll - Defense Roll + Strength Bonus + Weapon Damage - Armor Protection

**Wound Levels** (based on damage vs Constitution):
- **Light**: damage ≤ CON/2 (penalty: -1 per wound)
- **Severe**: damage ≤ CON (penalty: -2 per wound)
- **Critical**: damage ≤ 2×CON (penalty: -4 per wound)
- **Death**: damage > 2×CON or 2+ Critical wounds

**Wound Stacking**:
- 4 Light wounds → 1 Severe wound
- 3 Severe wounds → 1 Critical wound
- 2 Critical wounds → Death

## Installation

```bash
cargo build --release
```

## Usage

### As a Library

```rust
use steelkilt::*;

// Create attributes (STR, DEX, CON, REA, INT, WIL, CHA, PER, EMP)
let attrs = Attributes::new(8, 6, 7, 5, 6, 5, 5, 7, 4);

// Create a character
let mut fighter = Character::new(
    "Aldric",
    attrs,
    7,  // weapon skill
    5,  // dodge skill
    Weapon::long_sword(),
    Armor::chain_mail(),
);

// Create an opponent
let mut opponent = Character::new(
    "Grimwald",
    Attributes::new(9, 5, 8, 4, 5, 6, 4, 6, 3),
    6,  // weapon skill
    4,  // dodge skill
    Weapon::two_handed_sword(),
    Armor::leather(),
);

// Execute combat round
let result = combat_round(&mut fighter, &mut opponent, DefenseAction::Parry);

if result.hit {
    println!("{} hit {} for {} damage!", result.attacker, result.defender, result.damage);
}
```

### Interactive Combat Simulation

Run the interactive combat simulator:

```bash
cargo run --bin combat-sim
```

The simulation features:
- Pre-generated characters with different attributes and equipment
- Turn-based combat with player input for defense actions
- Real-time wound tracking and status updates
- Detailed character sheets and combat logs

## Project Structure

```
steelkilt/
├── src/
│   ├── lib.rs              # Core library implementation
│   ├── modules/            # Advanced feature modules
│   │   ├── mod.rs          # Module exports
│   │   ├── skills.rs       # Skill development system
│   │   ├── exhaustion.rs   # Exhaustion tracking
│   │   ├── maneuvers.rs    # Combat maneuvers
│   │   ├── hit_location.rs # Hit location tracking
│   │   ├── ranged_combat.rs # Ranged combat mechanics
│   │   └── magic.rs        # Magic system
│   └── bin/
│       └── combat_sim.rs   # Interactive combat simulation
├── examples/
│   ├── quick_combat.rs     # Basic combat example
│   ├── advanced_features.rs # Showcase of all advanced features
│   └── magic_combat.rs     # Wizard duel simulation
├── Cargo.toml
└── README.md
```

## Core Types

### `Attributes`
Contains all 9 character attributes (range 1-10)

### `Character`
Represents a combatant with attributes, skills, equipment, and wounds

### `Weapon`
Weapon definitions with impact levels (Small=1, Medium=2, Large=3, Huge=4)

### `Armor`
Armor types with protection values and movement penalties

### `Wounds`
Tracks Light, Severe, and Critical wounds with automatic stacking

### `CombatResult`
Contains the outcome of a combat round

## Testing

Run the test suite:

```bash
cargo test
```

The project includes **39 comprehensive tests** covering:

**Core Combat System**:
- Dice rolling (d10 range validation)
- Attribute calculations
- Wound stacking mechanics
- Death thresholds

**Advanced Features**:
- Skill progression and cost calculation (6 tests)
- Exhaustion levels and recovery (4 tests)
- Combat maneuvers and modifiers (6 tests)
- Hit location determination and damage (6 tests)
- Ranged combat mechanics and modifiers (7 tests)
- Magic system, spell casting, and exhaustion (7 tests)

All tests validate proper implementation of Draft 0.4 RPG rules.

## Draft RPG Rules Reference

This implementation is based on specific rules from Draft 0.4:

**Core Systems**:
- **Section 2.4-2.9**: Attributes system (9 attributes)
- **Section 3**: Skills and skill checks
- **Section 3.13**: Skill development and progression
- **Section 4.17-4.24**: Combat mechanics
  - Attack/parry/dodge rolls
  - Weapon impact and armor protection
  - Damage calculation
  - Wound levels and stacking
  - Death conditions

**Advanced Features**:
- **Section 4.21**: Ranged combat mechanics
- **Section 4.22**: Special combat maneuvers
- **Section 4.24.1**: Exhaustion system
- **Section 4.24.3**: Hit location tracking
- **Chapter 5**: Magic system (9 branches of magic)

## License

This implementation is released under the MIT license. The original Draft 0.4 RPG Rule Set was released under the OpenContent License (OPL) Version 1.0.

## Example Combat Output

```
=== DRAFT RPG COMBAT SIMULATOR ===

╔═══════════════════════════════════════╗
║ Aldric the Bold                       ║
╠═══════════════════════════════════════╣
║ PHYSICAL ATTRIBUTES:                  ║
║   STR: 8   DEX: 6   CON: 7            ║
║ Weapon: Long Sword                    ║
║   Damage: 5                           ║
║ Armor: Chain Mail                     ║
║   Protection: 3                       ║
╚═══════════════════════════════════════╝

--- ROUND 1 ---

Aldric the Bold's turn to attack!
How does Grimwald Ironfist defend? [P]arry or [D]odge? p

>>> Attack: Aldric the Bold rolls 14 vs Grimwald Ironfist's defense 11
>>> HIT! 6 damage dealt
>>> Severe wound inflicted!
```

## Advanced Features

The library now includes comprehensive implementations of advanced Draft RPG mechanics:

### 1. Skill Development & Progression (Section 3.13)

Complete skill learning and advancement system with:
- **Skill Difficulty Classes**: Easy, Normal, Hard, Very Hard
- **Progressive Cost System**: Costs increase beyond attribute scores
- **Prerequisites**: Skills can require other skills at minimum levels
- **Skill Points**: Track and manage character advancement

```rust
use steelkilt::modules::*;

let mut skill_set = SkillSet::new(30); // 30 skill points

// Add skills with different difficulties
let sword = Skill::new("Longsword", 7, SkillDifficulty::Normal);
skill_set.add_skill(sword);

// Raise skill level
skill_set.raise_skill("Longsword").unwrap();
```

**Cost Mechanics**:
- Normal skills: 1 point per level up to attribute score
- Easy skills: 1 point total to reach attribute score
- Hard skills: 2x normal cost
- Very Hard skills: 3x normal cost
- Beyond attribute score: costs increase progressively

### 2. Exhaustion System (Section 4.24.1)

Track physical and magical exhaustion:
- **Exhaustion Levels**: None, Light, Severe, Critical
- **Combat Fatigue**: 1 point per combat round
- **Penalties**: -1 to -4 based on exhaustion level
- **Recovery**: 1 point per 2 rounds of rest
- **Willpower Checks**: Required at Severe+ levels

```rust
let mut exhaustion = Exhaustion::new(stamina);
exhaustion.add_points(1); // Each combat round
let penalty = exhaustion.penalty(); // -1 to -4
exhaustion.rest(10); // Recover from rest
```

### 3. Special Combat Maneuvers (Section 4.22)

Tactical combat options:
- **Normal**: Standard attack (+0/+0/+0)
- **Defensive Position**: +2 defense, cannot attack
- **Charge**: +1 attack, +1 damage, -2 defense
- **All-Out Attack**: +2 attack, -4 defense (risky!)
- **Aimed Attack**: -2 attack, +2 damage (requires aiming)

```rust
use steelkilt::modules::*;

let mut stance = CombatStance::new();
stance.set_maneuver(CombatManeuver::Charge).unwrap();

// Apply modifiers
let attack_bonus = stance.total_attack_modifier(); // +1
let defense_penalty = stance.total_defense_modifier(); // -2
let damage_bonus = stance.total_damage_modifier(); // +1
```

### 4. Hit Location Tracking (Section 4.24.3)

Detailed wound tracking by body part:
- **6 Hit Locations**: Head, Torso, Left/Right Arm, Left/Right Leg
- **Attack Direction**: Front/Back/Left/Right/Above/Below
- **Damage Multipliers**: Head 1.5x, Torso 1.0x, Limbs 0.75x
- **Disabling Wounds**: Severe hits can disable limbs
- **Severed Limbs**: Multiple Critical wounds can sever

```rust
use steelkilt::modules::*;

let location = HitLocation::determine(AttackDirection::Front);
let multiplier = location.damage_multiplier();

let mut arm = LocationalDamage::new(HitLocation::RightArm);
arm.add_wound(WoundSeverity::Severe);
if !arm.is_functional() {
    println!("Arm disabled!");
}
```

### 5. Ranged Combat (Section 4.21)

Complete ranged weapon system:
- **Weapon Types**: Bows, crossbows, firearms, thrown weapons
- **Range Modifiers**: Point blank, medium, max range
- **Aiming System**: +1 bonus after 1 round of aiming
- **Target Size**: -4 (Tiny) to +6 (Gigantic)
- **Cover**: None, Partial (-2), 3/4 (-4), Full (-8)
- **Preparation Time**: Different for each weapon type

```rust
use steelkilt::modules::*;

let bow = RangedWeapon::long_bow();
let mut state = RangedAttackState::new();

state.prepare_weapon(&bow);
state.start_aiming();
state.continue_aiming(); // Aim for a round

let modifier = calculate_ranged_modifiers(
    50,                    // distance in meters
    TargetSize::Medium,
    Cover::Partial,
    &bow,
    &state,
);

state.fire().unwrap(); // Shoot!
```

**Weapon Examples**:
- **Short Bow**: 20m point blank, 100m max, damage 4
- **Long Bow**: 30m point blank, 120m max, damage 6
- **Crossbow**: 30m point blank, 100m max, damage 6 (slow reload)
- **Rifle**: 40m point blank, 200m max, damage 8

### 6. Magic System (Chapter 5)

Full implementation of Draft RPG magic:
- **9 Branches of Magic**: Alchemy, Animation, Conjuration, Divination, Elementalism, Mentalism, Necromancy, Thaumaturgy, Transportation
- **Lore System**: Must learn branch lore before spells
- **Spell Difficulty**: Easy (8), Normal (10), Hard (12) target numbers
- **Magical Exhaustion**: Casting causes exhaustion based on spell power
- **Empathy Attribute**: Core attribute for magic use

```rust
use steelkilt::modules::*;

let mut mage = MagicUser::new(7); // Empathy 7

// Learn a branch of magic
mage.add_lore(MagicBranch::Divination, 5);

// Create and learn a spell
let spell = Spell {
    name: "Detect Magic".to_string(),
    branch: MagicBranch::Divination,
    difficulty: SpellDifficulty::Easy,
    preparation_time: 5,
    casting_time: 1,
    range: SpellRange::Short(20),
    duration: SpellDuration::Minutes(10),
};

mage.learn_spell(spell, 4).unwrap(); // Level 4 skill

// Cast spell: skill + empathy + roll
let result = mage.cast_spell("Detect Magic", d10()).unwrap();
if result.success {
    println!("Quality: {}", result.quality);
}

// Magical exhaustion penalties apply
let penalty = mage.exhaustion_penalty();
```

**Branches of Magic**:
- **Divination** (Normal): Gather information, foresee events
- **Alchemy** (Hard): Transform matter
- **Elementalism** (Very Hard): Control fire, water, air, earth
- **Conjuration** (Very Hard): Summon creatures
- **Necromancy** (Very Hard): Control undead
- **Thaumaturgy** (Hard): Telekinesis, control matter
- **Transportation** (Very Hard): Teleportation, time travel
- **Animation** (Hard): Healing, physical enhancement
- **Mentalism** (Hard): Mind reading, mental control

## Examples

The project includes comprehensive examples:

### Quick Combat
```bash
cargo run --example quick_combat
```
Basic melee combat demonstration showing the core combat system in action.

### Advanced Features Demo
```bash
cargo run --example advanced_features
```
Comprehensive showcase of all advanced features:
- Skill progression with different difficulties
- Combat maneuvers (charge, defensive position, etc.)
- Exhaustion tracking during extended combat
- Hit location determination and effects
- Ranged combat with modifiers
- Magic system with spell casting

### Magic Combat (Wizard's Duel)
```bash
cargo run --example magic_combat
```
Simulated duel between two magic users demonstrating:
- Offensive and defensive spell casting
- Magical exhaustion tracking
- Spell quality and targeting
- Multiple magic branches (Elementalism, Necromancy, Animation, Mentalism)
- Integration of magic with the wound system
