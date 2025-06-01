use std::fs;
use std::path::Path;
use serde_json;
use crate::core::game::GameState;

const SAVE_FILE: &str = "game_state.json";

pub fn save(state: &GameState) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(state)?;
    fs::write(SAVE_FILE, json)?;
    Ok(())
}

pub fn load() -> Result<GameState, Box<dyn std::error::Error>> {
    if !Path::new(SAVE_FILE).exists() {
        return Err("Save file not found".into());
    }
    
    let contents = fs::read_to_string(SAVE_FILE)?;
    let state: GameState = serde_json::from_str(&contents)?;
    Ok(state)
}

pub fn delete_save() -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(SAVE_FILE).exists() {
        fs::remove_file(SAVE_FILE)?;
    }
    Ok(())
} 