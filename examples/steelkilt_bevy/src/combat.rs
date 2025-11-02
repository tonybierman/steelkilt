use bevy::prelude::*;
use rand::Rng;
use steelkilt::{combat_round, DefenseAction, WoundLevel};

use crate::components::{CombatLogText, CombatUI, Fighter, InstructionText, StatusText};
use crate::main_menu::spawn_main_menu_ui;
use crate::state::{
    CombatMode, CombatState, Distance, GameState, GameStateEnum, RangedAttackPhase,
};

/// Spawns the combat UI hierarchy.
pub fn spawn_combat_ui(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            CombatUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("=== DRAFT RPG COMBAT SIMULATOR ==="),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.1)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Fighter Status Container
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|parent| {
                    // Fighter 1 Status
                    parent.spawn((
                        Text::new(""),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        StatusText { fighter_id: 1 },
                    ));

                    // Fighter 2 Status
                    parent.spawn((
                        Text::new(""),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        StatusText { fighter_id: 2 },
                    ));
                });

            // Combat Log
            parent.spawn((
                Text::new(""),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                CombatLogText,
            ));

            // Instructions
            parent.spawn((
                Text::new(
                    "Press [P] for Parry or [D] for Dodge\nPress [SPACE] to continue | [Q] to quit",
                ),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.2, 0.8, 1.0)),
                InstructionText,
            ));
        });
}

