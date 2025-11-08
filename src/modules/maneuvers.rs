//! Special combat maneuvers based on Draft RPG Section 4.22

use std::fmt;
use inquire_derive::Selectable;

/// Special combat maneuvers that characters can perform
#[derive(Debug, Clone, Copy, PartialEq, Eq, Selectable)]
pub enum CombatManeuver {
    /// Normal attack with no special effects
    Normal,
    /// Defensive position: +2 to parry/dodge, cannot attack
    DefensivePosition,
    /// Charge: +1 attack, +1 damage, -2 defense, requires movement
    Charge,
    /// All-out attack: +2 attack, -4 defense
    AllOutAttack,
    /// Aimed attack: -2 attack, +2 damage (requires aiming previous round)
    AimedAttack,
}

impl CombatManeuver {
    /// Get attack modifier for this maneuver
    pub fn attack_modifier(&self) -> i32 {
        match self {
            CombatManeuver::Normal => 0,
            CombatManeuver::DefensivePosition => 0, // Can't attack
            CombatManeuver::Charge => 1,
            CombatManeuver::AllOutAttack => 2,
            CombatManeuver::AimedAttack => -2,
        }
    }

    /// Get defense modifier for this maneuver
    pub fn defense_modifier(&self) -> i32 {
        match self {
            CombatManeuver::Normal => 0,
            CombatManeuver::DefensivePosition => 2,
            CombatManeuver::Charge => -2,
            CombatManeuver::AllOutAttack => -4,
            CombatManeuver::AimedAttack => 0,
        }
    }

    /// Get damage modifier for this maneuver
    pub fn damage_modifier(&self) -> i32 {
        match self {
            CombatManeuver::Normal => 0,
            CombatManeuver::DefensivePosition => 0,
            CombatManeuver::Charge => 1,
            CombatManeuver::AllOutAttack => 0,
            CombatManeuver::AimedAttack => 2,
        }
    }

    /// Check if this maneuver allows attacking
    pub fn can_attack(&self) -> bool {
        !matches!(self, CombatManeuver::DefensivePosition)
    }

    /// Check if this maneuver requires preparation
    pub fn requires_preparation(&self) -> bool {
        matches!(self, CombatManeuver::AimedAttack)
    }
}

impl fmt::Display for CombatManeuver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CombatManeuver::Normal => write!(f, "Normal"),
            CombatManeuver::DefensivePosition => write!(f, "Defensive Position"),
            CombatManeuver::Charge => write!(f, "Charge"),
            CombatManeuver::AllOutAttack => write!(f, "All-Out Attack"),
            CombatManeuver::AimedAttack => write!(f, "Aimed Attack"),
        }
    }
}

/// Tracks combat stance and preparation
#[derive(Debug, Clone)]
pub struct CombatStance {
    pub current_maneuver: CombatManeuver,
    pub aiming: bool,
    pub charged_this_round: bool,
}

impl CombatStance {
    pub fn new() -> Self {
        Self {
            current_maneuver: CombatManeuver::Normal,
            aiming: false,
            charged_this_round: false,
        }
    }

    /// Set the combat maneuver for next action
    pub fn set_maneuver(&mut self, maneuver: CombatManeuver) -> Result<(), ManeuverError> {
        // Check if aimed attack without aiming
        if maneuver == CombatManeuver::AimedAttack && !self.aiming {
            return Err(ManeuverError::NotPrepared);
        }

        self.current_maneuver = maneuver;

        // Reset aiming after using aimed attack
        if maneuver == CombatManeuver::AimedAttack {
            self.aiming = false;
        }

        Ok(())
    }

    /// Start aiming for next round
    pub fn start_aiming(&mut self) {
        self.aiming = true;
    }

    /// Record that character charged this round
    pub fn record_charge(&mut self) {
        self.charged_this_round = true;
    }

    /// Reset stance at end of round
    pub fn end_round(&mut self) {
        self.charged_this_round = false;
        // Aiming persists across rounds until used
    }

    /// Get total attack modifier including maneuver
    pub fn total_attack_modifier(&self) -> i32 {
        self.current_maneuver.attack_modifier()
    }

    /// Get total defense modifier including maneuver
    pub fn total_defense_modifier(&self) -> i32 {
        self.current_maneuver.defense_modifier()
    }

    /// Get total damage modifier including maneuver
    pub fn total_damage_modifier(&self) -> i32 {
        self.current_maneuver.damage_modifier()
    }
}

impl Default for CombatStance {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManeuverError {
    NotPrepared,
}

impl fmt::Display for ManeuverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ManeuverError::NotPrepared => write!(f, "Maneuver requires preparation"),
        }
    }
}

impl std::error::Error for ManeuverError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maneuver_modifiers() {
        let charge = CombatManeuver::Charge;
        assert_eq!(charge.attack_modifier(), 1);
        assert_eq!(charge.defense_modifier(), -2);
        assert_eq!(charge.damage_modifier(), 1);

        let defensive = CombatManeuver::DefensivePosition;
        assert_eq!(defensive.defense_modifier(), 2);
        assert!(!defensive.can_attack());

        let all_out = CombatManeuver::AllOutAttack;
        assert_eq!(all_out.attack_modifier(), 2);
        assert_eq!(all_out.defense_modifier(), -4);
    }

    #[test]
    fn test_defensive_position() {
        let mut stance = CombatStance::new();
        stance
            .set_maneuver(CombatManeuver::DefensivePosition)
            .unwrap();

        assert_eq!(stance.total_defense_modifier(), 2);
        assert!(!stance.current_maneuver.can_attack());
    }

    #[test]
    fn test_charge_maneuver() {
        let mut stance = CombatStance::new();
        stance.set_maneuver(CombatManeuver::Charge).unwrap();
        stance.record_charge();

        assert_eq!(stance.total_attack_modifier(), 1);
        assert_eq!(stance.total_damage_modifier(), 1);
        assert_eq!(stance.total_defense_modifier(), -2);
        assert!(stance.charged_this_round);

        stance.end_round();
        assert!(!stance.charged_this_round);
    }

    #[test]
    fn test_aimed_attack() {
        let mut stance = CombatStance::new();

        // Can't do aimed attack without aiming
        assert!(stance.set_maneuver(CombatManeuver::AimedAttack).is_err());

        // Start aiming
        stance.start_aiming();
        assert!(stance.aiming);

        // Now can do aimed attack
        assert!(stance.set_maneuver(CombatManeuver::AimedAttack).is_ok());
        assert_eq!(stance.total_attack_modifier(), -2);
        assert_eq!(stance.total_damage_modifier(), 2);

        // Aiming is consumed
        assert!(!stance.aiming);
    }

    #[test]
    fn test_all_out_attack() {
        let mut stance = CombatStance::new();
        stance.set_maneuver(CombatManeuver::AllOutAttack).unwrap();

        assert_eq!(stance.total_attack_modifier(), 2);
        assert_eq!(stance.total_defense_modifier(), -4);
    }
}
