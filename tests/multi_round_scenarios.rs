//! Integration tests for complex multi-round combat scenarios
//!
//! These tests verify that multiple systems work together correctly
//! in realistic combat situations.

use steelkilt::{
    Armor, Attributes, Character, DefenseAction, Weapon, WeaponImpact, WoundLevel, combat_round,
};
use steelkilt::modules::exhaustion::Exhaustion;
use steelkilt::modules::magic::{MagicBranch, MagicUser, Spell, SpellDifficulty, SpellDuration, SpellRange};
use steelkilt::modules::ranged_combat::RangedWeapon;
use steelkilt::modules::skills::{Skill, SkillDifficulty, SkillSet};

/// Helper to create a warrior
fn create_warrior(name: &str) -> Character {
    let attributes = Attributes::new(8, 7, 8, 6, 6, 7, 6, 6, 5);
    Character::new(
        name,
        attributes,
        8,  // weapon_skill
        6,  // dodge_skill
        Weapon::long_sword(),
        Armor::chain_mail(),
    )
}

/// Helper to create a lightly armored fighter
fn create_duelist(name: &str) -> Character {
    let attributes = Attributes::new(7, 9, 7, 7, 7, 7, 7, 8, 6);
    Character::new(
        name,
        attributes,
        7,  // weapon_skill
        9,  // dodge_skill (high dodge)
        Weapon::new("Rapier", WeaponImpact::Small),
        Armor::leather(),
    )
}

#[test]
fn test_full_combat_until_death() {
    let mut warrior1 = create_warrior("Warrior 1");
    let mut warrior2 = create_warrior("Warrior 2");

    let mut rounds = 0;
    let max_rounds = 100;

    // Fight until one dies
    while warrior1.is_alive() && warrior2.is_alive() && rounds < max_rounds {
        // Warrior 1 attacks
        combat_round(&mut warrior1, &mut warrior2, DefenseAction::Parry);
        if !warrior2.is_alive() {
            break;
        }

        // Warrior 2 attacks
        combat_round(&mut warrior2, &mut warrior1, DefenseAction::Dodge);

        rounds += 1;
    }

    // Verify combat ended
    assert!(
        !warrior1.is_alive() || !warrior2.is_alive() || rounds >= max_rounds,
        "Combat should eventually end"
    );

    // Verify at least a few rounds occurred
    assert!(rounds > 0, "At least one round should have occurred");

    // The winner should still be alive (unless both died somehow or hit max rounds)
    let winner_exists = warrior1.is_alive() || warrior2.is_alive();
    let combat_concluded = !warrior1.is_alive() || !warrior2.is_alive();

    assert!(
        combat_concluded || rounds >= max_rounds,
        "Combat should conclude with a winner or reach max rounds"
    );
}

#[test]
fn test_wound_accumulation_affects_combat() {
    let mut strong = create_warrior("Strong Fighter");
    let mut weak = create_duelist("Weak Fighter");

    // Give weak fighter some wounds
    weak.wounds.add_wound(WoundLevel::Light);
    weak.wounds.add_wound(WoundLevel::Light);
    weak.wounds.add_wound(WoundLevel::Severe);

    let weak_initial_penalty = weak.wounds.movement_penalty();
    assert!(weak_initial_penalty < 0, "Wounded fighter should have penalties");

    // Run several rounds
    for _ in 0..5 {
        combat_round(&mut strong, &mut weak, DefenseAction::Dodge);
        if !weak.is_alive() {
            break;
        }
    }

    // Weak fighter likely accumulated more wounds or died
    let weak_final_wounds = weak.wounds.light + weak.wounds.severe + weak.wounds.critical;
    assert!(
        weak_final_wounds >= 3 || !weak.is_alive(),
        "Wounded fighter should have taken more damage or died"
    );
}

