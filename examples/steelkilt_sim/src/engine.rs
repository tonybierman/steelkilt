//! Combat engine module
//!
//! Orchestrates the main combat loop, managing:
//! - Round progression and turn order
//! - Player input and AI decision-making
//! - Combat state updates and validation
//! - Victory condition checking
//! - Combat logging and status reporting

use crate::combat::*;
use crate::models::*;
use crate::ui::*;
use inquire::error::InquireResult;
use steelkilt::modules::*;
use steelkilt::Character;

// ============================================================================
// Constants
// ============================================================================

/// Maximum number of combat rounds before forcing a draw
const MAX_COMBAT_ROUNDS: usize = 10;

// ============================================================================
// Public API
// ============================================================================

/// Execute a complete combat encounter between two characters.
///
/// Manages the full combat lifecycle from initialization through resolution,
/// handling both player-controlled and AI combatants.
///
/// # Arguments
/// * `character1` - The first combatant (typically player-controlled)
/// * `character2` - The second combatant (typically AI-controlled)
/// * `is_auto` - If true, both combatants use AI; if false, character1 is player-controlled
///
/// # Combat Flow
/// 1. Initialize combat state and display header
/// 2. Loop through rounds until victory or max rounds
/// 3. Process player/AI input for maneuver selection
/// 4. Execute attacks in turn order
/// 5. Check victory conditions
/// 6. Display round summary
/// 7. Display final combat results
pub fn run_combat_rounds(character1: Character, character2: Character, is_auto: bool) {
    let mut engine = CombatEngine::new(character1, character2, is_auto);
    engine.run();
}

// ============================================================================
// Combat Engine
// ============================================================================

/// Main combat engine that orchestrates the combat encounter
struct CombatEngine {
    combat: Melee,
    is_auto: bool,
}

impl CombatEngine {
    /// Create a new combat engine with initialized combatants
    fn new(character1: Character, character2: Character, is_auto: bool) -> Self {
        print_combat_header();

        let combatant1 = Combatant::new(character1);
        let combatant2 = Combatant::new(character2);

        print_section_divider("COMBAT BEGINS!");

        Self {
            combat: Melee::new(combatant1, combatant2),
            is_auto,
        }
    }

    /// Execute the main combat loop
    fn run(&mut self) {
        while self.should_continue_combat() {
            self.execute_round();

            if self.check_combat_ended() {
                break;
            }
        }

        self.display_final_results();
    }

    /// Check if combat should continue
    fn should_continue_combat(&self) -> bool {
        self.combat.combat_continues() && self.combat.round < MAX_COMBAT_ROUNDS
    }

    /// Execute a single combat round
    fn execute_round(&mut self) {
        self.combat.next_round();
        self.display_round_header();

        // Process combatant 1's turn
        if let Some(victor) = self.process_combatant_turn(CombatantId::First) {
            self.announce_victory(victor);
            return;
        }

        // Process combatant 2's turn
        if let Some(victor) = self.process_combatant_turn(CombatantId::Second) {
            self.announce_victory(victor);
            return;
        }

        self.combat.end_round();
        self.display_round_summary();
    }

    /// Process a single combatant's turn, returning Some(victor) if combat ends
    fn process_combatant_turn(&mut self, combatant_id: CombatantId) -> Option<CombatantId> {
        let (attacker_id, defender_id) = match combatant_id {
            CombatantId::First => (CombatantId::First, CombatantId::Second),
            CombatantId::Second => (CombatantId::Second, CombatantId::First),
        };

        // Select maneuver for the attacking combatant
        if attacker_id == CombatantId::First && !self.is_auto {
            if let Err(e) = self.handle_player_maneuver_selection() {
                println!("Error during maneuver selection: {}", e);
                return None;
            }
        }

        // Execute attack if able
        let attacker_name = self.get_combatant_name(attacker_id);
        
        if self.get_combatant_mut(attacker_id).can_attack() {
            self.execute_attack(attacker_id, defender_id);

            if !self.get_combatant(defender_id).is_alive() {
                return Some(attacker_id);
            }
        } else {
            println!("{} maintains defensive stance", attacker_name);
        }

        None
    }

