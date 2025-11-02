//! Magic system based on Draft RPG Chapter 5

use std::collections::HashMap;
use std::fmt;

/// Branches of magic as defined in Draft RPG
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MagicBranch {
    Alchemy,        // Constitution and alteration of matter
    Animation,      // Healing wounds, modifying physical abilities
    Conjuration,    // Summoning creatures
    Divination,     // Foresee events, gather information
    Elementalism,   // Control elements (fire, water, air, earth)
    Mentalism,      // Penetrate and control minds
    Necromancy,     // Animate and control dead
    Thaumaturgy,    // Control matter macroscopically
    Transportation, // Transport through space and time
}

impl MagicBranch {
    /// Get the difficulty of learning this branch's lore
    pub fn lore_difficulty(&self) -> LoreDifficulty {
        match self {
            MagicBranch::Alchemy => LoreDifficulty::Hard,
            MagicBranch::Animation => LoreDifficulty::Hard,
            MagicBranch::Conjuration => LoreDifficulty::VeryHard,
            MagicBranch::Divination => LoreDifficulty::Normal,
            MagicBranch::Elementalism => LoreDifficulty::VeryHard,
            MagicBranch::Mentalism => LoreDifficulty::Hard,
            MagicBranch::Necromancy => LoreDifficulty::VeryHard,
            MagicBranch::Thaumaturgy => LoreDifficulty::Hard,
            MagicBranch::Transportation => LoreDifficulty::VeryHard,
        }
    }
}

impl fmt::Display for MagicBranch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MagicBranch::Alchemy => write!(f, "Alchemy"),
            MagicBranch::Animation => write!(f, "Animation"),
            MagicBranch::Conjuration => write!(f, "Conjuration"),
            MagicBranch::Divination => write!(f, "Divination"),
            MagicBranch::Elementalism => write!(f, "Elementalism"),
            MagicBranch::Mentalism => write!(f, "Mentalism"),
            MagicBranch::Necromancy => write!(f, "Necromancy"),
            MagicBranch::Thaumaturgy => write!(f, "Thaumaturgy"),
            MagicBranch::Transportation => write!(f, "Transportation"),
        }
    }
}

/// Difficulty of learning a branch's lore
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoreDifficulty {
    Normal,   // 1x cost
    Hard,     // 2x cost
    VeryHard, // 3x cost
}

impl LoreDifficulty {
    pub fn cost_multiplier(&self) -> i32 {
        match self {
            LoreDifficulty::Normal => 1,
            LoreDifficulty::Hard => 2,
            LoreDifficulty::VeryHard => 3,
        }
    }
}

