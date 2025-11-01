//! Ranged combat mechanics based on Draft RPG Section 4.21

use std::fmt;

/// Types of ranged weapons
#[derive(Debug, Clone)]
pub struct RangedWeapon {
    pub name: String,
    pub damage: i32,
    pub point_blank_range: i32,  // meters
    pub max_range: i32,           // meters
    pub preparation_time: i32,    // segments
    pub rate_of_fire: i32,        // shots per round (usually 1-3)
}

impl RangedWeapon {
    pub fn short_bow() -> Self {
        Self {
            name: "Short Bow".to_string(),
            damage: 4,
            point_blank_range: 20,
            max_range: 100,
            preparation_time: 3,
            rate_of_fire: 1,
        }
    }

    pub fn long_bow() -> Self {
        Self {
            name: "Long Bow".to_string(),
            damage: 6,
            point_blank_range: 30,
            max_range: 120,
            preparation_time: 3,
            rate_of_fire: 1,
        }
    }

    pub fn crossbow() -> Self {
        Self {
            name: "Crossbow".to_string(),
            damage: 6,
            point_blank_range: 30,
            max_range: 100,
            preparation_time: 6, // Takes longer to reload
            rate_of_fire: 1,
        }
    }

    pub fn pistol() -> Self {
        Self {
            name: "Pistol".to_string(),
            damage: 6,
            point_blank_range: 20,
            max_range: 80,
            preparation_time: 1,
            rate_of_fire: 3,
        }
    }

    pub fn rifle() -> Self {
        Self {
            name: "Rifle".to_string(),
            damage: 8,
            point_blank_range: 40,
            max_range: 200,
            preparation_time: 2,
            rate_of_fire: 2,
        }
    }

    pub fn javelin() -> Self {
        Self {
            name: "Javelin".to_string(),
            damage: 4,
            point_blank_range: 15,
            max_range: 40,
            preparation_time: 1,
            rate_of_fire: 1,
        }
    }

    /// Calculate distance modifier for attack roll
    pub fn distance_modifier(&self, distance: i32) -> i32 {
        if distance <= self.point_blank_range {
            0
        } else if distance <= self.max_range {
            // -1 per 10m beyond point blank for bows
            // -1 per 20m for guns
            let increment = if self.name.contains("Bow") || self.name == "Javelin" {
                10
            } else {
                20
            };
            let beyond = distance - self.point_blank_range;
            -(beyond / increment)
        } else {
            -999 // Out of range
        }
    }

    /// Check if weapon is in range
    pub fn in_range(&self, distance: i32) -> bool {
        distance <= self.max_range
    }
}

/// Target size modifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetSize {
    Tiny,      // -4 (rat, small bird)
    Small,     // -2 (cat, small dog)
    Medium,    // 0 (human)
    Large,     // +2 (horse, car)
    Huge,      // +4 (dragon, tank)
    Gigantic,  // +6 (whale, building)
}

impl TargetSize {
    pub fn modifier(&self) -> i32 {
        match self {
            TargetSize::Tiny => -4,
            TargetSize::Small => -2,
            TargetSize::Medium => 0,
            TargetSize::Large => 2,
            TargetSize::Huge => 4,
            TargetSize::Gigantic => 6,
        }
    }
}

/// Ranged attack state
#[derive(Debug, Clone)]
pub struct RangedAttackState {
    pub weapon_ready: bool,
    pub aiming: bool,
    pub aiming_rounds: i32,
    pub shots_remaining: i32,
}

impl RangedAttackState {
    pub fn new() -> Self {
        Self {
            weapon_ready: false,
            aiming: false,
            aiming_rounds: 0,
            shots_remaining: 0,
        }
    }

    /// Prepare weapon (draw and ready)
    pub fn prepare_weapon(&mut self, weapon: &RangedWeapon) {
        self.weapon_ready = true;
        self.shots_remaining = weapon.rate_of_fire;
    }

    /// Start aiming
    pub fn start_aiming(&mut self) {
        self.aiming = true;
        self.aiming_rounds = 0;
    }

    /// Continue aiming (called each round)
    pub fn continue_aiming(&mut self) {
        if self.aiming {
            self.aiming_rounds += 1;
        }
    }

    /// Get aiming bonus (+1 for 1 round of aiming, max +1)
    pub fn aiming_bonus(&self) -> i32 {
        if self.aiming && self.aiming_rounds >= 1 {
            1
        } else {
            0
        }
    }

    /// Fire weapon
    pub fn fire(&mut self) -> Result<(), RangedCombatError> {
        if !self.weapon_ready {
            return Err(RangedCombatError::WeaponNotReady);
        }

        if self.shots_remaining <= 0 {
            return Err(RangedCombatError::NoAmmunition);
        }

        self.shots_remaining -= 1;
        self.aiming = false;
        self.aiming_rounds = 0;

        Ok(())
    }

