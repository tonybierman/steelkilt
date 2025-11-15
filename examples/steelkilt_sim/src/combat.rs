//! Combat execution module
//!
//! Handles combat resolution including:
//! - Attack execution with modifiers
//! - Hit location determination
//! - Damage calculation with stance and location multipliers
//! - Wound application and tracking
//!
//! # Design Notes
//! This module serves as the orchestration layer between the game's combat models
//! and the core steelkilt combat library, handling application-specific logic like
//! stance modifiers, exhaustion, and locational damage tracking.
use text_colorizer::*;
use crate::models::Combatant;
use steelkilt::modules::*;
use steelkilt::*;

// ============================================================================
// Constants
// ============================================================================

/// Number of attack directions in the rotation cycle
const ATTACK_DIRECTION_CYCLE: usize = 3;

// ============================================================================
// Public API
// ============================================================================

/// Determine hit location based on round number using a rotating attack pattern.
///
/// Uses a simple 3-direction rotation (Above → Left → Front) to provide variety
/// in combat while maintaining predictability for testing.
///
/// # Arguments
/// * `round` - The current combat round number (0-indexed)
///
/// # Returns
/// A randomly selected `HitLocation` appropriate for the attack direction
///
/// # Examples
/// ```
/// let location = determine_hit_location(0); // Above attack
/// let location = determine_hit_location(1); // Left attack
/// let location = determine_hit_location(2); // Front attack
/// let location = determine_hit_location(3); // Above attack (cycle repeats)
/// ```
pub fn determine_hit_location(round: usize) -> HitLocation {
    let direction = attack_direction_for_round(round);
    HitLocation::determine(direction)
}

/// Execute a complete attack sequence with all modifiers and systems applied.
///
/// This is the main entry point for combat actions, handling:
/// - Attack roll with stance, exhaustion, and other modifiers
/// - Defense roll processing
/// - Hit location determination
/// - Damage calculation and application
/// - Wound tracking and location-specific effects
///
/// # Arguments
/// * `attacker` - The combatant performing the attack
/// * `defender` - The combatant being attacked
/// * `round` - The current round number for hit location determination
///
/// # Side Effects
/// - Modifies both attacker and defender state
/// - Prints combat log messages to stdout
pub fn perform_attack(
    attacker: &mut Combatant,
    defender: &mut Combatant,
    round: usize,
) {
    let attack_context = AttackContext::new(attacker, defender, round);
    attack_context.log_attack_start();

    let result = execute_attack_roll(attacker, defender);

    if result.hit {
        handle_successful_hit(attacker, defender, &result, attack_context.hit_location);
    } else {
        log_missed_attack(&result);
    }
}

// ============================================================================
// Internal Helpers - Attack Execution
// ============================================================================

/// Context information for a single attack
struct AttackContext {
    hit_location: HitLocation,
    attacker_name: String,
    defender_name: String,
    stance: String,
    exhaustion_status: String,
    total_modifier: i32,
}

impl AttackContext {
    fn new(attacker: &Combatant, defender: &Combatant, round: usize) -> Self {
        Self {
            hit_location: determine_hit_location(round),
            attacker_name: attacker.character.name.clone(),
            defender_name: defender.character.name.clone(),
            stance: attacker.stance.current_maneuver.to_string(),
            exhaustion_status: attacker.exhaustion.status().to_string(),
            total_modifier: attacker.total_attack_modifier(),
        }
    }

    fn log_attack_start(&self) {
        println!(
            "\n{} attacks {} (Stance: {}, Exhaustion: {}, Total Mod: {:+})",
            self.attacker_name,
            self.defender_name,
            self.stance,
            self.exhaustion_status,
            self.total_modifier
        );
    }
}

/// Execute the core attack roll using the steelkilt library
fn execute_attack_roll(
    attacker: &mut Combatant,
    defender: &mut Combatant,
) -> CombatResult {
    combat_round(
        &mut attacker.character,
        &mut defender.character,
        DefenseAction::Parry,
    )
}

/// Map round number to attack direction in the rotation cycle
#[inline]
fn attack_direction_for_round(round: usize) -> AttackDirection {
    match round % ATTACK_DIRECTION_CYCLE {
        0 => AttackDirection::Above,
        1 => AttackDirection::Left,
        _ => AttackDirection::Front,
    }
}

// ============================================================================
// Internal Helpers - Hit Processing
// ============================================================================

/// Process a successful hit with damage calculation and wound application
fn handle_successful_hit(
    attacker: &Combatant,
    defender: &mut Combatant,
    result: &CombatResult,
    hit_location: HitLocation,
) {
    let damage_calc = DamageCalculation::new(attacker, result, hit_location);
    damage_calc.log_hit_details();

    if let Some(wound_level) = result.wound_level {
        apply_wound_with_effects(defender, wound_level, hit_location);
    } else {
        log_damage_absorbed();
    }
}

/// Encapsulates damage calculation logic with all modifiers
struct DamageCalculation {
    base_damage: i32,
    location_multiplier: f32,
    stance_modifier: i32,
    final_damage: i32,
    location: HitLocation,
}

