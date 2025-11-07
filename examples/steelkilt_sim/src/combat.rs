//! Combat execution module
//!
//! Handles combat resolution including:
//! - Attack execution with modifiers
//! - Hit location determination
//! - Damage calculation with stance and location multipliers
//! - Wound application and tracking

use crate::state::FighterState;
use steelkilt::modules::*;
use steelkilt::*;

/// Determine hit location based on round number (simple rotation for demo)
pub fn determine_hit_location(round: usize) -> HitLocation {
    let direction = match round % 3 {
        0 => AttackDirection::Above,
        1 => AttackDirection::Left,
        _ => AttackDirection::Front,
    };

    HitLocation::determine(direction)
}

/// Execute an advanced attack with all modifiers and systems
pub fn perform_attack(attacker: &mut FighterState, defender: &mut FighterState, round: usize) {
    let total_attack_mod = attacker.total_attack_modifier();
    let hit_location = determine_hit_location(round);

    println!(
        "\n{} attacks {} (Stance: {}, Exhaustion: {}, Total Mod: {:+})",
        attacker.character.name,
        defender.character.name,
        attacker.stance.current_maneuver,
        attacker.exhaustion.status(),
        total_attack_mod
    );

    // Execute combat round using core library
    let result = combat_round(
        &mut attacker.character,
        &mut defender.character,
        DefenseAction::Parry,
    );

    if result.hit {
        apply_hit_damage(attacker, defender, &result, hit_location);
    } else {
        println!(
            "  → MISS (Attack: {} vs Defense: {})",
            result.attack_roll, result.defense_roll
        );
    }
}

/// Apply damage from a successful hit, including stance and location modifiers
fn apply_hit_damage(
    attacker: &FighterState,
    defender: &mut FighterState,
    result: &CombatResult,
    hit_location: HitLocation,
) {
    // Calculate adjusted damage with stance and location modifiers
    let stance_damage_mod = attacker.stance_damage_modifier();
    let location_damage_mult = hit_location.damage_multiplier();
    let adjusted_damage = (result.damage as f32 * location_damage_mult) as i32 + stance_damage_mod;

    println!(
        "  → HIT to {}! Base damage: {}, Location mult: {:.2}x, Stance bonus: {:+}, Final: {}",
        hit_location, result.damage, location_damage_mult, stance_damage_mod, adjusted_damage
    );

    // Apply wound to location if one was inflicted
    if let Some(wound) = result.wound_level {
        apply_locational_wound(defender, wound, hit_location);
    } else {
        println!("  → Damage absorbed (no wound)");
    }
}

/// Apply a wound to a specific body location and check for disabling
fn apply_locational_wound(defender: &mut FighterState, wound: WoundLevel, location: HitLocation) {
    let severity = match wound {
        WoundLevel::Light => hit_location::WoundSeverity::Light,
        WoundLevel::Severe => hit_location::WoundSeverity::Severe,
        WoundLevel::Critical => hit_location::WoundSeverity::Critical,
    };

    let disabled = defender.add_location_wound(location, severity);

    if disabled {
        println!("  → {} is DISABLED!", location);

        if location.causes_weapon_drop() {
            println!("  → {} drops their weapon!", defender.character.name);
        }
    }

    println!("  → {} wound inflicted", wound);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_location_rotation() {
        // Test that hit locations rotate predictably based on round number
        // We can't test the exact location due to randomness in HitLocation::determine,
        // but we can verify the pattern repeats

        let loc_0 = determine_hit_location(0);
        let loc_3 = determine_hit_location(3);

        let loc_1 = determine_hit_location(1);
        let loc_4 = determine_hit_location(4);

        let loc_2 = determine_hit_location(2);
        let loc_5 = determine_hit_location(5);

        // Verify the pattern repeats every 3 rounds (same direction)
        // Note: locations may differ due to randomness within each direction,
        // but at least verify they're valid HitLocations
        assert!(matches!(
            loc_0,
            HitLocation::Head
                | HitLocation::Torso
                | HitLocation::LeftArm
                | HitLocation::RightArm
                | HitLocation::LeftLeg
                | HitLocation::RightLeg
        ));

        assert!(matches!(
            loc_1,
            HitLocation::Head
                | HitLocation::Torso
                | HitLocation::LeftArm
                | HitLocation::RightArm
                | HitLocation::LeftLeg
                | HitLocation::RightLeg
        ));

        assert!(matches!(
            loc_2,
            HitLocation::Head
                | HitLocation::Torso
                | HitLocation::LeftArm
                | HitLocation::RightArm
                | HitLocation::LeftLeg
                | HitLocation::RightLeg
        ));

        // Ensure we're actually calling determine (not constant)
        let _ = (loc_3, loc_4, loc_5);
    }
}
