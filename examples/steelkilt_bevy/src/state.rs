use bevy::prelude::*;

use crate::file_ops::load_available_combatants;

// ===== GAME STATE =====

#[derive(Resource, Default, PartialEq, Clone)]
pub enum GameStateEnum {
    #[default]
    MainMenu,
    Management,
    Selection,
    Combat,
}

#[derive(Resource)]
pub struct GameState {
    pub current: GameStateEnum,
    pub previous: Option<GameStateEnum>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current: GameStateEnum::MainMenu,
            previous: None,
        }
    }
}

impl GameState {
    pub fn transition_to(&mut self, new_state: GameStateEnum) {
        self.previous = Some(self.current.clone());
        self.current = new_state;
    }

    pub fn is_in(&self, state: GameStateEnum) -> bool {
        self.current == state
    }
}

// ===== COMBATANT MANAGEMENT STATE =====

#[derive(Resource)]
pub struct ManagementState {
    pub combatants: Vec<String>,
    pub selected_index: usize,
    pub mode: ManagementMode,
    pub confirm_delete: Option<String>,
}

#[derive(PartialEq)]
pub enum ManagementMode {
    List,
    View,
}

impl Default for ManagementState {
    fn default() -> Self {
        Self {
            combatants: load_available_combatants(),
            selected_index: 0,
            mode: ManagementMode::List,
            confirm_delete: None,
        }
    }
}

// ===== COMBAT STATE =====

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Distance {
    Close,  // Within point blank range
    Medium, // Beyond point blank, within max range
    Long,   // Near max range
}

impl Distance {
    pub fn meters(&self) -> i32 {
        match self {
            Distance::Close => 15,
            Distance::Medium => 40,
            Distance::Long => 80,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CombatMode {
    Melee,
    Ranged,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum RangedAttackPhase {
    ChoosingMode, // Deciding whether to use ranged or melee
    Preparing,    // Drawing and readying weapon
    Aiming,       // Optional aiming phase
    ReadyToFire,  // Can fire this round
    Fired,        // Already fired this round
}

#[derive(Resource)]
pub struct CombatState {
    pub round: u32,
    pub waiting_for_defense: bool,
    pub current_attacker: u8,
    pub combat_log: Vec<String>,
    pub game_over: bool,
    pub paused: bool,
    pub selected_fighter1: Option<usize>,
    pub selected_fighter2: Option<usize>,
    pub selection_cursor: usize,
    // Ranged combat additions
    pub combat_mode: CombatMode,
    pub distance: Distance,
    pub ranged_phase: Option<RangedAttackPhase>,
    pub aiming_rounds: i32,
}

impl Default for CombatState {
    fn default() -> Self {
        Self {
            round: 1,
            waiting_for_defense: true,
            current_attacker: 1,
            combat_log: vec!["=== DRAFT RPG COMBAT SIMULATOR ===".to_string()],
            game_over: false,
            paused: false,
            selected_fighter1: None,
            selected_fighter2: None,
            selection_cursor: 0,
            combat_mode: CombatMode::Melee,
            distance: Distance::Close, // Start in melee range
            ranged_phase: None,
            aiming_rounds: 0,
        }
    }
}
