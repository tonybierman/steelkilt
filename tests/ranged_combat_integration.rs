//! Integration tests for ranged combat
//!
//! Tests ranged combat mechanics following Draft RPG Section 4.21

use steelkilt::modules::ranged_combat::{RangedAttackState, RangedWeapon, TargetSize};

#[test]
fn test_ranged_weapon_creation() {
    let bow = RangedWeapon::long_bow();

    assert_eq!(bow.name, "Long Bow");
    assert_eq!(bow.damage, 6);
    assert_eq!(bow.point_blank_range, 30);
    assert_eq!(bow.max_range, 120);
    assert_eq!(bow.rate_of_fire, 1);
}

#[test]
fn test_all_weapon_types() {
    let short_bow = RangedWeapon::short_bow();
    let long_bow = RangedWeapon::long_bow();
    let crossbow = RangedWeapon::crossbow();
    let pistol = RangedWeapon::pistol();
    let rifle = RangedWeapon::rifle();
    let javelin = RangedWeapon::javelin();

    // Verify they all have valid properties
    assert!(short_bow.damage > 0);
    assert!(long_bow.damage > 0);
    assert!(crossbow.damage > 0);
    assert!(pistol.damage > 0);
    assert!(rifle.damage > 0);
    assert!(javelin.damage > 0);

    // Bows should have consistent naming
    assert!(short_bow.name.contains("Bow"));
    assert!(long_bow.name.contains("Bow"));
}

#[test]
fn test_weapon_damage_values() {
    let short_bow = RangedWeapon::short_bow();
    let long_bow = RangedWeapon::long_bow();
    let crossbow = RangedWeapon::crossbow();
    let pistol = RangedWeapon::pistol();
    let rifle = RangedWeapon::rifle();

    // Rifle should be most damaging
    assert!(rifle.damage > pistol.damage);
    assert!(rifle.damage > crossbow.damage);

    // Crossbow and long bow have same damage
    assert_eq!(crossbow.damage, long_bow.damage);

    // Long bow more damaging than short bow
    assert!(long_bow.damage > short_bow.damage);
}

#[test]
fn test_weapon_ranges() {
    let long_bow = RangedWeapon::long_bow();
    let rifle = RangedWeapon::rifle();
    let javelin = RangedWeapon::javelin();

    // Rifle has longest range
    assert!(rifle.max_range > long_bow.max_range);
    assert!(rifle.max_range > javelin.max_range);

    // Javelin has shortest range
    assert!(javelin.max_range < long_bow.max_range);
    assert!(javelin.max_range < rifle.max_range);
}

#[test]
fn test_distance_modifier_at_point_blank() {
    let bow = RangedWeapon::long_bow();

    // At point blank range (30m or less), no modifier
    assert_eq!(bow.distance_modifier(10), 0);
    assert_eq!(bow.distance_modifier(30), 0);
}

#[test]
fn test_distance_modifier_beyond_point_blank() {
    let bow = RangedWeapon::long_bow();

    // Beyond point blank: -1 per 10m for bows
    // At 40m: (40 - 30) / 10 = 1, so -1
    assert_eq!(bow.distance_modifier(40), -1);

    // At 50m: (50 - 30) / 10 = 2, so -2
    assert_eq!(bow.distance_modifier(50), -2);

    // At 80m: (80 - 30) / 10 = 5, so -5
    assert_eq!(bow.distance_modifier(80), -5);
}

#[test]
fn test_distance_modifier_for_guns() {
    let pistol = RangedWeapon::pistol();

    // Pistol point blank is 20m
    assert_eq!(pistol.distance_modifier(20), 0);

    // Beyond point blank: -1 per 20m for guns
    // At 40m: (40 - 20) / 20 = 1, so -1
    assert_eq!(pistol.distance_modifier(40), -1);

    // At 60m: (60 - 20) / 20 = 2, so -2
    assert_eq!(pistol.distance_modifier(60), -2);
}

#[test]
fn test_distance_modifier_out_of_range() {
    let bow = RangedWeapon::long_bow();

    // Beyond max range (120m) should return large negative
    let modifier = bow.distance_modifier(150);
    assert!(modifier < -100); // Very large penalty
}

