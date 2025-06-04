use axum::extract::ws::{Message, WebSocket};
use tokio::sync::{mpsc, RwLock, broadcast};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::game::{GameState, Command, CommandResult};

#[derive(Debug, Clone)]
pub struct GameServer {
    pub game_state: Arc<RwLock<GameState>>,
    pub broadcast_tx: broadcast::Sender<String>,
}

#[derive(Deserialize)]
struct ClientMessage {
    #[serde(rename = "type")]
    msg_type: String,
    player_id: Option<String>,
    content: Option<String>,
}

#[derive(Serialize)]
struct ServerMessage {
    #[serde(rename = "type")]
    msg_type: String,
    player: Option<PlayerUpdate>,
    sectors: Option<Vec<SectorUpdate>>,
    message: Option<String>,
}

#[derive(Serialize)]
struct PlayerUpdate {
    ap: i32,
    max_ap: i32,
    credits: i32,
    level: u32,
    ship_count: usize,
}

#[derive(Serialize)]
struct SectorUpdate {
    id: String,
    name: String,
    position: (i32, i32),
    controlled_by: Option<String>,
    ship_count: usize,
    has_garrison: bool,
}

impl GameServer {
    pub fn new(game_state: GameState) -> Self {
        let (broadcast_tx, _) = broadcast::channel(100);
        
        Self {
            game_state: Arc::new(RwLock::new(game_state)),
            broadcast_tx,
        }
    }
    
    pub async fn handle_websocket(&self, socket: WebSocket, player_id: String) {
        let (mut sender, mut receiver) = socket.split();
        
        // Subscribe to broadcasts
        let mut broadcast_rx = self.broadcast_tx.subscribe();
        
        // Send initial state
        self.send_game_update(&mut sender, &player_id).await;
        
        // Spawn task to forward broadcasts
        let broadcast_task = tokio::spawn(async move {
            while let Ok(msg) = broadcast_rx.recv().await {
                if sender.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        });
        
        // Handle incoming messages
        while let Some(msg) = receiver.recv().await {
            if let Ok(msg) = msg {
                match msg {
                    Message::Text(text) => {
                        if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                            self.handle_client_message(client_msg, &player_id).await;
                        }
                    }
                    Message::Close(_) => break,
                    _ => {}
                }
            }
        }
        
        // Clean up
        broadcast_task.abort();
    }
    
    async fn handle_client_message(&self, msg: ClientMessage, player_id: &str) {
        match msg.msg_type.as_str() {
            "command" => {
                if let Some(content) = msg.content {
                    self.process_command(player_id, &content).await;
                }
            }
            "ping" => {
                // Heartbeat - send current state
                self.broadcast_player_update(player_id).await;
            }
            _ => {}
        }
    }
    
    async fn process_command(&self, player_id: &str, command_str: &str) {
        // Parse command string into Command enum
        let command = match self.parse_command(command_str) {
            Ok(cmd) => cmd,
            Err(e) => {
                self.send_error(player_id, &e).await;
                return;
            }
        };
        
        // Execute command
        let mut game_state = self.game_state.write().await;
        let result = game_state.execute_command(player_id, command);
        
        // Broadcast result
        let update = ServerMessage {
            msg_type: "command_result".to_string(),
            player: Some(self.get_player_update(&result.game_state, player_id)),
            sectors: Some(self.get_sector_updates(&result.game_state)),
            message: Some(result.message),
        };
        
        let _ = self.broadcast_tx.send(serde_json::to_string(&update).unwrap());
    }
    
    fn parse_command(&self, input: &str) -> Result<Command, String> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        if parts.is_empty() {
            return Err("Empty command".to_string());
        }
        
