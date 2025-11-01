use steelkilt::modules::*;

fn main() {
    println!("=== Draft RPG Advanced Features Demo ===\n");

    // 1. Skill Development System
    println!("1. SKILL DEVELOPMENT SYSTEM");
    println!("{}", "=".repeat(50));
    demo_skills();
    println!();

    // 2. Combat Maneuvers
    println!("2. COMBAT MANEUVERS");
    println!("{}", "=".repeat(50));
    demo_maneuvers();
    println!();

    // 3. Exhaustion System
    println!("3. EXHAUSTION SYSTEM");
    println!("{}", "=".repeat(50));
    demo_exhaustion();
    println!();

    // 4. Hit Location Tracking
    println!("4. HIT LOCATION TRACKING");
    println!("{}", "=".repeat(50));
    demo_hit_locations();
    println!();

    // 5. Ranged Combat
    println!("5. RANGED COMBAT");
    println!("{}", "=".repeat(50));
    demo_ranged_combat();
    println!();

    // 6. Magic System
    println!("6. MAGIC SYSTEM");
    println!("{}", "=".repeat(50));
    demo_magic();
    println!();
}

fn demo_skills() {
    let mut skill_set = SkillSet::new(30);

    // Add some skills
    let sword_skill = Skill::new("Longsword", 7, SkillDifficulty::Normal);
    skill_set.add_skill(sword_skill);

    let language = Skill::new("Native Language", 7, SkillDifficulty::Easy);
    skill_set.add_skill(language);

    let arcane_lore = Skill::new("Arcane Lore", 5, SkillDifficulty::Hard);
    skill_set.add_skill(arcane_lore);

    println!("Starting skill points: {}", skill_set.available_points);

    // Raise Longsword skill
    println!("\nRaising Longsword skill:");
    for i in 1..=5 {
        match skill_set.raise_skill("Longsword") {
            Ok(()) => println!("  Level {}: Success! Points remaining: {}",
                i, skill_set.available_points),
            Err(e) => println!("  Failed: {}", e),
        }
    }

    // Easy skill - Native Language
    println!("\nRaising Native Language (Easy skill) to level 7:");
    // For easy skills, can go from 0 to attribute score for just 1 point
    println!("  Cost from 0 to 7: 1 point");

    // Hard skill - Arcane Lore
    println!("\nRaising Arcane Lore (Hard skill):");
    match skill_set.raise_skill("Arcane Lore") {
        Ok(()) => println!("  Level 1: Success! Points remaining: {}",
            skill_set.available_points),
        Err(e) => println!("  Failed: {}", e),
    }

    println!("\nFinal state:");
    println!("  Longsword: {}", skill_set.get_skill_level("Longsword"));
    println!("  Arcane Lore: {}", skill_set.get_skill_level("Arcane Lore"));
    println!("  Points remaining: {}", skill_set.available_points);
}

fn demo_maneuvers() {
    let mut stance = CombatStance::new();

    println!("Available maneuvers:");
    let maneuvers = vec![
        CombatManeuver::Normal,
        CombatManeuver::DefensivePosition,
        CombatManeuver::Charge,
        CombatManeuver::AllOutAttack,
        CombatManeuver::AimedAttack,
    ];

    for maneuver in &maneuvers {
        println!("\n{:?}:", maneuver);
        println!("  Attack modifier: {:+}", maneuver.attack_modifier());
        println!("  Defense modifier: {:+}", maneuver.defense_modifier());
        println!("  Damage modifier: {:+}", maneuver.damage_modifier());
        println!("  Can attack: {}", maneuver.can_attack());
    }

    println!("\nUsing Charge maneuver:");
    stance.set_maneuver(CombatManeuver::Charge).unwrap();
    println!("  Total attack modifier: {:+}", stance.total_attack_modifier());
    println!("  Total defense modifier: {:+}", stance.total_defense_modifier());
    println!("  Total damage modifier: {:+}", stance.total_damage_modifier());

    println!("\nAiming for Aimed Attack:");
    stance.start_aiming();
    println!("  Started aiming...");
    match stance.set_maneuver(CombatManeuver::AimedAttack) {
        Ok(()) => println!("  Aimed attack ready!"),
        Err(e) => println!("  Error: {}", e),
    }
}

fn demo_exhaustion() {
    let mut exhaustion = Exhaustion::new(7); // Stamina of 7

    println!("Simulating 15 rounds of intense combat:");
    for round in 1..=15 {
        exhaustion.add_points(1);
        println!("Round {}: {} points, Level: {}, Penalty: {}, Status: {}",
            round,
            exhaustion.points,
            exhaustion.level(),
            exhaustion.penalty(),
            exhaustion.status());

        if exhaustion.needs_willpower_check() && round == 14 {
            println!("  → Willpower check required to continue!");
        }

        if !exhaustion.can_perform_exhaustive_actions() {
            println!("  → Cannot perform exhaustive actions!");
            break;
        }
    }

    println!("\nResting for 10 rounds:");
    exhaustion.rest(10);
    println!("After rest: {} points, Status: {}", exhaustion.points, exhaustion.status());
}

