//! Advanced features for the Draft RPG system
//!
//! This module contains optional advanced features including:
//! - Skill development and progression
//! - Exhaustion system
//! - Special combat maneuvers
//! - Hit location tracking
//! - Ranged combat
//! - Magic system

pub mod skills;
pub mod exhaustion;
pub mod maneuvers;
pub mod hit_location;
pub mod ranged_combat;
pub mod magic;

// Re-export commonly used types
pub use skills::{Skill, SkillSet, SkillDifficulty, SkillError};
pub use exhaustion::{Exhaustion, ExhaustionLevel};
pub use maneuvers::{CombatManeuver, CombatStance, ManeuverError};
pub use hit_location::{HitLocation, AttackDirection, LocationalDamage};
pub use ranged_combat::{RangedWeapon, TargetSize, Cover, RangedAttackState, calculate_ranged_modifiers};
pub use magic::{MagicBranch, MagicLore, Spell, MagicUser, CastingResult, MagicError};
