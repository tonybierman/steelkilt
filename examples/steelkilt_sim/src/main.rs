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
//!
//! ## Usage
//!
//! Run with interactive character selection:
//! ```sh
//! cargo run
//! ```
//!
//! Specify one or both characters by slug:
//! ```sh
//! cargo run warrior
//! cargo run warrior mage
//! ```
//!
//! Run in automatic mode (AI controls both characters):
//! ```sh
//! cargo run warrior mage --auto
//! ```

mod combat;
mod models;
mod ui;
mod engine;
mod file_ops;

use clap::Parser;
use engine::*;
use inquire::Select;
use inquire::error::InquireError;
use file_ops::*;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Character slugs (0-2 arguments). If not provided, interactive selection will be used.
    #[arg(value_name = "CHARACTER_SLUG", num_args = 0..=2)]
    slugs: Vec<String>,

    /// Enable automatic mode where AI controls both characters
    #[arg(long, help = "Run combat in automatic mode (no user input required)")]
    auto: bool,
}

/// Prompts the user to select a character from available options
fn get_slug(prompt: &str) -> Result<String, InquireError> {
    let options = load_available_characters();
    if options.is_empty() {
        return Err(InquireError::Custom(
            "No characters available. Please create character files first.".into()
        ));
    }
    Select::new(prompt, options).prompt()
}

/// Validates that a character slug exists in the available characters
fn validate_slug(slug: &str) -> Result<(), String> {
    let options = load_available_characters();
    if options.contains(&slug.to_string()) {
        Ok(())
    } else {
        Err(format!(
            "'{}' is not a valid character slug. Available characters: {}",
            slug,
            options.join(", ")
        ))
    }
}

/// Gets the second character slug, either from arguments or interactive selection
fn get_second_character(args: &Args) -> Result<String, Box<dyn Error>> {
    if args.slugs.len() >= 2 {
        validate_slug(&args.slugs[1])?;
        Ok(args.slugs[1].clone())
    } else {
        Ok(get_slug("Select Second Character")?)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Determine first character
    let first_slug = if args.slugs.is_empty() {
        get_slug("Select First Character")?
    } else {
        validate_slug(&args.slugs[0])?;
        args.slugs[0].clone()
    };

    // Determine second character
    let second_slug = get_second_character(&args)?;

    // Run the combat simulation
    run_combat(&first_slug, &second_slug, args.auto)?;

    Ok(())
}

/// Loads characters and initiates combat
fn run_combat(pc_slug: &str, ai_slug: &str, is_auto: bool) -> Result<(), Box<dyn Error>> {
    // Load first character
    let pc_character = load_character_from_file(pc_slug)
        .map_err(|e| format!("Failed to load character '{}': {}", pc_slug, e))?;
    
    println!("{} enters the arena!", pc_character.name);
    
    // Load second character
    let ai_character = load_character_from_file(ai_slug)
        .map_err(|e| format!("Failed to load character '{}': {}", ai_slug, e))?;
    
    println!("{} enters the arena!", ai_character.name);
    
    // Start combat
    run_combat_rounds(pc_character, ai_character, is_auto);
    
    Ok(())
}