fn demo_hit_locations() {
    println!("Hit location examples:\n");

    let directions = vec![
        AttackDirection::Front,
        AttackDirection::Above,
        AttackDirection::Left,
    ];

    for direction in directions {
        println!("Attack from {:?}:", direction);
        for _ in 0..3 {
            let location = HitLocation::determine(direction);
            println!("  → Hit: {} (damage multiplier: {:.2}x)",
                location, location.damage_multiplier());
        }
        println!();
    }

    println!("Tracking damage to Right Arm:");
    let mut arm = LocationalDamage::new(HitLocation::RightArm);

    println!("  Initial: functional={}, penalty={}",
        arm.is_functional(), arm.penalty());

    arm.add_wound(hit_location::WoundSeverity::Light);
    println!("  After light wound: functional={}, penalty={}",
        arm.is_functional(), arm.penalty());

    arm.add_wound(hit_location::WoundSeverity::Severe);
    println!("  After severe wound: functional={}, disabled={}, penalty={}",
        arm.is_functional(), arm.disabled, arm.penalty());

    if arm.location.causes_weapon_drop() {
        println!("  → Weapon dropped!");
    }
}

fn demo_ranged_combat() {
    let bow = RangedWeapon::long_bow();
    let mut state = RangedAttackState::new();

    println!("Weapon: {}", bow.name);
    println!("  Damage: {}", bow.damage);
    println!("  Point Blank Range: {}m", bow.point_blank_range);
    println!("  Max Range: {}m", bow.max_range);
    println!("  Preparation Time: {} segments", bow.preparation_time);

    println!("\nPreparing weapon...");
    state.prepare_weapon(&bow);

    println!("Starting to aim...");
    state.start_aiming();
    state.continue_aiming();

    println!("\nCalculating attack modifiers:");
    let distances = vec![10, 30, 50, 80];
    for distance in distances {
        let total_mod = calculate_ranged_modifiers(
            distance,
            TargetSize::Medium,
            Cover::None,
            &bow,
            &state,
        );
        println!("  At {}m: {:+} modifier", distance, total_mod);
    }

    println!("\nWith cover:");
    let total_mod = calculate_ranged_modifiers(
        25,
        TargetSize::Medium,
        Cover::Partial,
        &bow,
        &state,
    );
    println!("  25m with partial cover: {:+} modifier", total_mod);

    println!("\nFiring...");
    match state.fire() {
        Ok(()) => println!("  Shot fired! Shots remaining: {}", state.shots_remaining),
        Err(e) => println!("  Error: {}", e),
    }
}

fn demo_magic() {
    let mut mage = MagicUser::new(7); // Empathy of 7

    println!("Creating a mage with Empathy 7");

    // Add Divination lore
    println!("\nLearning Divination lore to level 5...");
    mage.add_lore(MagicBranch::Divination, 5);

    // Create and learn a spell
    let spell = Spell {
        name: "Detect Magic".to_string(),
        branch: MagicBranch::Divination,
        difficulty: magic::SpellDifficulty::Easy,
        preparation_time: 5,
        casting_time: 1,
        range: magic::SpellRange::Short(20),
        duration: magic::SpellDuration::Minutes(10),
    };

    println!("Learning spell: {}", spell.name);
    match mage.learn_spell(spell, 4) {
        Ok(()) => println!("  Successfully learned at skill level 4!"),
        Err(e) => println!("  Failed: {}", e),
    }

    // Cast the spell
    println!("\nCasting 'Detect Magic':");
    println!("  Skill: 4, Empathy: 7, Roll: 6");
    println!("  Total: 4 + 7 + 6 = 17 vs target 8 (Easy spell)");

    let result = mage.cast_spell("Detect Magic", 6).unwrap();
    println!("  Success: {}", result.success);
    println!("  Quality: {} ({})", result.quality,
        if result.quality >= 5 { "Excellent" }
        else if result.quality >= 0 { "Good" }
        else { "Poor" });

    println!("\nMagical exhaustion:");
    println!("  Points: {}", mage.exhaustion_points);
    println!("  Level: {:?}", mage.exhaustion_level());
    println!("  Penalty: {:+}", mage.exhaustion_penalty());

    // Show different branches
    println!("\nBranches of Magic:");
    let branches = vec![
        MagicBranch::Alchemy,
        MagicBranch::Elementalism,
        MagicBranch::Necromancy,
        MagicBranch::Thaumaturgy,
    ];

    for branch in branches {
        println!("  {}: {:?} difficulty", branch, branch.lore_difficulty());
    }
}
