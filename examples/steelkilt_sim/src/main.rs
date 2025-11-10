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
    #[arg(value_name = "VALUE", num_args = 0..=2)]
    slugs: Vec<String>,

    #[arg(long)]
    auto: bool,
}

fn get_slug(slug: &str) -> Result<String, InquireError> {
    if slug == "" {
        let options = load_available_characters();
        Select::new("Select a combatant:", options).prompt()
    } else {
        Ok(slug.to_string())
    }
}

fn main() {
    let args = Args::parse();

    match args.slugs.len() {
        0 => { 
            run_combat("aldric_the_bold", "grimwald_ironfist", args.auto);
        },
        1 => {
            match get_slug(&args.slugs[0]) {
                Ok(selection) => {
                    run_combat(&selection, "grimwald_ironfist", args.auto);
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        2 => {
            run_combat(&args.slugs[0], &args.slugs[1], args.auto);
        },
        _ => {}, // won't happen due to num_args = 0..=2
    }
}
    
fn run_combat(pc_slug: &str, ai_slug: &str, auto: bool) {
    let pc_choice = load_character_from_file(&pc_slug);
    match pc_choice {
        Ok(pc_character) => {
            println!("{} enters the arena!", pc_character.name);
            let ai_choice = load_character_from_file(&ai_slug);
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

