//! Combat state management module
//!
//! Manages all mutable state during combat including:
//! - Character stance/maneuvers
//! - Exhaustion tracking
//! - Hit location damage accumulation

use steelkilt::modules::*;
use steelkilt::*;

/// Encapsulates all combat state for a single fighter
pub struct CombatantModel {
    pub character: Character,
    pub stance: CombatStance,
    pub exhaustion: Exhaustion,
    pub locations: Vec<LocationalDamage>,
}

impl CombatantModel {
    /// Create a new fighter state from character and skills
    pub fn new(character: Character) -> Self {
        let stamina = character.attributes.stamina();

        Self {
            character,
            stance: CombatStance::new(),
            exhaustion: Exhaustion::new(stamina),
            locations: Vec::new(),
        }
    }

    /// Add exhaustion points (e.g., at the start of each round)
    pub fn add_exhaustion(&mut self, points: i32) {
        self.exhaustion.add_points(points);
    }

    /// Set the fighter's combat maneuver
    pub fn set_maneuver(&mut self, maneuver: CombatManeuver) -> Result<(), ManeuverError> {
        self.stance.set_maneuver(maneuver)
    }

    /// Check if the fighter can attack with their current stance
    pub fn can_attack(&self) -> bool {
        self.stance.current_maneuver.can_attack()
    }

    /// Check if the fighter is still alive and capable of action
    pub fn can_act(&self) -> bool {
        self.character.can_act()
    }

    /// Check if the fighter is alive
    pub fn is_alive(&self) -> bool {
        self.character.is_alive()
    }

    /// Reset round-specific stance flags
    pub fn end_round(&mut self) {
        self.stance.end_round();
    }

    /// Get total attack modifier from stance and exhaustion
    pub fn total_attack_modifier(&self) -> i32 {
        self.stance.total_attack_modifier() + self.exhaustion.penalty()
    }

    /// Get stance damage modifier
    pub fn stance_damage_modifier(&self) -> i32 {
        self.stance.total_damage_modifier()
    }

    /// Add wound to a specific hit location
    pub fn add_location_wound(
        &mut self,
        location: HitLocation,
        severity: hit_location::WoundSeverity,
    ) -> bool {
        // Find or create location tracker
        let loc_dmg = self.locations.iter_mut().find(|l| l.location == location);

        if let Some(loc) = loc_dmg {
            loc.add_wound(severity);
            loc.disabled
        } else {
            let mut new_loc = LocationalDamage::new(location);
            new_loc.add_wound(severity);
            let disabled = new_loc.disabled;
            self.locations.push(new_loc);
            disabled
        }
    }
}

/// Manages the overall combat simulation state
pub struct MeleeModel {
    pub combatant1: CombatantModel,
    pub combatant2: CombatantModel,
    pub round: usize,
}

impl MeleeModel {
    /// Create new combat state with two fighters
    pub fn new(c1: CombatantModel, c2: CombatantModel) -> Self {
        Self {
            combatant1: c1,
            combatant2: c2,
            round: 0,
        }
    }

    /// Check if combat should continue
    pub fn combat_continues(&self) -> bool {
        self.combatant1.can_act() && self.combatant2.can_act()
    }

    /// Increment round counter and apply per-round effects
    pub fn next_round(&mut self) {
        self.round += 1;

        // Add exhaustion each round
        self.combatant1.add_exhaustion(1);
        self.combatant2.add_exhaustion(1);
    }

    /// Reset round-specific flags for both fighters
    pub fn end_round(&mut self) {
        self.combatant1.end_round();
        self.combatant2.end_round();
    }
}
