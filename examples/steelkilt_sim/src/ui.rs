//! User interface module for combat simulator
//!
//! Handles all user input/output including:
//! - Generic choice prompts
//! - Status displays
//! - Combat event logging

use std::collections::HashMap;
use std::io::{self, Write};
use steelkilt::modules::*;
use steelkilt::*;

/// Generic console prompt that returns a user-selected value from a dictionary of options
pub fn get_user_choice<T: Clone + PartialEq + std::fmt::Display>(
    prompt_prefix: &str,
    options: &HashMap<char, T>,
    default_value: T,
) -> T {
    let prompt_suffix = generate_prompt_suffix(options, &default_value);
    let full_prompt = format!("{} {}: ", prompt_prefix, prompt_suffix);

    loop {
        print!("{}", full_prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim().to_uppercase();

                if input.is_empty() {
                    return default_value.clone();
                }

                if let Some(first_char) = input.chars().next() {
                    if let Some(value) = options.get(&first_char) {
                        return value.clone();
                    }
                }

                println!("Invalid option. Please try again.");
            }
            Err(_) => {
                println!("Error reading input. Please try again.");
            }
        }
    }
}

/// Generates the prompt suffix showing available options and default value
fn generate_prompt_suffix<T: PartialEq + std::fmt::Display>(
    options: &HashMap<char, T>,
    default_value: &T,
) -> String {
    let mut parts: Vec<String> = options
        .iter()
        .map(|(&key, value)| {
            let value_str = value.to_string();
            format!("[{}]{}", key, value_str)
        })
        .collect();

    parts.sort();

    let default_value_str = default_value.to_string();
    format!(
        "Select {} (default: {})",
        parts.join(", "),
        default_value_str
    )
}

/// Print initial fighter status with attributes, equipment, and skills
pub fn print_fighter_status(character: &Character, skills: &SkillSet, exhaustion: &Exhaustion) {
    println!("\n{}", "=".repeat(70));
    println!("{}", character.name);
    println!("{}", "=".repeat(70));
    println!(
        "Attributes: STR:{} DEX:{} CON:{} STA:{}",
        character.attributes.strength,
        character.attributes.dexterity,
        character.attributes.constitution,
        character.attributes.stamina(),
    );
    println!(
        "Weapon: {} (dmg: {:+})",
        character.weapon.name, character.weapon.damage
    );
    println!(
        "Armor: {} (protection: {})",
        character.armor.name, character.armor.protection
    );

    println!("\nSkills:");
    for (name, skill) in &skills.skills {
        println!("  {}: {}", name, skill.level);
    }

    println!(
        "\nExhaustion: {} points ({})",
        exhaustion.points,
        exhaustion.status()
    );
}

/// Print round-by-round status summary
pub fn print_round_status(
    knight: &Character,
    barbarian: &Character,
    knight_exhaustion: &Exhaustion,
    barbarian_exhaustion: &Exhaustion,
    knight_locations: &[LocationalDamage],
    barbarian_locations: &[LocationalDamage],
) {
    println!("\n{}", "-".repeat(70));
    println!("Status:");

    println!(
        "  {}: Wounds(L:{} S:{} C:{}) Exhaustion({} {}) Disabled:{}",
        knight.name,
        knight.wounds.light,
        knight.wounds.severe,
        knight.wounds.critical,
        knight_exhaustion.points,
        knight_exhaustion.status(),
        knight_locations.iter().filter(|l| l.disabled).count(),
    );

    println!(
        "  {}: Wounds(L:{} S:{} C:{}) Exhaustion({} {}) Disabled:{}",
        barbarian.name,
        barbarian.wounds.light,
        barbarian.wounds.severe,
        barbarian.wounds.critical,
        barbarian_exhaustion.points,
        barbarian_exhaustion.status(),
        barbarian_locations.iter().filter(|l| l.disabled).count(),
    );
}

/// Print final post-combat status with detailed wound and location information
pub fn print_final_status(
    character: &Character,
    exhaustion: &Exhaustion,
    locations: &[LocationalDamage],
) {
    println!(
        "{}: {}",
        character.name,
        if character.is_alive() {
            "ALIVE"
        } else {
            "DEAD"
        }
    );
    println!(
        "  Total Wounds: {} Light, {} Severe, {} Critical",
        character.wounds.light, character.wounds.severe, character.wounds.critical,
    );
    println!(
        "  Exhaustion: {} points ({})",
        exhaustion.points,
        exhaustion.status()
    );

    if !locations.is_empty() {
        println!("  Injured Locations:");
        for loc in locations {
            println!(
                "    {}: L:{} S:{} C:{}, penalty: {}, disabled: {}",
                loc.location,
                loc.light_wounds,
                loc.severe_wounds,
                loc.critical_wounds,
                loc.penalty(),
                loc.disabled,
            );
        }
    }
}

/// Print header for combat simulation
pub fn print_combat_header() {
    println!("=== Advanced Combat Simulation ===");
    println!("Demonstrating: Skills, Stances, Exhaustion, Hit Locations\n");
}

/// Print section divider
pub fn print_section_divider(title: &str) {
    println!("\n{}", "=".repeat(70));
    println!("{}", title);
    println!("{}\n", "=".repeat(70));
}
