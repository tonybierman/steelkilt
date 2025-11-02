use steelkilt::modules::*;
use steelkilt::*;

fn main() {
    println!("=== DRAFT RPG: WIZARD'S DUEL ===\n");

    // Create two magic users
    let mut elara = create_elara();
    let mut malachar = create_malachar();

    print_character_sheet(&elara);
    println!();
    print_character_sheet(&malachar);
    println!();

    // Combat simulation
    let mut round = 1;
    while round <= 10 && elara.magic_user.is_alive() && malachar.magic_user.is_alive() {
        println!("--- ROUND {} ---", round);
        println!();

        // Elara's turn
        if elara.magic_user.can_act() {
            elara_turn(&mut elara, &mut malachar, round);
        } else {
            println!("{} is incapacitated and cannot act!", elara.character.name);
        }

        println!();

        // Malachar's turn
        if malachar.magic_user.can_act() {
            malachar_turn(&mut malachar, &mut elara, round);
        } else {
            println!(
                "{} is incapacitated and cannot act!",
                malachar.character.name
            );
        }

        println!();
        print_status(&elara, &malachar);
        println!();

        // Add exhaustion from combat
        elara.magic_user.add_combat_exhaustion(1);
        malachar.magic_user.add_combat_exhaustion(1);

        round += 1;
    }

    println!("=== DUEL CONCLUDED ===");
    print_final_status(&elara, &malachar);
}

struct Combatant {
    character: Character,
    magic_user: CombatMagicUser,
}

struct CombatMagicUser {
    magic: MagicUser,
    wounds: Wounds,
    constitution: i32,
}

impl CombatMagicUser {
    fn new(empathy: i32, constitution: i32) -> Self {
        Self {
            magic: MagicUser::new(empathy),
            wounds: Wounds::new(),
            constitution,
        }
    }

    fn is_alive(&self) -> bool {
        !self.wounds.is_dead()
    }

    fn can_act(&self) -> bool {
        self.is_alive() && !self.wounds.is_incapacitated()
    }

    fn add_combat_exhaustion(&mut self, points: i32) {
        self.magic.exhaustion_points += points;
    }

    fn take_damage(&mut self, damage: i32) -> Option<WoundLevel> {
        if damage <= 1 {
            return None;
        }

        let level = if damage > self.constitution * 2 {
            WoundLevel::Critical
        } else if damage > self.constitution {
            WoundLevel::Critical
        } else if damage > self.constitution / 2 {
            WoundLevel::Severe
        } else {
            WoundLevel::Light
        };

        self.wounds.add_wound(level);
        Some(level)
    }
}

fn create_elara() -> Combatant {
    let attrs = Attributes::new(5, 6, 6, 7, 8, 7, 6, 7, 9); // High Empathy (9)
    let character = Character::new(
        "Elara the Wise",
        attrs,
        4, // weapon skill
        5, // dodge skill
        Weapon::dagger(),
        Armor::none(),
    );

    let mut magic_user = CombatMagicUser::new(9, 6);

    // Learn Elementalism (Very Hard) - offensive magic
    magic_user.magic.add_lore(MagicBranch::Elementalism, 6);

    let fireball = Spell {
        name: "Fireball".to_string(),
        branch: MagicBranch::Elementalism,
        difficulty: magic::SpellDifficulty::Normal,
        preparation_time: 3,
        casting_time: 1,
        range: magic::SpellRange::Medium(50),
        duration: magic::SpellDuration::Instant,
    };
    magic_user.magic.learn_spell(fireball, 6).unwrap();

    // Learn Animation (Hard) - healing magic
    magic_user.magic.add_lore(MagicBranch::Animation, 5);

    let heal = Spell {
        name: "Healing Touch".to_string(),
        branch: MagicBranch::Animation,
        difficulty: magic::SpellDifficulty::Normal,
        preparation_time: 2,
        casting_time: 1,
        range: magic::SpellRange::Touch,
        duration: magic::SpellDuration::Instant,
    };
    magic_user.magic.learn_spell(heal, 5).unwrap();

    Combatant {
        character,
        magic_user,
    }
}

fn create_malachar() -> Combatant {
    let attrs = Attributes::new(6, 5, 7, 6, 7, 8, 5, 6, 8); // High Empathy (8), Willpower (8)
    let character = Character::new(
        "Malachar the Dark",
        attrs,
        5, // weapon skill
        4, // dodge skill
        Weapon::dagger(),
        Armor::none(),
    );

    let mut magic_user = CombatMagicUser::new(8, 7);

    // Learn Necromancy (Very Hard) - offensive dark magic
    magic_user.magic.add_lore(MagicBranch::Necromancy, 5);

    let death_bolt = Spell {
        name: "Death Bolt".to_string(),
        branch: MagicBranch::Necromancy,
        difficulty: magic::SpellDifficulty::Normal,
        preparation_time: 3,
        casting_time: 1,
        range: magic::SpellRange::Medium(40),
        duration: magic::SpellDuration::Instant,
    };
    magic_user.magic.learn_spell(death_bolt, 5).unwrap();

    // Learn Mentalism (Hard) - mental attacks
    magic_user.magic.add_lore(MagicBranch::Mentalism, 4);

    let mind_blast = Spell {
        name: "Mind Blast".to_string(),
        branch: MagicBranch::Mentalism,
        difficulty: magic::SpellDifficulty::Easy,
        preparation_time: 2,
        casting_time: 1,
        range: magic::SpellRange::Short(30),
        duration: magic::SpellDuration::Instant,
    };
    magic_user.magic.learn_spell(mind_blast, 4).unwrap();

    Combatant {
        character,
        magic_user,
    }
}

