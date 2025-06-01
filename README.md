# Interstellar Command

A terminal-based strategy game with point-based mechanics and RPG progression inspired by Red Rising.

## Overview

Two players compete for control of the solar system using Action Points (AP) and military ranks. Features include password-protected turns, multiple visualization options, and optional AI-generated events.

## Features

- Turn-based strategy with 25 AP per turn
- 10-level progression system with military ranks
- Password authentication for secure gameplay
- ASCII and HTML visualization exports
- Fleet-based combat with multiple ship types
- Sector control and outpost building

## Requirements

- Rust 1.70+
- Terminal with UTF-8 support

## Installation

```bash
git clone <repository>
cd interstellar-command
cargo build --release
cargo run
```

## Gameplay

### Actions and AP Costs

| Action | AP Cost | Description |
|--------|---------|-------------|
| Move Fleet | 5 | Move to adjacent sector |
| Attack Enemy | 8 | Deal damage based on level |
| Scan Sector | 3 | Reveal sector information |
| Build Outpost | 10 | Fortify controlled sector |
| Reinforce | 15 | Heal 20 HP (Level 3+) |
| Sabotage | 12 | Destroy enemy outpost (Level 5+) |
| Orbital Strike | 20 | Ranged attack (Level 7+) |

### Progression System

Players advance through 10 military ranks from Legionnaire to Archsovereign. Each level provides:
- Increased AP capacity
- Additional ships for your fleet
- New abilities at levels 3, 5, and 7
- Combat damage bonuses

### Fleet Composition

Ships provide combat strength and special capabilities:
- **Scouts**: Reconnaissance and extended scan range
- **Frigates**: Standard combat vessels
- **Destroyers**: Heavy firepower
- **Command Centers**: Required to capture sectors (unlocked at Level 4)

## Architecture

```
src/
├── core/           # Game state and player logic
├── map/            # Sector management
├── persistence/    # Save/load functionality
├── ai/             # Event generation
├── visualization/  # Display and export features
└── database/       # Storage abstraction
```

## Optional Features

### AI Events
The game includes atmospheric flavor text between turns. To disable:
```bash
DISABLE_AI=1 cargo run
```

### Visualization Exports
- ASCII map display in terminal
- HTML exports with password protection
- Interactive browser-based map

## Building

```bash
# Development
cargo build

# Release
cargo build --release

# Cross-compilation examples
cargo build --release --target armv7-unknown-linux-gnueabihf  # Raspberry Pi 32-bit
cargo build --release --target aarch64-unknown-linux-gnu      # Raspberry Pi 64-bit
```

## Documentation

- [Quick Start Guide](QUICKSTART.md) - Basic gameplay instructions
- [Visualization Guide](VISUALIZATION_GUIDE.md) - Display options
- [Implementation Status](IMPLEMENTATION_STATUS.md) - Technical details

## License

Open source - modify and extend as needed.