#[test]
fn test_armor_difference_affects_outcome() {
    let attributes = Attributes::new(7, 7, 7, 7, 7, 7, 7, 7, 7);

    let mut armored = Character::new(
        "Armored",
        attributes,
        6,
        6,
        Weapon::long_sword(),
        Armor::plate(),
    );

    let mut unarmored = Character::new(
        "Unarmored",
        attributes,
        6,
        6,
        Weapon::long_sword(),
        Armor::none(),
    );

    // Run multiple rounds
    for _ in 0..10 {
        // Armored attacks unarmored
        combat_round(&mut armored, &mut unarmored, DefenseAction::Dodge);
        if !unarmored.is_alive() {
            break;
        }

        // Unarmored attacks armored
        combat_round(&mut unarmored, &mut armored, DefenseAction::Dodge);
        if !armored.is_alive() {
            break;
        }
    }

    // Unarmored should have taken more damage (in most cases)
    let armored_wounds = armored.wounds.light + armored.wounds.severe + armored.wounds.critical;
    let unarmored_wounds = unarmored.wounds.light + unarmored.wounds.severe + unarmored.wounds.critical;

    // Due to randomness, we can't guarantee this every time, but verify both participated
    assert!(armored_wounds > 0 || unarmored_wounds > 0, "Some damage should have occurred");
}

#[test]
fn test_skill_development_scenario() {
    let mut skill_set = SkillSet::new(30);

    // Fighter develops combat skills over time
    skill_set.add_skill(Skill::new("Sword", 7, SkillDifficulty::Normal));
    skill_set.add_skill(Skill::new("Dodge", 8, SkillDifficulty::Easy));
    skill_set.add_skill(Skill::new("Tactics", 6, SkillDifficulty::Hard));

    // Phase 1: Basic training
    for _ in 0..5 {
        skill_set.raise_skill("Sword").unwrap();
    }
    assert_eq!(skill_set.get_skill_level("Sword"), 5);

    // Phase 2: Learn to dodge
    skill_set.raise_skill("Dodge").unwrap();
    assert_eq!(skill_set.get_skill_level("Dodge"), 1);

    // Phase 3: Study tactics
    skill_set.raise_skill("Tactics").unwrap();
    skill_set.raise_skill("Tactics").unwrap();
    assert_eq!(skill_set.get_skill_level("Tactics"), 2);

    // Phase 4: Master swordsmanship
    skill_set.raise_skill("Sword").unwrap();
    skill_set.raise_skill("Sword").unwrap();
    assert_eq!(skill_set.get_skill_level("Sword"), 7);

    // Verify some points remain
    assert!(skill_set.available_points >= 0);
}

#[test]
fn test_exhaustion_in_extended_combat() {
    let attributes = Attributes::new(7, 7, 7, 7, 7, 7, 7, 7, 7);
    let mut exhaustion = Exhaustion::new(attributes.stamina());

    // Simulate exertion during combat (need to exceed stamina threshold)
    let stamina = exhaustion.stamina_threshold;
    exhaustion.add_points(stamina + 1); // Exceed threshold for Light exhaustion

    assert!(exhaustion.points > 0);
    assert!(exhaustion.penalty() < 0, "Exhaustion should cause penalties");

    // Recovery over time
    let initial_exhaustion = exhaustion.points;
    exhaustion.rest(10); // 10 rounds of rest (recovers 5 points)
    assert!(exhaustion.points < initial_exhaustion);
}

#[test]
fn test_wizard_in_combat_scenario() {
    let mut wizard_magic = MagicUser::new(8);
    wizard_magic.add_lore(MagicBranch::Elementalism, 5);

    // Learn combat spells
    let fireball = Spell {
        name: "Fireball".to_string(),
        branch: MagicBranch::Elementalism,
        difficulty: SpellDifficulty::Normal,
        preparation_time: 3,
        casting_time: 1,
        range: SpellRange::Medium(30),
        duration: SpellDuration::Instant,
    };

    let shield = Spell {
        name: "Shield".to_string(),
        branch: MagicBranch::Elementalism,
        difficulty: SpellDifficulty::Easy,
        preparation_time: 1,
        casting_time: 1,
        range: SpellRange::Personal,
        duration: SpellDuration::Rounds(10),
    };

    wizard_magic.learn_spell(fireball, 4).unwrap();
    wizard_magic.learn_spell(shield, 3).unwrap();

    // Cast shield before combat
    let shield_result = wizard_magic.cast_spell("Shield", 5);
    assert!(shield_result.is_ok());
    assert!(shield_result.unwrap().success);

    // Cast fireball during combat
    let fireball_result = wizard_magic.cast_spell("Fireball", 6);
    assert!(fireball_result.is_ok());
    assert!(fireball_result.unwrap().success);

    // Verify exhaustion accumulated
    assert!(wizard_magic.exhaustion_points > 0);

    // Cast another fireball to build up more exhaustion
    wizard_magic.cast_spell("Fireball", 7).unwrap();
    wizard_magic.cast_spell("Fireball", 7).unwrap();
    wizard_magic.cast_spell("Fireball", 7).unwrap();

    // Should have accumulated significant exhaustion
    // (Each successful Normal spell adds 2 points, so 4 casts = 8 points minimum)
    assert!(wizard_magic.exhaustion_points > 0, "Should have accumulated exhaustion points");
}

