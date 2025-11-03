use rand::Rng;

use crate::components::Fighter;
use crate::state::CombatState;

/// Executes a ranged attack from attacker to defender
pub fn execute_ranged_attack(
    attacker: &Fighter,
    defender: &Fighter,
    combat_state: &CombatState,
) -> (bool, i32, String) {
    let ranged_weapon = match &attacker.character.ranged_weapon {
        Some(w) => w,
        None => return (false, 0, "No ranged weapon equipped!".to_string()),
    };

    let attacker_skill = attacker.character.ranged_skill.unwrap_or(0);
    let distance = combat_state.distance.meters();

    // Calculate modifiers
    let distance_mod = ranged_weapon.distance_modifier(distance);
    let aiming_bonus = combat_state.aiming_rounds.min(1); // Max +1 from aiming
    let total_modifier = distance_mod + aiming_bonus;

    // Check if target is in range
    if !ranged_weapon.in_range(distance) {
        return (
            false,
            0,
            format!(
                "Target out of range! ({}m > {}m max)",
                distance, ranged_weapon.max_range
            ),
        );
    }

    // Attacker rolls
    let mut rng = rand::thread_rng();
    let attack_roll_dice = rng.gen_range(1..=10);
    let attack_total = attacker_skill + attack_roll_dice + total_modifier;

    // Defender can only dodge ranged attacks (parrying is very difficult)
    let defender_dodge = defender.character.dodge_skill;
    let defense_roll_dice = rng.gen_range(1..=10);
    let defense_total = defender_dodge + defense_roll_dice;

    let mut log_msg = format!(
        "Ranged Attack: {} fires {} at {}m\n  Attack: {} (skill {}) + d10({}) + modifiers({}) = {}\n  Defense: {} dodges with d10({}) + dodge({}) = {}",
        attacker.character.name,
        ranged_weapon.name,
        distance,
        attacker.character.name,
        attacker_skill,
        attack_roll_dice,
        total_modifier,
        attack_total,
        defender.character.name,
        defense_roll_dice,
        defender_dodge,
        defense_total
    );

    // Determine if hit
    if attack_total > defense_total {
        let base_damage = attack_total - defense_total;
        let weapon_damage = ranged_weapon.damage;
        let armor_protection = defender.character.armor.protection;
        let total_damage = (base_damage + weapon_damage - armor_protection).max(0);

        log_msg.push_str(&format!("\n  HIT! {} damage dealt", total_damage));
        (true, total_damage, log_msg)
    } else {
        log_msg.push_str("\n  MISS! Target dodged successfully");
        (false, 0, log_msg)
    }
}