#[test]
fn test_in_range_check() {
    let bow = RangedWeapon::long_bow();

    assert!(bow.in_range(10));
    assert!(bow.in_range(50));
    assert!(bow.in_range(120)); // Exactly at max range
    assert!(!bow.in_range(121)); // Beyond max range
    assert!(!bow.in_range(200));
}

#[test]
fn test_target_size_modifiers() {
    assert_eq!(TargetSize::Tiny.modifier(), -4);
    assert_eq!(TargetSize::Small.modifier(), -2);
    assert_eq!(TargetSize::Medium.modifier(), 0);
    assert_eq!(TargetSize::Large.modifier(), 2);
    assert_eq!(TargetSize::Huge.modifier(), 4);
    assert_eq!(TargetSize::Gigantic.modifier(), 6);
}

#[test]
fn test_target_size_progression() {
    // Modifiers should increase with size
    assert!(TargetSize::Tiny.modifier() < TargetSize::Small.modifier());
    assert!(TargetSize::Small.modifier() < TargetSize::Medium.modifier());
    assert!(TargetSize::Medium.modifier() < TargetSize::Large.modifier());
    assert!(TargetSize::Large.modifier() < TargetSize::Huge.modifier());
    assert!(TargetSize::Huge.modifier() < TargetSize::Gigantic.modifier());
}

#[test]
fn test_ranged_attack_state_creation() {
    let state = RangedAttackState::new();

    assert!(!state.weapon_ready);
    assert!(!state.aiming);
    assert_eq!(state.aiming_rounds, 0);
    assert_eq!(state.shots_remaining, 0);
}

#[test]
fn test_weapon_rate_of_fire() {
    let bow = RangedWeapon::long_bow();
    let pistol = RangedWeapon::pistol();
    let rifle = RangedWeapon::rifle();

    // Bow fires once per round
    assert_eq!(bow.rate_of_fire, 1);

    // Pistol fires multiple times
    assert_eq!(pistol.rate_of_fire, 3);

    // Rifle fires twice
    assert_eq!(rifle.rate_of_fire, 2);

    // Pistol has highest rate of fire
    assert!(pistol.rate_of_fire > rifle.rate_of_fire);
    assert!(pistol.rate_of_fire > bow.rate_of_fire);
}

#[test]
fn test_weapon_preparation_times() {
    let bow = RangedWeapon::long_bow();
    let crossbow = RangedWeapon::crossbow();
    let pistol = RangedWeapon::pistol();

    // Crossbow takes longest to prepare (reload)
    assert!(crossbow.preparation_time > bow.preparation_time);
    assert!(crossbow.preparation_time > pistol.preparation_time);

    // Pistol is fastest
    assert!(pistol.preparation_time < bow.preparation_time);
    assert!(pistol.preparation_time < crossbow.preparation_time);
}

#[test]
fn test_complete_ranged_attack_scenario() {
    let archer = RangedWeapon::long_bow();
    let mut state = RangedAttackState::new();

    // Archer prepares weapon
    state.weapon_ready = true;
    state.shots_remaining = 1;

    // Check distance to target (50m)
    let distance = 50;
    assert!(archer.in_range(distance));

    // Calculate modifier: 50m is beyond point blank (30m)
    // (50 - 30) / 10 = 2, so -2 modifier
    let distance_mod = archer.distance_modifier(distance);
    assert_eq!(distance_mod, -2);

    // Target is medium sized (human)
    let size_mod = TargetSize::Medium.modifier();
    assert_eq!(size_mod, 0);

    // Archer aims for a round
    state.aiming = true;
    state.aiming_rounds = 1;

    // Total modifiers: distance (-2) + size (0) + aiming (+1) = -1
    // (Note: aiming bonus calculated elsewhere, but state tracks it)

    // Fire!
    state.shots_remaining -= 1;
    assert_eq!(state.shots_remaining, 0);

    // After firing, weapon needs to be readied again
    state.weapon_ready = false;
    state.aiming = false;
    state.aiming_rounds = 0;
}

