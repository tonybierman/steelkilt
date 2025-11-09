use std::fs;
use std::path::Path;
use steelkilt::Character;

/// Scans the combatants directory and returns a list of available combatant names.
pub fn load_available_combatants() -> Vec<String> {
    let combatants_dir = "../combatants";
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

/// Loads a character from a JSON file in the combatants directory.
pub fn load_character_from_file(filename: &str) -> Result<Character, Box<dyn std::error::Error>> {
    let path = Path::new("../combatants").join(format!("{}.json", filename));
    let contents = fs::read_to_string(path)?;
    let character: Character = serde_json::from_str(&contents)?;
    Ok(character)
}

/// Saves a character to a JSON file in the combatants directory.
#[allow(dead_code)]
pub fn save_character_to_file(
    filename: &str,
    character: &Character,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("../combatants").join(format!("{}.json", filename));
    let contents = serde_json::to_string_pretty(character)?;
    fs::write(path, contents)?;
    Ok(())
}

/// Deletes a character file from the combatants directory.
#[allow(dead_code)]
pub fn delete_character_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("../combatants").join(format!("{}.json", filename));
    fs::remove_file(path)?;
    Ok(())
}
