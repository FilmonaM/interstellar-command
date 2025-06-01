use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fleet {
    pub scouts: u32,           // Fast reconnaissance ships
    pub frigates: u32,         // Standard combat ships
    pub destroyers: u32,       // Heavy attack ships
    pub command_centers: u32,  // Can capture/control sectors
    pub defense_platforms: u32, // Stationary defense (in sectors with outposts)
}

impl Fleet {
    pub fn new_starter() -> Self {
        Fleet {
            scouts: 0,
            frigates: 1,  // Start with one frigate
            destroyers: 0,
            command_centers: 0,
            defense_platforms: 0,
        }
    }
    
    pub fn total_ships(&self) -> u32 {
        self.scouts + self.frigates + self.destroyers + self.command_centers
    }
    
    pub fn combat_strength(&self) -> u32 {
        self.scouts * 5 + self.frigates * 10 + self.destroyers * 20 + self.command_centers * 15
    }
    
    pub fn can_capture_sectors(&self) -> bool {
        self.command_centers > 0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: u8,                     // 0 or 1
    pub name: String,               // "Cassia" or "Darrow"
    pub health: i32,                // starting at 100
    pub ap_cap: u8,                 // action point cap (starts at 25, can increase)
    pub ap_remaining: u8,           // resets to ap_cap at each turn
    pub current_sector: u8,         // ID of the sector where their fleet sits
    pub level: u8,                  // Player level (1-10)
    pub experience: u32,            // Experience points
    pub rank: String,               // Military rank based on level
    pub fleet: Fleet,               // Player's fleet composition
    #[serde(skip_serializing_if = "Option::is_none")]
    password_hash: Option<String>,  // Hashed password for authentication
}

impl Player {
    pub fn new(id: u8, name: String, starting_sector: u8) -> Self {
        Player {
            id,
            name,
            health: 100,
            ap_cap: 25,
            ap_remaining: 25,
            current_sector: starting_sector,
            level: 1,
            experience: 0,
            rank: "Legionnaire".to_string(),
            fleet: Fleet::new_starter(),
            password_hash: None,
        }
    }

    pub fn set_password(&mut self, password: &str) {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        self.password_hash = Some(format!("{:x}", hasher.finalize()));
    }

    pub fn verify_password(&self, password: &str) -> bool {
        if let Some(ref hash) = self.password_hash {
            let mut hasher = Sha256::new();
            hasher.update(password.as_bytes());
            let input_hash = format!("{:x}", hasher.finalize());
            input_hash == *hash
        } else {
            true // No password set means open access
        }
    }

    pub fn reset_ap(&mut self) {
        self.ap_remaining = self.ap_cap;
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
        if self.health < 0 {
            self.health = 0;
        }
    }

    pub fn can_perform(&self, action_cost: u8) -> bool {
        self.ap_remaining >= action_cost
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn gain_experience(&mut self, amount: u32) {
        self.experience += amount;
        self.check_level_up();
    }

    pub fn check_level_up(&mut self) {
        let xp_needed = self.level as u32 * 100;
        while self.experience >= xp_needed && self.level < 10 {
            self.experience -= xp_needed;
            self.level += 1;
            self.update_rank();
            self.apply_level_bonuses();
        }
    }

    fn update_rank(&mut self) {
        self.rank = match self.level {
            1 => "Legionnaire",
            2 => "Centurion",
            3 => "Tribune",
            4 => "Prefect",
            5 => "Legate",
            6 => "Praetor",
            7 => "Consul",
            8 => "Imperator",
            9 => "Sovereign",
            10 => "Archsovereign",
            _ => "Unknown",
        }.to_string();
    }
    
    fn apply_level_bonuses(&mut self) {
        // Apply stat bonuses
        match self.level {
            2 => {
                self.ap_cap += 3;
                self.fleet.scouts += 1; // Gain a scout ship
            }
            3 => {
                self.health += 20;
                self.fleet.frigates += 1; // Gain another frigate
            }
            4 => {
                self.ap_cap += 3;
                self.fleet.command_centers += 1; // First command center!
            }
            5 => {
                self.health += 20;
                self.fleet.destroyers += 1; // First destroyer
                self.fleet.scouts += 1;
            }
            6 => {
                self.ap_cap += 4;
                self.fleet.frigates += 2; // Fleet expansion
            }
            7 => {
                self.health += 30;
                self.fleet.command_centers += 1; // Second command center
                self.fleet.destroyers += 1;
            }
            8 => {
                self.ap_cap += 4;
                self.fleet.frigates += 2;
                self.fleet.destroyers += 1;
            }
            9 => {
                self.health += 30;
                self.fleet.command_centers += 1; // Third command center
                self.fleet.destroyers += 2;
            }
            10 => {
                self.ap_cap += 5;
                self.health += 50;
                // Massive fleet bonus
                self.fleet.scouts += 2;
                self.fleet.frigates += 3;
                self.fleet.destroyers += 2;
                self.fleet.command_centers += 1;
            }
            _ => {}
        }
    }

    pub fn get_damage_bonus(&self) -> i32 {
        match self.level {
            1..=2 => 0,
            3..=4 => 2,
            5..=6 => 5,
            7..=8 => 8,
            9..=10 => 12,
            _ => 0,
        }
    }

    pub fn get_scan_range_bonus(&self) -> u8 {
        if self.level >= 5 { 1 } else { 0 }
    }
    
    pub fn can_capture_sector(&self) -> bool {
        self.fleet.command_centers > 0
    }
    
    pub fn get_password_hash(&self) -> Option<&String> {
        self.password_hash.as_ref()
    }
} 