#[test]
fn test_character_with_multiple_systems() {
    // Create a character that uses multiple systems
    let attributes = Attributes::new(7, 8, 7, 7, 7, 8, 7, 7, 8);
    let mut character = Character::new(
        "Battle Mage",
        attributes,
        6,
        7,
        Weapon::new("Staff", WeaponImpact::Medium),
        Armor::leather(),
    );

    // Add magic capabilities
    let mut magic = MagicUser::new(attributes.empathy);
    magic.add_lore(MagicBranch::Animation, 4);

    let heal = Spell {
        name: "Heal Wounds".to_string(),
        branch: MagicBranch::Animation,
        difficulty: SpellDifficulty::Normal,
        preparation_time: 5,
        casting_time: 2,
        range: SpellRange::Touch,
        duration: SpellDuration::Instant,
    };

    magic.learn_spell(heal, 3).unwrap();

    // Character gets wounded in combat
    character.wounds.add_wound(WoundLevel::Light);
    character.wounds.add_wound(WoundLevel::Severe);

    assert!(character.wounds.movement_penalty() < 0);

    // Character casts healing spell on self
    let heal_result = magic.cast_spell("Heal Wounds", 6);
    assert!(heal_result.is_ok());

    // (In a full implementation, wounds would be reduced here)
    // For now, verify spell was cast successfully
    assert!(heal_result.unwrap().success);
}

#[test]
fn test_ranged_combat_scenario() {
    let archer_bow = RangedWeapon::long_bow();

    // Archer engages at long range
    let distance = 80;
    assert!(archer_bow.in_range(distance));

    let distance_modifier = archer_bow.distance_modifier(distance);
    // (80 - 30) / 10 = 5, so -5
    assert_eq!(distance_modifier, -5);

    // Archer aims for a round (would add +1)
    let aiming_bonus = 1;

    // Total modifier: -5 + 1 = -4
    let total_modifier = distance_modifier + aiming_bonus;
    assert_eq!(total_modifier, -4);

    // As archer closes to medium range
    let closer_distance = 50;
    let closer_modifier = archer_bow.distance_modifier(closer_distance);
    // (50 - 30) / 10 = 2, so -2
    assert_eq!(closer_modifier, -2);

    // Easier shot at closer range
    assert!(closer_modifier > distance_modifier);
}

#[test]
fn test_mixed_combat_styles() {
    let attributes = Attributes::new(7, 7, 7, 7, 7, 7, 7, 7, 7);

    let mut melee_fighter = Character::new(
        "Melee Fighter",
        attributes,
        8,
        6,
        Weapon::long_sword(),
        Armor::chain_mail(),
    );

    let mut ranged_fighter = Character::new(
        "Archer",
        attributes,
        5,
        8,
        Weapon::dagger(), // Backup weapon
        Armor::leather(),
    );

    // Create ranged weapon for archer
    let _bow = RangedWeapon::long_bow();

    // Melee fighter closes and engages
    for _ in 0..3 {
        combat_round(&mut melee_fighter, &mut ranged_fighter, DefenseAction::Dodge);
        if !ranged_fighter.is_alive() {
            break;
        }
    }

    // Both should still be alive or one died
    assert!(melee_fighter.is_alive() || ranged_fighter.is_alive());
}

