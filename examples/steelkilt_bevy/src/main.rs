use bevy::prelude::*;
use std::fs;
use std::path::Path;
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
        .init_resource::<GameState>()
        .init_resource::<CombatState>()
        .init_resource::<ManagementState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_main_menu_input,
                handle_management_input,
                handle_selection_input,
                handle_combat_input,
                update_combat,
                update_main_menu_ui,
                update_management_ui,
                update_selection_ui,
                update_combat_ui,
            )
                .chain(),
        )
        .run();
}

// ===== STATE MANAGEMENT =====

#[derive(Resource, Default, PartialEq, Clone)]
enum GameStateEnum {
    #[default]
    MainMenu,
    Management,
    Selection,
    Combat,
}

#[derive(Resource)]
struct GameState {
    current: GameStateEnum,
    previous: Option<GameStateEnum>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current: GameStateEnum::MainMenu,
            previous: None,
        }
    }
}

impl GameState {
    fn transition_to(&mut self, new_state: GameStateEnum) {
        self.previous = Some(self.current.clone());
        self.current = new_state;
    }

    fn is_in(&self, state: GameStateEnum) -> bool {
        self.current == state
    }
}

// ===== COMBATANT MANAGEMENT =====

#[derive(Resource)]
struct ManagementState {
    combatants: Vec<String>,
    selected_index: usize,
    mode: ManagementMode,
    confirm_delete: Option<String>,
}

#[derive(PartialEq)]
enum ManagementMode {
    List,
    View,
}

impl Default for ManagementState {
    fn default() -> Self {
        Self {
            combatants: load_available_combatants(),
            selected_index: 0,
            mode: ManagementMode::List,
            confirm_delete: None,
        }
    }
}

// ===== COMBAT STATE =====

#[derive(Resource)]
struct CombatState {
    round: u32,
    waiting_for_defense: bool,
    current_attacker: u8,
    combat_log: Vec<String>,
    game_over: bool,
    paused: bool,
    selected_fighter1: Option<usize>,
    selected_fighter2: Option<usize>,
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
            selected_fighter1: None,
            selected_fighter2: None,
        }
    }
}

// ===== COMPONENTS =====

#[derive(Component)]
struct Fighter {
    character: Character,
    is_player_one: bool,
}

#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
struct ManagementUI;

#[derive(Component)]
struct SelectionUI;

#[derive(Component)]
struct SelectionText;

#[derive(Component)]
struct CombatUI;

#[derive(Component)]
struct CombatLogText;

#[derive(Component)]
struct StatusText {
    fighter_id: u8,
}

#[derive(Component)]
struct InstructionText;

#[derive(Component)]
struct DynamicText;

#[derive(Component)]
struct ManagementText;

// ===== FILE OPERATIONS =====

fn load_available_combatants() -> Vec<String> {
    let combatants_dir = "combatants";
    let mut combatants = Vec::new();

    if let Ok(entries) = fs::read_dir(combatants_dir) {
        for entry in entries.flatten() {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with(".json") {
                    combatants.push(filename.trim_end_matches(".json").to_string());
                }
            }
        }
    }

    combatants.sort();
    combatants
}

fn load_character_from_file(filename: &str) -> Result<Character, Box<dyn std::error::Error>> {
    let path = Path::new("combatants").join(format!("{}.json", filename));
    let contents = fs::read_to_string(path)?;
    let character: Character = serde_json::from_str(&contents)?;
    Ok(character)
}

fn save_character_to_file(
    filename: &str,
    character: &Character,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("combatants").join(format!("{}.json", filename));
    let contents = serde_json::to_string_pretty(character)?;
    fs::write(path, contents)?;
    Ok(())
}

fn delete_character_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("combatants").join(format!("{}.json", filename));
    fs::remove_file(path)?;
    Ok(())
}

// ===== SETUP =====

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    spawn_main_menu_ui(&mut commands);
}

// ===== MAIN MENU =====

fn spawn_main_menu_ui(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("=== DRAFT RPG COMBAT SIMULATOR ==="),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.1)),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            // Menu options
            parent.spawn((
                Text::new("[1] Start Combat\n[2] Manage Combatants\n\n[Q] Quit"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
            ));
        });
}

fn handle_main_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut app_exit_events: EventWriter<AppExit>,
    mut commands: Commands,
    ui_query: Query<Entity, With<MainMenuUI>>,
) {
    if !game_state.is_in(GameStateEnum::MainMenu) {
        return;
    }

    if keyboard.just_pressed(KeyCode::Digit1) {
        // Start Combat - go to character selection
        game_state.transition_to(GameStateEnum::Selection);
        for entity in ui_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        spawn_selection_ui(&mut commands);
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        // Manage Combatants
        game_state.transition_to(GameStateEnum::Management);
        for entity in ui_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        spawn_management_ui(&mut commands);
    } else if keyboard.just_pressed(KeyCode::KeyQ) || keyboard.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}

fn update_main_menu_ui() {
    // Main menu is static, no updates needed
}

