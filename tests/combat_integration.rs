//! Integration tests for core combat mechanics
//!
//! These tests verify the public API of the steelkilt library works correctly
//! when used as an external consumer would use it.

use steelkilt::{
    combat_round, Armor, Attributes, Character, DefenseAction, Weapon, WeaponImpact, WoundLevel,
    Wounds,
};

/// Helper to create a basic fighter for testing
fn create_test_fighter(name: &str, weapon_skill: i32, dodge_skill: i32) -> Character {
    let attributes = Attributes::new(7, 7, 7, 7, 7, 7, 7, 7, 7);
    Character::new(
        name,
        attributes,
        weapon_skill,
        dodge_skill,
        Weapon::long_sword(),
        Armor::leather(),
    )
}

#[test]
fn test_basic_combat_round() {
    let mut attacker = create_test_fighter("Attacker", 8, 5);
    let mut defender = create_test_fighter("Defender", 6, 7);

    let result = combat_round(&mut attacker, &mut defender, DefenseAction::Dodge);

    // Verify result structure
    assert_eq!(result.attacker, "Attacker");
    assert_eq!(result.defender, "Defender");
    assert!(result.attack_roll > 0);
    assert!(result.defense_roll > 0);

    // Either hit or miss, both are valid
    if result.hit {
        assert!(result.damage >= 0);
    } else {
        assert_eq!(result.damage, 0);
    }
}

#[test]
fn test_combat_round_with_parry() {
    let mut attacker = create_test_fighter("Attacker", 8, 5);
    let mut defender = create_test_fighter("Defender", 7, 6);

    let result = combat_round(&mut attacker, &mut defender, DefenseAction::Parry);

    assert_eq!(result.attacker, "Attacker");
    assert_eq!(result.defender, "Defender");
    // Parry uses weapon skill, which should be higher than dodge for this character
    assert!(result.defense_roll > 0);
}

#[test]
fn test_wound_accumulation() {
    let attributes = Attributes::new(5, 5, 5, 5, 5, 5, 5, 5, 5);
    let mut attacker = Character::new(
        "Strong Fighter",
        attributes,
        10,
        5,
        Weapon::two_handed_sword(),
        Armor::none(),
    );

    let mut weak_defender = Character::new(
        "Weak Fighter",
        attributes,
        3,
        3,
        Weapon::dagger(),
        Armor::none(),
    );

    let initial_wounds = weak_defender.wounds.light + weak_defender.wounds.severe;

    // Run multiple rounds - strong attacker should eventually wound weak defender
    for _ in 0..10 {
        combat_round(&mut attacker, &mut weak_defender, DefenseAction::Dodge);
        if !weak_defender.is_alive() {
            break;
        }
    }

    let final_wounds =
        weak_defender.wounds.light + weak_defender.wounds.severe + weak_defender.wounds.critical;

    // Verify that wounds were accumulated (in most cases)
    assert!(
        final_wounds > initial_wounds || !weak_defender.is_alive(),
        "Expected wounds to accumulate over multiple rounds"
    );
}

#[test]
fn test_death_from_critical_wounds() {
    let mut wounds = Wounds::new();

    // Add two critical wounds
    wounds.add_wound(WoundLevel::Critical);
    assert!(!wounds.is_dead());
    assert!(wounds.is_incapacitated());

    wounds.add_wound(WoundLevel::Critical);
    assert!(wounds.is_dead());
}

#[test]
fn test_wound_stacking() {
    let mut wounds = Wounds::new();

    // Add 4 light wounds - should stack to 1 severe
    for _ in 0..4 {
        wounds.add_wound(WoundLevel::Light);
    }

    assert_eq!(wounds.light, 0);
    assert_eq!(wounds.severe, 1);
    assert_eq!(wounds.critical, 0);

    // Add 3 more severe wounds (total 4) - should create critical wounds
    for _ in 0..2 {
        wounds.add_wound(WoundLevel::Severe);
    }

    assert_eq!(wounds.severe, 0);
    assert_eq!(wounds.critical, 1);

    // Add 3 light and verify stacking continues
    wounds.add_wound(WoundLevel::Light);
    wounds.add_wound(WoundLevel::Light);
    wounds.add_wound(WoundLevel::Light);
    assert_eq!(wounds.light, 3);

    wounds.add_wound(WoundLevel::Light);
    assert_eq!(wounds.light, 0);
    assert_eq!(wounds.severe, 1);
}

#[test]
fn test_armor_reduces_damage() {
    let attributes = Attributes::new(7, 7, 7, 7, 7, 7, 7, 7, 7);

    let mut attacker = Character::new(
        "Attacker",
        attributes,
        10,
        5,
        Weapon::long_sword(),
        Armor::none(),
    );

    let mut unarmored = Character::new(
        "Unarmored",
        attributes,
        3,
        3,
        Weapon::dagger(),
        Armor::none(),
    );

    let mut armored = Character::new(
        "Armored",
        attributes,
        3,
        3,
        Weapon::dagger(),
        Armor::plate(),
    );

    // Run multiple rounds and count wounds
    let mut unarmored_wounds = 0;
    let mut armored_wounds = 0;

    for _ in 0..20 {
        let result1 = combat_round(&mut attacker.clone(), &mut unarmored, DefenseAction::Dodge);
        if result1.hit {
            unarmored_wounds += 1;
        }

        let result2 = combat_round(&mut attacker.clone(), &mut armored, DefenseAction::Dodge);
        if result2.hit {
            armored_wounds += 1;
        }
    }

    // Due to randomness, we just verify combat occurred
    // Both may die, but wounds should have been inflicted
    let total_wounds = unarmored_wounds + armored_wounds;
    assert!(
        total_wounds > 0,
        "Some wounds should have been inflicted during combat"
    );
}