    /// Handle player input for maneuver selection
    fn handle_player_maneuver_selection(&mut self) -> Result<(), String> {
        let maneuver = prompt_maneuver_selection()
            .map_err(|e| format!("Failed to get player input: {}", e))?;

        self.combat
            .combatant1
            .set_maneuver(maneuver)
            .map_err(|e| format!("Invalid maneuver: {}", e))?;

        Ok(())
    }

    /// Execute an attack between two combatants
    fn execute_attack(&mut self, attacker_id: CombatantId, defender_id: CombatantId) {
        let round = self.combat.round;

        // We need to borrow both combatants mutably, which requires careful handling
        match (attacker_id, defender_id) {
            (CombatantId::First, CombatantId::Second) => {
                perform_attack(
                    &mut self.combat.combatant1,
                    &mut self.combat.combatant2,
                    round,
                );
            }
            (CombatantId::Second, CombatantId::First) => {
                perform_attack(
                    &mut self.combat.combatant2,
                    &mut self.combat.combatant1,
                    round,
                );
            }
            _ => unreachable!("Invalid combatant pairing"),
        }
    }

    /// Get immutable reference to a combatant
    fn get_combatant(&self, id: CombatantId) -> &Combatant {
        match id {
            CombatantId::First => &self.combat.combatant1,
            CombatantId::Second => &self.combat.combatant2,
        }
    }

    /// Get mutable reference to a combatant
    fn get_combatant_mut(&mut self, id: CombatantId) -> &mut Combatant {
        match id {
            CombatantId::First => &mut self.combat.combatant1,
            CombatantId::Second => &mut self.combat.combatant2,
        }
    }

    /// Get the name of a combatant
    fn get_combatant_name(&self, id: CombatantId) -> String {
        self.get_combatant(id).character.name.clone()
    }

    /// Check if combat has ended (one combatant dead)
    fn check_combat_ended(&self) -> bool {
        !self.combat.combatant1.is_alive() || !self.combat.combatant2.is_alive()
    }

    /// Announce the victor of the combat
    fn announce_victory(&self, victor_id: CombatantId) {
        let loser_id = victor_id.opponent();
        let loser_name = self.get_combatant_name(loser_id);
        println!("\n{} has been slain!", loser_name);
    }

    // ========================================================================
    // Display Methods
    // ========================================================================

    fn display_round_header(&self) {
        println!("\n--- BEGIN ROUND {} ---", self.combat.round);
    }

    fn display_round_summary(&self) {
        println!("\n--- END OF ROUND {} ---", self.combat.round);
        print_round_status(vec![&self.combat.combatant1, &self.combat.combatant2]);
    }

    fn display_final_results(&self) {
        print_section_divider("END OF COMBAT");
        print_final_status(&self.combat.combatant1);
        println!();
        print_final_status(&self.combat.combatant2);
    }
}

// ============================================================================
// Helper Types
// ============================================================================

/// Identifies which combatant in a two-combatant fight
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CombatantId {
    First,
    Second,
}

impl CombatantId {
    /// Get the opposing combatant
    fn opponent(self) -> Self {
        match self {
            CombatantId::First => CombatantId::Second,
            CombatantId::Second => CombatantId::First,
        }
    }
}

// ============================================================================
// Input Handling
// ============================================================================

/// Prompt the player to select a combat maneuver
fn prompt_maneuver_selection() -> InquireResult<CombatManeuver> {
    let maneuver = CombatManeuver::select("Choose a maneuver:").prompt()?;
    println!("Selected: {}", maneuver);
    Ok(maneuver)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combatant_id_opponent() {
        assert_eq!(CombatantId::First.opponent(), CombatantId::Second);
        assert_eq!(CombatantId::Second.opponent(), CombatantId::First);
    }

    #[test]
    fn test_combatant_id_double_opponent() {
        // Applying opponent twice should return to original
        assert_eq!(
            CombatantId::First.opponent().opponent(),
            CombatantId::First
        );
        assert_eq!(
            CombatantId::Second.opponent().opponent(),
            CombatantId::Second
        );
    }

    #[test]
    fn test_max_rounds_constant() {
        // Ensure max rounds is reasonable
        assert!(MAX_COMBAT_ROUNDS > 0);
        assert!(MAX_COMBAT_ROUNDS <= 100);
    }
}