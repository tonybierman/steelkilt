use bevy::prelude::*;

use crate::components::{CombatLogText, CombatUI, Fighter, InstructionText, StatusText};
use crate::state::{CombatMode, CombatState, GameState, GameStateEnum, RangedAttackPhase};

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
