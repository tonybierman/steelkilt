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
mod models;
mod ui;
mod engine;
mod file_ops;

use clap::{Parser, Subcommand};
use engine::*;

use inquire::Select;
use inquire::error::*;
use file_ops::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_name = "VALUE", default_value = "default-slug")]
    value: String,

    #[arg(long)]
    auto: bool,
}

fn get_something(slug: &str) -> Result<String, InquireError> {
    let options = load_available_characters();
    Select::new("Select a combatant:", options).prompt()
}

fn set_something(key: String, value: String, flag: bool) {

}

fn main() {
    let args = Args::parse();

    match get_something(&args.value) {
        Ok(selection) => {
            run_combat(selection, args.auto);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
    
fn run_combat(pc_slug: String, auto: bool) {
    let pc_choice = load_character_from_file(&pc_slug);
    match pc_choice {
        Ok(pc_character) => {
            println!("{} enters the arena!", pc_character.name);
            let ai_choice = load_character_from_file("grimwald_ironfist");
            match ai_choice {
                Ok(ai_character) => {
                    println!("{} enters the arena!", ai_character.name);
                    run_combat_rounds(pc_character, ai_character);
                }
                Err(_) => println!("There was an error, please try again"),
            }
        }
        Err(_) => println!("There was an error, please try again"),
    }
}

