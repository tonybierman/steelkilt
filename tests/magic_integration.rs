//! Integration tests for the magic system
//!
//! Tests magic following Draft RPG Chapter 5

use steelkilt::modules::magic::{
    MagicBranch, MagicError, MagicUser, Spell, SpellDifficulty, SpellDuration, SpellRange,
};

/// Helper to create a test spell
fn create_test_spell(name: &str, branch: MagicBranch, difficulty: SpellDifficulty) -> Spell {
    Spell {
        name: name.to_string(),
        branch,
        difficulty,
        preparation_time: 5,
        casting_time: 1,
        range: SpellRange::Short(10),
        duration: SpellDuration::Minutes(10),
    }
}

#[test]
fn test_magic_user_creation() {
    let magic_user = MagicUser::new(8);

    assert_eq!(magic_user.empathy, 8);
    assert_eq!(magic_user.exhaustion_points, 0);
    assert!(magic_user.lores.is_empty());
    assert!(magic_user.spells.is_empty());
}

#[test]
fn test_add_lore() {
    let mut magic_user = MagicUser::new(7);

    magic_user.add_lore(MagicBranch::Divination, 5);

    assert!(magic_user.lores.contains_key(&MagicBranch::Divination));
    let lore = magic_user.lores.get(&MagicBranch::Divination).unwrap();
    assert_eq!(lore.level, 5);
}

#[test]
fn test_learn_spell_with_sufficient_lore() {
    let mut magic_user = MagicUser::new(8);

    magic_user.add_lore(MagicBranch::Divination, 5);

    let spell = create_test_spell("Detect Magic", MagicBranch::Divination, SpellDifficulty::Easy);

    let result = magic_user.learn_spell(spell, 3);
    assert!(result.is_ok());
    assert!(magic_user.spells.contains_key("Detect Magic"));
}

#[test]
fn test_learn_spell_without_lore() {
    let mut magic_user = MagicUser::new(8);

    let spell = create_test_spell(
        "Fireball",
        MagicBranch::Elementalism,
        SpellDifficulty::Normal,
    );

    let result = magic_user.learn_spell(spell, 3);
    assert!(result.is_err());

    match result {
        Err(MagicError::LoreNotKnown(branch)) => {
            assert_eq!(branch, MagicBranch::Elementalism);
        }
        _ => panic!("Expected LoreNotKnown error"),
    }
}

#[test]
fn test_learn_spell_with_insufficient_lore() {
    let mut magic_user = MagicUser::new(8);

    magic_user.add_lore(MagicBranch::Divination, 3);

    let spell = create_test_spell("Detect Magic", MagicBranch::Divination, SpellDifficulty::Easy);

    // Try to learn spell at level 5, but lore is only 3
    let result = magic_user.learn_spell(spell, 5);
    assert!(result.is_err());

    match result {
        Err(MagicError::InsufficientLore {
            required,
            available,
        }) => {
            assert_eq!(required, 5);
            assert_eq!(available, 3);
        }
        _ => panic!("Expected InsufficientLore error"),
    }
}

#[test]
fn test_cast_spell_success() {
    let mut magic_user = MagicUser::new(8);

    magic_user.add_lore(MagicBranch::Divination, 5);

    let spell = create_test_spell("Detect Magic", MagicBranch::Divination, SpellDifficulty::Easy);
    magic_user.learn_spell(spell, 4).unwrap();

    // Easy spell has target 8
    // skill_level (4) + empathy (8) + roll (5) = 17 > 8
    let result = magic_user.cast_spell("Detect Magic", 5);

    assert!(result.is_ok());
    let casting = result.unwrap();
    assert_eq!(casting.spell_name, "Detect Magic");
    assert!(casting.success);
    assert_eq!(casting.total, 17); // 4 + 8 + 5
    assert_eq!(casting.target, 8);
    assert!(casting.quality > 0);

    // Casting should cause exhaustion
    assert!(magic_user.exhaustion_points > 0);
}

#[test]
fn test_cast_spell_failure() {
    let mut magic_user = MagicUser::new(5);

    magic_user.add_lore(MagicBranch::Elementalism, 3);

    let spell = create_test_spell(
        "Fireball",
        MagicBranch::Elementalism,
        SpellDifficulty::Hard,
    );
    magic_user.learn_spell(spell, 2).unwrap();

    // Hard spell has target 12
    // skill_level (2) + empathy (5) + roll (1) = 8 < 12
    let result = magic_user.cast_spell("Fireball", 1);

    assert!(result.is_ok());
    let casting = result.unwrap();
    assert!(!casting.success);
    assert_eq!(casting.total, 8);
    assert_eq!(casting.target, 12);
    assert!(casting.quality < 0);

    // Failed casting doesn't cause exhaustion
    assert_eq!(magic_user.exhaustion_points, 0);
}

