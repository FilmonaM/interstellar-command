# Interstellar Command

Asynchronous turn-based strategy game for terminal. Play over SSH at your own pace.

## Quick Start

```bash
# Build
cargo build --release

# New game
./play.sh new

# Play turn
./play.sh

# Check status
./play.sh status
```

## Features

- **Asynchronous multiplayer** - Take turns whenever via SSH
- **Action points** - 25 AP per turn for strategic choices  
- **Fleet combat** - Multiple ship types with different strengths
- **Territory control** - Capture sectors, build outposts
- **Leveling** - 10 ranks from Legionnaire to Archsovereign

## How to Play

1. Start game: `./play.sh new`
2. Set player names and passwords
3. Choose map size (8 or 17 sectors)
4. Players take turns by running `./play.sh`

### Commands

- `move <sector>` - Move fleet (5 AP)
- `attack` - Attack enemy (8 AP)
- `scan <sector>` - Reveal info (3 AP)
- `build` - Build outpost (10 AP)
- `reinforce` - Heal fleet (15 AP, level 3+)
- `end` - End turn
- `help` - Show commands

## Architecture

```
src/
├── core/          # Game logic
├── map/           # Sector maps
└── persistence.rs # Save/load
```

Built with Rust. Uses JSON for saves. SHA256 for passwords.

## License

MIT