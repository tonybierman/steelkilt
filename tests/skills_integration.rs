//! Integration tests for the skills module
//!
//! Tests skill development and progression following Draft RPG Section 3.13

use steelkilt::modules::skills::{Skill, SkillDifficulty, SkillError, SkillSet};

#[test]
fn test_skill_creation_and_level() {
    let skill = Skill::new("Swordsmanship", 7, SkillDifficulty::Normal);

    assert_eq!(skill.name, "Swordsmanship");
    assert_eq!(skill.level, 0);
    assert_eq!(skill.associated_attribute, 7);
    assert_eq!(skill.difficulty, SkillDifficulty::Normal);
}

#[test]
fn test_skill_set_basic_operations() {
    let mut skill_set = SkillSet::new(10);

    let sword_skill = Skill::new("Sword", 7, SkillDifficulty::Normal);
    skill_set.add_skill(sword_skill);

    assert_eq!(skill_set.available_points, 10);
    assert_eq!(skill_set.get_skill_level("Sword"), 0);
    assert!(skill_set.get_skill("Sword").is_some());
    assert!(skill_set.get_skill("Archery").is_none());
}

#[test]
fn test_easy_skill_learning() {
    let mut skill_set = SkillSet::new(10);

    // Easy skill up to attribute costs only 1 point total
    let mut dodge = Skill::new("Dodge", 7, SkillDifficulty::Easy);
    dodge.level = 0;
    let cost = dodge.calculate_upgrade_cost(0, 7);
    assert_eq!(
        cost, 1,
        "Easy skill up to attribute should cost 1 point total"
    );

    skill_set.add_skill(Skill::new("Dodge", 7, SkillDifficulty::Easy));

    // Raising to level 1 should cost 1 point
    assert!(skill_set.raise_skill("Dodge").is_ok());
    assert_eq!(skill_set.get_skill_level("Dodge"), 1);
    assert_eq!(skill_set.available_points, 9);
}

#[test]
fn test_normal_skill_progression_within_attribute() {
    let mut skill_set = SkillSet::new(20);

    skill_set.add_skill(Skill::new("Sword", 5, SkillDifficulty::Normal));

    // Within attribute score: 1 point per level
    for expected_level in 1..=5 {
        assert!(skill_set.raise_skill("Sword").is_ok());
        assert_eq!(skill_set.get_skill_level("Sword"), expected_level);
    }

    // Should have spent 5 points (1 per level)
    assert_eq!(skill_set.available_points, 15);
}

#[test]
fn test_normal_skill_progression_beyond_attribute() {
    let mut skill_set = SkillSet::new(30);

    skill_set.add_skill(Skill::new("Sword", 5, SkillDifficulty::Normal));

    // Raise to attribute level first
    for _ in 0..5 {
        skill_set.raise_skill("Sword").unwrap();
    }

    let points_before = skill_set.available_points;

    // Beyond attribute: cost = (level - attribute)
    // Level 6 costs: 6 - 5 = 1
    skill_set.raise_skill("Sword").unwrap();
    assert_eq!(skill_set.available_points, points_before - 1);

    let points_before = skill_set.available_points;
    // Level 7 costs: 7 - 5 = 2
    skill_set.raise_skill("Sword").unwrap();
    assert_eq!(skill_set.available_points, points_before - 2);
}

#[test]
fn test_hard_skill_multiplier() {
    let mut skill_set = SkillSet::new(20);

    skill_set.add_skill(Skill::new("Magic", 5, SkillDifficulty::Hard));

    // Hard skills cost 2x
    skill_set.raise_skill("Magic").unwrap(); // Level 1: 1 * 2 = 2 points
    assert_eq!(skill_set.available_points, 18);

    skill_set.raise_skill("Magic").unwrap(); // Level 2: 1 * 2 = 2 points
    assert_eq!(skill_set.available_points, 16);
}

#[test]
fn test_very_hard_skill_multiplier() {
    let mut skill_set = SkillSet::new(30);

    skill_set.add_skill(Skill::new("Psionics", 5, SkillDifficulty::VeryHard));

    // Very hard skills cost 3x
    skill_set.raise_skill("Psionics").unwrap(); // Level 1: 1 * 3 = 3 points
    assert_eq!(skill_set.available_points, 27);

    skill_set.raise_skill("Psionics").unwrap(); // Level 2: 1 * 3 = 3 points
    assert_eq!(skill_set.available_points, 24);
}

