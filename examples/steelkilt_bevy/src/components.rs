use bevy::prelude::*;
use steelkilt::Character;
use steelkilt::modules::ranged_combat::RangedAttackState;

// ===== GAME ENTITIES =====

#[derive(Component, Clone)]
pub struct Fighter {
    pub character: Character,
    pub is_player_one: bool,
    pub ranged_state: Option<RangedAttackState>,
}

// ===== UI MARKERS =====

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct ManagementUI;

#[derive(Component)]
pub struct SelectionUI;

#[derive(Component)]
pub struct SelectionText;

#[derive(Component)]
pub struct CombatUI;

#[derive(Component)]
pub struct CombatLogText;

#[derive(Component)]
pub struct StatusText {
    pub fighter_id: u8,
}

#[derive(Component)]
pub struct InstructionText;

// Allow dead code for component intended for future features
#[allow(dead_code)]
#[derive(Component)]
pub struct DynamicText;

#[derive(Component)]
pub struct ManagementText;