#[test]
fn test_cast_unknown_spell() {
    let mut magic_user = MagicUser::new(8);

    let result = magic_user.cast_spell("Unknown Spell", 5);

    assert!(result.is_err());
    match result {
        Err(MagicError::SpellNotKnown(name)) => {
            assert_eq!(name, "Unknown Spell");
        }
        _ => panic!("Expected SpellNotKnown error"),
    }
}

#[test]
fn test_spell_difficulty_targets() {
    assert_eq!(SpellDifficulty::Easy.base_target(), 8);
    assert_eq!(SpellDifficulty::Normal.base_target(), 10);
    assert_eq!(SpellDifficulty::Hard.base_target(), 12);
}

#[test]
fn test_magic_branch_lore_difficulties() {
    // Normal difficulty branches
    assert_eq!(
        MagicBranch::Divination.lore_difficulty().cost_multiplier(),
        1
    );

    // Hard difficulty branches
    assert_eq!(
        MagicBranch::Alchemy.lore_difficulty().cost_multiplier(),
        2
    );
    assert_eq!(
        MagicBranch::Animation.lore_difficulty().cost_multiplier(),
        2
    );
    assert_eq!(
        MagicBranch::Mentalism.lore_difficulty().cost_multiplier(),
        2
    );
    assert_eq!(
        MagicBranch::Thaumaturgy.lore_difficulty().cost_multiplier(),
        2
    );

    // Very hard difficulty branches
    assert_eq!(
        MagicBranch::Conjuration.lore_difficulty().cost_multiplier(),
        3
    );
    assert_eq!(
        MagicBranch::Elementalism.lore_difficulty().cost_multiplier(),
        3
    );
    assert_eq!(
        MagicBranch::Necromancy.lore_difficulty().cost_multiplier(),
        3
    );
    assert_eq!(
        MagicBranch::Transportation
            .lore_difficulty()
            .cost_multiplier(),
        3
    );
}

#[test]
fn test_exhaustion_accumulation() {
    let mut magic_user = MagicUser::new(9);

    magic_user.add_lore(MagicBranch::Divination, 6);

    let easy_spell = create_test_spell("Detect Magic", MagicBranch::Divination, SpellDifficulty::Easy);
    magic_user.learn_spell(easy_spell, 5).unwrap();

    let initial_exhaustion = magic_user.exhaustion_points;

    // Cast spell successfully
    magic_user.cast_spell("Detect Magic", 5).unwrap();

    // Should have gained exhaustion
    assert!(magic_user.exhaustion_points > initial_exhaustion);
}

#[test]
fn test_multiple_spells_multiple_branches() {
    let mut magic_user = MagicUser::new(8);

    // Learn lore in multiple branches
    magic_user.add_lore(MagicBranch::Divination, 5);
    magic_user.add_lore(MagicBranch::Animation, 4);

    // Learn spells from different branches
    let detect_magic = create_test_spell("Detect Magic", MagicBranch::Divination, SpellDifficulty::Easy);
    let heal_wounds = create_test_spell("Heal Wounds", MagicBranch::Animation, SpellDifficulty::Normal);

    magic_user.learn_spell(detect_magic, 4).unwrap();
    magic_user.learn_spell(heal_wounds, 3).unwrap();

    assert_eq!(magic_user.spells.len(), 2);
    assert!(magic_user.spells.contains_key("Detect Magic"));
    assert!(magic_user.spells.contains_key("Heal Wounds"));

    // Can cast spells from different branches
    let result1 = magic_user.cast_spell("Detect Magic", 3);
    assert!(result1.is_ok());

    let result2 = magic_user.cast_spell("Heal Wounds", 4);
    assert!(result2.is_ok());
}

#[test]
fn test_exhaustion_recovery() {
    let mut magic_user = MagicUser::new(8);

    magic_user.add_lore(MagicBranch::Divination, 5);
    let spell = create_test_spell("Detect Magic", MagicBranch::Divination, SpellDifficulty::Easy);
    magic_user.learn_spell(spell, 4).unwrap();

    // Cast spell to gain exhaustion
    magic_user.cast_spell("Detect Magic", 5).unwrap();
    let exhaustion_after_casting = magic_user.exhaustion_points;
    assert!(exhaustion_after_casting > 0);

    // Recover over time
    magic_user.recover_exhaustion(8);

    assert!(magic_user.exhaustion_points < exhaustion_after_casting);
}

#[test]
fn test_exhaustion_level_thresholds() {
    let mut magic_user = MagicUser::new(8);

    // No exhaustion
    assert_eq!(
        format!("{:?}", magic_user.exhaustion_level()),
        "None"
    );

    // Light exhaustion (> empathy)
    magic_user.exhaustion_points = magic_user.empathy + 1;
    assert_eq!(
        format!("{:?}", magic_user.exhaustion_level()),
        "Light"
    );

    // Severe exhaustion (>= empathy * 2)
    magic_user.exhaustion_points = magic_user.empathy * 2;
    assert_eq!(
        format!("{:?}", magic_user.exhaustion_level()),
        "Severe"
    );

    // Critical exhaustion (>= empathy * 3)
    magic_user.exhaustion_points = magic_user.empathy * 3;
    assert_eq!(
        format!("{:?}", magic_user.exhaustion_level()),
        "Critical"
    );
}

