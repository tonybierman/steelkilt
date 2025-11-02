//! Skill development and progression system based on Draft RPG Section 3.13

use std::collections::HashMap;
use std::fmt;

/// Difficulty of learning a skill
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillDifficulty {
    Easy,     // Cost: 1 point up to attribute score
    Normal,   // Cost: normal progression
    Hard,     // Cost: 2x normal progression
    VeryHard, // Cost: 3x normal progression
}

impl SkillDifficulty {
    /// Calculate the multiplier for skill cost
    pub fn cost_multiplier(&self) -> i32 {
        match self {
            SkillDifficulty::Easy => 1,
            SkillDifficulty::Normal => 1,
            SkillDifficulty::Hard => 2,
            SkillDifficulty::VeryHard => 3,
        }
    }
}

/// A skill with its current level and associated attribute
#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub level: i32,
    pub associated_attribute: i32,
    pub difficulty: SkillDifficulty,
    pub prerequisites: Vec<SkillPrerequisite>,
}

/// Prerequisite for learning a skill
#[derive(Debug, Clone)]
pub struct SkillPrerequisite {
    pub skill_name: String,
    pub minimum_level: i32,
}

impl Skill {
    pub fn new(name: &str, associated_attribute: i32, difficulty: SkillDifficulty) -> Self {
        Self {
            name: name.to_string(),
            level: 0,
            associated_attribute,
            difficulty,
            prerequisites: Vec::new(),
        }
    }

    pub fn with_prerequisite(mut self, skill_name: &str, minimum_level: i32) -> Self {
        self.prerequisites.push(SkillPrerequisite {
            skill_name: skill_name.to_string(),
            minimum_level,
        });
        self
    }

    /// Calculate cost to raise skill from current level to target level
    pub fn calculate_upgrade_cost(&self, from_level: i32, to_level: i32) -> i32 {
        if to_level <= from_level {
            return 0;
        }

        // Special handling for Easy skills up to attribute score
        if self.difficulty == SkillDifficulty::Easy
            && from_level == 0
            && to_level <= self.associated_attribute
        {
            return 1; // One-time cost of 1 point to learn easy skill up to attribute
        }

        let mut total_cost = 0;

        for level in (from_level + 1)..=to_level {
            let base_cost = if level <= self.associated_attribute {
                // Within attribute: easy progression
                1
            } else {
                // Beyond attribute: harder progression
                level - self.associated_attribute
            };

            total_cost += base_cost * self.difficulty.cost_multiplier();
        }

        total_cost
    }
}

/// Manages a character's skills and skill points
#[derive(Debug, Clone)]
pub struct SkillSet {
    pub skills: HashMap<String, Skill>,
    pub available_points: i32,
}

impl SkillSet {
    pub fn new(initial_points: i32) -> Self {
        Self {
            skills: HashMap::new(),
            available_points: initial_points,
        }
    }

    /// Add a new skill to the skill set
    pub fn add_skill(&mut self, skill: Skill) {
        self.skills.insert(skill.name.clone(), skill);
    }

    /// Get a skill by name
    pub fn get_skill(&self, name: &str) -> Option<&Skill> {
        self.skills.get(name)
    }

    /// Get a skill mutably by name
    pub fn get_skill_mut(&mut self, name: &str) -> Option<&mut Skill> {
        self.skills.get_mut(name)
    }

    /// Get skill level (returns 0 if skill not found)
    pub fn get_skill_level(&self, name: &str) -> i32 {
        self.skills.get(name).map(|s| s.level).unwrap_or(0)
    }

    /// Check if prerequisites are met for a skill
    pub fn check_prerequisites(&self, skill: &Skill) -> bool {
        for prereq in &skill.prerequisites {
            let current_level = self.get_skill_level(&prereq.skill_name);
            if current_level < prereq.minimum_level {
                return false;
            }
        }
        true
    }

    /// Attempt to raise a skill by one level
    pub fn raise_skill(&mut self, skill_name: &str) -> Result<(), SkillError> {
        // Check if skill exists
        let skill = self
            .skills
            .get(skill_name)
            .ok_or_else(|| SkillError::SkillNotFound(skill_name.to_string()))?;

        // Check prerequisites
        if !self.check_prerequisites(skill) {
            return Err(SkillError::PrerequisitesNotMet);
        }

        let current_level = skill.level;
        let cost = skill.calculate_upgrade_cost(current_level, current_level + 1);

        // Check if we have enough points
        if self.available_points < cost {
            return Err(SkillError::InsufficientPoints {
                needed: cost,
                available: self.available_points,
            });
        }

        // Perform the upgrade
        let skill = self.skills.get_mut(skill_name).unwrap();
        skill.level += 1;
        self.available_points -= cost;

        Ok(())
    }