#[test]
fn test_insufficient_points_error() {
    let mut skill_set = SkillSet::new(1);

    skill_set.add_skill(Skill::new("Sword", 5, SkillDifficulty::Hard));

    // Hard skill costs 2 points per level, we only have 1
    let result = skill_set.raise_skill("Sword");

    assert!(result.is_err());
    match result {
        Err(SkillError::InsufficientPoints { needed, available }) => {
            assert_eq!(needed, 2);
            assert_eq!(available, 1);
        }
        _ => panic!("Expected InsufficientPoints error"),
    }
}

#[test]
fn test_skill_not_found_error() {
    let mut skill_set = SkillSet::new(10);

    let result = skill_set.raise_skill("NonexistentSkill");

    assert!(result.is_err());
    match result {
        Err(SkillError::SkillNotFound(name)) => {
            assert_eq!(name, "NonexistentSkill");
        }
        _ => panic!("Expected SkillNotFound error"),
    }
}

#[test]
fn test_skill_prerequisites() {
    let mut skill_set = SkillSet::new(20);

    // Basic sword skill
    skill_set.add_skill(Skill::new("Sword", 5, SkillDifficulty::Normal));

    // Advanced technique requires Sword at level 5
    skill_set.add_skill(
        Skill::new("Master Strike", 5, SkillDifficulty::Hard).with_prerequisite("Sword", 5),
    );

    // Try to raise advanced skill without prerequisite
    let result = skill_set.raise_skill("Master Strike");
    assert!(result.is_err());
    match result {
        Err(SkillError::PrerequisitesNotMet) => {}
        _ => panic!("Expected PrerequisitesNotMet error"),
    }

    // Raise sword to level 5
    for _ in 0..5 {
        skill_set.raise_skill("Sword").unwrap();
    }

    // Now advanced skill should work
    assert!(skill_set.raise_skill("Master Strike").is_ok());
    assert_eq!(skill_set.get_skill_level("Master Strike"), 1);
}

#[test]
fn test_multiple_prerequisites() {
    let mut skill_set = SkillSet::new(50);

    skill_set.add_skill(Skill::new("Sword", 5, SkillDifficulty::Normal));
    skill_set.add_skill(Skill::new("Dodge", 6, SkillDifficulty::Easy));

    // Combat Master requires both skills
    skill_set.add_skill(
        Skill::new("Combat Master", 5, SkillDifficulty::VeryHard)
            .with_prerequisite("Sword", 5)
            .with_prerequisite("Dodge", 5),
    );

    // Raise Sword to 5
    for _ in 0..5 {
        skill_set.raise_skill("Sword").unwrap();
    }

    // Still missing Dodge prerequisite
    assert!(skill_set.raise_skill("Combat Master").is_err());

    // Raise Dodge to 5
    for _ in 0..5 {
        skill_set.raise_skill("Dodge").unwrap();
    }

    // Now it should work
    assert!(skill_set.raise_skill("Combat Master").is_ok());
}

#[test]
fn test_grant_points() {
    let mut skill_set = SkillSet::new(5);

    assert_eq!(skill_set.available_points, 5);

    skill_set.grant_points(10);
    assert_eq!(skill_set.available_points, 15);

    skill_set.grant_points(5);
    assert_eq!(skill_set.available_points, 20);
}

#[test]
fn test_complete_skill_development_scenario() {
    let mut skill_set = SkillSet::new(50);

    // Character starts learning basic combat skills
    skill_set.add_skill(Skill::new("Sword", 7, SkillDifficulty::Normal));
    skill_set.add_skill(Skill::new("Dodge", 6, SkillDifficulty::Easy));
    skill_set.add_skill(Skill::new("Tactics", 5, SkillDifficulty::Hard));

    // Raise sword to attribute level (7 points)
    for _ in 0..7 {
        skill_set.raise_skill("Sword").unwrap();
    }
    assert_eq!(skill_set.get_skill_level("Sword"), 7);

    // Raise dodge (easy skill, costs 1 for first level)
    skill_set.raise_skill("Dodge").unwrap();
    assert_eq!(skill_set.get_skill_level("Dodge"), 1);

    // Raise tactics (hard skill, costs 2 per level)
    skill_set.raise_skill("Tactics").unwrap();
    skill_set.raise_skill("Tactics").unwrap();
    assert_eq!(skill_set.get_skill_level("Tactics"), 2);

    // Verify points spent
    // Sword: 7 points
    // Dodge: 1 point
    // Tactics: 4 points (2 levels * 2 multiplier)
    // Total: 12 points
    assert_eq!(skill_set.available_points, 38);
}