/// A spell within a magic branch
#[derive(Debug, Clone)]
pub struct Spell {
    pub name: String,
    pub branch: MagicBranch,
    pub difficulty: SpellDifficulty,
    pub preparation_time: i32, // minutes
    pub casting_time: i32,     // segments
    pub range: SpellRange,
    pub duration: SpellDuration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellDifficulty {
    Easy,
    Normal,
    Hard,
}

impl SpellDifficulty {
    pub fn base_target(&self) -> i32 {
        match self {
            SpellDifficulty::Easy => 8,
            SpellDifficulty::Normal => 10,
            SpellDifficulty::Hard => 12,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SpellRange {
    Personal,
    Touch,
    Short(i32),  // meters
    Medium(i32), // meters
    Long(i32),   // meters
    Unlimited,
}

#[derive(Debug, Clone)]
pub enum SpellDuration {
    Instant,
    Rounds(i32),
    Minutes(i32),
    Hours(i32),
    Permanent,
}

/// Lore knowledge in a branch of magic
#[derive(Debug, Clone)]
pub struct MagicLore {
    pub branch: MagicBranch,
    pub level: i32,
    pub empathy_attribute: i32,
}

impl MagicLore {
    pub fn new(branch: MagicBranch, empathy: i32) -> Self {
        Self {
            branch,
            level: 0,
            empathy_attribute: empathy,
        }
    }

    /// Calculate cost to raise lore from current to target level
    pub fn calculate_upgrade_cost(&self, from_level: i32, to_level: i32) -> i32 {
        if to_level <= from_level {
            return 0;
        }

        let difficulty = self.branch.lore_difficulty();
        let mut total_cost = 0;

        for level in (from_level + 1)..=to_level {
            let base_cost = if level <= self.empathy_attribute {
                1
            } else {
                level - self.empathy_attribute
            };
            total_cost += base_cost * difficulty.cost_multiplier();
        }

        total_cost
    }

    /// Check if a spell can be learned (spell level <= lore level)
    pub fn can_learn_spell(&self, spell_level: i32) -> bool {
        spell_level <= self.level
    }
}

/// A learned spell with skill level
#[derive(Debug, Clone)]
pub struct LearnedSpell {
    pub spell: Spell,
    pub skill_level: i32,
}

/// Manages a character's magic capabilities
#[derive(Debug, Clone)]
pub struct MagicUser {
    pub lores: HashMap<MagicBranch, MagicLore>,
    pub spells: HashMap<String, LearnedSpell>,
    pub empathy: i32,
    pub exhaustion_points: i32, // From casting spells
}

impl MagicUser {
    pub fn new(empathy: i32) -> Self {
        Self {
            lores: HashMap::new(),
            spells: HashMap::new(),
            empathy,
            exhaustion_points: 0,
        }
    }

    /// Add a lore to the magic user
    pub fn add_lore(&mut self, branch: MagicBranch, level: i32) {
        let mut lore = MagicLore::new(branch, self.empathy);
        lore.level = level;
        self.lores.insert(branch, lore);
    }

    /// Learn a new spell
    pub fn learn_spell(&mut self, spell: Spell, initial_level: i32) -> Result<(), MagicError> {
        // Check if we have the lore for this branch
        let lore = self
            .lores
            .get(&spell.branch)
            .ok_or(MagicError::LoreNotKnown(spell.branch))?;

        // Check if lore level is high enough
        if !lore.can_learn_spell(initial_level) {
            return Err(MagicError::InsufficientLore {
                required: initial_level,
                available: lore.level,
            });
        }

        let learned_spell = LearnedSpell {
            spell,
            skill_level: initial_level,
        };

        self.spells
            .insert(learned_spell.spell.name.clone(), learned_spell);
        Ok(())
    }

    /// Attempt to cast a spell
    pub fn cast_spell(&mut self, spell_name: &str, roll: i32) -> Result<CastingResult, MagicError> {
        let learned_spell = self
            .spells
            .get(spell_name)
            .ok_or_else(|| MagicError::SpellNotKnown(spell_name.to_string()))?;

        // Calculate total: skill level + empathy + roll
        let total = learned_spell.skill_level + self.empathy + roll;
        let target = learned_spell.spell.difficulty.base_target();

        let success = total >= target;
        let quality = total - target;

        // Casting causes exhaustion
        if success {
            self.exhaustion_points += self.calculate_exhaustion(&learned_spell.spell, quality);
        }

        Ok(CastingResult {
            spell_name: spell_name.to_string(),
            success,
            quality,
            total,
            target,
        })
    }

    /// Calculate exhaustion from casting a spell
    fn calculate_exhaustion(&self, spell: &Spell, quality: i32) -> i32 {
        let base_exhaustion = match spell.difficulty {
            SpellDifficulty::Easy => 1,
            SpellDifficulty::Normal => 2,
            SpellDifficulty::Hard => 3,
        };

        // Casting beyond capabilities causes more exhaustion
        if quality < 0 {
            base_exhaustion * 2
        } else {
            base_exhaustion
        }
    }

    /// Recover from magical exhaustion (takes hours)
    pub fn recover_exhaustion(&mut self, hours: i32) {
        self.exhaustion_points = (self.exhaustion_points - hours).max(0);
    }

    /// Get current exhaustion level
    pub fn exhaustion_level(&self) -> ExhaustionLevel {
        if self.exhaustion_points >= self.empathy * 3 {
            ExhaustionLevel::Critical
        } else if self.exhaustion_points >= self.empathy * 2 {
            ExhaustionLevel::Severe
        } else if self.exhaustion_points > self.empathy {
            ExhaustionLevel::Light
        } else {
            ExhaustionLevel::None
        }
    }

    /// Get penalty from magical exhaustion
    pub fn exhaustion_penalty(&self) -> i32 {
        match self.exhaustion_level() {
            ExhaustionLevel::None => 0,
            ExhaustionLevel::Light => -1,
            ExhaustionLevel::Severe => -2,
            ExhaustionLevel::Critical => -4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExhaustionLevel {
    None,
    Light,
    Severe,
    Critical,
}

#[derive(Debug)]
pub struct CastingResult {
    pub spell_name: String,
    pub success: bool,
    pub quality: i32,
    pub total: i32,
    pub target: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MagicError {
    LoreNotKnown(MagicBranch),
    InsufficientLore { required: i32, available: i32 },
    SpellNotKnown(String),
}

impl fmt::Display for MagicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MagicError::LoreNotKnown(branch) => write!(f, "Lore not known: {}", branch),
            MagicError::InsufficientLore {
                required,
                available,
            } => {
                write!(
                    f,
                    "Insufficient lore: need {}, have {}",
                    required, available
                )
            }
            MagicError::SpellNotKnown(name) => write!(f, "Spell not known: {}", name),
        }
    }
}

impl std::error::Error for MagicError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lore_cost_calculation() {
        let lore = MagicLore::new(MagicBranch::Divination, 6);

        // Normal difficulty (1x cost)
        assert_eq!(lore.calculate_upgrade_cost(0, 1), 1);
        assert_eq!(lore.calculate_upgrade_cost(0, 6), 6);

        // Beyond empathy
        assert_eq!(lore.calculate_upgrade_cost(6, 7), 1); // 7-6 = 1
        assert_eq!(lore.calculate_upgrade_cost(7, 8), 2); // 8-6 = 2
    }

    #[test]
    fn test_hard_lore_cost() {
        let lore = MagicLore::new(MagicBranch::Thaumaturgy, 5);

        // Hard difficulty (2x cost)
        assert_eq!(lore.calculate_upgrade_cost(0, 1), 2);
        assert_eq!(lore.calculate_upgrade_cost(0, 5), 10);
    }

    #[test]
    fn test_very_hard_lore_cost() {
        let lore = MagicLore::new(MagicBranch::Elementalism, 5);

        // VeryHard difficulty (3x cost)
        assert_eq!(lore.calculate_upgrade_cost(0, 1), 3);
        assert_eq!(lore.calculate_upgrade_cost(0, 5), 15);
    }

    #[test]
    fn test_learning_spells() {
        let mut mage = MagicUser::new(7);

        // Add Divination lore at level 5
        mage.add_lore(MagicBranch::Divination, 5);

        // Create a simple spell
        let spell = Spell {
            name: "Detect Magic".to_string(),
            branch: MagicBranch::Divination,
            difficulty: SpellDifficulty::Easy,
            preparation_time: 5,
            casting_time: 1,
            range: SpellRange::Short(10),
            duration: SpellDuration::Minutes(10),
        };

        // Learn spell at level 3
        assert!(mage.learn_spell(spell, 3).is_ok());

        // Try to learn spell at level 6 (exceeds lore)
        let hard_spell = Spell {
            name: "True Seeing".to_string(),
            branch: MagicBranch::Divination,
            difficulty: SpellDifficulty::Hard,
            preparation_time: 30,
            casting_time: 2,
            range: SpellRange::Personal,
            duration: SpellDuration::Hours(1),
        };

        assert!(matches!(
            mage.learn_spell(hard_spell, 6),
            Err(MagicError::InsufficientLore { .. })
        ));
    }

    #[test]
    fn test_casting_spell() {
        let mut mage = MagicUser::new(7);
        mage.add_lore(MagicBranch::Thaumaturgy, 5);

        let spell = Spell {
            name: "Levitate".to_string(),
            branch: MagicBranch::Thaumaturgy,
            difficulty: SpellDifficulty::Normal,
            preparation_time: 10,
            casting_time: 2,
            range: SpellRange::Short(20),
            duration: SpellDuration::Minutes(5),
        };

        mage.learn_spell(spell, 4).unwrap();

        // Cast with roll of 5: 4 (skill) + 7 (empathy) + 5 (roll) = 16 vs 10
        let result = mage.cast_spell("Levitate", 5).unwrap();
        assert!(result.success);
        assert_eq!(result.quality, 6);

        // Should have exhaustion from casting
        assert!(mage.exhaustion_points > 0);
    }

    #[test]
    fn test_magical_exhaustion() {
        let mut mage = MagicUser::new(6);

        assert_eq!(mage.exhaustion_level(), ExhaustionLevel::None);

        // Add some exhaustion
        mage.exhaustion_points = 7;
        assert_eq!(mage.exhaustion_level(), ExhaustionLevel::Light);
        assert_eq!(mage.exhaustion_penalty(), -1);

        mage.exhaustion_points = 12;
        assert_eq!(mage.exhaustion_level(), ExhaustionLevel::Severe);
        assert_eq!(mage.exhaustion_penalty(), -2);

        mage.exhaustion_points = 18;
        assert_eq!(mage.exhaustion_level(), ExhaustionLevel::Critical);
        assert_eq!(mage.exhaustion_penalty(), -4);

        // Recovery
        mage.recover_exhaustion(10);
        assert_eq!(mage.exhaustion_points, 8);
    }

    #[test]
    fn test_unknown_branch() {
        let mut mage = MagicUser::new(5);

        let spell = Spell {
            name: "Fireball".to_string(),
            branch: MagicBranch::Elementalism,
            difficulty: SpellDifficulty::Normal,
            preparation_time: 15,
            casting_time: 1,
            range: SpellRange::Medium(50),
            duration: SpellDuration::Instant,
        };

        // Don't have Elementalism lore
        assert!(matches!(
            mage.learn_spell(spell, 3),
            Err(MagicError::LoreNotKnown(MagicBranch::Elementalism))
        ));
    }
}