        match parts[0].to_lowercase().as_str() {
            "move" => {
                if parts.len() < 3 {
                    return Err("Usage: move <ship-id> <sector-id>".to_string());
                }
                Ok(Command::Move {
                    ship_id: parts[1].to_string(),
                    sector_id: parts[2].to_string(),
                })
            }
            "scan" => {
                if parts.len() < 2 {
                    return Err("Usage: scan <sector-id>".to_string());
                }
                Ok(Command::Scan {
                    sector_id: parts[1].to_string(),
                })
            }
            "status" => Ok(Command::Status),
            "fleet" => Ok(Command::Fleet),
            "declare" => {
                if parts.len() < 3 {
                    return Err("Usage: declare <sector-id> <command-ship-id>".to_string());
                }
                Ok(Command::DeclareControl {
                    sector_id: parts[1].to_string(),
                    command_ship_id: parts[2].to_string(),
                })
            }
            "garrison" => {
                if parts.len() < 3 {
                    return Err("Usage: garrison <sector-id> <garrison-ship-id>".to_string());
                }
                Ok(Command::SetGarrison {
                    sector_id: parts[1].to_string(),
                    garrison_ship_id: parts[2].to_string(),
                })
            }
            _ => Err(format!("Unknown command: {}", parts[0])),
        }
    }
    
    async fn send_game_update(&self, sender: &mut futures_util::stream::SplitSink<WebSocket, Message>, player_id: &str) {
        let game_state = self.game_state.read().await;
        
        let update = ServerMessage {
            msg_type: "game_update".to_string(),
            player: Some(self.get_player_update(&game_state, player_id)),
            sectors: Some(self.get_sector_updates(&game_state)),
            message: Some("Connected to game server".to_string()),
        };
        
        let _ = sender.send(Message::Text(serde_json::to_string(&update).unwrap())).await;
    }
    
    async fn broadcast_player_update(&self, player_id: &str) {
        let game_state = self.game_state.read().await;
        
        let update = ServerMessage {
            msg_type: "player_update".to_string(),
            player: Some(self.get_player_update(&game_state, player_id)),
            sectors: None,
            message: None,
        };
        
        let _ = self.broadcast_tx.send(serde_json::to_string(&update).unwrap());
    }
    
    async fn send_error(&self, player_id: &str, error: &str) {
        let update = ServerMessage {
            msg_type: "error".to_string(),
            player: None,
            sectors: None,
            message: Some(error.to_string()),
        };
        
        let _ = self.broadcast_tx.send(serde_json::to_string(&update).unwrap());
    }
    
    fn get_player_update(&self, game_state: &GameState, player_id: &str) -> PlayerUpdate {
        if let Some(player) = game_state.players.get(player_id) {
            PlayerUpdate {
                ap: player.current_ap,
                max_ap: player.max_ap,
                credits: player.credits,
                level: player.level,
                ship_count: player.owned_ships.len(),
            }
        } else {
            PlayerUpdate {
                ap: 0,
                max_ap: 0,
                credits: 0,
                level: 0,
                ship_count: 0,
            }
        }
    }
    
    fn get_sector_updates(&self, game_state: &GameState) -> Vec<SectorUpdate> {
        game_state.sectors.iter().map(|sector| {
            SectorUpdate {
                id: sector.id.clone(),
                name: sector.name.clone(),
                position: sector.position,
                controlled_by: sector.controlled_by.clone(),
                ship_count: sector.ships_present.len(),
                has_garrison: sector.garrison_ship.is_some(),
            }
        }).collect()
    }
    
    // Cycle processing task
    pub async fn run_cycle_task(game_server: Arc<GameServer>) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(8 * 60 * 60)); // 8 hours
        
        loop {
            interval.tick().await;
            
            let mut game_state = game_server.game_state.write().await;
            game_state.process_cycle();
            
            // Broadcast cycle update
            let update = ServerMessage {
                msg_type: "cycle_update".to_string(),
                player: None,
                sectors: Some(game_server.get_sector_updates(&game_state)),
                message: Some(format!("Cycle {} complete! +50 AP added", game_state.cycle_number)),
            };
            
            let _ = game_server.broadcast_tx.send(serde_json::to_string(&update).unwrap());
        }
    }
} 