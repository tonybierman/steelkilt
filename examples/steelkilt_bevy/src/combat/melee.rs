use bevy::prelude::*;
use steelkilt::{combat_round, DefenseAction};

use crate::components::Fighter;
use crate::state::CombatState;

use super::helpers::{advance_turn, log_combat_result, log_death, update_fighters};

/// Executes a melee combat round with given defense action
pub fn execute_melee_round(
    att: &mut steelkilt::Character,
    def: &mut steelkilt::Character,
    defense_action: DefenseAction,
    combat_state: &mut CombatState,
    fighters: &mut Query<(Entity, &mut Fighter)>,
    is_feeble_defense: bool,
) -> bool {
    // Perform combat round
    let result = combat_round(att, def, defense_action);

    // Log specific message for feeble defense
    if is_feeble_defense {
        combat_state.combat_log.push(format!(
            "\n>>> Attack: {} rolls {} vs {}'s feeble defense {}",
            result.attacker, result.attack_roll, result.defender, result.defense_roll
        ));

        if result.hit {
            combat_state
                .combat_log
                .push(format!(">>> HIT! {} damage dealt", result.damage));
            if let Some(level) = result.wound_level {
                combat_state
                    .combat_log
                    .push(format!(">>> {} wound inflicted!", level));
            }
            if result.defender_died {
                combat_state.combat_log.push(">>> FATAL BLOW!".to_string());
            }
        } else {
            combat_state
                .combat_log
                .push(">>> MISS! The attack was fumbled.".to_string());
        }
    } else {
        log_combat_result(combat_state, &result);
    }

    // Update fighters
    update_fighters(fighters, combat_state.current_attacker, att.clone(), def.clone());

    // Check for death
    if !def.is_alive() {
        log_death(combat_state, &def.name, &att.name);
        combat_state.game_over = true;
        combat_state.waiting_for_defense = false;
        return true; // Combat ended
    }

    // Switch turns
    combat_state.waiting_for_defense = false;
    advance_turn(combat_state);
    false // Combat continues
}
