//! Advanced Combat Example
//!
//! Demonstrates integration of multiple advanced Draft RPG features:
//! - Skills system for weapon proficiency
//! - Combat stances and maneuvers (Charge, Defensive, All-Out Attack, Aimed Attack)
//! - Exhaustion tracking through prolonged combat
//! - Hit location system with locational damage
//! - Progressive wound accumulation and death spiral

use steelkilt::*;
use steelkilt::modules::*;

fn main() {
    println!("=== Advanced Combat Simulation ===");
    println!("Demonstrating: Skills, Stances, Exhaustion, Hit Locations\n");

    // Create two advanced fighters with full skill sets
    let mut knight = create_knight();
    let mut barbarian = create_barbarian();

    // Initialize advanced combat state
    let knight_skills = create_knight_skills();
    let barbarian_skills = create_barbarian_skills();

    let mut knight_stance = CombatStance::new();
    let mut barbarian_stance = CombatStance::new();

    let mut knight_exhaustion = Exhaustion::new(knight.attributes.stamina());
    let mut barbarian_exhaustion = Exhaustion::new(barbarian.attributes.stamina());

    let mut knight_locations: Vec<LocationalDamage> = Vec::new();
    let mut barbarian_locations: Vec<LocationalDamage> = Vec::new();

    print_fighter_status(&knight, &knight_skills, &knight_exhaustion);
    print_fighter_status(&barbarian, &barbarian_skills, &barbarian_exhaustion);

    println!("\n{}", "=".repeat(70));
    println!("COMBAT BEGINS!");
    println!("{}\n", "=".repeat(70));

    // Combat sequence demonstrating different tactics
    for round in 1..=10 {
        println!("\n--- ROUND {} ---", round);

        if !knight.can_act() || !barbarian.can_act() {
            break;
        }

        // Add exhaustion each round
        knight_exhaustion.add_points(1);
        barbarian_exhaustion.add_points(1);

        // Knights tactical choice based on round
        match round {
            1 => {
                // Knight starts defensively
                knight_stance.set_maneuver(CombatManeuver::DefensivePosition).unwrap();
                println!("{} takes Defensive Position (+2 defense, cannot attack)", knight.name);
            },
            2 => {
                // Barbarian charges
                barbarian_stance.charged_this_round = true;
                barbarian_stance.set_maneuver(CombatManeuver::Charge).unwrap();
                println!("{} CHARGES! (+1 attack, +1 damage, -2 defense)", barbarian.name);
            },
            3 => {
                // Knight recovers, attacks normally
                knight_stance.set_maneuver(CombatManeuver::Normal).unwrap();
            },
            4 => {
                // Knight starts aiming for precise strike
                knight_stance.start_aiming();
                println!("{} begins aiming for a precise strike...", knight.name);
            },
            5 => {
                // Knight executes aimed attack
                knight_stance.set_maneuver(CombatManeuver::AimedAttack).unwrap();
                println!("{} executes Aimed Attack! (-2 attack, +2 damage)", knight.name);
            },
            6..=8 => {
                // Barbarian goes all-out
                if round == 6 {
                    barbarian_stance.set_maneuver(CombatManeuver::AllOutAttack).unwrap();
                    println!("{} goes All-Out Attack! (+2 attack, -4 defense)", barbarian.name);
                }
            },
            _ => {
                // Reset to normal combat
                knight_stance.set_maneuver(CombatManeuver::Normal).unwrap();
                barbarian_stance.set_maneuver(CombatManeuver::Normal).unwrap();
            }
        }

        // Knight's turn (if can attack with current stance)
        if knight_stance.current_maneuver.can_attack() {
            perform_advanced_attack(
                &mut knight,
                &mut barbarian,
                &knight_skills,
                &knight_stance,
                &knight_exhaustion,
                &mut barbarian_locations,
                round,
            );

            if !barbarian.is_alive() {
                println!("\n{} has been slain!", barbarian.name);
                break;
            }
        } else {
            println!("{} maintains defensive stance", knight.name);
        }

        // Barbarian's turn
        if barbarian_stance.current_maneuver.can_attack() {
            perform_advanced_attack(
                &mut barbarian,
                &mut knight,
                &barbarian_skills,
                &barbarian_stance,
                &barbarian_exhaustion,
                &mut knight_locations,
                round,
            );

            if !knight.is_alive() {
                println!("\n{} has been slain!", knight.name);
                break;
            }
        }

        // Reset round-specific flags
        knight_stance.end_round();
        barbarian_stance.end_round();
        // Note: aiming is automatically reset when AimedAttack is executed

        // Show status at end of round
        print_round_status(
            &knight,
            &barbarian,
            &knight_exhaustion,
            &barbarian_exhaustion,
            &knight_locations,
            &barbarian_locations,
        );
    }

    // Final summary
    println!("\n{}", "=".repeat(70));
    println!("COMBAT CONCLUDED");
    println!("{}\n", "=".repeat(70));

    print_final_status(&knight, &knight_exhaustion, &knight_locations);
    print_final_status(&barbarian, &barbarian_exhaustion, &barbarian_locations);
}

