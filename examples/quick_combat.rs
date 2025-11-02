use steelkilt::*;

fn main() {
    println!("=== Quick Combat Example ===\n");

    // Create two fighters
    let mut knight = Character::new(
        "Sir Roland",
        Attributes::new(8, 6, 7, 5, 6, 5, 6, 7, 4),
        7, // weapon skill
        5, // dodge skill
        Weapon::long_sword(),
        Armor::plate(),
    );

    let mut barbarian = Character::new(
        "Thorgar",
        Attributes::new(9, 7, 9, 4, 5, 6, 4, 6, 3),
        8, // weapon skill
        6, // dodge skill
        Weapon::two_handed_sword(),
        Armor::leather(),
    );

    println!(
        "{} (STR:{}, CON:{}, Weapon Skill:{}, Armor:{})",
        knight.name,
        knight.attributes.strength,
        knight.attributes.constitution,
        knight.weapon_skill,
        knight.armor.name
    );

    println!(
        "{} (STR:{}, CON:{}, Weapon Skill:{}, Armor:{})\n",
        barbarian.name,
        barbarian.attributes.strength,
        barbarian.attributes.constitution,
        barbarian.weapon_skill,
        barbarian.armor.name
    );

    // Simulate a few rounds
    for round in 1..=5 {
        println!("--- Round {} ---", round);

        if !knight.can_act() || !barbarian.can_act() {
            break;
        }

        // Knight attacks
        let result1 = combat_round(&mut knight, &mut barbarian, DefenseAction::Parry);
        println!(
            "{} attacks: roll {} vs {} (defense {})",
            result1.attacker, result1.attack_roll, result1.defender, result1.defense_roll
        );

        if result1.hit {
            println!(
                "  → HIT! {} damage, {} wound",
                result1.damage,
                result1
                    .wound_level
                    .map(|w| format!("{}", w))
                    .unwrap_or("no".to_string())
            );
        } else {
            println!("  → MISS");
        }

        if !barbarian.is_alive() {
            println!("\n{} has been slain!", barbarian.name);
            break;
        }

        // Barbarian counter-attacks
        let result2 = combat_round(&mut barbarian, &mut knight, DefenseAction::Parry);
        println!(
            "{} attacks: roll {} vs {} (defense {})",
            result2.attacker, result2.attack_roll, result2.defender, result2.defense_roll
        );

        if result2.hit {
            println!(
                "  → HIT! {} damage, {} wound",
                result2.damage,
                result2
                    .wound_level
                    .map(|w| format!("{}", w))
                    .unwrap_or("no".to_string())
            );
        } else {
            println!("  → MISS");
        }

        if !knight.is_alive() {
            println!("\n{} has been slain!", knight.name);
            break;
        }

        // Show wounds
        println!(
            "\nWounds: {} (L:{} S:{} C:{}) | {} (L:{} S:{} C:{})\n",
            knight.name,
            knight.wounds.light,
            knight.wounds.severe,
            knight.wounds.critical,
            barbarian.name,
            barbarian.wounds.light,
            barbarian.wounds.severe,
            barbarian.wounds.critical
        );
    }

    println!("\n=== Final Status ===");
    println!(
        "{}: {} (Wounds: L:{} S:{} C:{})",
        knight.name,
        if knight.is_alive() { "Alive" } else { "Dead" },
        knight.wounds.light,
        knight.wounds.severe,
        knight.wounds.critical
    );
    println!(
        "{}: {} (Wounds: L:{} S:{} C:{})",
        barbarian.name,
        if barbarian.is_alive() {
            "Alive"
        } else {
            "Dead"
        },
        barbarian.wounds.light,
        barbarian.wounds.severe,
        barbarian.wounds.critical
    );
}