// ===== COMBATANT MANAGEMENT =====

fn spawn_management_ui(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ManagementUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("=== COMBATANT MANAGEMENT ==="),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.1)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            parent.spawn((
                Text::new("Loading..."),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                ManagementText,
            ));
        });
}

fn handle_management_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut management_state: ResMut<ManagementState>,
    mut commands: Commands,
    ui_query: Query<Entity, With<ManagementUI>>,
) {
    if !game_state.is_in(GameStateEnum::Management) {
        return;
    }

    // Handle delete confirmation
    if let Some(ref filename) = management_state.confirm_delete {
        if keyboard.just_pressed(KeyCode::KeyY) {
            // Confirm delete
            let filename_clone = filename.clone();
            if delete_character_file(&filename_clone).is_ok() {
                management_state.combatants = load_available_combatants();
                if management_state.selected_index >= management_state.combatants.len()
                    && management_state.selected_index > 0
                {
                    management_state.selected_index -= 1;
                }
            }
            management_state.confirm_delete = None;
        } else if keyboard.just_pressed(KeyCode::KeyN) || keyboard.just_pressed(KeyCode::Escape) {
            // Cancel delete
            management_state.confirm_delete = None;
        }
        return;
    }

    // Navigation
    if keyboard.just_pressed(KeyCode::ArrowUp) && management_state.selected_index > 0 {
        management_state.selected_index -= 1;
    } else if keyboard.just_pressed(KeyCode::ArrowDown)
        && management_state.selected_index < management_state.combatants.len().saturating_sub(1)
    {
        management_state.selected_index += 1;
    }

    // Actions
    if keyboard.just_pressed(KeyCode::KeyV) && !management_state.combatants.is_empty() {
        // View selected combatant
        management_state.mode = ManagementMode::View;
    } else if keyboard.just_pressed(KeyCode::KeyD) && !management_state.combatants.is_empty() {
        // Delete confirmation
        let filename = management_state.combatants[management_state.selected_index].clone();
        management_state.confirm_delete = Some(filename);
    } else if keyboard.just_pressed(KeyCode::KeyR) {
        // Refresh list
        management_state.combatants = load_available_combatants();
        management_state.selected_index = 0;
    } else if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::KeyB) {
        // Back to main menu
        if management_state.mode == ManagementMode::View {
            management_state.mode = ManagementMode::List;
        } else {
            game_state.transition_to(GameStateEnum::MainMenu);
            for entity in ui_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            spawn_main_menu_ui(&mut commands);
        }
    }
}

fn update_management_ui(
    management_state: Res<ManagementState>,
    game_state: Res<GameState>,
    mut query: Query<&mut Text, With<ManagementText>>,
) {
    if !game_state.is_in(GameStateEnum::Management) {
        return;
    }

    for mut text in query.iter_mut() {
        if let Some(ref filename) = management_state.confirm_delete {
            **text = format!("Delete '{}'?\n\n[Y] Yes, delete\n[N] No, cancel", filename);
        } else if management_state.mode == ManagementMode::View
            && !management_state.combatants.is_empty()
        {
            // View mode - show character details
            let filename = &management_state.combatants[management_state.selected_index];
            if let Ok(character) = load_character_from_file(filename) {
                **text = format!(
                    "Viewing: {}\n{}\n\nName: {}\nSTR: {} | DEX: {} | CON: {}\nREA: {} | INT: {} | WIL: {}\nCHA: {} | PER: {} | EMP: {}\n\nWeapon Skill: {} | Dodge Skill: {}\nWeapon: {} (Dmg: {})\nArmor: {} (Prot: {})\n\n[ESC] Back to list",
                    filename,
                    "=".repeat(50),
                    character.name,
                    character.attributes.strength,
                    character.attributes.dexterity,
                    character.attributes.constitution,
                    character.attributes.reason,
                    character.attributes.intuition,
                    character.attributes.willpower,
                    character.attributes.charisma,
                    character.attributes.perception,
                    character.attributes.empathy,
                    character.weapon_skill,
                    character.dodge_skill,
                    character.weapon.name,
                    character.weapon.damage,
                    character.armor.name,
                    character.armor.protection,
                );
            } else {
                **text = "Error loading character\n\n[ESC] Back to list".to_string();
            }
        } else {
            // List mode
            let mut display = String::from("Combatants:\n\n");

            if management_state.combatants.is_empty() {
                display.push_str("No combatants found.\n\n");
            } else {
                for (i, name) in management_state.combatants.iter().enumerate() {
                    let marker = if i == management_state.selected_index {
                        ">"
                    } else {
                        " "
                    };
                    display.push_str(&format!("{} {}\n", marker, name));
                }
                display.push_str("\n");
            }

            display.push_str("\n");
            display.push_str("[↑/↓] Navigate | [V] View | [D] Delete | [R] Refresh\n");
            display.push_str("[ESC] Back to Main Menu");

            **text = display;
        }
    }
}

