//! Interactive Combat Simulator
//!
//! An advanced combat simulation demonstrating integration of multiple Draft RPG systems:
//! - Skills system for weapon proficiency
//! - Combat stances and maneuvers (Charge, Defensive, All-Out Attack, Aimed Attack)
//! - Exhaustion tracking through prolonged combat
//! - Hit location system with locational damage
//! - Progressive wound accumulation and death spiral
//!
//! The simulator allows interactive control of combat stances and provides detailed
//! feedback on combat resolution, wound effects, and character status.

mod combat;
mod fighters;
mod models;
mod ui;
mod engine;
mod file_ops;

use clap::{Parser, Subcommand};
use engine::*;

use steelkilt::modules::*;
use inquire::Select;
use inquire::error::*;
use file_ops::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Get {
        #[arg(value_name = "VALUE")]
        value: String,
    },
    Set {
        key: String,
        value: String,
        is_true: bool
    },
    Go,
    Usage,
    List
}

fn main() {
    loop {
        let mut buf = format!("{} ", env!("CARGO_PKG_NAME"));

        std::io::stdin().read_line(&mut buf).expect("Couldn't parse stdin");
        let line = buf.trim();
        let args = shlex::split(line).ok_or("error: Invalid quoting").unwrap();

        println!("{:?}", args);

        match Args::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    // Commands::Get(value) => get_something(value),
                    // Commands::Set{key, value, is_true} => set_something(key, value, is_true),
                    Commands::Usage => show_commands(),
                    Commands::List => show_combatants(),
                    Commands::Go => run_combat_rounds(args),
                    _ => show_commands()
                }
            }
            Err(_) => println!("That's not a valid command - use the help command if you are stuck.")
         };
    }
}

fn show_combatants() {

    let options= load_available_combatants();

    let ans: Result<String, InquireError> = Select::new("Select a fighter:", options).prompt();

    match ans {
        Ok(choice) => println!("{}! That's mine too!", choice),
        Err(_) => println!("There was an error, please try again"),
    }
}

fn show_commands() {
    println!(r#"COMMANDS:
get <KEY> - Gets the value of a given key and displays it. If no key given, retrieves all values and displays them.
set <KEY> <VALUE> - Sets the value of a given key.
    Flags: --is-true
"#);
}