#[test]
fn test_skill_upgrade_cost_calculation() {
    let normal_skill = Skill::new("Sword", 5, SkillDifficulty::Normal);

    // Within attribute (1 point per level)
    assert_eq!(normal_skill.calculate_upgrade_cost(0, 1), 1);
    assert_eq!(normal_skill.calculate_upgrade_cost(0, 5), 5);
    assert_eq!(normal_skill.calculate_upgrade_cost(3, 5), 2);

    // Beyond attribute
    // Level 6: costs (6 - 5) = 1
    // Level 7: costs (7 - 5) = 2
    assert_eq!(normal_skill.calculate_upgrade_cost(5, 6), 1);
    assert_eq!(normal_skill.calculate_upgrade_cost(5, 7), 3); // 1 + 2

    let hard_skill = Skill::new("Magic", 5, SkillDifficulty::Hard);
    // Hard skills cost 2x
    assert_eq!(hard_skill.calculate_upgrade_cost(0, 1), 2); // 1 * 2
    assert_eq!(hard_skill.calculate_upgrade_cost(0, 5), 10); // 5 * 2
}

#[test]
fn test_easy_skill_special_case() {
    let easy_skill = Skill::new("Stealth", 6, SkillDifficulty::Easy);

    // Easy skill from 0 to attribute costs only 1 point total
    let cost_to_6 = easy_skill.calculate_upgrade_cost(0, 6);
    assert_eq!(cost_to_6, 1);

    // But raising from 0 to 3 should also cost 1 (special case)
    let cost_to_3 = easy_skill.calculate_upgrade_cost(0, 3);
    assert_eq!(cost_to_3, 1);

    // Once learned, normal rules apply
    let mut learned_skill = Skill::new("Stealth", 6, SkillDifficulty::Easy);
    learned_skill.level = 3;
    let cost_from_3_to_4 = learned_skill.calculate_upgrade_cost(3, 4);
    assert_eq!(cost_from_3_to_4, 1); // Normal cost within attribute
}

#[test]
fn test_skill_difficulty_cost_multipliers() {
    assert_eq!(SkillDifficulty::Easy.cost_multiplier(), 1);
    assert_eq!(SkillDifficulty::Normal.cost_multiplier(), 1);
    assert_eq!(SkillDifficulty::Hard.cost_multiplier(), 2);
    assert_eq!(SkillDifficulty::VeryHard.cost_multiplier(), 3);
}

#[test]
fn test_skill_integration_with_character_advancement() {
    let mut skill_set = SkillSet::new(10);

    skill_set.add_skill(Skill::new("Sword", 5, SkillDifficulty::Normal));

    // Raise skill to level 5 with initial points (costs 5 points)
    for _ in 0..5 {
        skill_set.raise_skill("Sword").unwrap();
    }

    assert_eq!(skill_set.get_skill_level("Sword"), 5);
    assert_eq!(skill_set.available_points, 5);

    // Raise to level 6 (costs 1 point) and level 7 (costs 2 points)
    skill_set.raise_skill("Sword").unwrap(); // Now at 6
    skill_set.raise_skill("Sword").unwrap(); // Now at 7

    // Should have 2 points left (5 - 1 - 2 = 2)
    assert_eq!(skill_set.available_points, 2);

    // Raising to 8 costs 3 points, but only have 2
    assert!(skill_set.raise_skill("Sword").is_err());

    // Character gains experience and earns more points
    skill_set.grant_points(5);
    assert_eq!(skill_set.available_points, 7);

    // Can continue development to level 8
    skill_set.raise_skill("Sword").unwrap();
    assert_eq!(skill_set.get_skill_level("Sword"), 8);
    assert_eq!(skill_set.available_points, 4); // 7 - 3 = 4
}