fn create_knight() -> Character {
    Character::new(
        "Sir Roland the Defender",
        Attributes::new(8, 6, 7, 5, 6, 5, 6, 7, 4), // STR, DEX, CON, REA, INT, WIL, CHA, PER, EMP
        7, // weapon skill
        5, // dodge skill
        Weapon::long_sword(),
        Armor::plate(),
    )
}

fn create_barbarian() -> Character {
    Character::new(
        "Thorgar the Fierce",
        Attributes::new(9, 7, 9, 4, 5, 6, 4, 6, 3),
        8, // weapon skill
        6, // dodge skill
        Weapon::two_handed_sword(),
        Armor::leather(),
    )
}

fn create_knight_skills() -> SkillSet {
    let mut skills = SkillSet::new(0); // No need to spend points, pre-set

    let longsword = Skill::new("Longsword", 6, SkillDifficulty::Normal);
    skills.skills.insert("Longsword".to_string(), longsword);

    let dodge = Skill::new("Dodge", 6, SkillDifficulty::Normal);
    skills.skills.insert("Dodge".to_string(), dodge);

    skills
}

fn create_barbarian_skills() -> SkillSet {
    let mut skills = SkillSet::new(0);

    let two_handed = Skill::new("Two-Handed Weapons", 7, SkillDifficulty::Normal);
    skills.skills.insert("Two-Handed Weapons".to_string(), two_handed);

    let dodge = Skill::new("Dodge", 7, SkillDifficulty::Normal);
    skills.skills.insert("Dodge".to_string(), dodge);

    skills
}

fn perform_advanced_attack(
    attacker: &mut Character,
    defender: &mut Character,
    _attacker_skills: &SkillSet,
    attacker_stance: &CombatStance,
    attacker_exhaustion: &Exhaustion,
    defender_locations: &mut Vec<LocationalDamage>,
    round: usize,
) {
    // Calculate total modifiers
    let stance_attack_mod = attacker_stance.total_attack_modifier();
    let exhaustion_penalty = attacker_exhaustion.penalty();
    let total_attack_mod = stance_attack_mod + exhaustion_penalty;

    // Determine hit location
    let direction = if round % 3 == 0 {
        AttackDirection::Above
    } else if round % 2 == 0 {
        AttackDirection::Left
    } else {
        AttackDirection::Front
    };

    let hit_location = HitLocation::determine(direction);

    // Simulate attack roll (in real use, this would use combat_round with modifiers)
    println!(
        "\n{} attacks {} (Stance: {}, Exhaustion: {}, Total Mod: {:+})",
        attacker.name,
        defender.name,
        attacker_stance.current_maneuver,
        attacker_exhaustion.status(),
        total_attack_mod
    );

    // For demonstration, use basic combat_round
    let result = combat_round(attacker, defender, DefenseAction::Parry);

    if result.hit {
        // Apply stance damage modifier
        let stance_damage_mod = attacker_stance.total_damage_modifier();
        let location_damage_mult = hit_location.damage_multiplier();
        let adjusted_damage = (result.damage as f32 * location_damage_mult) as i32 + stance_damage_mod;

        println!(
            "  → HIT to {}! Base damage: {}, Location mult: {:.2}x, Stance bonus: {:+}, Final: {}",
            hit_location,
            result.damage,
            location_damage_mult,
            stance_damage_mod,
            adjusted_damage
        );

        // Track locational damage
        if let Some(wound) = result.wound_level {
            let severity = match wound {
                WoundLevel::Light => hit_location::WoundSeverity::Light,
                WoundLevel::Severe => hit_location::WoundSeverity::Severe,
                WoundLevel::Critical => hit_location::WoundSeverity::Critical,
            };

            // Find or create location tracker
            let mut found = false;
            for loc_dmg in defender_locations.iter_mut() {
                if loc_dmg.location == hit_location {
                    loc_dmg.add_wound(severity);
                    found = true;

                    if loc_dmg.disabled {
                        println!("  → {} is DISABLED!", hit_location);

                        if hit_location.causes_weapon_drop() {
                            println!("  → {} drops their weapon!", defender.name);
                        }
                    }
                    break;
                }
            }

            if !found {
                let mut new_loc = LocationalDamage::new(hit_location);
                new_loc.add_wound(severity);

                if new_loc.disabled && hit_location.causes_weapon_drop() {
                    println!("  → {} drops their weapon!", defender.name);
                }

                defender_locations.push(new_loc);
            }

            println!("  → {} wound inflicted", wound);
        } else {
            println!("  → Damage absorbed (no wound)");
        }
    } else {
        println!("  → MISS (Attack: {} vs Defense: {})", result.attack_roll, result.defense_roll);
    }
}

