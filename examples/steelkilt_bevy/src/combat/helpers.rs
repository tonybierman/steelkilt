use bevy::prelude::*;
use steelkilt::Character;

use crate::components::Fighter;
use crate::state::CombatState;

/// Gets the attacking and defending fighters based on current_attacker
pub fn get_fighters(
    fighters: &Query<(Entity, &mut Fighter)>,
    current_attacker: u8,
) -> (Option<Character>, Option<Character>, bool) {
    let mut attacker = None;
    let mut defender = None;
    let mut defender_can_act = true;

    for (_, fighter) in fighters.iter() {
        if (current_attacker == 1 && fighter.is_player_one)
            || (current_attacker == 2 && !fighter.is_player_one)
        {
            attacker = Some(fighter.character.clone());
        } else {
            defender = Some(fighter.character.clone());
            defender_can_act = fighter.character.can_act();
        }
    }

    (attacker, defender, defender_can_act)
}

/// Updates fighters after combat with new character states
pub fn update_fighters(
    fighters: &mut Query<(Entity, &mut Fighter)>,
    current_attacker: u8,
    att: Character,
    def: Character,
) {
    for (_, mut fighter) in fighters.iter_mut() {
        if (current_attacker == 1 && fighter.is_player_one)
            || (current_attacker == 2 && !fighter.is_player_one)
        {
            fighter.character = att.clone();
        } else {
            fighter.character = def.clone();
        }
    }
}

/// Checks if current attacker can act
pub fn current_attacker_can_act(
    fighters: &Query<(Entity, &mut Fighter)>,
    current_attacker: u8,
) -> (bool, String) {
    for (_, fighter) in fighters.iter() {
        if (current_attacker == 1 && fighter.is_player_one)
            || (current_attacker == 2 && !fighter.is_player_one)
        {
            return (fighter.character.can_act(), fighter.character.name.clone());
        }
    }
    // Default to true if fighter not found (matches original behavior)
    (true, String::new())
}

/// Checks if both fighters are incapacitated
pub fn both_incapacitated(fighters: &Query<(Entity, &mut Fighter)>) -> bool {
    let mut count = 0;
    let mut incapacitated_count = 0;

    for (_, fighter) in fighters.iter() {
        count += 1;
        if !fighter.character.can_act() {
            incapacitated_count += 1;
        }
    }

    // Both are incapacitated only if we have at least 2 fighters and all are incapacitated
    count >= 2 && incapacitated_count == count
}

/// Checks if current attacker has a ranged weapon
pub fn attacker_has_ranged_weapon(
    fighters: &Query<(Entity, &mut Fighter)>,
    current_attacker: u8,
) -> bool {
    fighters
        .iter()
        .find(|(_, f)| {
            (current_attacker == 1 && f.is_player_one)
                || (current_attacker == 2 && !f.is_player_one)
        })
        .map(|(_, f)| f.character.ranged_weapon.is_some())
        .unwrap_or(false)
}

/// Advances to the next turn, switching attacker and incrementing round
pub fn advance_turn(combat_state: &mut CombatState) {
    if combat_state.current_attacker == 1 {
        combat_state.current_attacker = 2;
        combat_state.waiting_for_defense = true;
    } else {
        combat_state.current_attacker = 1;
        combat_state.round += 1;
        let round = combat_state.round;
        combat_state.paused = true;
        combat_state
            .combat_log
            .push(format!("\n--- ROUND {} ---", round));
    }
}

/// Logs combat result to combat log
pub fn log_combat_result(combat_state: &mut CombatState, result: &steelkilt::CombatResult) {
    combat_state.combat_log.push(format!(
        "\n>>> Attack: {} rolls {} vs {}'s defense {}",
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
            .push(">>> MISS! The attack was successfully defended.".to_string());
    }
}

/// Logs death message
pub fn log_death(combat_state: &mut CombatState, defender_name: &str, attacker_name: &str) {
    combat_state
        .combat_log
        .push(format!("\n{} has been slain!", defender_name));
    combat_state
        .combat_log
        .push(format!("{} is victorious!", attacker_name));
}
