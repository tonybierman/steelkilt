//! Fighter creation and configuration module
//!
//! Provides predefined characters and their skill sets for combat simulation.
//! Each fighter has distinct attributes, equipment, and combat skills.

use steelkilt::modules::*;
use steelkilt::*;

/// Create a heavily armored knight with balanced attributes
pub fn create_knight() -> Character {
    Character::new(
        "Sir Roland the Defender",
        Attributes::new(8, 6, 7, 5, 6, 5, 6, 7, 4), // STR, DEX, CON, REA, INT, WIL, CHA, PER, EMP
        7,                                          // weapon skill
        5,                                          // dodge skill
        Weapon::long_sword(),
        Armor::plate(),
    )
}

/// Create a powerful barbarian with high strength and constitution
pub fn create_barbarian() -> Character {
    Character::new(
        "Thorgar the Fierce",
        Attributes::new(9, 7, 9, 4, 5, 6, 4, 6, 3),
        8, // weapon skill
        6, // dodge skill
        Weapon::two_handed_sword(),
        Armor::leather(),
    )
}

/// Create knight's skill set (longsword and dodge specialization)
pub fn create_knight_skills() -> SkillSet {
    let mut skills = SkillSet::new(0); // No need to spend points, pre-set

    let longsword = Skill::new("Longsword", 6, SkillDifficulty::Normal);
    skills.skills.insert("Longsword".to_string(), longsword);

    let dodge = Skill::new("Dodge", 6, SkillDifficulty::Normal);
    skills.skills.insert("Dodge".to_string(), dodge);

    skills
}

/// Create barbarian's skill set (two-handed weapons and dodge specialization)
pub fn create_barbarian_skills() -> SkillSet {
    let mut skills = SkillSet::new(0);

    let two_handed = Skill::new("Two-Handed Weapons", 7, SkillDifficulty::Normal);
    skills
        .skills
        .insert("Two-Handed Weapons".to_string(), two_handed);

    let dodge = Skill::new("Dodge", 7, SkillDifficulty::Normal);
    skills.skills.insert("Dodge".to_string(), dodge);

    skills
}
