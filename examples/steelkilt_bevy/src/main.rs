use bevy::prelude::*;
use steelkilt::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Draft RPG Combat Simulator".to_string(),
                resolution: (1400., 900.).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<CombatState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_input,
            update_combat,
            update_ui,
        ).chain())
        .run();
}

#[derive(Component)]
struct Fighter {
    character: Character,
    is_player_one: bool,
}

#[derive(Component)]
struct CombatLogText;

#[derive(Component)]
struct StatusText {
    fighter_id: u8, // 1 or 2
}

#[derive(Component)]
struct InstructionText;

#[derive(Resource)]
struct CombatState {
    round: u32,
    waiting_for_defense: bool,
    current_attacker: u8, // 1 or 2
    combat_log: Vec<String>,
    game_over: bool,
    paused: bool,
}

impl Default for CombatState {
    fn default() -> Self {
        Self {
            round: 1,
            waiting_for_defense: true,
            current_attacker: 1,
            combat_log: vec!["=== DRAFT RPG COMBAT SIMULATOR ===".to_string()],
            game_over: false,
            paused: false,
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Create two combatants
    let fighter1 = Fighter {
        character: Character::new(
            "Aldric the Bold",
            Attributes::new(8, 6, 7, 5, 6, 5, 5, 7, 4),
            7, // weapon skill
            5, // dodge skill
            Weapon::long_sword(),
            Armor::chain_mail(),
        ),
        is_player_one: true,
    };

    let fighter2 = Fighter {
        character: Character::new(
            "Grimwald Ironfist",
            Attributes::new(9, 5, 8, 4, 5, 6, 4, 6, 3),
            6, // weapon skill
            4, // dodge skill
            Weapon::two_handed_sword(),
            Armor::leather(),
        ),
        is_player_one: false,
    };

    commands.spawn(fighter1);
    commands.spawn(fighter2);

    // UI Root
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        })
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
                Text::new("Press [P] for Parry or [D] for Dodge\nPress [SPACE] to continue | [Q] to quit"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.2, 0.8, 1.0)),
                InstructionText,
            ));
        });
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut combat_state: ResMut<CombatState>,
    mut fighters: Query<&mut Fighter>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if combat_state.game_over {
        if keyboard.just_pressed(KeyCode::KeyQ) || keyboard.just_pressed(KeyCode::Escape) {
            app_exit_events.send(AppExit::Success);
        }
        return;
    }

    // Handle quit
    if keyboard.just_pressed(KeyCode::KeyQ) || keyboard.just_pressed(KeyCode::Escape) {
        combat_state.game_over = true;
        combat_state.combat_log.push("\nCombat ended by user.".to_string());
        return;
    }

    // Handle defense choice when waiting
    if combat_state.waiting_for_defense {
        let mut attacker: Option<Character> = None;
        let mut defender: Option<Character> = None;

        for fighter in fighters.iter() {
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
                    combat_state.combat_log.push(format!(">>> HIT! {} damage dealt", result.damage));
                    if let Some(level) = result.wound_level {
                        combat_state.combat_log.push(format!(">>> {} wound inflicted!", level));
                    }
                    if result.defender_died {
                        combat_state.combat_log.push(">>> FATAL BLOW!".to_string());
                    }
                } else {
                    combat_state.combat_log.push(">>> MISS! The attack was successfully defended.".to_string());
                }

                // Update fighters
                for mut fighter in fighters.iter_mut() {
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
                    combat_state.combat_log.push(format!("\n{} has been slain!", def.name));
                    combat_state.combat_log.push(format!("{} is victorious!", att.name));
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
                    combat_state.combat_log.push(format!("\n--- ROUND {} ---", round));
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
            for fighter in fighters.iter() {
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
                for fighter in fighters.iter() {
                    if fighter.character.can_act() {
                        both_incapacitated = false;
                        break;
                    }
                }

                if both_incapacitated {
                    combat_state.combat_log.push("\nBoth fighters are incapacitated!".to_string());
                    combat_state.game_over = true;
                }
            }
        }
    }
}

fn update_combat(
    combat_state: Res<CombatState>,
    _fighters: Query<&Fighter>,
) {
    // This system can be expanded for automatic combat logic
    // Currently, all combat logic is in handle_input
    if combat_state.is_changed() {
        // Log state changes if needed
    }
}

fn update_ui(
    combat_state: Res<CombatState>,
    fighters: Query<&Fighter>,
    mut log_query: Query<&mut Text, (With<CombatLogText>, Without<StatusText>, Without<InstructionText>)>,
    mut status_query: Query<(&mut Text, &StatusText), (Without<CombatLogText>, Without<InstructionText>)>,
    mut instruction_query: Query<&mut Text, (With<InstructionText>, Without<CombatLogText>, Without<StatusText>)>,
) {
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

                **text = format!(
                    "{}\n{}\nWeapon: {} (Dmg: {})\nArmor: {} (Prot: {})\nWeapon Skill: {} | Dodge: {}\nWounds: Light:{} Severe:{} Critical:{}\nStatus: {}",
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
                    c.wounds.critical,
                    status_str
                );
            }
        }
    }

    // Update instructions
    if let Ok(mut instruction_text) = instruction_query.get_single_mut() {
        if combat_state.game_over {
            **instruction_text = "Combat Over! Press [Q] to quit".to_string();
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
            **instruction_text = "Press [SPACE] to continue to next round | [Q] to quit".to_string();
        }
    }
}
