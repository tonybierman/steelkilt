use bevy::prelude::*;
use steelkilt::{DefenseAction, WoundLevel};

use crate::components::{CombatUI, Fighter};
use crate::main_menu::spawn_main_menu_ui;
use crate::state::{
    CombatMode, CombatState, Distance, GameState, GameStateEnum, RangedAttackPhase,
};

use super::helpers::{
    advance_turn, attacker_has_ranged_weapon, both_incapacitated, current_attacker_can_act,
    get_fighters,
};
use super::melee::execute_melee_round;
use super::ranged::execute_ranged_attack;

/// Handles combat keyboard input.
pub fn handle_combat_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut combat_state: ResMut<CombatState>,
    mut fighters: Query<(Entity, &mut Fighter)>,
    mut commands: Commands,
    combat_ui: Query<Entity, With<CombatUI>>,
) {
    if !game_state.is_in(GameStateEnum::Combat) {
        return;
    }

    // Handle game over state
    if combat_state.game_over {
        if keyboard.just_pressed(KeyCode::KeyQ) || keyboard.just_pressed(KeyCode::Escape) {
            // Return to main menu
            for entity in combat_ui.iter() {
                commands.entity(entity).despawn_recursive();
            }
            for (entity, _) in fighters.iter() {
                commands.entity(entity).despawn();
            }
            *combat_state = CombatState::default();
            game_state.transition_to(GameStateEnum::MainMenu);
            spawn_main_menu_ui(&mut commands);
        }
        return;
    }

    // Handle quit
    if keyboard.just_pressed(KeyCode::KeyQ) || keyboard.just_pressed(KeyCode::Escape) {
        combat_state.game_over = true;
        combat_state
            .combat_log
            .push("\nCombat ended by user.".to_string());
        return;
    }

    // Check if current attacker can act before allowing any combat actions
    if combat_state.waiting_for_defense {
        let (can_act, name) = current_attacker_can_act(&fighters, combat_state.current_attacker);

        if !can_act {
            // Current attacker cannot act - skip their turn
            combat_state.combat_log.push(format!(
                "{} is incapacitated and cannot act!",
                name
            ));

            if both_incapacitated(&fighters) {
                combat_state
                    .combat_log
                    .push("\nBoth fighters are incapacitated!".to_string());
                combat_state.game_over = true;
                return;
            }

            // Skip to next fighter
            advance_turn(&mut combat_state);
            combat_state.waiting_for_defense = true;
            return;
        }
    }

    // Handle combat mode selection (R for ranged, M for melee)
    if combat_state.waiting_for_defense {
        if attacker_has_ranged_weapon(&fighters, combat_state.current_attacker) {
            if keyboard.just_pressed(KeyCode::KeyR) {
                let fighter_name = if combat_state.current_attacker == 1 {
                    "Fighter 1"
                } else {
                    "Fighter 2"
                };
                combat_state.combat_mode = CombatMode::Ranged;
                combat_state.ranged_phase = Some(RangedAttackPhase::Preparing);
                combat_state
                    .combat_log
                    .push(format!("{} switches to ranged combat mode", fighter_name));
                return;
            }
            if keyboard.just_pressed(KeyCode::KeyM) {
                let fighter_name = if combat_state.current_attacker == 1 {
                    "Fighter 1"
                } else {
                    "Fighter 2"
                };
                combat_state.combat_mode = CombatMode::Melee;
                combat_state.ranged_phase = None;
                combat_state.aiming_rounds = 0;
                combat_state
                    .combat_log
                    .push(format!("{} switches to melee combat mode", fighter_name));
                return;
            }
        }

        // Handle distance changes (1=Close, 2=Medium, 3=Long)
        if keyboard.just_pressed(KeyCode::Digit1) {
            combat_state.distance = Distance::Close;
            combat_state
                .combat_log
                .push("Distance: Close range".to_string());
            return;
        }
        if keyboard.just_pressed(KeyCode::Digit2) {
            combat_state.distance = Distance::Medium;
            combat_state
                .combat_log
                .push("Distance: Medium range".to_string());
            return;
        }
        if keyboard.just_pressed(KeyCode::Digit3) {
            combat_state.distance = Distance::Long;
            combat_state
                .combat_log
                .push("Distance: Long range".to_string());
            return;
        }
    }

    // Handle ranged combat sequence
    if combat_state.combat_mode == CombatMode::Ranged {
        handle_ranged_combat(&keyboard, &mut combat_state, &mut fighters);
        return;
    }

    // Handle melee defense choice
    if combat_state.waiting_for_defense && combat_state.combat_mode == CombatMode::Melee {
        handle_melee_combat(&keyboard, &mut combat_state, &mut fighters);
        return;
    }

    // Handle pause (waiting for space to continue)
    if combat_state.paused {
        handle_pause(&keyboard, &mut combat_state, &fighters);
    }
}

