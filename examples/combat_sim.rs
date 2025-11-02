use std::io::{self, Write};
use steelkilt::*;

fn main() {
    println!("=== DRAFT RPG COMBAT SIMULATOR ===");
    println!("Based on Draft 0.4 RPG Rules\n");

    // Create two combatants
    let mut fighter1 = create_character(
        "Aldric the Bold",
        Attributes::new(8, 6, 7, 5, 6, 5, 5, 7, 4),
        7, // weapon skill
        5, // dodge skill
        Weapon::long_sword(),
        Armor::chain_mail(),
    );

    let mut fighter2 = create_character(
        "Grimwald Ironfist",
        Attributes::new(9, 5, 8, 4, 5, 6, 4, 6, 3),
        6, // weapon skill
        4, // dodge skill
        Weapon::two_handed_sword(),
        Armor::leather(),
    );

    // Display character sheets
    display_character(&fighter1);
    println!();
    display_character(&fighter2);
    println!("\n{}", "=".repeat(60));

    // Combat loop
    let mut round = 1;
    loop {
        println!("\n--- ROUND {} ---", round);

        if !fighter1.can_act() && !fighter2.can_act() {
            println!("\nBoth fighters are incapacitated!");
            break;
        }

        // Fighter 1's turn
        if fighter1.can_act() {
            println!("\n{}'s turn to attack!", fighter1.name);
            let action = get_defense_action(&fighter2.name);
            let result = combat_round(&mut fighter1, &mut fighter2, action);
            display_combat_result(&result);

            if !fighter2.is_alive() {
                println!("\n{} has been slain!", fighter2.name);
                println!("{} is victorious!", fighter1.name);
                break;
            }
        }

        // Fighter 2's turn
        if fighter2.can_act() {
            println!("\n{}'s turn to attack!", fighter2.name);
            let action = get_defense_action(&fighter1.name);
            let result = combat_round(&mut fighter2, &mut fighter1, action);
            display_combat_result(&result);

            if !fighter1.is_alive() {
                println!("\n{} has been slain!", fighter1.name);
                println!("{} is victorious!", fighter2.name);
                break;
            }
        }

        // Display status after round
        println!("\n{}", "-".repeat(60));
        display_status(&fighter1);
        display_status(&fighter2);
        println!("{}", "-".repeat(60));

        // Ask to continue
        print!("\nPress Enter to continue to next round (or 'q' to quit): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().eq_ignore_ascii_case("q") {
            println!("\nCombat ended by user.");
            break;
        }

        round += 1;
    }

    println!("\n=== FINAL STATUS ===");
    display_character(&fighter1);
    println!();
    display_character(&fighter2);
}

fn create_character(
    name: &str,
    attributes: Attributes,
    weapon_skill: i32,
    dodge_skill: i32,
    weapon: Weapon,
    armor: Armor,
) -> Character {
    Character::new(name, attributes, weapon_skill, dodge_skill, weapon, armor)
}

fn display_character(character: &Character) {
    println!("╔═══════════════════════════════════════╗");
    println!("║ {:<37} ║", character.name);
    println!("╠═══════════════════════════════════════╣");
    println!("║ PHYSICAL ATTRIBUTES:                  ║");
    println!(
        "║   STR: {:<2}  DEX: {:<2}  CON: {:<2}         ║",
        character.attributes.strength,
        character.attributes.dexterity,
        character.attributes.constitution
    );
    println!("║ MENTAL ATTRIBUTES:                    ║");
    println!(
        "║   REA: {:<2}  INT: {:<2}  WIL: {:<2}         ║",
        character.attributes.reason, character.attributes.intuition, character.attributes.willpower
    );
    println!("║ INTERACTIVE ATTRIBUTES:               ║");
    println!(
        "║   CHA: {:<2}  PER: {:<2}  EMP: {:<2}         ║",
        character.attributes.charisma,
        character.attributes.perception,
        character.attributes.empathy
    );
    println!("╠═══════════════════════════════════════╣");
    println!(
        "║ Weapon Skill: {:<2}                     ║",
        character.weapon_skill
    );
    println!(
        "║ Dodge Skill:  {:<2}                     ║",
        character.dodge_skill
    );
    println!("║ Weapon: {:<28} ║", character.weapon.name);
    println!(
        "║   Damage: {:<2}                         ║",
        character.weapon.damage
    );
    println!("║ Armor: {:<29} ║", character.armor.name);
    println!(
        "║   Protection: {:<2}                     ║",
        character.armor.protection
    );
    println!("╠═══════════════════════════════════════╣");
    println!("║ WOUNDS:                               ║");
    println!(
        "║   Light:    {:<2}                       ║",
        character.wounds.light
    );
    println!(
        "║   Severe:   {:<2}                       ║",
        character.wounds.severe
    );
    println!(
        "║   Critical: {:<2}                       ║",
        character.wounds.critical
    );
    println!(
        "║ Status: {:<30} ║",
        if !character.is_alive() {
            "DEAD"
        } else if character.wounds.is_incapacitated() {
            "INCAPACITATED"
        } else if character.wounds.severe > 0 || character.wounds.light > 0 {
            "WOUNDED"
        } else {
            "HEALTHY"
        }
    );
    println!("╚═══════════════════════════════════════╝");
}

fn display_status(character: &Character) {
    let status = if !character.is_alive() {
        "DEAD"
    } else if character.wounds.is_incapacitated() {
        "INCAPACITATED"
    } else if character.wounds.severe > 0 {
        "SEVERELY WOUNDED"
    } else if character.wounds.light > 0 {
        "LIGHTLY WOUNDED"
    } else {
        "HEALTHY"
    };

    println!(
        "{}: {} (Wounds: L:{} S:{} C:{})",
        character.name,
        status,
        character.wounds.light,
        character.wounds.severe,
        character.wounds.critical
    );
}

fn display_combat_result(result: &CombatResult) {
    println!(
        "\n>>> Attack: {} rolls {} vs {}'s defense {}",
        result.attacker, result.attack_roll, result.defender, result.defense_roll
    );

    if result.hit {
        println!(">>> HIT! {} damage dealt", result.damage);
        if let Some(level) = result.wound_level {
            println!(">>> {} wound inflicted!", level);
        }
        if result.defender_died {
            println!(">>> FATAL BLOW!");
        }
    } else {
        println!(">>> MISS! The attack was successfully defended.");
    }
}

fn get_defense_action(defender_name: &str) -> DefenseAction {
    loop {
        print!("\nHow does {} defend? [P]arry or [D]odge? ", defender_name);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "p" | "parry" => return DefenseAction::Parry,
            "d" | "dodge" => return DefenseAction::Dodge,
            _ => println!("Invalid input. Please enter 'P' for Parry or 'D' for Dodge."),
        }
    }
}
