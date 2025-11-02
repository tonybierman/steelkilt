//! # Steelkilt - Draft RPG Combat System
//!
//! A Rust implementation of the Draft 0.4 RPG rule set, focusing on the combat system.
//!
//! This library provides:
//! - Character creation with attributes and skills
//! - Combat mechanics including attacks, parries, and dodges
//! - Weapon and armor systems
//! - Damage and wound tracking
//!
//! ## Advanced Features
//!
//! The `modules` submodule provides advanced features:
//! - Skill development and progression
//! - Exhaustion tracking
//! - Special combat maneuvers
//! - Hit location tracking
//! - Ranged combat mechanics
//! - Magic system

pub mod modules;

use rand::Rng;
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Roll a d10 (10-sided die)
pub fn d10() -> i32 {
    rand::thread_rng().gen_range(1..=10)
}

/// Character attributes as defined in Draft RPG
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Attributes {
    // Physical
    pub strength: i32,     // STR
    pub dexterity: i32,    // DEX
    pub constitution: i32, // CON
    // Mental
    pub reason: i32,    // REA
    pub intuition: i32, // INT
    pub willpower: i32, // WIL
    // Interactive
    pub charisma: i32,   // CHA
    pub perception: i32, // PER
    pub empathy: i32,    // EMP
}

impl Attributes {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        str: i32,
        dex: i32,
        con: i32,
        rea: i32,
        int: i32,
        wil: i32,
        cha: i32,
        per: i32,
        emp: i32,
    ) -> Self {
        Self {
            strength: str.clamp(1, 10),
            dexterity: dex.clamp(1, 10),
            constitution: con.clamp(1, 10),
            reason: rea.clamp(1, 10),
            intuition: int.clamp(1, 10),
            willpower: wil.clamp(1, 10),
            charisma: cha.clamp(1, 10),
            perception: per.clamp(1, 10),
            empathy: emp.clamp(1, 10),
        }
    }

    /// Combined attribute: Stamina = (STR + CON) / 2
    pub fn stamina(&self) -> i32 {
        ((self.strength + self.constitution) as f32 / 2.0).round() as i32
    }
}

/// Weapon impact classes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WeaponImpact {
    Small = 1,
    Medium = 2,
    Large = 3,
    Huge = 4,
}

/// Weapon types
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Weapon {
    pub name: String,
    pub impact: WeaponImpact,
    pub damage: i32, // (impact × 2) + bonus
}

impl Weapon {
    pub fn new(name: &str, impact: WeaponImpact) -> Self {
        let damage = (impact as i32) * 2 + 1; // +1 for pointed/sharp weapons
        Self {
            name: name.to_string(),
            impact,
            damage,
        }
    }

    pub fn dagger() -> Self {
        Self::new("Dagger", WeaponImpact::Small)
    }

    pub fn long_sword() -> Self {
        Self::new("Long Sword", WeaponImpact::Medium)
    }

    pub fn two_handed_sword() -> Self {
        Self::new("Two-Handed Sword", WeaponImpact::Large)
    }
}

/// Armor types and protection values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ArmorType {
    HeavyCloth = 1,
    Leather = 2,
    Chain = 3,
    Plate = 4,
    FullPlate = 5,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Armor {
    pub name: String,
    pub armor_type: ArmorType,
    pub protection: i32,
    pub movement_penalty: i32,
}

impl Armor {
    pub fn new(name: &str, armor_type: ArmorType, movement_penalty: i32) -> Self {
        Self {
            name: name.to_string(),
            armor_type,
            protection: armor_type as i32,
            movement_penalty,
        }
    }

    pub fn none() -> Self {
        Self {
            name: "None".to_string(),
            armor_type: ArmorType::HeavyCloth,
            protection: 0,
            movement_penalty: 0,
        }
    }

    pub fn leather() -> Self {
        Self::new("Leather Armor", ArmorType::Leather, 0)
    }