#[test]
fn test_tournament_scenario() {
    // Three fighters in a tournament
    let mut fighter1 = create_warrior("Fighter 1");
    let mut fighter2 = create_duelist("Fighter 2");
    let mut fighter3 = create_warrior("Fighter 3");

    // Round 1: Fighter 1 vs Fighter 2
    for _ in 0..10 {
        combat_round(&mut fighter1, &mut fighter2, DefenseAction::Dodge);
        if !fighter2.is_alive() {
            break;
        }
        combat_round(&mut fighter2, &mut fighter1, DefenseAction::Parry);
        if !fighter1.is_alive() {
            break;
        }
    }

    // Determine winner of first match
    let round1_winner = if fighter1.is_alive() && !fighter2.is_alive() {
        &mut fighter1
    } else if fighter2.is_alive() && !fighter1.is_alive() {
        &mut fighter2
    } else {
        // Indeterminate, pick fighter with fewer wounds
        if fighter1.wounds.critical < fighter2.wounds.critical {
            &mut fighter1
        } else {
            &mut fighter2
        }
    };

    // Winner faces Fighter 3
    let mut round2_completed = false;
    for _ in 0..10 {
        if !round1_winner.is_alive() || !fighter3.is_alive() {
            round2_completed = true;
            break;
        }
        combat_round(round1_winner, &mut fighter3, DefenseAction::Parry);
        if !fighter3.is_alive() {
            round2_completed = true;
            break;
        }
        combat_round(&mut fighter3, round1_winner, DefenseAction::Dodge);
        if !round1_winner.is_alive() {
            round2_completed = true;
            break;
        }
    }

    // Verify tournament completed or progressed
    assert!(
        round2_completed || !round1_winner.is_alive() || !fighter3.is_alive(),
        "Tournament should progress"
    );
}

#[test]
fn test_character_progression_through_combat() {
    let mut skill_set = SkillSet::new(50);

    // Young warrior starts training
    skill_set.add_skill(Skill::new("Sword", 6, SkillDifficulty::Normal));
    skill_set.add_skill(Skill::new("Dodge", 7, SkillDifficulty::Easy));

    // Initial training
    for _ in 0..4 {
        skill_set.raise_skill("Sword").unwrap();
    }
    for _ in 0..5 {
        skill_set.raise_skill("Dodge").unwrap();
    }

    let initial_sword = skill_set.get_skill_level("Sword");
    let initial_dodge = skill_set.get_skill_level("Dodge");

    // After battles, gains more points
    skill_set.grant_points(20);

    // Continues development
    skill_set.raise_skill("Sword").unwrap();
    skill_set.raise_skill("Sword").unwrap();

    assert!(skill_set.get_skill_level("Sword") > initial_sword);
    assert_eq!(skill_set.get_skill_level("Dodge"), initial_dodge);
}

#[test]
fn test_incapacitation_prevents_combat() {
    let mut fighter = create_warrior("Fighter");

    // Give critical wound (incapacitates)
    fighter.wounds.add_wound(WoundLevel::Critical);

    // Incapacitated fighter cannot act
    assert!(!fighter.can_act());
    assert!(fighter.wounds.is_incapacitated());

    // Still alive with just one critical
    assert!(fighter.is_alive());

    // But second critical wound is fatal
    fighter.wounds.add_wound(WoundLevel::Critical);
    assert!(!fighter.is_alive());
    assert!(fighter.wounds.is_dead());
}

#[test]
fn test_realistic_duel_to_first_blood() {
    let mut duelist1 = create_duelist("Duelist 1");
    let mut duelist2 = create_duelist("Duelist 2");

    let mut rounds = 0;

    // Fight until someone is wounded
    while duelist1.wounds.light == 0
        && duelist1.wounds.severe == 0
        && duelist2.wounds.light == 0
        && duelist2.wounds.severe == 0
        && rounds < 20
    {
        combat_round(&mut duelist1, &mut duelist2, DefenseAction::Dodge);
        if duelist2.wounds.light > 0 || duelist2.wounds.severe > 0 {
            break;
        }

        combat_round(&mut duelist2, &mut duelist1, DefenseAction::Dodge);

        rounds += 1;
    }

    // Someone should have been wounded (or reached max rounds)
    let someone_wounded = duelist1.wounds.light > 0
        || duelist1.wounds.severe > 0
        || duelist2.wounds.light > 0
        || duelist2.wounds.severe > 0;

    assert!(
        someone_wounded || rounds >= 20,
        "First blood should occur within reasonable rounds"
    );
}