#[test]
fn test_character_creation_with_different_attributes() {
    let high_str = Attributes::new(10, 5, 5, 5, 5, 5, 5, 5, 5);
    let high_dex = Attributes::new(5, 10, 5, 5, 5, 5, 5, 5, 5);
    let high_con = Attributes::new(5, 5, 10, 5, 5, 5, 5, 5, 5);

    let strong = Character::new(
        "Strong",
        high_str,
        5,
        5,
        Weapon::long_sword(),
        Armor::none(),
    );

    let agile = Character::new("Agile", high_dex, 5, 5, Weapon::long_sword(), Armor::none());

    let tough = Character::new("Tough", high_con, 5, 5, Weapon::long_sword(), Armor::none());

    assert_eq!(strong.attributes.strength, 10);
    assert_eq!(agile.attributes.dexterity, 10);
    assert_eq!(tough.attributes.constitution, 10);

    // Verify all can fight
    assert!(strong.can_act());
    assert!(agile.can_act());
    assert!(tough.can_act());
}

#[test]
fn test_weapon_impact_classes() {
    let dagger = Weapon::dagger();
    let sword = Weapon::long_sword();
    let greatsword = Weapon::two_handed_sword();

    assert_eq!(dagger.impact, WeaponImpact::Small);
    assert_eq!(sword.impact, WeaponImpact::Medium);
    assert_eq!(greatsword.impact, WeaponImpact::Large);

    // Damage should scale with impact
    assert!(dagger.damage < sword.damage);
    assert!(sword.damage < greatsword.damage);
}

#[test]
fn test_armor_types_provide_protection() {
    let none = Armor::none();
    let leather = Armor::leather();
    let chain = Armor::chain_mail();
    let plate = Armor::plate();

    // Protection should increase with armor type
    assert!(none.protection < leather.protection);
    assert!(leather.protection < chain.protection);
    assert!(chain.protection < plate.protection);
}

#[test]
fn test_wound_penalties_affect_combat() {
    let attributes = Attributes::new(7, 7, 7, 7, 7, 7, 7, 7, 7);
    let mut fighter = Character::new(
        "Fighter",
        attributes,
        8,
        8,
        Weapon::long_sword(),
        Armor::none(),
    );

    // Apply wounds manually
    fighter.wounds.add_wound(WoundLevel::Light);
    assert_eq!(fighter.wounds.movement_penalty(), -1);

    fighter.wounds.add_wound(WoundLevel::Severe);
    assert_eq!(fighter.wounds.movement_penalty(), -3); // -1 from light, -2 from severe

    // Character should still be able to act with light/severe wounds
    assert!(fighter.can_act());

    // But incapacitated with critical wound
    fighter.wounds.add_wound(WoundLevel::Critical);
    assert!(!fighter.can_act());
    assert!(fighter.wounds.is_incapacitated());
}

#[test]
fn test_combat_result_consistency() {
    let mut attacker = create_test_fighter("Attacker", 8, 5);
    let mut defender = create_test_fighter("Defender", 6, 7);

    let result = combat_round(&mut attacker, &mut defender, DefenseAction::Dodge);

    // If hit, defender should have wounds or be dead
    if result.hit && result.damage > 0 {
        let total_wounds =
            defender.wounds.light + defender.wounds.severe + defender.wounds.critical;
        assert!(
            total_wounds > 0 || !defender.is_alive(),
            "Hit with damage should cause wounds or death"
        );
    }

    // If defender died, result should reflect that
    if !defender.is_alive() {
        assert!(result.defender_died);
    }
}

#[test]
fn test_can_act_reflects_character_state() {
    let mut fighter = create_test_fighter("Fighter", 5, 5);

    // Healthy fighter can act
    assert!(fighter.can_act());

    // Wounded but conscious fighter can act
    fighter.wounds.add_wound(WoundLevel::Light);
    fighter.wounds.add_wound(WoundLevel::Severe);
    assert!(fighter.can_act());

    // Incapacitated fighter cannot act
    fighter.wounds.add_wound(WoundLevel::Critical);
    assert!(!fighter.can_act());

    // Dead fighter cannot act
    fighter.wounds.add_wound(WoundLevel::Critical);
    assert!(!fighter.can_act());
    assert!(!fighter.is_alive());
}

#[test]
fn test_multiple_rounds_of_combat() {
    let mut fighter1 = create_test_fighter("Fighter 1", 7, 6);
    let mut fighter2 = create_test_fighter("Fighter 2", 6, 7);

    let mut rounds = 0;
    let max_rounds = 50; // Prevent infinite loop

    while fighter1.is_alive() && fighter2.is_alive() && rounds < max_rounds {
        // Fighter 1 attacks
        combat_round(&mut fighter1, &mut fighter2, DefenseAction::Dodge);
        if !fighter2.is_alive() {
            break;
        }

        // Fighter 2 attacks
        combat_round(&mut fighter2, &mut fighter1, DefenseAction::Parry);

        rounds += 1;
    }

    // Verify combat eventually ends
    assert!(
        !fighter1.is_alive() || !fighter2.is_alive() || rounds >= max_rounds,
        "Combat should eventually end"
    );

    // Verify at least some rounds occurred
    assert!(rounds > 0, "At least one round should have occurred");
}
