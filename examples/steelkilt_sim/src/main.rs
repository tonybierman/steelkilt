//! Interactive Combat Simulator
//!
//! An advanced combat simulation demonstrating integration of multiple Draft RPG systems:
//! - Skills system for weapon proficiency
//! - Combat stances and maneuvers (Charge, Defensive, All-Out Attack, Aimed Attack)
//! - Exhaustion tracking through prolonged combat
//! - Hit location system with locational damage
//! - Progressive wound accumulation and death spiral
//!
//! The simulator allows interactive control of combat stances and provides detailed
//! feedback on combat resolution, wound effects, and character status.

mod combat;
mod fighters;
mod state;
mod ui;

use shlex::split;
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use steelkilt::modules::*;

use combat::*;
use fighters::*;
use state::*;
use ui::*;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Get {
        #[arg(value_name = "VALUE")]
        value: String,
    },
    Set {
        key: String,
        value: String,
        is_true: bool
    },
    Go,
    Usage
}

fn main() {
    loop {
        let mut buf = format!("{} ", env!("CARGO_PKG_NAME"));

        std::io::stdin().read_line(&mut buf).expect("Couldn't parse stdin");
        let line = buf.trim();
        let args = shlex::split(line).ok_or("error: Invalid quoting").unwrap();

        println!("{:?}", args);

        match Args::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    // Commands::Get(value) => get_something(value),
                    // Commands::Set{key, value, is_true} => set_something(key, value, is_true),
                    Commands::Usage => show_commands(),
                    Commands::Go => run_round(args),
                    _ => show_commands()
                }
            }
            Err(_) => println!("That's not a valid command - use the help command if you are stuck.")
         };
    }
}

fn show_commands() {
    println!(r#"COMMANDS:
get <KEY> - Gets the value of a given key and displays it. If no key given, retrieves all values and displays them.
set <KEY> <VALUE> - Sets the value of a given key.
    Flags: --is-true
"#);
}


fn run_round(args: Vec<String>) {

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

    // Display initial fighter status
    print_fighter_status(
        &knight_state.character,
        &knight_state.skills,
        &knight_state.exhaustion,
    );
    print_fighter_status(
        &barbarian_state.character,
        &barbarian_state.skills,
        &barbarian_state.exhaustion,
    );

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
