//! User interface module for combat simulator
//!
//! Handles all user input/output including:
//! - Generic choice prompts
//! - Status displays
//! - Combat event logging

use comfy_table::Table;
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

pub fn print_fighters(characters: Vec<&Character>, skills: Vec<&SkillSet>, exhaustions: Vec<&Exhaustion>) {
    println!("== 1. Attributes");
    let mut table = Table::new();
    table
        .set_header(vec!["Name", "STR", "DEX", "CON", "STA", "Exhaustion"])
        .add_row(vec![
            &characters[0].name,
            &characters[0].attributes.strength.to_string(),
            &characters[0].attributes.dexterity.to_string(),
            &characters[0].attributes.constitution.to_string(),
            &characters[0].attributes.stamina().to_string(),
            &format!("{} ({})", exhaustions[0].points.to_string(), exhaustions[0].status().to_string()),

        ])
        .add_row(vec![
            &characters[1].name,
            &characters[1].attributes.strength.to_string(),
            &characters[1].attributes.dexterity.to_string(),
            &characters[1].attributes.constitution.to_string(),
            &characters[1].attributes.stamina().to_string(),
            &format!("{} ({})", exhaustions[0].points.to_string(), exhaustions[0].status().to_string()),
        ]);
    println!("{table}\n");
    
    println!("== 2. Equipment");
    let mut table2 = Table::new();
    table2
        .set_header(vec!["Name", "Weapon", "Armor"])
        .add_row(vec![
            &characters[0].name,
            &format!("{} ({})", characters[0].weapon.name, characters[0].weapon.damage.to_string()),
            &format!("{} ({})", characters[0].armor.name, characters[0].armor.protection.to_string()),
        ])
        .add_row(vec![
            &characters[1].name,
            &format!("{} ({})", characters[1].weapon.name, characters[1].weapon.damage.to_string()),
            &format!("{} ({})", characters[1].armor.name, characters[1].armor.protection.to_string()),
        ]);
    println!("{table2}\n");
    
    println!("== 3. Skills");
    let mut table3 = Table::new();
    table3
        .set_header(vec!["Name", "Skills"])
        .add_row(vec![
            &characters[0].name,
            &skills[0].skills.iter().map(|(name, skill)| format!("{}: {}", name, skill.level)).collect::<Vec<_>>().join(", ")
        ])
        .add_row(vec![
            &characters[1].name,
            &skills[1].skills.iter().map(|(name, skill)| format!("{}: {}", name, skill.level)).collect::<Vec<_>>().join(", ")
        ]);
    println!("{table3}");

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
    println!("\n{}", "=".repeat(70));
    println!(" Steelkilt - Advanced Combat Simulation");
    println!(" Demonstrating skills, stances, exhaustion, and hit locations");
    println!("{}\n", "=".repeat(70));
}

/// Print section divider
pub fn print_section_divider(title: &str) {
    println!("\n{}", "=".repeat(70));
    println!(" {}", title);
    println!("{}\n", "=".repeat(70));
}
