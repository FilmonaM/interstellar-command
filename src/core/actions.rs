use crate::core::game::GameState;
use serde::{Serialize, Deserialize};

// Action costs in AP
pub const MOVE_COST: u8 = 5;
pub const ATTACK_COST: u8 = 8;
pub const SCAN_COST: u8 = 3;
pub const BUILD_COST: u8 = 10;
pub const REINFORCE_COST: u8 = 15;
pub const SABOTAGE_COST: u8 = 12;
pub const ORBITAL_STRIKE_COST: u8 = 20;

// Experience rewards
pub const XP_MOVE: u32 = 10;
pub const XP_ATTACK: u32 = 25;
pub const XP_SCAN: u32 = 5;
pub const XP_BUILD: u32 = 30;
pub const XP_CAPTURE: u32 = 50;
pub const XP_KILL_BONUS: u32 = 200;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Move { from: u8, to: u8 },
    Attack { target_player: u8 },
    Scan { sector: u8 },
    Build { sector: u8 },
    Reinforce,
    Sabotage { sector: u8 },
    OrbitalStrike { target_player: u8, target_sector: u8 },
}

pub trait Action {
    fn cost(&self) -> u8;
    fn validate(&self, state: &GameState, player_id: u8) -> Result<(), String>;
    fn execute(&self, state: &mut GameState, player_id: u8) -> Result<String, String>;
    fn required_level(&self) -> u8 { 1 }
}

pub struct MoveAction {
    pub from: u8,
    pub to: u8,
}

impl Action for MoveAction {
    fn cost(&self) -> u8 { MOVE_COST }

    fn validate(&self, state: &GameState, player_id: u8) -> Result<(), String> {
        let player = &state.players[player_id as usize];
        
        if player.current_sector != self.from {
            return Err("You are not in the specified sector".to_string());
        }

        let current_sector = &state.sectors[self.from as usize];
        if !current_sector.is_adjacent(self.to) {
            return Err(format!("Sector {} is not adjacent to your current location", self.to));
        }

        if self.to >= state.sectors.len() as u8 {
            return Err("Invalid target sector".to_string());
        }

        Ok(())
    }

    fn execute(&self, state: &mut GameState, player_id: u8) -> Result<String, String> {
        let player = &mut state.players[player_id as usize];
        player.current_sector = self.to;
        player.ap_remaining -= self.cost();
        player.gain_experience(XP_MOVE);

        let target_name = state.sectors[self.to as usize].name.clone();

        // Mark sector as visible
        if !state.sectors[self.to as usize].visible_to.contains(&player_id) {
            state.sectors[self.to as usize].visible_to.push(player_id);
        }

        // Auto-capture if unowned AND player has command center
        let mut result = format!("Fleet moved to {} (+{} XP)", target_name, XP_MOVE);
        
        if state.sectors[self.to as usize].owner.is_none() && player.can_capture_sector() {
            state.sectors[self.to as usize].capture(player_id);
            player.gain_experience(XP_CAPTURE);
            result.push_str(&format!("\nSector captured! (+{} XP)", XP_CAPTURE));
        }

        state.event_log.push(format!("{} {} moved fleet to {}", 
            player.rank, player.name, target_name));

        Ok(result)
    }
}

pub struct AttackAction {
    pub target_player: u8,
}

impl Action for AttackAction {
    fn cost(&self) -> u8 { ATTACK_COST }

    fn validate(&self, state: &GameState, player_id: u8) -> Result<(), String> {
        let attacker = &state.players[player_id as usize];
        let defender = &state.players[self.target_player as usize];

        if self.target_player == player_id {
            return Err("Cannot attack yourself".to_string());
        }

        if attacker.current_sector != defender.current_sector {
            return Err("Target is not in the same sector".to_string());
        }

        if !defender.is_alive() {
            return Err("Target is already defeated".to_string());
        }

        Ok(())
    }

    fn execute(&self, state: &mut GameState, player_id: u8) -> Result<String, String> {
        let damage = 10 + state.players[player_id as usize].get_damage_bonus();
        let attacker_name = state.players[player_id as usize].name.clone();
        let attacker_rank = state.players[player_id as usize].rank.clone();
        
        // Get defender info before mutation
        let defender_name = state.players[self.target_player as usize].name.clone();
        
        // Apply damage to defender
        state.players[self.target_player as usize].take_damage(damage);
        let defender_alive = state.players[self.target_player as usize].is_alive();
        let defender_health = state.players[self.target_player as usize].health;
        
        // Update attacker's AP and XP
        state.players[player_id as usize].ap_remaining -= self.cost();
        state.players[player_id as usize].gain_experience(XP_ATTACK);

        let mut result = format!("Attacked {} for {} damage! (+{} XP)", 
            defender_name, damage, XP_ATTACK);

        if !defender_alive {
            state.players[player_id as usize].gain_experience(XP_KILL_BONUS);
            result.push_str(&format!("\n{} has been defeated! (+{} XP)", 
                defender_name, XP_KILL_BONUS));
            state.game_over = true;
            state.event_log.push(format!("{} {} defeated {}!", 
                attacker_rank, attacker_name, defender_name));
        } else {
            result.push_str(&format!("\n{} health: {} HP", defender_name, defender_health));
            state.event_log.push(format!("{} {} attacked {} for {} damage", 
                attacker_rank, attacker_name, defender_name, damage));
        }

        Ok(result)
    }
}

pub struct ScanAction {
    pub sector: u8,
}

impl Action for ScanAction {
    fn cost(&self) -> u8 { SCAN_COST }

