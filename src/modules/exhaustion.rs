//! Exhaustion system based on Draft RPG Section 4.24.1

use std::fmt;

/// Tracks character exhaustion from combat and physical exertion
#[derive(Debug, Clone)]
pub struct Exhaustion {
    pub points: i32,
    pub stamina_threshold: i32,
}

impl Exhaustion {
    pub fn new(stamina: i32) -> Self {
        Self {
            points: 0,
            stamina_threshold: stamina,
        }
    }

    /// Add exhaustion points from combat or exertion
    pub fn add_points(&mut self, points: i32) {
        self.points += points;
    }

    /// Recover exhaustion points through rest (1 point per 2 rounds of rest)
    pub fn rest(&mut self, rounds: i32) {
        let recovery = rounds / 2;
        self.points = (self.points - recovery).max(0);
    }

    /// Get the current exhaustion level
    pub fn level(&self) -> ExhaustionLevel {
        if self.points >= self.stamina_threshold * 3 {
            ExhaustionLevel::Critical
        } else if self.points >= self.stamina_threshold * 2 {
            ExhaustionLevel::Severe
        } else if self.points > self.stamina_threshold {
            ExhaustionLevel::Light
        } else {
            ExhaustionLevel::None
        }
    }

    /// Get penalty from exhaustion for actions
    pub fn penalty(&self) -> i32 {
        match self.level() {
            ExhaustionLevel::None => 0,
            ExhaustionLevel::Light => -1,
            ExhaustionLevel::Severe => -2,
            ExhaustionLevel::Critical => -4,
        }
    }

    /// Check if character needs a willpower check to continue
    pub fn needs_willpower_check(&self) -> bool {
        self.points >= self.stamina_threshold * 2
    }

    /// Check if character can perform exhaustive actions (sprinting, jumping)
    pub fn can_perform_exhaustive_actions(&self) -> bool {
        self.level() != ExhaustionLevel::Critical
    }

    /// Get descriptive status
    pub fn status(&self) -> &str {
        match self.level() {
            ExhaustionLevel::None => "Fresh",
            ExhaustionLevel::Light => "Tired",
            ExhaustionLevel::Severe => "Exhausted",
            ExhaustionLevel::Critical => "Completely Drained",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExhaustionLevel {
    None,
    Light,
    Severe,
    Critical,
}

impl fmt::Display for ExhaustionLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExhaustionLevel::None => write!(f, "None"),
            ExhaustionLevel::Light => write!(f, "Light"),
            ExhaustionLevel::Severe => write!(f, "Severe"),
            ExhaustionLevel::Critical => write!(f, "Critical"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exhaustion_levels() {
        let mut exhaustion = Exhaustion::new(7); // Stamina of 7

        assert_eq!(exhaustion.level(), ExhaustionLevel::None);
        assert_eq!(exhaustion.penalty(), 0);

        // Add points to reach light exhaustion (> stamina)
        exhaustion.add_points(8);
        assert_eq!(exhaustion.level(), ExhaustionLevel::Light);
        assert_eq!(exhaustion.penalty(), -1);

        // Reach severe exhaustion (>= 2x stamina)
        exhaustion.add_points(6);
        assert_eq!(exhaustion.level(), ExhaustionLevel::Severe);
        assert_eq!(exhaustion.penalty(), -2);
        assert!(exhaustion.needs_willpower_check());

        // Reach critical exhaustion (>= 3x stamina)
        exhaustion.add_points(7);
        assert_eq!(exhaustion.level(), ExhaustionLevel::Critical);
        assert_eq!(exhaustion.penalty(), -4);
        assert!(!exhaustion.can_perform_exhaustive_actions());
    }

    #[test]
    fn test_exhaustion_recovery() {
        let mut exhaustion = Exhaustion::new(7);
        exhaustion.add_points(10); // Light exhaustion

        assert_eq!(exhaustion.points, 10);
        assert_eq!(exhaustion.level(), ExhaustionLevel::Light);

        // Rest for 4 rounds (recovers 2 points)
        exhaustion.rest(4);
        assert_eq!(exhaustion.points, 8);

        // Rest for 16 rounds (recovers 8 points)
        exhaustion.rest(16);
        assert_eq!(exhaustion.points, 0);
        assert_eq!(exhaustion.level(), ExhaustionLevel::None);

        // Can't go below 0
        exhaustion.rest(10);
        assert_eq!(exhaustion.points, 0);
    }

    #[test]
    fn test_exhaustion_status() {
        let mut exhaustion = Exhaustion::new(5);

        assert_eq!(exhaustion.status(), "Fresh");

        exhaustion.add_points(6);
        assert_eq!(exhaustion.status(), "Tired");

        exhaustion.add_points(4);
        assert_eq!(exhaustion.status(), "Exhausted");

        exhaustion.add_points(5);
        assert_eq!(exhaustion.status(), "Completely Drained");
    }

    #[test]
    fn test_combat_exhaustion() {
        // Simulate 10 rounds of combat
        let mut exhaustion = Exhaustion::new(7);

        for _ in 0..10 {
            exhaustion.add_points(1); // 1 point per combat round
        }

        assert_eq!(exhaustion.points, 10);
        assert_eq!(exhaustion.level(), ExhaustionLevel::Light);

        // Continue for 4 more rounds
        for _ in 0..4 {
            exhaustion.add_points(1);
        }

        assert_eq!(exhaustion.level(), ExhaustionLevel::Severe);
    }
}
