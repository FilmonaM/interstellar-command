use crate::core::game::GameState;
use std::error::Error;

/// Database trait that can be implemented by different backends
/// This allows easy switching between JSON files and MySQL in the future
pub trait Database: Send + Sync {
    /// Save the entire game state
    fn save_game_state(&self, state: &GameState) -> Result<(), Box<dyn Error>>;
    
    /// Load the entire game state
    fn load_game_state(&self) -> Result<GameState, Box<dyn Error>>;
    
    /// Save a player-specific view (for password-protected access)
    fn save_player_view(&self, player_id: u8, view_data: &str) -> Result<(), Box<dyn Error>>;
    
    /// Load a player-specific view (requires password verification)
    fn load_player_view(&self, player_id: u8, password: &str) -> Result<String, Box<dyn Error>>;
    
    /// Check if a save exists
    fn save_exists(&self) -> Result<bool, Box<dyn Error>>;
    
    /// Delete all saved data
    fn delete_save(&self) -> Result<(), Box<dyn Error>>;
}

/// JSON file-based implementation
pub struct JsonDatabase {
    base_path: String,
}

impl JsonDatabase {
    pub fn new(base_path: String) -> Self {
        JsonDatabase { base_path }
    }
}

impl Database for JsonDatabase {
    fn save_game_state(&self, state: &GameState) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string_pretty(state)?;
        std::fs::write(format!("{}/game_state.json", self.base_path), json)?;
        Ok(())
    }
    
    fn load_game_state(&self) -> Result<GameState, Box<dyn Error>> {
        let path = format!("{}/game_state.json", self.base_path);
        if !std::path::Path::new(&path).exists() {
            return Err("Save file not found".into());
        }
        
        let contents = std::fs::read_to_string(path)?;
        let state: GameState = serde_json::from_str(&contents)?;
        Ok(state)
    }
    
    fn save_player_view(&self, player_id: u8, view_data: &str) -> Result<(), Box<dyn Error>> {
        // Save encrypted/protected view data
        let filename = format!("{}/player_{}_view.dat", self.base_path, player_id);
        
        // For now, save as plain text but in future could encrypt with player's password
        std::fs::write(filename, view_data)?;
        Ok(())
    }
    
    fn load_player_view(&self, player_id: u8, password: &str) -> Result<String, Box<dyn Error>> {
        // First load the game state to verify password
        let state = self.load_game_state()?;
        
        // Verify password matches player's stored hash
        if player_id as usize >= state.players.len() {
            return Err("Invalid player ID".into());
        }
        
        let player = &state.players[player_id as usize];
        if !player.verify_password(password) {
            return Err("Invalid password".into());
        }
        
        // If password is correct, load the view data
        let filename = format!("{}/player_{}_view.dat", self.base_path, player_id);
        if !std::path::Path::new(&filename).exists() {
            return Err("Player view not found".into());
        }
        
        let contents = std::fs::read_to_string(filename)?;
        Ok(contents)
    }
    
    fn save_exists(&self) -> Result<bool, Box<dyn Error>> {
        Ok(std::path::Path::new(&format!("{}/game_state.json", self.base_path)).exists())
    }
    
    fn delete_save(&self) -> Result<(), Box<dyn Error>> {
        let path = format!("{}/game_state.json", self.base_path);
        if std::path::Path::new(&path).exists() {
            std::fs::remove_file(path)?;
        }
        
        // Also clean up player view files
        for i in 0..2 {
            let view_path = format!("{}/player_{}_view.dat", self.base_path, i);
            if std::path::Path::new(&view_path).exists() {
                let _ = std::fs::remove_file(view_path);
            }
        }
        
        // Clean up any HTML exports
        let _ = std::fs::remove_file("player_cassia_view.html");
        let _ = std::fs::remove_file("player_darrow_view.html");
        
        Ok(())
    }
}


/*
Future MySQL implementation placeholder
pub struct MySqlDatabase {
    connection_string: String,
}

impl MySqlDatabase {
    pub fn new(connection_string: String) -> Self {
        MySqlDatabase { connection_string }
    }
}

impl Database for MySqlDatabase {
    fn save_game_state(&self, state: &GameState) -> Result<(), Box<dyn Error>> {
        // Connect to MySQL
        // Execute INSERT/UPDATE query for game state
        // Tables needed:
        // - game_states (id, turn_number, current_player, game_over, created_at, updated_at)
        // - players (id, game_id, player_id, name, health, ap_cap, ap_remaining, level, xp, rank, password_hash)
        // - sectors (id, game_id, sector_id, name, owner_id, has_outpost)
        // - event_logs (id, game_id, turn_number, event_text, created_at)
        todo!("MySQL implementation")
    }
    
    fn load_game_state(&self) -> Result<GameState, Box<dyn Error>> {
        // Connect to MySQL
        // Execute SELECT queries to reconstruct GameState
        todo!("MySQL implementation")
    }
    
    fn save_player_view(&self, player_id: u8, view_data: &str) -> Result<(), Box<dyn Error>> {
        // Save to player_views table
        todo!("MySQL implementation")
    }
    
    fn load_player_view(&self, player_id: u8, password: &str) -> Result<String, Box<dyn Error>> {
        // Verify password against players table
        // Load from player_views table
        todo!("MySQL implementation")
    }
    
    fn save_exists(&self) -> Result<bool, Box<dyn Error>> {
        // Check if active game exists in database
        todo!("MySQL implementation")
    }
    
    fn delete_save(&self) -> Result<(), Box<dyn Error>> {
        // Mark game as deleted or remove records
        todo!("MySQL implementation")
    }
}
*/ 