/// Executes a ranged attack from attacker to defender
fn execute_ranged_attack(
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

/// Handles combat keyboard input.
pub fn handle_combat_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut combat_state: ResMut<CombatState>,
    mut fighters: Query<(Entity, &mut Fighter)>,
    mut commands: Commands,
    combat_ui: Query<Entity, With<CombatUI>>,
) {
    if !game_state.is_in(GameStateEnum::Combat) {
        return;
    }

    if combat_state.game_over {
        if keyboard.just_pressed(KeyCode::KeyQ) || keyboard.just_pressed(KeyCode::Escape) {
            // Return to main menu
            for entity in combat_ui.iter() {
                commands.entity(entity).despawn_recursive();
            }
            for (entity, _) in fighters.iter() {
                commands.entity(entity).despawn();
            }
            *combat_state = CombatState::default();
            game_state.transition_to(GameStateEnum::MainMenu);
            spawn_main_menu_ui(&mut commands);
        }
        return;
    }

    // Handle quit
    if keyboard.just_pressed(KeyCode::KeyQ) || keyboard.just_pressed(KeyCode::Escape) {
        combat_state.game_over = true;
        combat_state
            .combat_log
            .push("\nCombat ended by user.".to_string());
        return;
    }

    // Check if current attacker can act before allowing any combat actions
    if combat_state.waiting_for_defense {
        let mut current_attacker_can_act = true;
        let mut current_attacker_name = String::new();

        for (_, fighter) in fighters.iter() {
            if (combat_state.current_attacker == 1 && fighter.is_player_one)
                || (combat_state.current_attacker == 2 && !fighter.is_player_one)
            {
                current_attacker_can_act = fighter.character.can_act();
                current_attacker_name = fighter.character.name.clone();
                break;
            }
        }

        if !current_attacker_can_act {
            // Current attacker cannot act - skip their turn
            combat_state.combat_log.push(format!(
                "{} is incapacitated and cannot act!",
                current_attacker_name
            ));

            // Check if both fighters are incapacitated
            let mut both_incapacitated = true;
            for (_, fighter) in fighters.iter() {
                if fighter.character.can_act() {
                    both_incapacitated = false;
                    break;
                }
            }

            if both_incapacitated {
                combat_state
                    .combat_log
                    .push("\nBoth fighters are incapacitated!".to_string());
                combat_state.game_over = true;
                return;
            }

            // Skip to next fighter
            if combat_state.current_attacker == 1 {
                combat_state.current_attacker = 2;
            } else {
                combat_state.current_attacker = 1;
                combat_state.round += 1;
                let round = combat_state.round;
                combat_state
                    .combat_log
                    .push(format!("\n--- ROUND {} ---", round));
            }
            combat_state.waiting_for_defense = true;
            return;
        }
    }

    // Handle combat mode selection (R for ranged, M for melee)
    if combat_state.waiting_for_defense {
        // Check if current attacker has ranged weapon
        let attacker_has_ranged = fighters
            .iter()
            .find(|(_, f)| {
                (combat_state.current_attacker == 1 && f.is_player_one)
                    || (combat_state.current_attacker == 2 && !f.is_player_one)
            })
            .map(|(_, f)| f.character.ranged_weapon.is_some())
            .unwrap_or(false);

        // Allow mode selection if attacker has ranged weapon
        if attacker_has_ranged {
            if keyboard.just_pressed(KeyCode::KeyR) {
                let fighter_name = if combat_state.current_attacker == 1 {
                    "Fighter 1"
                } else {
                    "Fighter 2"
                };
                combat_state.combat_mode = CombatMode::Ranged;
                combat_state.ranged_phase = Some(RangedAttackPhase::Preparing);
                combat_state
                    .combat_log
                    .push(format!("{} switches to ranged combat mode", fighter_name));
                return;
            }
            if keyboard.just_pressed(KeyCode::KeyM) {
                let fighter_name = if combat_state.current_attacker == 1 {
                    "Fighter 1"
                } else {
                    "Fighter 2"
                };
                combat_state.combat_mode = CombatMode::Melee;
                combat_state.ranged_phase = None;
                combat_state.aiming_rounds = 0;
                combat_state
                    .combat_log
                    .push(format!("{} switches to melee combat mode", fighter_name));
                return;
            }
        }

        // Handle distance changes (1=Close, 2=Medium, 3=Long)
        if keyboard.just_pressed(KeyCode::Digit1) {
            combat_state.distance = Distance::Close;
            combat_state
                .combat_log
                .push("Distance: Close range".to_string());
            return;
        }
        if keyboard.just_pressed(KeyCode::Digit2) {
            combat_state.distance = Distance::Medium;
            combat_state
                .combat_log
                .push("Distance: Medium range".to_string());
            return;
        }
        if keyboard.just_pressed(KeyCode::Digit3) {
            combat_state.distance = Distance::Long;
            combat_state
                .combat_log
                .push("Distance: Long range".to_string());
            return;
        }
    }

    // Handle ranged combat sequence
    if combat_state.combat_mode == CombatMode::Ranged {
        if let Some(phase) = combat_state.ranged_phase {
            match phase {
                RangedAttackPhase::Preparing => {
                    if keyboard.just_pressed(KeyCode::KeyA) {
                        // Start aiming
                        combat_state.ranged_phase = Some(RangedAttackPhase::Aiming);
                        combat_state.aiming_rounds = 0;
                        combat_state.combat_log.push("Aiming...".to_string());
                        return;
                    }
                    if keyboard.just_pressed(KeyCode::KeyF) {
                        // Fire without aiming
                        combat_state.ranged_phase = Some(RangedAttackPhase::ReadyToFire);
                    }
                }
                RangedAttackPhase::Aiming => {
                    if keyboard.just_pressed(KeyCode::KeyA) {
                        // Continue aiming (max 1 round for +1 bonus)
                        if combat_state.aiming_rounds < 1 {
                            combat_state.aiming_rounds += 1;
                            let aiming_rounds = combat_state.aiming_rounds;
                            combat_state
                                .combat_log
                                .push(format!("Aiming carefully... (+{} bonus)", aiming_rounds));
                        }
                        return;
                    }
                    if keyboard.just_pressed(KeyCode::KeyF) {
                        // Fire after aiming
                        combat_state.ranged_phase = Some(RangedAttackPhase::ReadyToFire);
                    }
                }
                RangedAttackPhase::ReadyToFire => {
                    // Execute ranged attack
                    let mut attacker_fighter = None;
                    let mut defender_fighter = None;

                    for (_, fighter) in fighters.iter() {
                        if (combat_state.current_attacker == 1 && fighter.is_player_one)
                            || (combat_state.current_attacker == 2 && !fighter.is_player_one)
                        {
                            attacker_fighter = Some(fighter.clone());
                        } else {
                            defender_fighter = Some(fighter.clone());
                        }
                    }

                    if let (Some(attacker), Some(defender)) = (attacker_fighter, defender_fighter) {
                        let (hit, damage, log_msg) =
                            execute_ranged_attack(&attacker, &defender, &combat_state);

                        combat_state.combat_log.push(log_msg);

                        if hit && damage > 0 {
                            // Apply damage to defender
                            for (_, mut fighter) in fighters.iter_mut() {
                                if (combat_state.current_attacker == 1 && !fighter.is_player_one)
                                    || (combat_state.current_attacker == 2 && fighter.is_player_one)
                                {
                                    // Determine wound level based on damage vs CON
                                    let defender_con = fighter.character.attributes.constitution;
                                    let wound_level = if damage > defender_con * 2 {
                                        combat_state.combat_log.push("FATAL HIT!".to_string());
                                        WoundLevel::Critical // Will result in death after stacking
                                    } else if damage > defender_con {
                                        combat_state.combat_log.push("Critical wound!".to_string());
                                        WoundLevel::Critical
                                    } else if damage > defender_con / 2 {
                                        combat_state.combat_log.push("Severe wound!".to_string());
                                        WoundLevel::Severe
                                    } else {
                                        combat_state.combat_log.push("Light wound!".to_string());
                                        WoundLevel::Light
                                    };

                                    fighter.character.wounds.add_wound(wound_level);

                                    if !fighter.character.is_alive() {
                                        combat_state.combat_log.push(format!(
                                            "{} has been slain!",
                                            fighter.character.name
                                        ));
                                        combat_state.game_over = true;
                                    }
                                }
                            }
                        }

                        // Reset ranged attack state and switch turns
                        combat_state.ranged_phase = None;
                        combat_state.aiming_rounds = 0;
                        combat_state.combat_mode = CombatMode::Melee; // Return to melee for next turn

                        // Switch attacker
                        if combat_state.current_attacker == 1 {
                            combat_state.current_attacker = 2;
                        } else {
                            combat_state.current_attacker = 1;
                            combat_state.round += 1;
                            let round = combat_state.round;
                            combat_state.paused = true;
                            combat_state
                                .combat_log
                                .push(format!("\n--- ROUND {} ---", round));
                        }
                    }
                    return;
                }
                _ => {}
            }
        }
    }

    // Handle defense choice when waiting (melee combat)
    if combat_state.waiting_for_defense && combat_state.combat_mode == CombatMode::Melee {
        let mut attacker = None;
        let mut defender = None;
        let mut defender_can_act = true;

        for (_, fighter) in fighters.iter() {
            if (combat_state.current_attacker == 1 && fighter.is_player_one)
                || (combat_state.current_attacker == 2 && !fighter.is_player_one)
            {
                attacker = Some(fighter.character.clone());
            } else {
                defender = Some(fighter.character.clone());
                defender_can_act = fighter.character.can_act();
            }
        }

        if let (Some(mut att), Some(mut def)) = (attacker, defender) {
            // Check if defender can actively defend
            if !defender_can_act {
                // Defender is incapacitated - cannot actively defend, auto-dodge with penalty
                combat_state
                    .combat_log
                    .push(format!("{} is too wounded to defend properly!", def.name));

                let result = combat_round(&mut att, &mut def, DefenseAction::Dodge);

                // Add combat result to log
                combat_state.combat_log.push(format!(
                    "\n>>> Attack: {} rolls {} vs {}'s feeble defense {}",
                    result.attacker, result.attack_roll, result.defender, result.defense_roll
                ));

                if result.hit {
                    combat_state
                        .combat_log
                        .push(format!(">>> HIT! {} damage dealt", result.damage));
                    if let Some(level) = result.wound_level {
                        combat_state
                            .combat_log
                            .push(format!(">>> {} wound inflicted!", level));
                    }
                    if result.defender_died {
                        combat_state.combat_log.push(">>> FATAL BLOW!".to_string());
                    }
                } else {
                    combat_state
                        .combat_log
                        .push(">>> MISS! The attack was fumbled.".to_string());
                }

                // Update fighters
                for (_, mut fighter) in fighters.iter_mut() {
                    if (combat_state.current_attacker == 1 && fighter.is_player_one)
                        || (combat_state.current_attacker == 2 && !fighter.is_player_one)
                    {
                        fighter.character = att.clone();
                    } else {
                        fighter.character = def.clone();
                    }
                }

                // Check for death
                if !def.is_alive() {
                    combat_state
                        .combat_log
                        .push(format!("\n{} has been slain!", def.name));
                    combat_state
                        .combat_log
                        .push(format!("{} is victorious!", att.name));
                    combat_state.game_over = true;
                    combat_state.waiting_for_defense = false;
                    return;
                }

                // Switch turns
                if combat_state.current_attacker == 1 {
                    combat_state.current_attacker = 2;
                } else {
                    combat_state.current_attacker = 1;
                    combat_state.round += 1;
                    let round = combat_state.round;
                    combat_state.paused = true;
                    combat_state.waiting_for_defense = false;
                    combat_state
                        .combat_log
                        .push(format!("\n--- ROUND {} ---", round));
                }
                return;
            }

            let defense_action = if keyboard.just_pressed(KeyCode::KeyP) {
                Some(DefenseAction::Parry)
            } else if keyboard.just_pressed(KeyCode::KeyD) {
                Some(DefenseAction::Dodge)
            } else {
                None
            };

            if let Some(action) = defense_action {
                // Perform combat round
                let result = combat_round(&mut att, &mut def, action);

                // Add combat result to log
                combat_state.combat_log.push(format!(
                    "\n>>> Attack: {} rolls {} vs {}'s defense {}",
                    result.attacker, result.attack_roll, result.defender, result.defense_roll
                ));

                if result.hit {
                    combat_state
                        .combat_log
                        .push(format!(">>> HIT! {} damage dealt", result.damage));
                    if let Some(level) = result.wound_level {
                        combat_state
                            .combat_log
                            .push(format!(">>> {} wound inflicted!", level));
                    }
                    if result.defender_died {
                        combat_state.combat_log.push(">>> FATAL BLOW!".to_string());
                    }
                } else {
                    combat_state
                        .combat_log
                        .push(">>> MISS! The attack was successfully defended.".to_string());
                }

                // Update fighters
                for (_, mut fighter) in fighters.iter_mut() {
                    if (combat_state.current_attacker == 1 && fighter.is_player_one)
                        || (combat_state.current_attacker == 2 && !fighter.is_player_one)
                    {
                        fighter.character = att.clone();
                    } else {
                        fighter.character = def.clone();
                    }
                }

                // Check for death
                if !def.is_alive() {
                    combat_state
                        .combat_log
                        .push(format!("\n{} has been slain!", def.name));
                    combat_state
                        .combat_log
                        .push(format!("{} is victorious!", att.name));
                    combat_state.game_over = true;
                    combat_state.waiting_for_defense = false;
                    return;
                }

                // Switch turns
                if combat_state.current_attacker == 1 {
                    combat_state.current_attacker = 2;
                } else {
                    combat_state.current_attacker = 1;
                    combat_state.round += 1;
                    let round = combat_state.round;
                    combat_state.paused = true;
                    combat_state.waiting_for_defense = false;
                    combat_state
                        .combat_log
                        .push(format!("\n--- ROUND {} ---", round));
                }
            }
        }
    } else if combat_state.paused {
        // Waiting for space to continue
        if keyboard.just_pressed(KeyCode::Space) {
            combat_state.paused = false;
            combat_state.waiting_for_defense = true;

            // Check if current attacker can act
            let mut current_fighter_can_act = false;
            for (_, fighter) in fighters.iter() {
                if (combat_state.current_attacker == 1 && fighter.is_player_one)
                    || (combat_state.current_attacker == 2 && !fighter.is_player_one)
                {
                    current_fighter_can_act = fighter.character.can_act();
                    break;
                }
            }

            if !current_fighter_can_act {
                // Skip to next fighter or end combat
                let mut both_incapacitated = true;
                for (_, fighter) in fighters.iter() {
                    if fighter.character.can_act() {
                        both_incapacitated = false;
                        break;
                    }
                }

                if both_incapacitated {
                    combat_state
                        .combat_log
                        .push("\nBoth fighters are incapacitated!".to_string());
                    combat_state.game_over = true;
                }
            }
        }
    }
}

