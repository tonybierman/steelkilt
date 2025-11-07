use crate::state::*;
use crate::ui::*;
use crate::combat::*;
use crate::fighters::*;
use std::collections::HashMap;
use steelkilt::modules::*;

pub fn run_combat_rounds(args: Vec<String>) {

    // Setup combat maneuver options for user input
    let mut maneuver_options = HashMap::new();
    maneuver_options.insert('N', CombatManeuver::Normal);
    maneuver_options.insert('C', CombatManeuver::Charge);
    maneuver_options.insert('A', CombatManeuver::AllOutAttack);
    maneuver_options.insert('D', CombatManeuver::DefensivePosition);

    print_combat_header();

    // Initialize fighters with their skills and combat state
    let knight_state = FighterState::new(create_knight(), create_knight_skills());
    let barbarian_state = FighterState::new(create_barbarian(), create_barbarian_skills());

    print_section_divider("COMBATANTS' DETAILS");

    print_fighters(
        vec![&knight_state.character, &barbarian_state.character],
        vec![&knight_state.skills, &barbarian_state.skills],
        vec![&knight_state.exhaustion, &barbarian_state.exhaustion]
    );

    // Display initial fighter status
    // print_fighter_status(
    //     &knight_state.character,
    //     &knight_state.skills,
    //     &knight_state.exhaustion,
    // );
    // print_fighter_status(
    //     &barbarian_state.character,
    //     &barbarian_state.skills,
    //     &barbarian_state.exhaustion,
    // );

    print_section_divider("COMBAT BEGINS!");

    // Initialize combat state manager
    let mut combat = CombatState::new(knight_state, barbarian_state);

    // Main combat loop
    while combat.combat_continues() && combat.round < 10 {
        combat.next_round();

        println!("\n--- ROUND {} ---", combat.round);

        // Get user input for knight's combat stance
        let chosen_maneuver = get_user_choice(
            "Knight's stance?",
            &maneuver_options,
            CombatManeuver::Normal,
        );

        if let Err(e) = combat.knight.set_maneuver(chosen_maneuver) {
            println!("Error setting maneuver: {}", e);
            continue;
        }

        // Knight's attack
        if combat.knight.can_attack() {
            perform_attack(&mut combat.knight, &mut combat.barbarian, combat.round);

            if !combat.barbarian.is_alive() {
                println!("\n{} has been slain!", combat.barbarian.character.name);
                break;
            }
        } else {
            println!(
                "{} maintains defensive stance",
                combat.knight.character.name
            );
        }

        // Barbarian's attack (AI-controlled for now)
        if combat.barbarian.can_attack() {
            perform_attack(&mut combat.barbarian, &mut combat.knight, combat.round);

            if !combat.knight.is_alive() {
                println!("\n{} has been slain!", combat.knight.character.name);
                break;
            }
        }

        // Reset round-specific flags
        combat.end_round();

        // Display round status
        print_round_status(
            &combat.knight.character,
            &combat.barbarian.character,
            &combat.knight.exhaustion,
            &combat.barbarian.exhaustion,
            &combat.knight.locations,
            &combat.barbarian.locations,
        );
    }

    // Final summary
    print_section_divider("COMBAT CONCLUDED");

    print_final_status(
        &combat.knight.character,
        &combat.knight.exhaustion,
        &combat.knight.locations,
    );

    print_final_status(
        &combat.barbarian.character,
        &combat.barbarian.exhaustion,
        &combat.barbarian.locations,
    );
}