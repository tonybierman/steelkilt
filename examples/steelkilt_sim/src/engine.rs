use crate::state::*;
use crate::ui::*;
use crate::combat::*;
use crate::fighters::*;
use steelkilt::modules::*;
use inquire::error::InquireResult;

pub fn run_combat_rounds(args: Vec<String>) {
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

    print_section_divider("COMBAT BEGINS!");

    // Initialize combat state manager
    let mut combat = CombatState::new(knight_state, barbarian_state);

    fn select_maneuver() -> InquireResult<CombatManeuver> {
        let chosen_maneuver: CombatManeuver = CombatManeuver::select("Choose a maneuver:")
            .prompt()?;
        println!("Selected: {}", chosen_maneuver);
        Ok(chosen_maneuver)
    }

    // Main combat loop
    while combat.combat_continues() && combat.round < 10 {
        combat.next_round();

        println!("\n--- ROUND {} ---", combat.round);

        // Get the maneuver choice
        let chosen_maneuver: CombatManeuver = match select_maneuver() {
                Ok(maneuver) => maneuver,
                Err(e) => {
                    println!("Error selecting maneuver: {}", e);
                    continue;
                }
            };
        
        // Set the maneuver
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
            vec![&combat.knight.character, &combat.barbarian.character],
            vec![&combat.knight.exhaustion, &combat.barbarian.exhaustion],
            vec![&combat.knight.locations, &combat.barbarian.locations]
        );

        // print_round_status(
        //     &combat.knight.character,
        //     &combat.barbarian.character,
        //     &combat.knight.exhaustion,
        //     &combat.barbarian.exhaustion,
        //     &combat.knight.locations,
        //     &combat.barbarian.locations,
        // );
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