fn print_fighter_status(character: &Character, skills: &SkillSet, exhaustion: &Exhaustion) {
    println!("\n{}", "=".repeat(70));
    println!("{}", character.name);
    println!("{}", "=".repeat(70));
    println!("Attributes: STR:{} DEX:{} CON:{} STA:{}",
        character.attributes.strength,
        character.attributes.dexterity,
        character.attributes.constitution,
        character.attributes.stamina(),
    );
    println!("Weapon: {} (dmg: {:+})", character.weapon.name, character.weapon.damage);
    println!("Armor: {} (protection: {})", character.armor.name, character.armor.protection);

    println!("\nSkills:");
    for (name, skill) in &skills.skills {
        println!("  {}: {}", name, skill.level);
    }

    println!("\nExhaustion: {} points ({})", exhaustion.points, exhaustion.status());
}

fn print_round_status(
    knight: &Character,
    barbarian: &Character,
    knight_exhaustion: &Exhaustion,
    barbarian_exhaustion: &Exhaustion,
    knight_locations: &Vec<LocationalDamage>,
    barbarian_locations: &Vec<LocationalDamage>,
) {
    println!("\n{}", "-".repeat(70));
    println!("Status:");

    println!("  {}: Wounds(L:{} S:{} C:{}) Exhaustion({} {}) Disabled:{}",
        knight.name,
        knight.wounds.light,
        knight.wounds.severe,
        knight.wounds.critical,
        knight_exhaustion.points,
        knight_exhaustion.status(),
        knight_locations.iter().filter(|l| l.disabled).count(),
    );

    println!("  {}: Wounds(L:{} S:{} C:{}) Exhaustion({} {}) Disabled:{}",
        barbarian.name,
        barbarian.wounds.light,
        barbarian.wounds.severe,
        barbarian.wounds.critical,
        barbarian_exhaustion.points,
        barbarian_exhaustion.status(),
        barbarian_locations.iter().filter(|l| l.disabled).count(),
    );
}

fn print_final_status(
    character: &Character,
    exhaustion: &Exhaustion,
    locations: &Vec<LocationalDamage>,
) {
    println!("{}: {}", character.name, if character.is_alive() { "ALIVE" } else { "DEAD" });
    println!("  Total Wounds: {} Light, {} Severe, {} Critical",
        character.wounds.light,
        character.wounds.severe,
        character.wounds.critical,
    );
    println!("  Exhaustion: {} points ({})", exhaustion.points, exhaustion.status());

    if !locations.is_empty() {
        println!("  Injured Locations:");
        for loc in locations {
            println!("    {}: L:{} S:{} C:{}, penalty: {}, disabled: {}",
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
