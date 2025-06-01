use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sector {
    pub id: u8,                       // unique ID 0…N
    pub name: String,                 // e.g., "Mars Orbit"
    pub owner: Option<u8>,            // None if uncontrolled, or Some(player_id)
    pub adjacent: Vec<u8>,            // IDs of neighboring sectors
    pub visible_to: Vec<u8>,          // which players have scanned/visited
    pub has_outpost: bool,            // true if the controlling player built an outpost
}

impl Sector {
    pub fn new(id: u8, name: String, adjacent: Vec<u8>) -> Self {
        Sector {
            id,
            name,
            owner: None,
            adjacent,
            visible_to: Vec::new(),
            has_outpost: false,
        }
    }

    pub fn is_adjacent(&self, other_id: u8) -> bool {
        self.adjacent.contains(&other_id)
    }

    pub fn capture(&mut self, player_id: u8) {
        self.owner = Some(player_id);
    }

    pub fn is_visible_to(&self, player_id: u8) -> bool {
        self.visible_to.contains(&player_id)
    }
} 