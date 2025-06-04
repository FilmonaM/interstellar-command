mod game;
mod websocket;

use axum::{
    Router,
    routing::{get, post},
    response::{Html, Json},
    extract::{ws::WebSocketUpgrade, Path, State},
    http::StatusCode,
};
use tower_http::{
    services::ServeDir,
    cors::{CorsLayer, Any},
};
use std::{sync::Arc, path::PathBuf, net::SocketAddr};
use serde::{Deserialize, Serialize};
use tracing_subscriber;
use tokio::net::TcpListener;

use game::{GameState, Player, Ship, ShipType};
use websocket::GameServer;
use uuid::Uuid;

#[derive(Deserialize)]
struct RegisterRequest {
    name: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    player_id: String,
    message: String,
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Load or create game state
    let game_state = load_or_create_game_state().await;
    let game_server = Arc::new(GameServer::new(game_state));
    
    // Start cycle processing task
    let cycle_server = game_server.clone();
    tokio::spawn(async move {
        websocket::GameServer::run_cycle_task(cycle_server).await;
    });
    
    // Set up router
    let app = Router::new()
        // API routes
        .route("/api/register", post(register_player))
        .route("/ws/:player_id", get(websocket_handler))
        // Serve frontend files
        .fallback_service(ServeDir::new("../frontend"))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .with_state(game_server);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Interstellar Command server running at http://{}", addr);
    println!("📡 WebSocket endpoint: ws://{}/ws/<player_id>", addr);
    
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn load_or_create_game_state() -> GameState {
    // Try to load from file
    match tokio::fs::read_to_string("../data/game_state.json").await {
        Ok(json) => {
            match serde_json::from_str(&json) {
                Ok(state) => {
                    println!("✅ Loaded existing game state");
                    state
                },
                Err(e) => {
                    println!("⚠️  Failed to parse game state: {}", e);
                    create_new_game_state()
                }
            }
        },
        Err(_) => {
            println!("📝 Creating new game state");
            create_new_game_state()
        }
    }
}

fn create_new_game_state() -> GameState {
    let mut state = GameState::new();
    
    // Create two test players for development
    let player1_id = "test-player-1";
    let player2_id = "test-player-2";
    
    // Player 1
    state.players.insert(player1_id.to_string(), Player {
        id: player1_id.to_string(),
        name: "Commander Alpha".to_string(),
        current_ap: 50,
        max_ap: 100,
        credits: 100,
        level: 1,
        xp: 0,
        reputation: 0,
        owned_ships: vec!["ship-1".to_string()],
        command_ships: vec![],
        garrison_slots: 0,
    });
    
    // Player 1's starting frigate
    let (hp, damage, ap_cost) = ShipType::Frigate.get_stats();
    state.ships.insert("ship-1".to_string(), Ship {
        id: "ship-1".to_string(),
        name: "Pioneer".to_string(),
        ship_type: ShipType::Frigate,
        owner: player1_id.to_string(),
        current_sector: "earth-1".to_string(),
        hp,
        max_hp: hp,
        damage,
        ap_cost,
    });
    
    // Player 2
    state.players.insert(player2_id.to_string(), Player {
        id: player2_id.to_string(),
        name: "Commander Beta".to_string(),
        current_ap: 50,
        max_ap: 100,
        credits: 100,
        level: 1,
        xp: 0,
        reputation: 0,
        owned_ships: vec!["ship-2".to_string()],
        command_ships: vec![],
        garrison_slots: 0,
    });
    
    // Player 2's starting frigate
    state.ships.insert("ship-2".to_string(), Ship {
        id: "ship-2".to_string(),
        name: "Voyager".to_string(),
        ship_type: ShipType::Frigate,
        owner: player2_id.to_string(),
        current_sector: "earth-16".to_string(),
        hp,
        max_hp: hp,
        damage,
        ap_cost,
    });
    
    // Add ships to their sectors
    state.sectors[0].ships_present.push("ship-1".to_string());
    state.sectors[15].ships_present.push("ship-2".to_string());
    
    state
}

async fn register_player(
    State(game_server): State<Arc<GameServer>>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, StatusCode> {
    let player_id = Uuid::new_v4().to_string();
    
    let mut game_state = game_server.game_state.write().await;
    
    // Create new player
    let player = Player {
        id: player_id.clone(),
        name: req.name.clone(),
        current_ap: 50,
        max_ap: 100,
        credits: 100,
        level: 1,
        xp: 0,
        reputation: 0,
        owned_ships: vec![],
        command_ships: vec![],
        garrison_slots: 0,
    };
    
    game_state.players.insert(player_id.clone(), player);
    
    // Save state
    save_game_state(&game_state).await;
    
    Ok(Json(RegisterResponse {
        player_id,
        message: format!("Welcome to Interstellar Command, {}!", req.name),
    }))
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    Path(player_id): Path<String>,
    State(game_server): State<Arc<GameServer>>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| async move {
        game_server.handle_websocket(socket, player_id).await;
    })
}

async fn save_game_state(state: &GameState) {
    match serde_json::to_string_pretty(state) {
        Ok(json) => {
            if let Err(e) = tokio::fs::write("../data/game_state.json", json).await {
                eprintln!("Failed to save game state: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to serialize game state: {}", e),
    }
} 