#[test]
fn test_spell_range_types() {
    let personal = SpellRange::Personal;
    let touch = SpellRange::Touch;
    let short = SpellRange::Short(10);
    let medium = SpellRange::Medium(50);
    let long = SpellRange::Long(100);
    let unlimited = SpellRange::Unlimited;

    // Just verify they can be created and used
    let spell1 = Spell {
        name: "Self Buff".to_string(),
        branch: MagicBranch::Animation,
        difficulty: SpellDifficulty::Easy,
        preparation_time: 1,
        casting_time: 1,
        range: personal,
        duration: SpellDuration::Minutes(10),
    };

    let spell2 = Spell {
        name: "Touch Heal".to_string(),
        branch: MagicBranch::Animation,
        difficulty: SpellDifficulty::Normal,
        preparation_time: 5,
        casting_time: 1,
        range: touch,
        duration: SpellDuration::Instant,
    };

    assert_eq!(spell1.name, "Self Buff");
    assert_eq!(spell2.name, "Touch Heal");

    // Verify other range types compile
    let _s3 = SpellRange::Short(5);
    let _s4 = SpellRange::Medium(25);
    let _s5 = SpellRange::Long(200);
    let _s6 = SpellRange::Unlimited;
}

#[test]
fn test_spell_duration_types() {
    let instant = SpellDuration::Instant;
    let rounds = SpellDuration::Rounds(10);
    let minutes = SpellDuration::Minutes(60);
    let hours = SpellDuration::Hours(8);
    let permanent = SpellDuration::Permanent;
    let concentration = SpellDuration::Permanent;

    // Create spells with different durations
    let _spell1 = Spell {
        name: "Quick Blast".to_string(),
        branch: MagicBranch::Elementalism,
        difficulty: SpellDifficulty::Easy,
        preparation_time: 1,
        casting_time: 1,
        range: SpellRange::Short(10),
        duration: instant,
    };

    let _spell2 = Spell {
        name: "Sustained Effect".to_string(),
        branch: MagicBranch::Thaumaturgy,
        difficulty: SpellDifficulty::Normal,
        preparation_time: 5,
        casting_time: 1,
        range: SpellRange::Touch,
        duration: concentration,
    };

    // Verify all duration types compile
    let _d1 = SpellDuration::Instant;
    let _d2 = SpellDuration::Rounds(5);
    let _d3 = SpellDuration::Minutes(30);
    let _d4 = SpellDuration::Hours(4);
    let _d5 = SpellDuration::Permanent;
    let _d6 = SpellDuration::Permanent;
}

#[test]
fn test_complete_wizard_scenario() {
    // Create a wizard character
    let mut wizard = MagicUser::new(9);

    // Learn Divination lore
    wizard.add_lore(MagicBranch::Divination, 6);

    // Learn several Divination spells
    wizard
        .learn_spell(
            create_test_spell("Detect Magic", MagicBranch::Divination, SpellDifficulty::Easy),
            4,
        )
        .unwrap();

    wizard
        .learn_spell(
            create_test_spell("Read Thoughts", MagicBranch::Divination, SpellDifficulty::Normal),
            3,
        )
        .unwrap();

    wizard
        .learn_spell(
            create_test_spell("Foresight", MagicBranch::Divination, SpellDifficulty::Hard),
            2,
        )
        .unwrap();

    // Wizard casts several spells in succession
    assert!(wizard.cast_spell("Detect Magic", 4).unwrap().success);
    assert!(wizard.cast_spell("Read Thoughts", 5).unwrap().success);

    // Accumulates exhaustion
    let exhaustion_before_hard_spell = wizard.exhaustion_points;
    assert!(exhaustion_before_hard_spell > 0);

    // Casts hard spell
    let foresight_result = wizard.cast_spell("Foresight", 8);
    assert!(foresight_result.is_ok());

    // More exhaustion accumulated
    assert!(wizard.exhaustion_points > exhaustion_before_hard_spell);

    // Wizard rests to recover
    wizard.recover_exhaustion(12);
    assert!(wizard.exhaustion_points < exhaustion_before_hard_spell);
}

#[test]
fn test_all_magic_branches_display() {
    let branches = [
        MagicBranch::Alchemy,
        MagicBranch::Animation,
        MagicBranch::Conjuration,
        MagicBranch::Divination,
        MagicBranch::Elementalism,
        MagicBranch::Mentalism,
        MagicBranch::Necromancy,
        MagicBranch::Thaumaturgy,
        MagicBranch::Transportation,
    ];

    for branch in &branches {
        let display_str = format!("{}", branch);
        assert!(!display_str.is_empty());
    }
}
