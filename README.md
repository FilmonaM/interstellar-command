# Interstellar Command

A web-based space strategy game with terminal aesthetics. Command your fleet across sectors, control territory, and battle for dominance.

## Features

- Terminal-style interface with visual sector map
- Real-time multiplayer via WebSocket
- 8-hour AP (Action Point) refresh cycles
- Command ships, control sectors, manage fleets
- Works on desktop and mobile (iPhone via Tailscale)

## Quick Start

### Prerequisites

- Rust (latest stable)
- Node.js (for development server, optional)

### Running the Game

1. **Start the backend server:**
   ```bash
   cd backend
   cargo run
   ```
   The server will start on `http://localhost:8080`

2. **Access the game:**
   - Open your browser to `http://localhost:8080`
   - Use one of the test players:
     - Commander Alpha (test-player-1)
     - Commander Beta (test-player-2)
   - Or create a new player

### For Mobile Access (iPhone)

1. **Install Tailscale** on your server and iPhone
2. **Connect both devices** to your Tailscale network
3. **Access via Tailscale IP:** `http://[your-tailscale-ip]:8080`
4. **Add to Home Screen** for app-like experience

## Commands

Basic commands you can type in the terminal:

- `status` - View your current stats
- `fleet` - List all your ships
- `scan <sector-id>` - Scan a sector (e.g., `scan earth-5`)
- `move <ship-id> <sector-id>` - Move a ship
- `declare <sector-id> <command-ship-id>` - Declare control (requires command ship)
- `garrison <sector-id> <garrison-ship-id>` - Set garrison to hold sector

## Game Rules

- Start with 1 Frigate and 50 AP
- Every 8 hours: +50 AP (up to your max)
- Move ships between sectors using AP
- Control sectors by:
  1. Moving a Command Ship there
  2. Declaring control (25 AP)
  3. Placing a Garrison Ship to hold it
- Win by eliminating all enemy ships

## Development

### Project Structure
```
interstellar-command/
├── backend/          # Rust game server
│   ├── src/
│   │   ├── main.rs      # Server entry point
│   │   ├── game.rs      # Game logic
│   │   └── websocket.rs # Real-time communication
│   └── Cargo.toml
├── frontend/         # Web interface
│   ├── index.html       # Main page
│   ├── style.css        # Terminal styling
│   ├── game.js          # Game client
│   └── map.js           # Sector visualization
└── data/            # Game state storage
    └── game_state.json
```

### Building for Production

```bash
cd backend
cargo build --release
./target/release/interstellar-backend
```

## Tips

- Click sectors on the map to auto-fill sector IDs in commands
- Use quick command buttons on mobile for faster input
- Monitor your AP usage - running out leaves you vulnerable
- Control strategic sectors between planets for tactical advantage