use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;

// Main game state that holds everything
#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub players: HashMap<String, Player>,
    pub sectors: Vec<Sector>,
    pub ships: HashMap<String, Ship>,
    pub last_cycle: DateTime<Utc>,
    pub cycle_number: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub current_ap: i32,
    pub max_ap: i32,
    pub credits: i32,
    pub level: u32,
    pub xp: u32,
    pub reputation: i32,
    pub owned_ships: Vec<String>, // Ship IDs
    pub command_ships: Vec<String>, // Command ship IDs
    pub garrison_slots: i32, // Available garrison ships
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Sector {
    pub id: String,
    pub name: String,
    pub position: (i32, i32), // Grid position
    pub planet: String, // "Earth" or "Mars"
    pub controlled_by: Option<String>, // Player ID
    pub garrison_ship: Option<String>, // Ship ID holding the sector
    pub ships_present: Vec<String>, // All ships currently in this sector
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ship {
    pub id: String,
    pub name: String,
    pub ship_type: ShipType,
    pub owner: String, // Player ID
    pub current_sector: String, // Sector ID
    pub hp: i32,
    pub max_hp: i32,
    pub damage: i32,
    pub ap_cost: i32, // Cost to move one sector
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum ShipType {
    // Tier 1
    ScoutDrone,
    MiningVessel,
    Interceptor,
    // Tier 2
    Corvette,
    Frigate,
    SupplyShip,
    // Tier 3
    Destroyer,
    GarrisonShip,
    Cruiser,
    // Tier 4
    Battleship,
    CommandShip,
    Carrier,
}

#[derive(Deserialize, Clone)]
pub enum Command {
    Move { ship_id: String, sector_id: String },
    Scan { sector_id: String },
    Attack { target_ship_id: String },
    DeclareControl { sector_id: String, command_ship_id: String },
    SetGarrison { sector_id: String, garrison_ship_id: String },
    Status,
    Fleet,
}

#[derive(Serialize)]
pub struct CommandResult {
    pub success: bool,
    pub message: String,
    pub ap_spent: i32,
    pub game_state: GameState,
}

impl GameState {
    pub fn new() -> Self {
        let mut sectors = Vec::new();
        
        // Create Earth sectors (4x4 grid + core)
        for x in 0..4 {
            for y in 0..4 {
                sectors.push(Sector {
                    id: format!("earth-{}", x * 4 + y + 1),
                    name: format!("E{}", x * 4 + y + 1),
                    position: (x, y),
                    planet: "Earth".to_string(),
                    controlled_by: None,
                    garrison_ship: None,
                    ships_present: Vec::new(),
                });
            }
        }
        
        // Earth core
        sectors.push(Sector {
            id: "earth-core".to_string(),
            name: "Earth Core".to_string(),
            position: (2, 2), // Center position
            planet: "Earth".to_string(),
            controlled_by: None,
            garrison_ship: None,
            ships_present: Vec::new(),
        });
        
        Self {
            players: HashMap::new(),
            sectors,
            ships: HashMap::new(),
            last_cycle: Utc::now(),
            cycle_number: 0,
        }
    }
    
    pub fn execute_command(&mut self, player_id: &str, command: Command) -> CommandResult {
        let player = match self.players.get(player_id) {
            Some(p) => p,
            None => return CommandResult {
                success: false,
                message: "Player not found".to_string(),
                ap_spent: 0,
                game_state: self.clone(),
            }
        };
        
        match command {
            Command::Move { ship_id, sector_id } => self.move_ship(player_id, &ship_id, &sector_id),
            Command::Scan { sector_id } => self.scan_sector(player_id, &sector_id),
            Command::Status => self.player_status(player_id),
            Command::Fleet => self.fleet_status(player_id),
            Command::DeclareControl { sector_id, command_ship_id } => 
                self.declare_control(player_id, &sector_id, &command_ship_id),
            Command::SetGarrison { sector_id, garrison_ship_id } => 
                self.set_garrison(player_id, &sector_id, &garrison_ship_id),
            _ => CommandResult {
                success: false,
                message: "Command not implemented yet".to_string(),
                ap_spent: 0,
                game_state: self.clone(),
            }
        }
    }
    
    fn move_ship(&mut self, player_id: &str, ship_id: &str, target_sector_id: &str) -> CommandResult {
        // Get ship and verify ownership
        let ship = match self.ships.get(ship_id) {
            Some(s) if s.owner == player_id => s.clone(),
            Some(_) => return CommandResult {
                success: false,
                message: "That's not your ship!".to_string(),
                ap_spent: 0,
                game_state: self.clone(),
            },
            None => return CommandResult {
                success: false,
                message: "Ship not found".to_string(),
                ap_spent: 0,
                game_state: self.clone(),
            }
        };
        
        // Check if player has enough AP
        let player = self.players.get_mut(player_id).unwrap();
        if player.current_ap < ship.ap_cost {
            return CommandResult {
                success: false,
                message: format!("Not enough AP. Need {} but have {}", ship.ap_cost, player.current_ap),
                ap_spent: 0,
                game_state: self.clone(),
            };
        }
        
        // Find target sector
        let target_sector = match self.sectors.iter().position(|s| s.id == target_sector_id) {
            Some(idx) => idx,
            None => return CommandResult {
                success: false,
                message: "Target sector not found".to_string(),
                ap_spent: 0,
                game_state: self.clone(),
            }
        };
        
        // Remove ship from current sector
        if let Some(current_sector) = self.sectors.iter_mut().find(|s| s.id == ship.current_sector) {
            current_sector.ships_present.retain(|id| id != ship_id);
        }
        
        // Add ship to new sector
        self.sectors[target_sector].ships_present.push(ship_id.to_string());
        
        // Update ship location
        self.ships.get_mut(ship_id).unwrap().current_sector = target_sector_id.to_string();
        
        // Deduct AP
        self.players.get_mut(player_id).unwrap().current_ap -= ship.ap_cost;
        
        CommandResult {
            success: true,
            message: format!("{} moved to {}", ship.name, self.sectors[target_sector].name),
            ap_spent: ship.ap_cost,
            game_state: self.clone(),
        }
    }
    
    fn scan_sector(&mut self, player_id: &str, sector_id: &str) -> CommandResult {
        const SCAN_COST: i32 = 3;
        
        // Check AP
        let player = self.players.get_mut(player_id).unwrap();
        if player.current_ap < SCAN_COST {
            return CommandResult {
                success: false,
                message: "Not enough AP for scan (need 3)".to_string(),
                ap_spent: 0,
                game_state: self.clone(),
            };
        }
        
        // Find sector
        let sector = match self.sectors.iter().find(|s| s.id == sector_id) {
            Some(s) => s,
            None => return CommandResult {
                success: false,
                message: "Sector not found".to_string(),
                ap_spent: 0,
                game_state: self.clone(),
            }
        };
        
        // Build scan report
        let mut report = format!("=== Sector {} ({}) ===\n", sector.name, sector.planet);
        report.push_str(&format!("Position: ({}, {})\n", sector.position.0, sector.position.1));
        
        if let Some(controller) = &sector.controlled_by {
            let controller_name = self.players.get(controller).map(|p| &p.name).unwrap_or(&"Unknown".to_string());
            report.push_str(&format!("Controlled by: {}\n", controller_name));
        } else {
            report.push_str("Status: Neutral\n");
        }
        
        if !sector.ships_present.is_empty() {
            report.push_str("\nShips present:\n");
            for ship_id in &sector.ships_present {
                if let Some(ship) = self.ships.get(ship_id) {
                    let owner_name = self.players.get(&ship.owner).map(|p| &p.name).unwrap_or(&"Unknown".to_string());
                    report.push_str(&format!("- {} ({:?}) [Owner: {}]\n", ship.name, ship.ship_type, owner_name));
                }
            }
        } else {
            report.push_str("\nNo ships detected\n");
        }
        
        // Deduct AP
        self.players.get_mut(player_id).unwrap().current_ap -= SCAN_COST;
        
        CommandResult {
            success: true,
            message: report,
            ap_spent: SCAN_COST,
            game_state: self.clone(),
        }
    }
    
    fn player_status(&self, player_id: &str) -> CommandResult {
        let player = match self.players.get(player_id) {
            Some(p) => p,
            None => return CommandResult {
                success: false,
                message: "Player not found".to_string(),
                ap_spent: 0,
                game_state: self.clone(),
            }
        };
        
        let status = format!(
            "=== Status ===\n\
            Name: {}\n\
            Level: {} (XP: {})\n\
            AP: {}/{}\n\
            Credits: {}\n\
            Reputation: {}\n\
            Ships: {}\n\
            Command Ships: {}\n\
            Available Garrisons: {}",
            player.name,
            player.level, player.xp,
            player.current_ap, player.max_ap,
            player.credits,
            player.reputation,
            player.owned_ships.len(),
            player.command_ships.len(),
            player.garrison_slots
        );
        
        CommandResult {
            success: true,
            message: status,
            ap_spent: 0,
            game_state: self.clone(),
        }
    }
    
    fn fleet_status(&self, player_id: &str) -> CommandResult {
        let player = match self.players.get(player_id) {
            Some(p) => p,
            None => return CommandResult {
                success: false,
                message: "Player not found".to_string(),
                ap_spent: 0,
                game_state: self.clone(),
            }
        };
        
        let mut report = "=== Fleet Status ===\n".to_string();
        
        for ship_id in &player.owned_ships {
            if let Some(ship) = self.ships.get(ship_id) {
                let sector_name = self.sectors.iter()
                    .find(|s| s.id == ship.current_sector)
                    .map(|s| &s.name)
                    .unwrap_or(&"Unknown".to_string());
                
                report.push_str(&format!(
                    "{} ({:?}) - Location: {} - HP: {}/{}\n",
                    ship.name, ship.ship_type, sector_name, ship.hp, ship.max_hp
                ));
            }
        }
        
        CommandResult {
            success: true,
            message: report,
            ap_spent: 0,
            game_state: self.clone(),
        }
    }
    
    fn declare_control(&mut self, player_id: &str, sector_id: &str, command_ship_id: &str) -> CommandResult {
        const DECLARE_COST: i32 = 25;
        
        // Verify command ship ownership and location
        let command_ship = match self.ships.get(command_ship_id) {
            Some(s) if s.owner == player_id && s.ship_type == ShipType::CommandShip => s.clone(),
            Some(s) if s.owner != player_id => return CommandResult {
                success: false,
                message: "That's not your command ship!".to_string(),
                ap_spent: 0,
                game_state: self.clone(),
            },
            Some(_) => return CommandResult {
                success: false,
                message: "Only command ships can declare control".to_string(),
                ap_spent: 0,
                game_state: self.clone(),
            },
            None => return CommandResult {
                success: false,
                message: "Command ship not found".to_string(),
                ap_spent: 0,
                game_state: self.clone(),
            }
        };
        
        // Check if command ship is in the target sector
        if command_ship.current_sector != sector_id {
            return CommandResult {
                success: false,
                message: "Command ship must be in the sector to declare control".to_string(),
                ap_spent: 0,
                game_state: self.clone(),
            };
        }
        
        // Check AP
        let player = self.players.get_mut(player_id).unwrap();
        if player.current_ap < DECLARE_COST {
            return CommandResult {
                success: false,
                message: format!("Not enough AP. Need {} but have {}", DECLARE_COST, player.current_ap),
                ap_spent: 0,
                game_state: self.clone(),
            };
        }
        
        // Declare control
        if let Some(sector) = self.sectors.iter_mut().find(|s| s.id == sector_id) {
            sector.controlled_by = Some(player_id.to_string());
        }
        
        // Deduct AP
        self.players.get_mut(player_id).unwrap().current_ap -= DECLARE_COST;
        
        CommandResult {
            success: true,
            message: format!("Control declared over sector {}", sector_id),
            ap_spent: DECLARE_COST,
            game_state: self.clone(),
        }
    }
    
    fn set_garrison(&mut self, player_id: &str, sector_id: &str, garrison_ship_id: &str) -> CommandResult {
        // Verify garrison ship
        let garrison_ship = match self.ships.get(garrison_ship_id) {
            Some(s) if s.owner == player_id && s.ship_type == ShipType::GarrisonShip => s.clone(),
            _ => return CommandResult {
                success: false,
                message: "Invalid garrison ship".to_string(),
                ap_spent: 0,
                game_state: self.clone(),
            }
        };
        
        // Check if player controls the sector
        let sector = match self.sectors.iter().find(|s| s.id == sector_id) {
            Some(s) if s.controlled_by.as_ref() == Some(&player_id.to_string()) => s.clone(),
            _ => return CommandResult {
                success: false,
                message: "You don't control this sector".to_string(),
                ap_spent: 0,
                game_state: self.clone(),
            }
        };
        
        // Check if garrison ship is in the sector
        if garrison_ship.current_sector != sector_id {
            return CommandResult {
                success: false,
                message: "Garrison ship must be in the sector".to_string(),
                ap_spent: 0,
                game_state: self.clone(),
            };
        }
        
        // Set garrison
        if let Some(sector) = self.sectors.iter_mut().find(|s| s.id == sector_id) {
            sector.garrison_ship = Some(garrison_ship_id.to_string());
        }
        
        CommandResult {
            success: true,
            message: format!("Garrison established in {}", sector.name),
            ap_spent: 0,
            game_state: self.clone(),
        }
    }
    
    // Process 8-hour cycle
    pub fn process_cycle(&mut self) {
        self.cycle_number += 1;
        self.last_cycle = Utc::now();
        
        // Add AP to all players
        for (_, player) in self.players.iter_mut() {
            player.current_ap = (player.current_ap + 50).min(player.max_ap);
        }
    }
}

impl ShipType {
    pub fn get_stats(&self) -> (i32, i32, i32) {
        // Returns (max_hp, damage, ap_cost)
        match self {
            // Tier 1
            ShipType::ScoutDrone => (10, 2, 1),
            ShipType::MiningVessel => (15, 1, 2),
            ShipType::Interceptor => (20, 5, 1),
            // Tier 2
            ShipType::Corvette => (50, 12, 3),
            ShipType::Frigate => (80, 15, 4),
            ShipType::SupplyShip => (40, 5, 5),
            // Tier 3
            ShipType::Destroyer => (150, 30, 6),
            ShipType::GarrisonShip => (200, 20, 5),
            ShipType::Cruiser => (250, 40, 8),
            // Tier 4
            ShipType::Battleship => (500, 80, 12),
            ShipType::CommandShip => (750, 100, 15),
            ShipType::Carrier => (400, 20, 14),
        }
    }
} 