//! Hit location tracking system based on Draft RPG Section 4.24.3

use crate::d10;
use std::fmt;

/// Body locations that can be hit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HitLocation {
    Head,
    Torso,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
}

impl HitLocation {
    /// Determine hit location based on attack direction
    pub fn determine(direction: AttackDirection) -> Self {
        let roll = d10();
        match direction {
            AttackDirection::Front | AttackDirection::Back => match roll {
                1..=2 => HitLocation::LeftLeg,
                3..=4 => HitLocation::RightLeg,
                5..=6 => HitLocation::Torso,
                7 => HitLocation::LeftArm,
                8 => HitLocation::RightArm,
                9..=10 => HitLocation::Head,
                _ => HitLocation::Torso,
            },
            AttackDirection::Left | AttackDirection::Right => match roll {
                1..=2 => HitLocation::LeftLeg,
                3..=4 => HitLocation::Torso,
                5..=7 => HitLocation::LeftArm,
                8 => HitLocation::RightArm,
                9..=10 => HitLocation::Head,
                _ => HitLocation::Torso,
            },
            AttackDirection::Above => match roll {
                1 => HitLocation::LeftLeg,
                2 => HitLocation::RightLeg,
                3 => HitLocation::Torso,
                4..=5 => HitLocation::LeftArm,
                6..=7 => HitLocation::RightArm,
                8..=10 => HitLocation::Head,
                _ => HitLocation::Torso,
            },
            AttackDirection::Below => match roll {
                1..=2 => HitLocation::LeftLeg,
                3..=4 => HitLocation::RightLeg,
                5..=7 => HitLocation::Torso,
                8 => HitLocation::LeftArm,
                9 => HitLocation::RightArm,
                10 => HitLocation::Head,
                _ => HitLocation::Torso,
            },
        }
    }

    /// Get damage multiplier for this location (critical hits)
    pub fn damage_multiplier(&self) -> f32 {
        match self {
            HitLocation::Head => 1.5,
            HitLocation::Torso => 1.0,
            HitLocation::LeftArm | HitLocation::RightArm => 0.75,
            HitLocation::LeftLeg | HitLocation::RightLeg => 0.75,
        }
    }

    /// Check if hit to this location causes weapon drop
    pub fn causes_weapon_drop(&self) -> bool {
        matches!(self, HitLocation::LeftArm | HitLocation::RightArm)
    }

    /// Check if hit to this location can sever limb (for severe/critical wounds)
    pub fn can_sever(&self) -> bool {
        matches!(
            self,
            HitLocation::LeftArm
                | HitLocation::RightArm
                | HitLocation::LeftLeg
                | HitLocation::RightLeg
        )
    }
}

impl fmt::Display for HitLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HitLocation::Head => write!(f, "Head"),
            HitLocation::Torso => write!(f, "Torso"),
            HitLocation::LeftArm => write!(f, "Left Arm"),
            HitLocation::RightArm => write!(f, "Right Arm"),
            HitLocation::LeftLeg => write!(f, "Left Leg"),
            HitLocation::RightLeg => write!(f, "Right Leg"),
        }
    }
}

/// Direction of attack for hit location determination
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackDirection {
    Front,
    Back,
    Left,
    Right,
    Above,
    Below,
}

impl fmt::Display for AttackDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttackDirection::Front => write!(f, "Front"),
            AttackDirection::Back => write!(f, "Back"),
            AttackDirection::Left => write!(f, "Left"),
            AttackDirection::Right => write!(f, "Right"),
            AttackDirection::Above => write!(f, "Above"),
            AttackDirection::Below => write!(f, "Below"),
        }
    }
}

/// Tracks injuries to specific body locations
#[derive(Debug, Clone)]
pub struct LocationalDamage {
    pub location: HitLocation,
    pub light_wounds: i32,
    pub severe_wounds: i32,
    pub critical_wounds: i32,
    pub severed: bool,
    pub disabled: bool,
}