/// Updates combat state (placeholder for future combat automation logic).
pub fn update_combat(combat_state: Res<CombatState>, _fighters: Query<&Fighter>) {
    if combat_state.is_changed() {
        // Log state changes if needed
    }
}

/// Updates the combat UI (status displays, combat log, instructions).
#[allow(clippy::type_complexity)]
pub fn update_combat_ui(
    combat_state: Res<CombatState>,
    game_state: Res<GameState>,
    fighters: Query<&Fighter>,
    mut log_query: Query<
        &mut Text,
        (
            With<CombatLogText>,
            Without<StatusText>,
            Without<InstructionText>,
        ),
    >,
    mut status_query: Query<
        (&mut Text, &StatusText),
        (Without<CombatLogText>, Without<InstructionText>),
    >,
    mut instruction_query: Query<
        &mut Text,
        (
            With<InstructionText>,
            Without<CombatLogText>,
            Without<StatusText>,
        ),
    >,
) {
    if !game_state.is_in(GameStateEnum::Combat) {
        return;
    }

    // Update combat log
    if let Ok(mut log_text) = log_query.get_single_mut() {
        let log_lines: Vec<String> = combat_state
            .combat_log
            .iter()
            .rev()
            .take(15)
            .rev()
            .cloned()
            .collect();
        **log_text = log_lines.join("\n");
    }

    // Update fighter status
    for (mut text, status) in status_query.iter_mut() {
        for fighter in fighters.iter() {
            let is_match = (status.fighter_id == 1 && fighter.is_player_one)
                || (status.fighter_id == 2 && !fighter.is_player_one);

            if is_match {
                let c = &fighter.character;
                let status_str = if !c.is_alive() {
                    "DEAD"
                } else if c.wounds.is_incapacitated() {
                    "INCAPACITATED"
                } else if c.wounds.severe > 0 {
                    "SEVERELY WOUNDED"
                } else if c.wounds.light > 0 {
                    "LIGHTLY WOUNDED"
                } else {
                    "HEALTHY"
                };

                let mut display = format!(
                    "{}\n{}\nWeapon: {} (Dmg: {})\nArmor: {} (Prot: {})\nWeapon Skill: {} | Dodge: {}\nWounds: Light:{} Severe:{} Critical:{}",
                    c.name,
                    "─".repeat(30),
                    c.weapon.name,
                    c.weapon.damage,
                    c.armor.name,
                    c.armor.protection,
                    c.weapon_skill,
                    c.dodge_skill,
                    c.wounds.light,
                    c.wounds.severe,
                    c.wounds.critical
                );

                // Add magic info if character has magic
                if let Some(ref magic) = c.magic {
                    display.push_str(&format!(
                        "\nSpells: {} | Exhaustion: {} ({:?})",
                        magic.spells.len(),
                        magic.exhaustion_points,
                        magic.exhaustion_level()
                    ));
                }

                // Add ranged weapon info if character has one
                if let Some(ref ranged) = c.ranged_weapon {
                    let ranged_skill = c.ranged_skill.unwrap_or(0);
                    display.push_str(&format!(
                        "\nRanged: {} (Dmg: {}, Range: {}-{}m, Skill: {})",
                        ranged.name,
                        ranged.damage,
                        ranged.point_blank_range,
                        ranged.max_range,
                        ranged_skill
                    ));
                }

                display.push_str(&format!("\nStatus: {}", status_str));
                **text = display;
            }
        }
    }

    // Update instructions
    if let Ok(mut instruction_text) = instruction_query.get_single_mut() {
        if combat_state.game_over {
            **instruction_text = "Combat Over! Press [Q] to return to main menu".to_string();
        } else if combat_state.waiting_for_defense {
            // Check if current attacker has ranged weapon
            let attacker_has_ranged = fighters
                .iter()
                .find(|f| {
                    (combat_state.current_attacker == 1 && f.is_player_one)
                        || (combat_state.current_attacker == 2 && !f.is_player_one)
                })
                .map(|f| f.character.ranged_weapon.is_some())
                .unwrap_or(false);

            let mut instructions = String::new();

            // Show combat mode and distance
            instructions.push_str(&format!(
                "Mode: {:?} | Distance: {:?} ({}m)\n",
                combat_state.combat_mode,
                combat_state.distance,
                combat_state.distance.meters()
            ));

            if combat_state.combat_mode == CombatMode::Ranged {
                // Ranged combat instructions
                if let Some(phase) = combat_state.ranged_phase {
                    match phase {
                        RangedAttackPhase::Preparing => {
                            instructions.push_str("Ranged weapon ready!\n");
                            instructions.push_str(
                                "[A] Aim for bonus | [F] Fire immediately | [M] Switch to melee",
                            );
                        }
                        RangedAttackPhase::Aiming => {
                            instructions.push_str(&format!(
                                "Aiming... (+{} bonus)\n",
                                combat_state.aiming_rounds
                            ));
                            instructions.push_str("[A] Continue aiming | [F] Fire shot");
                        }
                        RangedAttackPhase::ReadyToFire => {
                            instructions.push_str("Firing ranged weapon...");
                        }
                        _ => {
                            instructions.push_str("Ranged combat in progress...");
                        }
                    }
                }
            } else {
                // Melee combat instructions
                let defender_name = fighters
                    .iter()
                    .find(|f| {
                        (combat_state.current_attacker == 1 && !f.is_player_one)
                            || (combat_state.current_attacker == 2 && f.is_player_one)
                    })
                    .map(|f| f.character.name.as_str())
                    .unwrap_or("Unknown");

                instructions.push_str(&format!(
                    "How does {} defend? [P] Parry | [D] Dodge\n",
                    defender_name
                ));

                if attacker_has_ranged {
                    instructions.push_str("[R] Switch to ranged combat | ");
                }
                instructions.push_str("[1/2/3] Change distance | [Q] Quit");
            }

            **instruction_text = instructions;
        } else if combat_state.paused {
            **instruction_text =
                "Press [SPACE] to continue to next round | [Q] to return to main menu".to_string();
        }
    }
}