/// Handles ranged combat phase
fn handle_ranged_combat(
    keyboard: &Res<ButtonInput<KeyCode>>,
    combat_state: &mut CombatState,
    fighters: &mut Query<(Entity, &mut Fighter)>,
) {
    if let Some(phase) = combat_state.ranged_phase {
        match phase {
            RangedAttackPhase::Preparing => {
                if keyboard.just_pressed(KeyCode::KeyA) {
                    // Start aiming
                    combat_state.ranged_phase = Some(RangedAttackPhase::Aiming);
                    combat_state.aiming_rounds = 0;
                    combat_state.combat_log.push("Aiming...".to_string());
                    return;
                }
                if keyboard.just_pressed(KeyCode::KeyF) {
                    // Fire without aiming
                    combat_state.ranged_phase = Some(RangedAttackPhase::ReadyToFire);
                }
            }
            RangedAttackPhase::Aiming => {
                if keyboard.just_pressed(KeyCode::KeyA) {
                    // Continue aiming (max 1 round for +1 bonus)
                    if combat_state.aiming_rounds < 1 {
                        combat_state.aiming_rounds += 1;
                        let aiming_rounds = combat_state.aiming_rounds;
                        combat_state
                            .combat_log
                            .push(format!("Aiming carefully... (+{} bonus)", aiming_rounds));
                    }
                    return;
                }
                if keyboard.just_pressed(KeyCode::KeyF) {
                    // Fire after aiming
                    combat_state.ranged_phase = Some(RangedAttackPhase::ReadyToFire);
                }
            }
            RangedAttackPhase::ReadyToFire => {
                execute_ranged_phase(combat_state, fighters);
                return;
            }
            _ => {}
        }
    }
}

/// Executes the ranged attack phase
fn execute_ranged_phase(
    combat_state: &mut CombatState,
    fighters: &mut Query<(Entity, &mut Fighter)>,
) {
    let mut attacker_fighter = None;
    let mut defender_fighter = None;

    for (_, fighter) in fighters.iter() {
        if (combat_state.current_attacker == 1 && fighter.is_player_one)
            || (combat_state.current_attacker == 2 && !fighter.is_player_one)
        {
            attacker_fighter = Some(fighter.clone());
        } else {
            defender_fighter = Some(fighter.clone());
        }
    }

    if let (Some(attacker), Some(defender)) = (attacker_fighter, defender_fighter) {
        let (hit, damage, log_msg) = execute_ranged_attack(&attacker, &defender, combat_state);

        combat_state.combat_log.push(log_msg);

        if hit && damage > 0 {
            // Apply damage to defender
            for (_, mut fighter) in fighters.iter_mut() {
                if (combat_state.current_attacker == 1 && !fighter.is_player_one)
                    || (combat_state.current_attacker == 2 && fighter.is_player_one)
                {
                    // Determine wound level based on damage vs CON
                    let defender_con = fighter.character.attributes.constitution;
                    let wound_level = if damage > defender_con * 2 {
                        combat_state.combat_log.push("FATAL HIT!".to_string());
                        WoundLevel::Critical // Will result in death after stacking
                    } else if damage > defender_con {
                        combat_state.combat_log.push("Critical wound!".to_string());
                        WoundLevel::Critical
                    } else if damage > defender_con / 2 {
                        combat_state.combat_log.push("Severe wound!".to_string());
                        WoundLevel::Severe
                    } else {
                        combat_state.combat_log.push("Light wound!".to_string());
                        WoundLevel::Light
                    };

                    fighter.character.wounds.add_wound(wound_level);

                    if !fighter.character.is_alive() {
                        combat_state.combat_log.push(format!(
                            "{} has been slain!",
                            fighter.character.name
                        ));
                        combat_state.game_over = true;
                    }
                }
            }
        }

        // Reset ranged attack state and switch turns
        combat_state.ranged_phase = None;
        combat_state.aiming_rounds = 0;
        combat_state.combat_mode = CombatMode::Melee; // Return to melee for next turn

        // Switch attacker
        advance_turn(combat_state);
    }
}

/// Handles melee combat defense choice
fn handle_melee_combat(
    keyboard: &Res<ButtonInput<KeyCode>>,
    combat_state: &mut CombatState,
    fighters: &mut Query<(Entity, &mut Fighter)>,
) {
    let (attacker, defender, defender_can_act) = get_fighters(fighters, combat_state.current_attacker);

    if let (Some(mut att), Some(mut def)) = (attacker, defender) {
        // Check if defender can actively defend
        if !defender_can_act {
            // Defender is incapacitated - cannot actively defend, auto-dodge with penalty
            combat_state
                .combat_log
                .push(format!("{} is too wounded to defend properly!", def.name));

            let combat_ended = execute_melee_round(
                &mut att,
                &mut def,
                DefenseAction::Dodge,
                combat_state,
                fighters,
                true, // is_feeble_defense
            );

            if combat_ended {
                return;
            }
        } else {
            // Defender can defend - wait for defense choice
            let defense_action = if keyboard.just_pressed(KeyCode::KeyP) {
                Some(DefenseAction::Parry)
            } else if keyboard.just_pressed(KeyCode::KeyD) {
                Some(DefenseAction::Dodge)
            } else {
                None
            };

            if let Some(action) = defense_action {
                let _combat_ended = execute_melee_round(
                    &mut att,
                    &mut def,
                    action,
                    combat_state,
                    fighters,
                    false, // not feeble defense
                );
            }
        }
    }
}

/// Handles the pause state (waiting for space to continue)
fn handle_pause(
    keyboard: &Res<ButtonInput<KeyCode>>,
    combat_state: &mut CombatState,
    fighters: &Query<(Entity, &mut Fighter)>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        combat_state.paused = false;
        combat_state.waiting_for_defense = true;

        // Check if current attacker can act
        let (can_act, _) = current_attacker_can_act(fighters, combat_state.current_attacker);

        if !can_act {
            // Skip to next fighter or end combat
            if both_incapacitated(fighters) {
                combat_state
                    .combat_log
                    .push("\nBoth fighters are incapacitated!".to_string());
                combat_state.game_over = true;
            }
        }
    }
}