    /// Grant skill points (e.g., from character advancement)
    pub fn grant_points(&mut self, points: i32) {
        self.available_points += points;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillError {
    SkillNotFound(String),
    InsufficientPoints { needed: i32, available: i32 },
    PrerequisitesNotMet,
}

impl fmt::Display for SkillError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SkillError::SkillNotFound(name) => write!(f, "Skill not found: {}", name),
            SkillError::InsufficientPoints { needed, available } => {
                write!(
                    f,
                    "Insufficient points: need {}, have {}",
                    needed, available
                )
            }
            SkillError::PrerequisitesNotMet => write!(f, "Prerequisites not met"),
        }
    }
}

impl std::error::Error for SkillError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_cost_calculation_normal() {
        let skill = Skill::new("Swordsmanship", 7, SkillDifficulty::Normal);

        // Within attribute range (1-7): costs 1 each
        assert_eq!(skill.calculate_upgrade_cost(0, 1), 1);
        assert_eq!(skill.calculate_upgrade_cost(0, 5), 5);
        assert_eq!(skill.calculate_upgrade_cost(5, 7), 2);

        // Beyond attribute (8+): costs increase
        assert_eq!(skill.calculate_upgrade_cost(7, 8), 1); // 8 - 7 = 1
        assert_eq!(skill.calculate_upgrade_cost(8, 9), 2); // 9 - 7 = 2
    }

    #[test]
    fn test_skill_cost_calculation_hard() {
        let skill = Skill::new("Arcane Lore", 5, SkillDifficulty::Hard);

        // Hard skills cost 2x
        assert_eq!(skill.calculate_upgrade_cost(0, 1), 2);
        assert_eq!(skill.calculate_upgrade_cost(0, 5), 10);
    }

    #[test]
    fn test_skill_cost_calculation_easy() {
        let skill = Skill::new("Native Language", 7, SkillDifficulty::Easy);

        // Easy skills cost 1 point up to attribute score
        assert_eq!(skill.calculate_upgrade_cost(0, 7), 1);
    }

    #[test]
    fn test_skill_progression() {
        let mut skill_set = SkillSet::new(30);

        let sword_skill = Skill::new("Longsword", 7, SkillDifficulty::Normal);
        skill_set.add_skill(sword_skill);

        // Raise skill from 0 to 1 (costs 1)
        assert!(skill_set.raise_skill("Longsword").is_ok());
        assert_eq!(skill_set.get_skill_level("Longsword"), 1);
        assert_eq!(skill_set.available_points, 29);

        // Raise to level 5 (costs 1 each)
        for _ in 0..4 {
            assert!(skill_set.raise_skill("Longsword").is_ok());
        }
        assert_eq!(skill_set.get_skill_level("Longsword"), 5);
        assert_eq!(skill_set.available_points, 25);
    }

    #[test]
    fn test_insufficient_points() {
        let mut skill_set = SkillSet::new(5);

        let hard_skill = Skill::new("Quantum Mechanics", 5, SkillDifficulty::VeryHard);
        skill_set.add_skill(hard_skill);

        // Costs 3 points (VeryHard multiplier)
        assert!(skill_set.raise_skill("Quantum Mechanics").is_ok());
        assert_eq!(skill_set.available_points, 2);

        // Not enough points for second level (would cost 3)
        assert!(matches!(
            skill_set.raise_skill("Quantum Mechanics"),
            Err(SkillError::InsufficientPoints { .. })
        ));
    }

    #[test]
    fn test_prerequisites() {
        let mut skill_set = SkillSet::new(50);

        // Add basic skill
        let basic = Skill::new("Mathematics", 6, SkillDifficulty::Normal);
        skill_set.add_skill(basic);

        // Add advanced skill with prerequisite
        let advanced =
            Skill::new("Calculus", 7, SkillDifficulty::Hard).with_prerequisite("Mathematics", 3);
        skill_set.add_skill(advanced);

        // Try to learn Calculus without prerequisite
        assert!(matches!(
            skill_set.raise_skill("Calculus"),
            Err(SkillError::PrerequisitesNotMet)
        ));

        // Learn Mathematics to level 3
        for _ in 0..3 {
            skill_set.raise_skill("Mathematics").unwrap();
        }

        // Now Calculus should be learnable
        assert!(skill_set.raise_skill("Calculus").is_ok());
    }
}
