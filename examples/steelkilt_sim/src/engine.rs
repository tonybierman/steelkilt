use crate::models::*;
use crate::ui::*;
use crate::combat::*;
use steelkilt::Character;
use steelkilt::modules::*;
use inquire::error::InquireResult;

pub fn run_combat_rounds(character1: Character, character2: Character) {
    print_combat_header();

    // Initialize fighters with their skills and combat state
    let combatant1 = CombatantModel::new(character1);
    let combatant2 = CombatantModel::new(character2);

    // TODO: Make a menu option to show these
    // print_section_divider("COMBATANTS' DETAILS");
    // print_fighters(
    //     vec![&knight_state.character, &barbarian_state.character],
    //     vec![&knight_state.skills, &barbarian_state.skills],
    //     vec![&knight_state.exhaustion, &barbarian_state.exhaustion]
    // );

    print_section_divider("COMBAT BEGINS!");

    // Initialize combat state manager
    let mut combat = CombatModel::new(combatant1, combatant2);

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
        if let Err(e) = combat.combatant1.set_maneuver(chosen_maneuver) {
            println!("Error setting maneuver: {}", e);
            continue;
        }

        // Knight's attack
        if combat.combatant1.can_attack() {
            perform_attack(&mut combat.combatant1, &mut combat.combatant2, combat.round);

            if !combat.combatant2.is_alive() {
                println!("\n{} has been slain!", combat.combatant2.character.name);
                break;
            }
        } else {
            println!(
                "{} maintains defensive stance",
                combat.combatant1.character.name
            );
        }

        // Barbarian's attack (AI-controlled for now)
        if combat.combatant2.can_attack() {
            perform_attack(&mut combat.combatant2, &mut combat.combatant1, combat.round);

            if !combat.combatant1.is_alive() {
                println!("\n{} has been slain!", combat.combatant1.character.name);
                break;
            }
        }

        // Reset round-specific flags
        combat.end_round();

        // Display round status
        println!("\n--- END OF ROUND {} STATUS SUMMARY---", combat.round);
        print_round_status(vec![&combat.combatant1, &combat.combatant2]);
    }

    // Final summary
    print_section_divider("COMBAT CONCLUDED");

    print_final_status(
        &combat.combatant1.character,
        &combat.combatant1.exhaustion,
        &combat.combatant1.locations,
    );

    print_final_status(
        &combat.combatant2.character,
        &combat.combatant2.exhaustion,
        &combat.combatant2.locations,
    );
}