impl DamageCalculation {
    fn new(attacker: &Combatant, result: &CombatResult, location: HitLocation) -> Self {
        let stance_modifier = attacker.stance_damage_modifier();
        let location_multiplier = location.damage_multiplier();
        let final_damage = Self::calculate_final_damage(
            result.damage,
            location_multiplier,
            stance_modifier,
        );

        Self {
            base_damage: result.damage,
            location_multiplier,
            stance_modifier,
            final_damage,
            location,
        }
    }

    fn calculate_final_damage(
        base_damage: i32,
        location_multiplier: f32,
        stance_modifier: i32,
    ) -> i32 {
        // Apply location multiplier first, then add stance modifier
        (base_damage as f32 * location_multiplier) as i32 + stance_modifier
    }

    fn log_hit_details(&self) {
        println!(
            "  → HIT to {}! Base damage: {}, Location mult: {:.2}x, Stance bonus: {:+}, Final: {}",
            self.location,
            self.base_damage,
            self.location_multiplier,
            self.stance_modifier,
            self.final_damage
        );
    }
}

// ============================================================================
// Internal Helpers - Wound Application
// ============================================================================

/// Apply wound to location and handle any special effects (disabling, weapon drops, etc.)
fn apply_wound_with_effects(
    defender: &mut Combatant,
    wound_level: WoundLevel,
    location: HitLocation,
) {
    let severity = convert_wound_level_to_severity(wound_level);
    let was_disabled = defender.add_location_wound(location, severity);

    if was_disabled {
        handle_location_disabled(defender, location);
    }

    log_wound_inflicted(wound_level);
}

/// Convert between wound level representations
#[inline]
fn convert_wound_level_to_severity(wound: WoundLevel) -> hit_location::WoundSeverity {
    match wound {
        WoundLevel::Light => hit_location::WoundSeverity::Light,
        WoundLevel::Severe => hit_location::WoundSeverity::Severe,
        WoundLevel::Critical => hit_location::WoundSeverity::Critical,
    }
}

/// Handle effects when a body location becomes disabled
fn handle_location_disabled(defender: &mut Combatant, location: HitLocation) {
    println!("  → {} is {}", location.to_string().red(), "DISABLED!".red());

    if location.causes_weapon_drop() {
        println!("  → {} {} their weapon!", defender.character.name, "drops".red());
    }
}

// ============================================================================
// Logging Helpers
// ============================================================================

fn log_missed_attack(result: &CombatResult) {
    println!(
        "  → MISS (Attack: {} vs Defense: {})",
        result.attack_roll, result.defense_roll
    );
}

fn log_damage_absorbed() {
    println!("  → Damage absorbed (no wound)");
}

fn log_wound_inflicted(wound_level: WoundLevel) {
    println!("  → {} wound inflicted", wound_level.to_string().red());
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attack_direction_cycle() {
        // Verify attack directions cycle correctly
        assert_eq!(
            attack_direction_for_round(0),
            AttackDirection::Above
        );
        assert_eq!(
            attack_direction_for_round(1),
            AttackDirection::Left
        );
        assert_eq!(
            attack_direction_for_round(2),
            AttackDirection::Front
        );
        
        // Verify cycle repeats
        assert_eq!(
            attack_direction_for_round(3),
            AttackDirection::Above
        );
        assert_eq!(
            attack_direction_for_round(4),
            AttackDirection::Left
        );
    }

    #[test]
    fn test_hit_location_determinism() {
        // Verify that same round produces same direction (though location may vary)
        let direction_0a = attack_direction_for_round(0);
        let direction_0b = attack_direction_for_round(0);
        assert_eq!(direction_0a, direction_0b);
    }

    #[test]
    fn test_hit_location_validity() {
        // Test that hit locations are always valid regardless of round
        for round in 0..10 {
            let location = determine_hit_location(round);
            assert!(matches!(
                location,
                HitLocation::Head
                    | HitLocation::Torso
                    | HitLocation::LeftArm
                    | HitLocation::RightArm
                    | HitLocation::LeftLeg
                    | HitLocation::RightLeg
            ));
        }
    }

    #[test]
    fn test_wound_severity_conversion() {
        assert!(matches!(
            convert_wound_level_to_severity(WoundLevel::Light),
            hit_location::WoundSeverity::Light
        ));
        assert!(matches!(
            convert_wound_level_to_severity(WoundLevel::Severe),
            hit_location::WoundSeverity::Severe
        ));
        assert!(matches!(
            convert_wound_level_to_severity(WoundLevel::Critical),
            hit_location::WoundSeverity::Critical
        ));
    }

    #[test]
    fn test_damage_calculation_with_multipliers() {
        // Test damage calculation logic in isolation
        let base_damage = 10;
        let location_mult = 1.5;
        let stance_mod = 2;
        
        let result = DamageCalculation::calculate_final_damage(
            base_damage,
            location_mult,
            stance_mod,
        );
        
        // (10 * 1.5) as i32 + 2 = 15 + 2 = 17
        assert_eq!(result, 17);
    }

    #[test]
    fn test_damage_calculation_no_multiplier() {
        let base_damage = 10;
        let location_mult = 1.0;
        let stance_mod = 0;
        
        let result = DamageCalculation::calculate_final_damage(
            base_damage,
            location_mult,
            stance_mod,
        );
        
        assert_eq!(result, 10);
    }
}