    pub fn chain_mail() -> Self {
        Self::new("Chain Mail", ArmorType::Chain, -1)
    }

    pub fn plate() -> Self {
        Self::new("Plate Armor", ArmorType::Plate, -1)
    }
}

/// Wound severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WoundLevel {
    Light,
    Severe,
    Critical,
}

impl fmt::Display for WoundLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WoundLevel::Light => write!(f, "Light"),
            WoundLevel::Severe => write!(f, "Severe"),
            WoundLevel::Critical => write!(f, "Critical"),
        }
    }
}

/// Character wounds tracking
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Wounds {
    pub light: i32,
    pub severe: i32,
    pub critical: i32,
}

impl Wounds {
    pub fn new() -> Self {
        Self {
            light: 0,
            severe: 0,
            critical: 0,
        }
    }

    /// Add a wound, handling stacking (4th light becomes severe, etc.)
    pub fn add_wound(&mut self, level: WoundLevel) {
        match level {
            WoundLevel::Light => {
                self.light += 1;
                if self.light >= 4 {
                    self.light = 0;
                    self.add_wound(WoundLevel::Severe);
                }
            }
            WoundLevel::Severe => {
                self.severe += 1;
                if self.severe >= 3 {
                    self.severe = 0;
                    self.add_wound(WoundLevel::Critical);
                }
            }
            WoundLevel::Critical => {
                self.critical += 1;
            }
        }
    }

    /// Check if character is dead (more than 1 critical wound)
    pub fn is_dead(&self) -> bool {
        self.critical > 1
    }

    /// Check if character is incapacitated (has critical wound)
    pub fn is_incapacitated(&self) -> bool {
        self.critical >= 1
    }

    /// Total penalty from wounds for movement-based actions
    pub fn movement_penalty(&self) -> i32 {
        -(self.light + self.severe * 2 + self.critical * 4)
    }
}

impl Default for Wounds {
    fn default() -> Self {
        Self::new()
    }
}

/// A character in the Draft RPG system
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Character {
    pub name: String,
    pub attributes: Attributes,
    pub weapon_skill: i32,
    pub dodge_skill: i32,
    pub weapon: Weapon,
    pub armor: Armor,
    pub wounds: Wounds,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub magic: Option<modules::magic::MagicUser>,
}

impl Character {
    pub fn new(
        name: &str,
        attributes: Attributes,
        weapon_skill: i32,
        dodge_skill: i32,
        weapon: Weapon,
        armor: Armor,
    ) -> Self {
        Self {
            name: name.to_string(),
            attributes,
            weapon_skill: weapon_skill.clamp(0, 10),
            dodge_skill: dodge_skill.clamp(0, 10),
            weapon,
            armor,
            wounds: Wounds::new(),
            magic: None,
        }
    }

    pub fn new_with_magic(
        name: &str,
        attributes: Attributes,
        weapon_skill: i32,
        dodge_skill: i32,
        weapon: Weapon,
        armor: Armor,
        magic: modules::magic::MagicUser,
    ) -> Self {
        Self {
            name: name.to_string(),
            attributes,
            weapon_skill: weapon_skill.clamp(0, 10),
            dodge_skill: dodge_skill.clamp(0, 10),
            weapon,
            armor,
            wounds: Wounds::new(),
            magic: Some(magic),
        }
    }

    /// Get strength bonus for damage (STR >= 7 gives +1, STR >= 9 gives +2)
    pub fn strength_bonus(&self) -> i32 {
        if self.attributes.strength >= 9 {
            2
        } else if self.attributes.strength >= 7 {
            1
        } else if self.attributes.strength <= 2 {
            -1
        } else {
            0
        }
    }

    /// Make an attack roll
    pub fn attack_roll(&self) -> i32 {
        let base = self.weapon_skill + d10();
        let penalty = self.armor.movement_penalty + self.wounds.movement_penalty();
        base + penalty
    }