// ===== CHARACTER SELECTION =====

fn spawn_selection_ui(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            SelectionUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("=== CHARACTER SELECTION ==="),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.1)),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            parent.spawn((
                Text::new("Loading combatants..."),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
                SelectionText,
            ));
        });
}

fn handle_selection_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut combat_state: ResMut<CombatState>,
    mut commands: Commands,
    selection_ui: Query<Entity, With<SelectionUI>>,
) {
    if !game_state.is_in(GameStateEnum::Selection) {
        return;
    }

    let combatants = load_available_combatants();

    // Number keys 1-9 and 0 for selecting fighters
    let number_keys = [
        (KeyCode::Digit1, 0),
        (KeyCode::Digit2, 1),
        (KeyCode::Digit3, 2),
        (KeyCode::Digit4, 3),
        (KeyCode::Digit5, 4),
        (KeyCode::Digit6, 5),
        (KeyCode::Digit7, 6),
        (KeyCode::Digit8, 7),
        (KeyCode::Digit9, 8),
        (KeyCode::Digit0, 9),
    ];

    for (key, index) in number_keys {
        if keyboard.just_pressed(key) && index < combatants.len() {
            if combat_state.selected_fighter1.is_none() {
                combat_state.selected_fighter1 = Some(index);
            } else if combat_state.selected_fighter2.is_none()
                && Some(index) != combat_state.selected_fighter1
            {
                combat_state.selected_fighter2 = Some(index);
            }
        }
    }

    // Enter to start combat
    if keyboard.just_pressed(KeyCode::Enter) {
        if let (Some(idx1), Some(idx2)) = (
            combat_state.selected_fighter1,
            combat_state.selected_fighter2,
        ) {
            let name1 = &combatants[idx1];
            let name2 = &combatants[idx2];

            if let (Ok(char1), Ok(char2)) = (
                load_character_from_file(name1),
                load_character_from_file(name2),
            ) {
                // Despawn selection UI
                for entity in selection_ui.iter() {
                    commands.entity(entity).despawn_recursive();
                }

                // Spawn fighters
                commands.spawn(Fighter {
                    character: char1,
                    is_player_one: true,
                });
                commands.spawn(Fighter {
                    character: char2,
                    is_player_one: false,
                });

                // Transition to combat
                game_state.transition_to(GameStateEnum::Combat);
                spawn_combat_ui(&mut commands);
            }
        }
    }

    // Backspace to clear selections
    if keyboard.just_pressed(KeyCode::Backspace) {
        combat_state.selected_fighter2 = None;
        if combat_state.selected_fighter1.is_some() && combat_state.selected_fighter2.is_none() {
            combat_state.selected_fighter1 = None;
        }
    }

    // Escape to go back to main menu
    if keyboard.just_pressed(KeyCode::Escape) {
        for entity in selection_ui.iter() {
            commands.entity(entity).despawn_recursive();
        }
        combat_state.selected_fighter1 = None;
        combat_state.selected_fighter2 = None;
        game_state.transition_to(GameStateEnum::MainMenu);
        spawn_main_menu_ui(&mut commands);
    }
}

fn update_selection_ui(
    combat_state: Res<CombatState>,
    game_state: Res<GameState>,
    mut query: Query<&mut Text, With<SelectionText>>,
) {
    if !game_state.is_in(GameStateEnum::Selection) {
        return;
    }

    let combatants = load_available_combatants();

    for mut text in query.iter_mut() {
        let mut display = String::from("=== CHARACTER SELECTION ===\n\n");
        display.push_str("Select two combatants to fight:\n\n");

        for (i, name) in combatants.iter().enumerate().take(10) {
            let num = if i == 9 { 0 } else { i + 1 };
            let marker = if Some(i) == combat_state.selected_fighter1 {
                " [FIGHTER 1] ✓"
            } else if Some(i) == combat_state.selected_fighter2 {
                " [FIGHTER 2] ✓"
            } else {
                ""
            };
            display.push_str(&format!("[{}] {}{}\n", num, name, marker));
        }

        display.push_str("\n");

        if combat_state.selected_fighter1.is_some() && combat_state.selected_fighter2.is_some() {
            display.push_str("Press [ENTER] to start combat\n");
        } else if combat_state.selected_fighter1.is_some() {
            display.push_str("Select second fighter\n");
        } else {
            display.push_str("Select first fighter\n");
        }

        display.push_str("Press [BACKSPACE] to clear selection\n");
        display.push_str("Press [ESC] to return to main menu");

        **text = display;
    }
}

// ===== COMBAT =====

fn spawn_combat_ui(commands: &mut Commands) {
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

fn handle_combat_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    game_state: Res<GameState>,
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
        let mut attacker: Option<Character> = None;
        let mut defender: Option<Character> = None;

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

fn update_combat(combat_state: Res<CombatState>, _fighters: Query<&Fighter>) {
    if combat_state.is_changed() {
        // Log state changes if needed
    }
}

fn update_combat_ui(
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
