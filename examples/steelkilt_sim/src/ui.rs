//! User interface module for combat simulator
//!
//! Handles all user input/output including:
//! - Generic choice prompts
//! - Status displays
//! - Combat event logging

use comfy_table::Table;
use steelkilt::modules::*;
use steelkilt::*;

pub fn print_fighters(characters: Vec<&Character>, skills: Vec<&SkillSet>, exhaustions: Vec<&Exhaustion>) {
    // Validate that all vectors have the same length
    assert_eq!(characters.len(), skills.len(), 
               "Characters and skills must have the same length");
    assert_eq!(characters.len(), exhaustions.len(), 
               "Characters and exhaustions must have the same length");
    
    println!("== 1. Attributes");
    let mut table = Table::new();
    table.set_header(vec!["Name", "STR", "DEX", "CON", "STA", "Exhaustion"]);
    
    for i in 0..characters.len() {
        table.add_row(vec![
            &characters[i].name,
            &characters[i].attributes.strength.to_string(),
            &characters[i].attributes.dexterity.to_string(),
            &characters[i].attributes.constitution.to_string(),
            &characters[i].attributes.stamina().to_string(),
            &format!("{} ({})", exhaustions[i].points.to_string(), exhaustions[i].status().to_string()),
        ]);
    }
    println!("{table}\n");
    
    println!("== 2. Equipment");
    let mut table2 = Table::new();
    table2.set_header(vec!["Name", "Weapon", "Armor"]);
    
    for i in 0..characters.len() {
        table2.add_row(vec![
            &characters[i].name,
            &format!("{} ({})", characters[i].weapon.name, characters[i].weapon.damage.to_string()),
            &format!("{} ({})", characters[i].armor.name, characters[i].armor.protection.to_string()),
        ]);
    }
    println!("{table2}\n");
    
    println!("== 3. Skills");
    let mut table3 = Table::new();
    table3.set_header(vec!["Name", "Skills"]);
    
    for i in 0..characters.len() {
        table3.add_row(vec![
            &characters[i].name,
            &skills[i].skills.iter().map(|(name, skill)| format!("{}: {}", name, skill.level)).collect::<Vec<_>>().join(", ")
        ]);
    }
    println!("{table3}");
}

/// Print round-by-round status summary
pub fn print_round_status(characters: Vec<&Character>, exhaustions: Vec<&Exhaustion>, locations: Vec<&Vec<LocationalDamage>>){
    // Validate that all vectors have the same length
    assert_eq!(characters.len(), exhaustions.len(), 
               "Characters and exhaustions must have the same length");
    assert_eq!(characters.len(), locations.len(), 
               "Characters and locations must have the same length");
    
    let mut table = Table::new();
    table.set_header(vec!["Name", "Light", "Severe", "Critical", "Exhaustion", "Disabled"]);
    
    for i in 0..characters.len() {
        table.add_row(vec![
            &characters[i].name,
            &characters[i].wounds.light.to_string(),
            &characters[i].wounds.severe.to_string(),
            &characters[i].wounds.critical.to_string(),
            &format!("{} ({})", exhaustions[i].points.to_string(), exhaustions[i].status().to_string()),
            &locations[i].iter().filter(|l| l.disabled).count().to_string()
        ]);
    }
    
    println!("{table}\n");
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
