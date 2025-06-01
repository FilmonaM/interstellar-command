use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShipType {
    Scout,           // Fast, weak - available to all
    Fighter,         // Basic combat ship - Centurion+
    Cruiser,         // Heavy combat - Tribune+
    CommandCenter,   // Can capture sectors - Prefect+
    Battleship,      // Elite combat - Legate+
    Dreadnought,     // Ultimate warship - Consul+
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ship {
    pub ship_type: ShipType,
    pub health: i32,
    pub max_health: i32,
    pub damage: i32,
    pub movement_range: u8,  // How many sectors it can move
    pub can_capture: bool,   // Only command centers can capture
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fleet {
    pub id: u32,
    pub name: String,
    pub owner: u8,  // Player ID
    pub location: u8,  // Sector ID
    pub ships: Vec<Ship>,
    pub is_stationed: bool,  // If true, provides defense bonus
}

impl ShipType {
    pub fn create_ship(&self) -> Ship {
        match self {
            ShipType::Scout => Ship {
                ship_type: self.clone(),
                health: 20,
                max_health: 20,
                damage: 5,
                movement_range: 2,
                can_capture: false,
            },
            ShipType::Fighter => Ship {
                ship_type: self.clone(),
                health: 40,
                max_health: 40,
                damage: 15,
                movement_range: 1,
                can_capture: false,
            },
            ShipType::Cruiser => Ship {
                ship_type: self.clone(),
                health: 60,
                max_health: 60,
                damage: 25,
                movement_range: 1,
                can_capture: false,
            },
            ShipType::CommandCenter => Ship {
                ship_type: self.clone(),
                health: 100,
                max_health: 100,
                damage: 10,
                movement_range: 1,
                can_capture: true,
            },
            ShipType::Battleship => Ship {
                ship_type: self.clone(),
                health: 80,
                max_health: 80,
                damage: 40,
                movement_range: 1,
                can_capture: false,
            },
            ShipType::Dreadnought => Ship {
                ship_type: self.clone(),
                health: 150,
                max_health: 150,
                damage: 60,
                movement_range: 1,
                can_capture: false,
            },
        }
    }
    
    pub fn required_rank(&self) -> u8 {
        match self {
            ShipType::Scout => 1,         // Legionnaire
            ShipType::Fighter => 2,       // Centurion
            ShipType::Cruiser => 3,       // Tribune
            ShipType::CommandCenter => 4, // Prefect
            ShipType::Battleship => 5,    // Legate
            ShipType::Dreadnought => 7,   // Consul
        }
    }
    
    pub fn build_cost(&self) -> u8 {
        match self {
            ShipType::Scout => 5,
            ShipType::Fighter => 8,
            ShipType::Cruiser => 12,
            ShipType::CommandCenter => 15,
            ShipType::Battleship => 20,
            ShipType::Dreadnought => 25,
        }
    }
}

impl Fleet {
    pub fn new(id: u32, name: String, owner: u8, location: u8) -> Self {
        Fleet {
            id,
            name,
            owner,
            location,
            ships: Vec::new(),
            is_stationed: false,
        }
    }
    
    pub fn add_ship(&mut self, ship_type: ShipType) {
        self.ships.push(ship_type.create_ship());
    }
    
    pub fn total_health(&self) -> i32 {
        self.ships.iter().map(|s| s.health).sum()
    }
    
    pub fn total_damage(&self) -> i32 {
        self.ships.iter().map(|s| s.damage).sum()
    }
    
    pub fn has_command_center(&self) -> bool {
        self.ships.iter().any(|s| s.can_capture)
    }
    
    pub fn remove_destroyed_ships(&mut self) {
        self.ships.retain(|s| s.health > 0);
    }
    
    pub fn ship_summary(&self) -> String {
        let mut summary = String::new();
        let ship_counts = self.count_ships();
        
        for (ship_type, count) in ship_counts {
            if count > 0 {
                summary.push_str(&format!("{} {}s, ", count, 
                    match ship_type {
                        ShipType::Scout => "Scout",
                        ShipType::Fighter => "Fighter",
                        ShipType::Cruiser => "Cruiser",
                        ShipType::CommandCenter => "Command Center",
                        ShipType::Battleship => "Battleship",
                        ShipType::Dreadnought => "Dreadnought",
                    }
                ));
            }
        }
        
        if summary.is_empty() {
            "Empty fleet".to_string()
        } else {
            summary.trim_end_matches(", ").to_string()
        }
    }
    
    fn count_ships(&self) -> Vec<(ShipType, usize)> {
        vec![
            (ShipType::Scout, self.ships.iter().filter(|s| matches!(s.ship_type, ShipType::Scout)).count()),
            (ShipType::Fighter, self.ships.iter().filter(|s| matches!(s.ship_type, ShipType::Fighter)).count()),
            (ShipType::Cruiser, self.ships.iter().filter(|s| matches!(s.ship_type, ShipType::Cruiser)).count()),
            (ShipType::CommandCenter, self.ships.iter().filter(|s| matches!(s.ship_type, ShipType::CommandCenter)).count()),
            (ShipType::Battleship, self.ships.iter().filter(|s| matches!(s.ship_type, ShipType::Battleship)).count()),
            (ShipType::Dreadnought, self.ships.iter().filter(|s| matches!(s.ship_type, ShipType::Dreadnought)).count()),
        ]
    }
} 