    fn validate(&self, state: &GameState, player_id: u8) -> Result<(), String> {
        let player = &state.players[player_id as usize];
        let base_range = 1 + player.get_scan_range_bonus();
        
        if self.sector >= state.sectors.len() as u8 {
            return Err("Invalid sector ID".to_string());
        }

        // Check if target is within scan range
        let distance = state.calculate_sector_distance(player.current_sector, self.sector);
        if distance > base_range {
            return Err(format!("Sector {} is out of scan range (max range: {})", 
                self.sector, base_range));
        }

        Ok(())
    }

    fn execute(&self, state: &mut GameState, player_id: u8) -> Result<String, String> {
        let sector = &mut state.sectors[self.sector as usize];
        let sector_name = sector.name.clone();
        
        if !sector.visible_to.contains(&player_id) {
            sector.visible_to.push(player_id);
        }

        state.players[player_id as usize].ap_remaining -= self.cost();
        state.players[player_id as usize].gain_experience(XP_SCAN);

        let mut result = format!("Scanned {} (+{} XP)\n", sector_name, XP_SCAN);
        
        // Report what was found
        if let Some(owner) = sector.owner {
            let owner_name = &state.players[owner as usize].name;
            result.push_str(&format!("Controlled by: {}\n", owner_name));
            if sector.has_outpost {
                result.push_str("Has outpost: Yes\n");
            }
        } else {
            result.push_str("Uncontrolled sector\n");
        }

        // Check for enemy presence
        for (i, player) in state.players.iter().enumerate() {
            if i as u8 != player_id && player.current_sector == self.sector {
                result.push_str(&format!("Enemy fleet detected: {}\n", player.name));
            }
        }

        state.event_log.push(format!("{} {} scanned {}", 
            state.players[player_id as usize].rank, 
            state.players[player_id as usize].name, 
            sector_name));

        Ok(result)
    }
}

pub struct BuildAction {
    pub sector: u8,
}

impl Action for BuildAction {
    fn cost(&self) -> u8 { BUILD_COST }

    fn validate(&self, state: &GameState, player_id: u8) -> Result<(), String> {
        let player = &state.players[player_id as usize];
        
        if player.current_sector != self.sector {
            return Err("You must be in a sector to build an outpost there".to_string());
        }

        let sector = &state.sectors[self.sector as usize];
        
        if sector.owner != Some(player_id) {
            return Err("You must control the sector to build an outpost".to_string());
        }

        if sector.has_outpost {
            return Err("This sector already has an outpost".to_string());
        }

        Ok(())
    }

    fn execute(&self, state: &mut GameState, player_id: u8) -> Result<String, String> {
        state.sectors[self.sector as usize].has_outpost = true;
        state.players[player_id as usize].ap_remaining -= self.cost();
        state.players[player_id as usize].gain_experience(XP_BUILD);

        let sector_name = state.sectors[self.sector as usize].name.clone();
        
        state.event_log.push(format!("{} {} built an outpost at {}", 
            state.players[player_id as usize].rank, 
            state.players[player_id as usize].name, 
            sector_name));

        Ok(format!("[■] Outpost constructed at {} (+{} XP)", sector_name, XP_BUILD))
    }
}

pub struct ReinforceAction;

impl Action for ReinforceAction {
    fn cost(&self) -> u8 { REINFORCE_COST }
    fn required_level(&self) -> u8 { 3 }

    fn validate(&self, state: &GameState, player_id: u8) -> Result<(), String> {
        let player = &state.players[player_id as usize];
        
        if player.level < self.required_level() {
            return Err(format!("Requires level {} (current: {})", 
                self.required_level(), player.level));
        }

        if player.health >= 100 {
            return Err("Already at full health".to_string());
        }

        Ok(())
    }

    fn execute(&self, state: &mut GameState, player_id: u8) -> Result<String, String> {
        let player = &mut state.players[player_id as usize];
        let heal_amount = 20;
        let old_health = player.health;
        
        player.health = (player.health + heal_amount).min(100);
        player.ap_remaining -= self.cost();
        
        let actual_heal = player.health - old_health;
        
        state.event_log.push(format!("{} {} reinforced their position", 
            player.rank, player.name));

        Ok(format!("[+] Fleet reinforced! Healed {} HP (now at {} HP)", 
            actual_heal, player.health))
    }
}

// Helper function to parse actions from user input
pub fn parse_action(input: &str, state: &GameState, player_id: u8) -> Result<Box<dyn Action>, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    if parts.is_empty() {
        return Err("No action specified".to_string());
    }

    match parts[0].to_lowercase().as_str() {
        "move" => {
            if parts.len() < 2 {
                return Err("Usage: move <sector_id>".to_string());
            }
            let to = parts[1].parse::<u8>()
                .map_err(|_| "Invalid sector ID".to_string())?;
            let from = state.players[player_id as usize].current_sector;
            Ok(Box::new(MoveAction { from, to }))
        }
        "attack" => {
            let target = if player_id == 0 { 1 } else { 0 };
            Ok(Box::new(AttackAction { target_player: target }))
        }
        "scan" => {
            if parts.len() < 2 {
                return Err("Usage: scan <sector_id>".to_string());
            }
            let sector = parts[1].parse::<u8>()
                .map_err(|_| "Invalid sector ID".to_string())?;
            Ok(Box::new(ScanAction { sector }))
        }
        "build" => {
            let sector = state.players[player_id as usize].current_sector;
            Ok(Box::new(BuildAction { sector }))
        }
        "reinforce" => {
            Ok(Box::new(ReinforceAction))
        }
        _ => Err(format!("Unknown action: {}", parts[0]))
    }
} 