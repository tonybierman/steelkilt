//! Advanced features for the Draft RPG system
//!
//! This module contains optional advanced features including:
//! - Skill development and progression
//! - Exhaustion system
//! - Special combat maneuvers
//! - Hit location tracking
//! - Ranged combat
//! - Magic system

pub mod exhaustion;
pub mod hit_location;
pub mod magic;
pub mod maneuvers;
pub mod ranged_combat;
pub mod skills;

// Re-export commonly used types
pub use exhaustion::{Exhaustion, ExhaustionLevel};
pub use hit_location::{AttackDirection, HitLocation, LocationalDamage};
pub use magic::{CastingResult, MagicBranch, MagicError, MagicLore, MagicUser, Spell};
pub use maneuvers::{CombatManeuver, CombatStance, ManeuverError};
pub use ranged_combat::{
    calculate_ranged_modifiers, Cover, RangedAttackState, RangedWeapon, TargetSize,
};
pub use skills::{Skill, SkillDifficulty, SkillError, SkillSet};