impl LocationalDamage {
    pub fn new(location: HitLocation) -> Self {
        Self {
            location,
            light_wounds: 0,
            severe_wounds: 0,
            critical_wounds: 0,
            severed: false,
            disabled: false,
        }
    }

    /// Add a wound to this location
    pub fn add_wound(&mut self, severity: WoundSeverity) {
        match severity {
            WoundSeverity::Light => self.light_wounds += 1,
            WoundSeverity::Severe => {
                self.severe_wounds += 1;
                // Severe wound to arm/leg might disable it
                if self.location.causes_weapon_drop() {
                    self.disabled = true;
                }
            }
            WoundSeverity::Critical => {
                self.critical_wounds += 1;
                self.disabled = true;
                // Critical wound might sever limb
                if self.location.can_sever() && self.critical_wounds >= 2 {
                    self.severed = true;
                }
            }
        }
    }

    /// Check if this location is functional
    pub fn is_functional(&self) -> bool {
        !self.disabled && !self.severed
    }

    /// Get penalty from wounds to this location
    pub fn penalty(&self) -> i32 {
        if self.severed {
            return -999; // Completely unusable
        }
        if self.disabled {
            return -4;
        }
        -(self.light_wounds + self.severe_wounds * 2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WoundSeverity {
    Light,
    Severe,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_location_determination() {
        // Test that we get valid locations
        for _ in 0..100 {
            let loc = HitLocation::determine(AttackDirection::Front);
            match loc {
                HitLocation::Head
                | HitLocation::Torso
                | HitLocation::LeftArm
                | HitLocation::RightArm
                | HitLocation::LeftLeg
                | HitLocation::RightLeg => {}
            }
        }
    }

    #[test]
    fn test_damage_multipliers() {
        assert_eq!(HitLocation::Head.damage_multiplier(), 1.5);
        assert_eq!(HitLocation::Torso.damage_multiplier(), 1.0);
        assert_eq!(HitLocation::LeftArm.damage_multiplier(), 0.75);
    }

    #[test]
    fn test_weapon_drop() {
        assert!(HitLocation::LeftArm.causes_weapon_drop());
        assert!(HitLocation::RightArm.causes_weapon_drop());
        assert!(!HitLocation::Torso.causes_weapon_drop());
    }

    #[test]
    fn test_locational_damage() {
        let mut arm = LocationalDamage::new(HitLocation::LeftArm);

        assert!(arm.is_functional());

        // Light wounds don't disable
        arm.add_wound(WoundSeverity::Light);
        assert!(arm.is_functional());
        assert_eq!(arm.penalty(), -1);

        // Severe wound disables arm
        arm.add_wound(WoundSeverity::Severe);
        assert!(!arm.is_functional());
        assert!(arm.disabled);

        // Critical wounds
        arm.add_wound(WoundSeverity::Critical);
        assert!(!arm.is_functional());

        // Second critical severs limb
        arm.add_wound(WoundSeverity::Critical);
        assert!(arm.severed);
        assert_eq!(arm.penalty(), -999);
    }

    #[test]
    fn test_head_wounds() {
        let mut head = LocationalDamage::new(HitLocation::Head);

        // Head can't be severed
        assert!(!head.location.can_sever());

        head.add_wound(WoundSeverity::Critical);
        assert!(head.disabled);
        assert!(!head.severed);
    }

    #[test]
    fn test_torso_wounds() {
        let mut torso = LocationalDamage::new(HitLocation::Torso);

        // Torso doesn't cause weapon drop
        assert!(!torso.location.causes_weapon_drop());

        torso.add_wound(WoundSeverity::Severe);
        // Torso wounds don't automatically disable
        assert!(!torso.disabled);
    }

    #[test]
    fn test_all_attack_directions() {
        // Test that all attack directions produce valid hit locations
        let directions = [
            AttackDirection::Front,
            AttackDirection::Back,
            AttackDirection::Left,
            AttackDirection::Right,
            AttackDirection::Above,
            AttackDirection::Below,
        ];

        for direction in &directions {
            for _ in 0..20 {
                let loc = HitLocation::determine(*direction);
                // Just verify it's a valid location
                match loc {
                    HitLocation::Head
                    | HitLocation::Torso
                    | HitLocation::LeftArm
                    | HitLocation::RightArm
                    | HitLocation::LeftLeg
                    | HitLocation::RightLeg => {}
                }
            }
        }
    }

    #[test]
    fn test_hit_location_display() {
        assert_eq!(format!("{}", HitLocation::Head), "Head");
        assert_eq!(format!("{}", HitLocation::Torso), "Torso");
        assert_eq!(format!("{}", HitLocation::LeftArm), "Left Arm");
        assert_eq!(format!("{}", HitLocation::RightArm), "Right Arm");
        assert_eq!(format!("{}", HitLocation::LeftLeg), "Left Leg");
        assert_eq!(format!("{}", HitLocation::RightLeg), "Right Leg");
    }

    #[test]
    fn test_attack_direction_display() {
        assert_eq!(format!("{}", AttackDirection::Front), "Front");
        assert_eq!(format!("{}", AttackDirection::Back), "Back");
        assert_eq!(format!("{}", AttackDirection::Left), "Left");
        assert_eq!(format!("{}", AttackDirection::Right), "Right");
        assert_eq!(format!("{}", AttackDirection::Above), "Above");
        assert_eq!(format!("{}", AttackDirection::Below), "Below");
    }

    #[test]
    fn test_all_damage_multipliers() {
        assert_eq!(HitLocation::Head.damage_multiplier(), 1.5);
        assert_eq!(HitLocation::Torso.damage_multiplier(), 1.0);
        assert_eq!(HitLocation::LeftArm.damage_multiplier(), 0.75);
        assert_eq!(HitLocation::RightArm.damage_multiplier(), 0.75);
        assert_eq!(HitLocation::LeftLeg.damage_multiplier(), 0.75);
        assert_eq!(HitLocation::RightLeg.damage_multiplier(), 0.75);
    }

    #[test]
    fn test_all_weapon_drop_locations() {
        // Arms cause weapon drop
        assert!(HitLocation::LeftArm.causes_weapon_drop());
        assert!(HitLocation::RightArm.causes_weapon_drop());

        // Other locations don't
        assert!(!HitLocation::Head.causes_weapon_drop());
        assert!(!HitLocation::Torso.causes_weapon_drop());
        assert!(!HitLocation::LeftLeg.causes_weapon_drop());
        assert!(!HitLocation::RightLeg.causes_weapon_drop());
    }

    #[test]
    fn test_all_can_sever_locations() {
        // Limbs can be severed
        assert!(HitLocation::LeftArm.can_sever());
        assert!(HitLocation::RightArm.can_sever());
        assert!(HitLocation::LeftLeg.can_sever());
        assert!(HitLocation::RightLeg.can_sever());

        // Head and torso cannot
        assert!(!HitLocation::Head.can_sever());
        assert!(!HitLocation::Torso.can_sever());
    }

    #[test]
    fn test_right_arm_wounds() {
        let mut arm = LocationalDamage::new(HitLocation::RightArm);

        assert!(arm.is_functional());
        assert_eq!(arm.penalty(), 0);

        // Light wound
        arm.add_wound(WoundSeverity::Light);
        assert!(arm.is_functional());
        assert_eq!(arm.penalty(), -1);

        // Severe wound disables
        arm.add_wound(WoundSeverity::Severe);
        assert!(!arm.is_functional());
        assert!(arm.disabled);
        assert_eq!(arm.penalty(), -4);
    }

    #[test]
    fn test_leg_wounds() {
        let mut leg = LocationalDamage::new(HitLocation::LeftLeg);

        // Legs don't cause weapon drop
        assert!(!leg.location.causes_weapon_drop());

        // But can be severed
        assert!(leg.location.can_sever());

        // Severe wound doesn't disable legs (only arms)
        leg.add_wound(WoundSeverity::Severe);
        assert!(!leg.disabled);
        assert!(leg.is_functional());

        // Critical wounds do disable
        leg.add_wound(WoundSeverity::Critical);
        assert!(leg.disabled);
        assert!(!leg.is_functional());

        // Two criticals sever
        leg.add_wound(WoundSeverity::Critical);
        assert!(leg.severed);
        assert_eq!(leg.penalty(), -999);
    }

    #[test]
    fn test_right_leg_wounds() {
        let mut leg = LocationalDamage::new(HitLocation::RightLeg);

        assert!(leg.location.can_sever());
        assert!(!leg.location.causes_weapon_drop());

        leg.add_wound(WoundSeverity::Critical);
        leg.add_wound(WoundSeverity::Critical);
        assert!(leg.severed);
    }

    #[test]
    fn test_multiple_light_wounds() {
        let mut torso = LocationalDamage::new(HitLocation::Torso);

        torso.add_wound(WoundSeverity::Light);
        assert_eq!(torso.penalty(), -1);

        torso.add_wound(WoundSeverity::Light);
        assert_eq!(torso.penalty(), -2);

        torso.add_wound(WoundSeverity::Light);
        assert_eq!(torso.penalty(), -3);

        assert!(torso.is_functional());
    }

    #[test]
    fn test_multiple_severe_wounds() {
        let mut torso = LocationalDamage::new(HitLocation::Torso);

        torso.add_wound(WoundSeverity::Severe);
        assert_eq!(torso.penalty(), -2);
        assert!(!torso.disabled); // Torso doesn't get disabled by severe

        torso.add_wound(WoundSeverity::Severe);
        assert_eq!(torso.penalty(), -4);
    }

    #[test]
    fn test_mixed_wounds() {
        let mut torso = LocationalDamage::new(HitLocation::Torso);

        torso.add_wound(WoundSeverity::Light);
        torso.add_wound(WoundSeverity::Severe);
        torso.add_wound(WoundSeverity::Light);

        // Penalty: 2 light (-2) + 1 severe (-2) = -4
        assert_eq!(torso.penalty(), -4);
    }

    #[test]
    fn test_critical_on_torso() {
        let mut torso = LocationalDamage::new(HitLocation::Torso);

        // Torso can't be severed
        assert!(!torso.location.can_sever());

        torso.add_wound(WoundSeverity::Critical);
        assert!(torso.disabled);
        assert!(!torso.severed); // Never severed

        torso.add_wound(WoundSeverity::Critical);
        assert!(torso.disabled);
        assert!(!torso.severed); // Still not severed

        assert_eq!(torso.penalty(), -4);
    }

    #[test]
    fn test_single_critical_doesnt_sever() {
        let mut arm = LocationalDamage::new(HitLocation::LeftArm);

        arm.add_wound(WoundSeverity::Critical);
        assert!(arm.disabled);
        assert!(!arm.severed); // Only 1 critical, need 2 to sever

        assert_eq!(arm.penalty(), -4);
    }

    #[test]
    fn test_disabled_but_not_severed_penalty() {
        let mut arm = LocationalDamage::new(HitLocation::RightArm);

        // Severe wound disables arm
        arm.add_wound(WoundSeverity::Severe);
        assert!(arm.disabled);
        assert!(!arm.severed);
        assert_eq!(arm.penalty(), -4);

        // Add light wounds - penalty from disabled, not individual wounds
        arm.add_wound(WoundSeverity::Light);
        assert_eq!(arm.penalty(), -4);
    }

    #[test]
    fn test_locational_damage_initialization() {
        let head = LocationalDamage::new(HitLocation::Head);
        assert_eq!(head.light_wounds, 0);
        assert_eq!(head.severe_wounds, 0);
        assert_eq!(head.critical_wounds, 0);
        assert!(!head.severed);
        assert!(!head.disabled);
        assert!(head.is_functional());
        assert_eq!(head.penalty(), 0);
    }
}