#[test]
fn test_bow_vs_gun_distance_modifiers() {
    let bow = RangedWeapon::long_bow();
    let rifle = RangedWeapon::rifle();

    // At 80m:
    // Bow: (80 - 30) / 10 = 5, so -5
    // Rifle: (80 - 40) / 20 = 2, so -2
    let bow_mod = bow.distance_modifier(80);
    let rifle_mod = rifle.distance_modifier(80);

    assert_eq!(bow_mod, -5);
    assert_eq!(rifle_mod, -2);

    // Rifles are better at long range
    assert!(rifle_mod > bow_mod);
}

#[test]
fn test_javelin_characteristics() {
    let javelin = RangedWeapon::javelin();

    // Javelin is short range thrown weapon
    assert!(javelin.max_range < 50);
    assert_eq!(javelin.point_blank_range, 15);

    // Quick to throw
    assert_eq!(javelin.preparation_time, 1);

    // Uses bow distance mechanics (10m increments)
    let modifier_25m = javelin.distance_modifier(25);
    // (25 - 15) / 10 = 1, so -1
    assert_eq!(modifier_25m, -1);
}

#[test]
fn test_crossbow_characteristics() {
    let crossbow = RangedWeapon::crossbow();

    // Powerful but slow to reload
    assert_eq!(crossbow.damage, 6);
    assert_eq!(crossbow.preparation_time, 6); // Longest prep time

    // Reasonable range
    assert_eq!(crossbow.max_range, 100);
    assert_eq!(crossbow.point_blank_range, 30);
}

#[test]
fn test_firearms_characteristics() {
    let pistol = RangedWeapon::pistol();
    let rifle = RangedWeapon::rifle();

    // Rifle is more powerful
    assert!(rifle.damage > pistol.damage);

    // Rifle has longer range
    assert!(rifle.max_range > pistol.max_range);
    assert!(rifle.point_blank_range > pistol.point_blank_range);

    // Pistol fires faster
    assert!(pistol.rate_of_fire > rifle.rate_of_fire);

    // Both prepare quickly compared to crossbow
    let crossbow = RangedWeapon::crossbow();
    assert!(pistol.preparation_time < crossbow.preparation_time);
    assert!(rifle.preparation_time < crossbow.preparation_time);
}

#[test]
fn test_extreme_ranges() {
    let rifle = RangedWeapon::rifle();

    // At maximum range, should still be valid but with high penalty
    assert!(rifle.in_range(200));
    let max_range_modifier = rifle.distance_modifier(200);
    // (200 - 40) / 20 = 8, so -8
    assert_eq!(max_range_modifier, -8);

    // Beyond maximum range
    assert!(!rifle.in_range(201));
    let out_of_range_modifier = rifle.distance_modifier(250);
    assert!(out_of_range_modifier < -100);
}

#[test]
fn test_aiming_state_progression() {
    let mut state = RangedAttackState::new();

    // Start with weapon not ready
    assert!(!state.weapon_ready);
    assert_eq!(state.aiming_rounds, 0);

    // Ready the weapon
    state.weapon_ready = true;
    state.shots_remaining = 1;

    // Start aiming
    state.aiming = true;
    state.aiming_rounds = 0;

    // Aim for multiple rounds
    state.aiming_rounds += 1;
    assert_eq!(state.aiming_rounds, 1);

    state.aiming_rounds += 1;
    assert_eq!(state.aiming_rounds, 2);

    // Fire
    state.shots_remaining -= 1;
    assert_eq!(state.shots_remaining, 0);

    // Reset after firing
    state.weapon_ready = false;
    state.aiming = false;
    state.aiming_rounds = 0;
}

#[test]
fn test_multiple_shots_with_high_rate_of_fire() {
    let pistol = RangedWeapon::pistol();
    let mut state = RangedAttackState::new();

    // Ready weapon with full capacity
    state.weapon_ready = true;
    state.shots_remaining = pistol.rate_of_fire;

    assert_eq!(state.shots_remaining, 3);

    // Fire first shot
    state.shots_remaining -= 1;
    assert_eq!(state.shots_remaining, 2);

    // Fire second shot
    state.shots_remaining -= 1;
    assert_eq!(state.shots_remaining, 1);

    // Fire third shot
    state.shots_remaining -= 1;
    assert_eq!(state.shots_remaining, 0);

    // Out of shots for this round
    assert_eq!(state.shots_remaining, 0);
}