    /// Make a parry roll
    pub fn parry_roll(&self) -> i32 {
        let base = self.weapon_skill + d10();
        let penalty = self.armor.movement_penalty + self.wounds.movement_penalty();
        base + penalty
    }

    /// Make a dodge roll
    pub fn dodge_roll(&self) -> i32 {
        let base = self.dodge_skill + d10();
        let penalty = self.armor.movement_penalty + self.wounds.movement_penalty();
        base + penalty
    }

    /// Check if character is alive and able to fight
    pub fn is_alive(&self) -> bool {
        !self.wounds.is_dead()
    }

    /// Check if character can still act
    pub fn can_act(&self) -> bool {
        self.is_alive() && !self.wounds.is_incapacitated()
    }
}

/// Combat action result
#[derive(Debug)]
pub struct CombatResult {
    pub attacker: String,
    pub defender: String,
    pub attack_roll: i32,
    pub defense_roll: i32,
    pub hit: bool,
    pub damage: i32,
    pub wound_level: Option<WoundLevel>,
    pub defender_died: bool,
}

/// Execute a combat round between two characters
pub fn combat_round(
    attacker: &mut Character,
    defender: &mut Character,
    defender_action: DefenseAction,
) -> CombatResult {
    let attack_roll = attacker.attack_roll();
    let defense_roll = match defender_action {
        DefenseAction::Parry => defender.parry_roll(),
        DefenseAction::Dodge => defender.dodge_roll(),
    };

    let hit = attack_roll > defense_roll;
    let mut damage = 0;
    let mut wound_level = None;
    let mut defender_died = false;

    if hit {
        // Calculate damage: attack_roll - defense_roll + strength_bonus + weapon_damage - armor_protection
        damage = (attack_roll - defense_roll) + attacker.strength_bonus() + attacker.weapon.damage
            - defender.armor.protection;

        damage = damage.max(0); // No negative damage

        if damage > 1 {
            // Determine wound level based on damage vs constitution
            let con = defender.attributes.constitution;
            let level = if damage > con * 2 {
                defender_died = true;
                WoundLevel::Critical
            } else if damage > con {
                WoundLevel::Critical
            } else if damage > con / 2 {
                WoundLevel::Severe
            } else {
                WoundLevel::Light
            };

            defender.wounds.add_wound(level);
            wound_level = Some(level);

            // Check if stacking caused death
            if defender.wounds.is_dead() {
                defender_died = true;
            }
        }
    }

    CombatResult {
        attacker: attacker.name.clone(),
        defender: defender.name.clone(),
        attack_roll,
        defense_roll,
        hit,
        damage,
        wound_level,
        defender_died,
    }
}

/// Defense action options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefenseAction {
    Parry,
    Dodge,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d10_range() {
        for _ in 0..100 {
            let roll = d10();
            assert!(roll >= 1 && roll <= 10);
        }
    }

    #[test]
    fn test_attributes() {
        let attrs = Attributes::new(8, 6, 7, 5, 6, 5, 5, 7, 4);
        assert_eq!(attrs.strength, 8);
        assert_eq!(attrs.stamina(), 8); // (8+7)/2 = 7.5 rounded to 8
    }

    #[test]
    fn test_wound_stacking() {
        let mut wounds = Wounds::new();
        wounds.add_wound(WoundLevel::Light);
        wounds.add_wound(WoundLevel::Light);
        wounds.add_wound(WoundLevel::Light);
        assert_eq!(wounds.light, 3);

        wounds.add_wound(WoundLevel::Light); // 4th light becomes severe
        assert_eq!(wounds.light, 0);
        assert_eq!(wounds.severe, 1);
    }

    #[test]
    fn test_death_threshold() {
        let mut wounds = Wounds::new();
        assert!(!wounds.is_dead());

        wounds.add_wound(WoundLevel::Critical);
        assert!(!wounds.is_dead());

        wounds.add_wound(WoundLevel::Critical);
        assert!(wounds.is_dead());
    }
}