    /// Reload weapon
    pub fn reload(&mut self, weapon: &RangedWeapon) {
        self.weapon_ready = true;
        self.shots_remaining = weapon.rate_of_fire;
    }
}

impl Default for RangedAttackState {
    fn default() -> Self {
        Self::new()
    }
}

/// Cover types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cover {
    None,
    Partial,    // -2 to hit, 1/2 body exposed
    ThreeQuarters, // -4 to hit, 1/4 body exposed
    Full,       // -8 to hit, only small parts visible
}

impl Cover {
    pub fn modifier(&self) -> i32 {
        match self {
            Cover::None => 0,
            Cover::Partial => -2,
            Cover::ThreeQuarters => -4,
            Cover::Full => -8,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangedCombatError {
    WeaponNotReady,
    NoAmmunition,
    OutOfRange,
}

impl fmt::Display for RangedCombatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RangedCombatError::WeaponNotReady => write!(f, "Weapon not ready"),
            RangedCombatError::NoAmmunition => write!(f, "No ammunition"),
            RangedCombatError::OutOfRange => write!(f, "Target out of range"),
        }
    }
}

impl std::error::Error for RangedCombatError {}

/// Calculate total modifier for ranged attack
pub fn calculate_ranged_modifiers(
    distance: i32,
    target_size: TargetSize,
    cover: Cover,
    weapon: &RangedWeapon,
    state: &RangedAttackState,
) -> i32 {
    let distance_mod = weapon.distance_modifier(distance);
    let size_mod = target_size.modifier();
    let cover_mod = cover.modifier();
    let aiming_mod = state.aiming_bonus();

    distance_mod + size_mod + cover_mod + aiming_mod
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ranged_weapon_range() {
        let bow = RangedWeapon::long_bow();

        // Within point blank
        assert_eq!(bow.distance_modifier(20), 0);

        // Beyond point blank
        assert_eq!(bow.distance_modifier(40), -1); // 10m beyond
        assert_eq!(bow.distance_modifier(50), -2); // 20m beyond

        // Out of range
        assert_eq!(bow.distance_modifier(150), -999);

        assert!(bow.in_range(100));
        assert!(!bow.in_range(150));
    }

    #[test]
    fn test_target_size_modifiers() {
        assert_eq!(TargetSize::Tiny.modifier(), -4);
        assert_eq!(TargetSize::Medium.modifier(), 0);
        assert_eq!(TargetSize::Huge.modifier(), 4);
    }

    #[test]
    fn test_cover_modifiers() {
        assert_eq!(Cover::None.modifier(), 0);
        assert_eq!(Cover::Partial.modifier(), -2);
        assert_eq!(Cover::Full.modifier(), -8);
    }

    #[test]
    fn test_aiming() {
        let mut state = RangedAttackState::new();
        let bow = RangedWeapon::short_bow();

        state.prepare_weapon(&bow);
        assert_eq!(state.aiming_bonus(), 0);

        state.start_aiming();
        assert_eq!(state.aiming_bonus(), 0); // Need to wait a round

        state.continue_aiming();
        assert_eq!(state.aiming_bonus(), 1);

        state.continue_aiming();
        assert_eq!(state.aiming_bonus(), 1); // Max +1
    }

    #[test]
    fn test_firing() {
        let mut state = RangedAttackState::new();
        let pistol = RangedWeapon::pistol();

        // Can't fire without preparation
        assert!(state.fire().is_err());

        state.prepare_weapon(&pistol);
        assert_eq!(state.shots_remaining, 3);

        // Fire shots
        assert!(state.fire().is_ok());
        assert_eq!(state.shots_remaining, 2);

        assert!(state.fire().is_ok());
        assert_eq!(state.shots_remaining, 1);

        assert!(state.fire().is_ok());
        assert_eq!(state.shots_remaining, 0);

        // Out of ammo
        assert!(state.fire().is_err());

        // Reload
        state.reload(&pistol);
        assert_eq!(state.shots_remaining, 3);
    }

    #[test]
    fn test_calculate_ranged_modifiers() {
        let bow = RangedWeapon::long_bow();
        let mut state = RangedAttackState::new();
        state.prepare_weapon(&bow);
        state.start_aiming();
        state.continue_aiming();

        let total = calculate_ranged_modifiers(
            25,                    // distance
            TargetSize::Medium,   // size
            Cover::Partial,       // cover
            &bow,
            &state,
        );

        // 0 (distance) + 0 (size) + (-2) (cover) + 1 (aiming) = -1
        assert_eq!(total, -1);
    }

    #[test]
    fn test_different_weapons() {
        let crossbow = RangedWeapon::crossbow();
        assert_eq!(crossbow.preparation_time, 6); // Slower to load

        let javelin = RangedWeapon::javelin();
        assert_eq!(javelin.preparation_time, 1); // Quick to throw

        let rifle = RangedWeapon::rifle();
        assert_eq!(rifle.max_range, 200); // Long range
    }
}
