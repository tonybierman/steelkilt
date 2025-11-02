use bevy::prelude::*;
use steelkilt::{combat_round, DefenseAction};

use crate::components::{CombatLogText, CombatUI, Fighter, InstructionText, StatusText};
use crate::main_menu::spawn_main_menu_ui;
use crate::state::{CombatState, GameState, GameStateEnum};

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

    // Handle defense choice when waiting
    if combat_state.waiting_for_defense {
        let mut attacker = None;
        let mut defender = None;

        for (_, fighter) in fighters.iter() {
            if (combat_state.current_attacker == 1 && fighter.is_player_one)
                || (combat_state.current_attacker == 2 && !fighter.is_player_one)
            {
                attacker = Some(fighter.character.clone());
            } else {
                defender = Some(fighter.character.clone());
            }
        }

        if let (Some(mut att), Some(mut def)) = (attacker, defender) {
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
            let defender_name = fighters
                .iter()
                .find(|f| {
                    (combat_state.current_attacker == 1 && !f.is_player_one)
                        || (combat_state.current_attacker == 2 && f.is_player_one)
                })
                .map(|f| f.character.name.as_str())
                .unwrap_or("Unknown");

            **instruction_text = format!(
                "How does {} defend? Press [P] for Parry or [D] for Dodge\nPress [Q] to quit",
                defender_name
            );
        } else if combat_state.paused {
            **instruction_text =
                "Press [SPACE] to continue to next round | [Q] to return to main menu".to_string();
        }
    }
}
