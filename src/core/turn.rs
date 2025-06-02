use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Turn {
    pub number: u32,
    pub active_player: u8,  // 0 or 1
    pub phase: TurnPhase,
    pub started_at: u64,    // Unix timestamp
    pub actions_taken: Vec<ActionRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TurnPhase {
    WaitingForPlayer,   // Player hasn't started their turn yet
    Active,             // Player is actively taking their turn
    Complete,           // Turn is done, waiting for other player
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRecord {
    pub action_type: String,
    pub details: String,
    pub ap_cost: u8,
    pub timestamp: u64,
}

impl Turn {
    pub fn new(number: u32, active_player: u8) -> Self {
        Turn {
            number,
            active_player,
            phase: TurnPhase::WaitingForPlayer,
            started_at: Self::current_timestamp(),
            actions_taken: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.phase = TurnPhase::Active;
        self.started_at = Self::current_timestamp();
    }

    pub fn complete(&mut self) {
        self.phase = TurnPhase::Complete;
    }

    pub fn is_active(&self) -> bool {
        self.phase == TurnPhase::Active
    }

    pub fn record_action(&mut self, action_type: &str, details: &str, ap_cost: u8) {
        self.actions_taken.push(ActionRecord {
            action_type: action_type.to_string(),
            details: details.to_string(),
            ap_cost,
            timestamp: Self::current_timestamp(),
        });
    }

    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnManager {
    pub current_turn: Turn,
    pub turn_history: Vec<Turn>,
    pub last_completed_turn: Option<u32>,
}

impl TurnManager {
    pub fn new() -> Self {
        TurnManager {
            current_turn: Turn::new(1, 0),
            turn_history: Vec::new(),
            last_completed_turn: None,
        }
    }

    pub fn can_player_act(&self, player_id: u8) -> bool {
        self.current_turn.active_player == player_id && 
        self.current_turn.phase != TurnPhase::Complete
    }

    pub fn advance_turn(&mut self) {
        // Save current turn to history
        let mut completed_turn = self.current_turn.clone();
        completed_turn.complete();
        self.turn_history.push(completed_turn);
        self.last_completed_turn = Some(self.current_turn.number);

        // Create new turn for other player
        let next_player = 1 - self.current_turn.active_player;
        let next_turn_number = self.current_turn.number + 1;
        self.current_turn = Turn::new(next_turn_number, next_player);
    }

    pub fn get_turn_summary(&self) -> String {
        format!(
            "Turn {}: {} (Player {})",
            self.current_turn.number,
            match self.current_turn.phase {
                TurnPhase::WaitingForPlayer => "Waiting for player",
                TurnPhase::Active => "In progress",
                TurnPhase::Complete => "Complete",
            },
            self.current_turn.active_player + 1
        )
    }
} 