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

/// Print round-by-round status summary
pub fn print_round_status(characters: Vec<&Character>, exhaustions: Vec<&Exhaustion>, locations: Vec<&Vec<LocationalDamage>>){
    let mut table = Table::new();
    table
        .set_header(vec!["Name", "Light", "Severe", "Critical", "Exhaustion", "Disabled"])
        .add_row(vec![
            &characters[0].name,
            &characters[0].wounds.light.to_string(),
            &characters[0].wounds.severe.to_string(),
            &characters[0].wounds.critical.to_string(),
            &format!("{} ({})", exhaustions[0].points.to_string(), exhaustions[0].status().to_string()),
            &locations[0].iter().filter(|l| l.disabled).count().to_string()
        ])
        .add_row(vec![
            &characters[1].name,
            &characters[1].wounds.light.to_string(),
            &characters[1].wounds.severe.to_string(),
            &characters[1].wounds.critical.to_string(),
            &format!("{} ({})", exhaustions[0].points.to_string(), exhaustions[0].status().to_string()),
            &locations[1].iter().filter(|l| l.disabled).count().to_string()
        ]);
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