fn elara_turn(elara: &mut Combatant, malachar: &mut Combatant, round: i32) {
    println!("{}' turn:", elara.character.name);

    // Decide action based on wounds
    if elara.magic_user.wounds.severe >= 1 && round > 3 {
        // Try to heal if wounded
        println!("  Casting Healing Touch...");
        let roll = d10();
        match elara.magic_user.magic.cast_spell("Healing Touch", roll) {
            Ok(result) => {
                println!(
                    "  → Roll: {} | Total: {} vs target {} | Quality: {}",
                    roll, result.total, result.target, result.quality
                );
                if result.success {
                    // Heal based on quality
                    if elara.magic_user.wounds.severe > 0 {
                        elara.magic_user.wounds.severe -= 1;
                        println!("  → Healed a Severe wound! (Quality: {})", result.quality);
                    } else if elara.magic_user.wounds.light > 0 {
                        elara.magic_user.wounds.light -= 1;
                        println!("  → Healed a Light wound! (Quality: {})", result.quality);
                    }
                } else {
                    println!("  → Spell failed!");
                }
            }
            Err(e) => println!("  → Error: {}", e),
        }
    } else {
        // Attack with fireball
        println!("  Casting Fireball at {}...", malachar.character.name);
        let roll = d10();
        match elara.magic_user.magic.cast_spell("Fireball", roll) {
            Ok(result) => {
                println!(
                    "  → Roll: {} | Total: {} vs target {} | Quality: {}",
                    roll, result.total, result.target, result.quality
                );
                if result.success {
                    let damage = result.quality.max(1) + 3; // Base damage + quality
                    println!("  → HIT! {} damage!", damage);
                    if let Some(wound) = malachar.magic_user.take_damage(damage) {
                        println!("  → {} wound inflicted!", wound);
                    }
                } else {
                    println!("  → Spell failed!");
                }
            }
            Err(e) => println!("  → Error: {}", e),
        }
    }

    let penalty = elara.magic_user.magic.exhaustion_penalty();
    println!(
        "  Exhaustion: {} points (penalty: {:+})",
        elara.magic_user.magic.exhaustion_points, penalty
    );
}

fn malachar_turn(malachar: &mut Combatant, elara: &mut Combatant, round: i32) {
    println!("{}' turn:", malachar.character.name);

    // Alternate between spells
    let spell_name = if round % 2 == 0 {
        "Death Bolt"
    } else {
        "Mind Blast"
    };

    println!("  Casting {} at {}...", spell_name, elara.character.name);
    let roll = d10();
    match malachar.magic_user.magic.cast_spell(spell_name, roll) {
        Ok(result) => {
            println!(
                "  → Roll: {} | Total: {} vs target {} | Quality: {}",
                roll, result.total, result.target, result.quality
            );
            if result.success {
                let damage = if spell_name == "Death Bolt" {
                    result.quality.max(1) + 4 // Death Bolt is more powerful
                } else {
                    result.quality.max(1) + 2 // Mind Blast is weaker but easier
                };
                println!("  → HIT! {} damage!", damage);
                if let Some(wound) = elara.magic_user.take_damage(damage) {
                    println!("  → {} wound inflicted!", wound);
                }
            } else {
                println!("  → Spell failed!");
            }
        }
        Err(e) => println!("  → Error: {}", e),
    }

    let penalty = malachar.magic_user.magic.exhaustion_penalty();
    println!(
        "  Exhaustion: {} points (penalty: {:+})",
        malachar.magic_user.magic.exhaustion_points, penalty
    );
}

fn print_character_sheet(combatant: &Combatant) {
    println!("╔═══════════════════════════════════════╗");
    println!("║ {:37} ║", combatant.character.name);
    println!("╠═══════════════════════════════════════╣");
    println!("║ ATTRIBUTES:                           ║");
    println!(
        "║   EMP: {}   INT: {}   WIL: {}          ║",
        combatant.character.attributes.empathy,
        combatant.character.attributes.intuition,
        combatant.character.attributes.willpower
    );
    println!(
        "║   CON: {}   STR: {}   DEX: {}          ║",
        combatant.magic_user.constitution,
        combatant.character.attributes.strength,
        combatant.character.attributes.dexterity
    );
    println!("╠═══════════════════════════════════════╣");
    println!("║ MAGIC:                                ║");

    for (branch, lore) in &combatant.magic_user.magic.lores {
        println!(
            "║   {}: Level {}                    ║",
            format!("{:?}", branch),
            lore.level
        );
    }

    println!("║ SPELLS:                               ║");
    for (name, learned) in &combatant.magic_user.magic.spells {
        println!(
            "║   {}: Skill {}                ║",
            name, learned.skill_level
        );
    }
    println!("╚═══════════════════════════════════════╝");
}

fn print_status(elara: &Combatant, malachar: &Combatant) {
    println!("Status:");
    println!(
        "  {}: Wounds (L:{} S:{} C:{})",
        elara.character.name,
        elara.magic_user.wounds.light,
        elara.magic_user.wounds.severe,
        elara.magic_user.wounds.critical
    );
    println!(
        "  {}: Wounds (L:{} S:{} C:{})",
        malachar.character.name,
        malachar.magic_user.wounds.light,
        malachar.magic_user.wounds.severe,
        malachar.magic_user.wounds.critical
    );
}

fn print_final_status(elara: &Combatant, malachar: &Combatant) {
    println!();
    if !elara.magic_user.is_alive() {
        println!("💀 {} has fallen!", elara.character.name);
        println!("🏆 {} is victorious!", malachar.character.name);
    } else if !malachar.magic_user.is_alive() {
        println!("💀 {} has fallen!", malachar.character.name);
        println!("🏆 {} is victorious!", elara.character.name);
    } else {
        println!("Both wizards survived the duel!");
        println!();
        print_status(elara, malachar);
    }
}
