# Interstellar Command

A web-based space strategy game with terminal aesthetics. Command your fleet across sectors, control territory, and battle for dominance.

## Quick Start (Local Testing)

```bash
# Clone the repository
git clone <your-repo-url>
cd interstellar-command

# Run the server (Windows)
./run.bat

# Run the server (Linux/Mac)
./run.sh
```

Then open your browser to `http://localhost:8080`

## Server Deployment (Linux/Raspberry Pi)

### First Time Setup

```bash
# Clone and enter directory
git clone <your-repo-url>
cd interstellar-command

# Make script executable
chmod +x run.sh

# Run the server (builds automatically on first run)
./run.sh
```

### Updating the Server

```bash
# Pull latest changes
git pull origin main

# Rebuild and run
cd backend
cargo build --release
cd ..
./run.sh
```

### Running 24/7

```bash
# Using screen (recommended)
screen -S interstellar
./run.sh
# Press Ctrl+A then D to detach

# To reattach later
screen -r interstellar

# OR using nohup
nohup ./run.sh > game.log 2>&1 &
```

## Mobile Access (iPhone/Android)

### Option 1: Tailscale (Recommended)
1. Install Tailscale on your server
2. Install Tailscale app on your phone
3. Connect both to same Tailnet
4. Access via: `http://[tailscale-ip]:8080`
5. Add to Home Screen for app experience

### Option 2: Local Network
- Access via: `http://[server-local-ip]:8080`
- Find server IP with `ip addr` or `ifconfig`

## How to Play

### Test Players (for quick testing)
- **Commander Alpha** (ID: test-player-1)
- **Commander Beta** (ID: test-player-2)

### Commands
```bash
status              # View your stats
fleet               # List all ships
scan earth-5        # Scan a sector
move ship-1 earth-7 # Move ship to sector
```

### Game Rules
- Start with 1 Frigate and 50 AP
- Every 8 hours: +50 AP refresh
- Control sectors by moving Command Ships
- Win by eliminating opponent

## Project Structure

```
interstellar-command/
├── backend/          # Rust game server
│   └── src/         # Server source code
├── frontend/        # Web interface
│   ├── index.html   # Game UI
│   ├── game.js      # Client logic
│   └── map.js       # Sector visualization
├── data/           # Game saves
├── run.sh          # Linux/Mac launcher
└── run.bat         # Windows launcher
```

## Configuration

### Change Port
Edit `backend/src/main.rs` line 60:
```rust
let addr = SocketAddr::from(([0, 0, 0, 0], 8080));  // Change 8080
```

### Modify AP Refresh Rate
Edit `backend/src/websocket.rs` line 250:
```rust
// Change from 8 hours to 1 hour for testing
let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1 * 60 * 60));
```

## Troubleshooting

### "Build failed! Make sure Rust is installed"
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### "Connection refused"
- Check firewall: `sudo ufw allow 8080`
- Verify server is running: `ps aux | grep interstellar`

### Can't access from phone
- Ensure devices on same network
- Check server IP is correct
- Try disabling firewall temporarily

## Development

### Making Changes
1. Edit code in `backend/src/` or `frontend/`
2. Rebuild: `cd backend && cargo build --release`
3. Restart server: `./run.sh`

### Reset Game State
```bash
rm data/game_state.json
./run.sh  # Creates fresh game
```