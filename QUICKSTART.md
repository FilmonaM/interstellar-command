# Quick Start Guide

## Starting the Game

1. Run `cargo run` in the terminal
2. Enter player names when prompted
3. Set passwords (optional) - press Enter to skip

## Game Basics

### Objective
Eliminate your opponent by reducing their health to zero or control the most sectors.

### Turn Structure
- Each player starts with 25 Action Points (AP) per turn
- Actions consume AP based on their cost
- Turn ends when AP is depleted or player chooses to end turn
- Unused AP does not carry over

### Map Layout
```
[Earth] ---- [Mars] ---- [Asteroid Belt] ---- [Jupiter]
                |
             [Venus]
```

## Actions

### Basic Actions (Always Available)
- **Move (5 AP)**: Navigate to adjacent sectors
- **Attack (8 AP)**: Damage enemy fleet in same sector
- **Scan (3 AP)**: Reveal sector information
- **Build Outpost (10 AP)**: Fortify owned sectors

### Advanced Actions (Level-Locked)
- **Reinforce (15 AP)**: Heal 20 HP - requires Level 3
- **Sabotage (12 AP)**: Destroy enemy outposts - requires Level 5  
- **Orbital Strike (20 AP)**: Long-range attack - requires Level 7

## Progression

Experience is gained through all actions. Every 100 XP advances your level.

### Key Level Milestones
- **Level 4**: First Command Center (required to capture sectors)
- **Level 5**: Destroyer ships and extended scan range
- **Level 7**: Orbital strike capability

## Controls

| Key | Function |
|-----|----------|
| 1-7 | Combat actions |
| 8 | View dashboard |
| 9 | Export game data |
| 10 | Interactive map |
| 11 | Game manual |
| 12 | End turn |
| 13 | Quit game |

## Save System

The game auto-saves after each turn to `game_state.json`. To start fresh, delete this file. 