# Visualization Guide

## Available Views

### Terminal Display
- ASCII map showing sector connections and ownership
- Box-drawing UI for clear information hierarchy
- Color-coded player territories

### HTML Exports
Generated via option 9 during gameplay:
- `player_[name]_view.html` - Password-protected player dashboards
- Includes stats, fleet composition, and territory control
- Auto-refreshes for real-time updates

### Interactive Map
Option 10 generates `interstellar_map.html`:
- Visual representation of the solar system
- Hover tooltips for sector information
- Real-time fleet positions
- Auto-refreshes every 5 seconds

## Map Symbols

| Symbol | Meaning |
|--------|---------|
| (C), (D) | First letter of player controlling sector |
| 📍 | Your current location |
| ⚔️ | Enemy fleet present |
| 🏭 | Outpost built |

## Export Security

HTML views are protected by player passwords:
- Uses client-side SHA-256 verification
- No passwords transmitted over network
- Empty password allows access if none was set

## File Locations

All visualization files are created in the game directory:
- `interstellar_map.html` - Interactive map
- `player_[name]_view.html` - Player dashboards
- `interstellar_manual.html` - Game documentation

## Password System

### Setting Passwords
When starting a new game:
```
Player 1, enter your name: Cassia
Player 1, set a password (or press Enter for none): mypassword123
Player 2, enter your name: Darrow  
Player 2, set a password (or press Enter for none): secret456
```

### Password Verification
Before each turn (after Turn 1), players must enter their password:
```
Cassia, enter your password (or press Enter if none): mypassword123
```

## Terminal Visualizations

### 1. ASCII Solar System Map
Displayed at the start of each turn:

```
═══════════════════════════════════════════════════════════════
                    SOLAR SYSTEM MAP                           
═══════════════════════════════════════════════════════════════

    [Earth](C) ---- [Mars]    ---- [Asteroid](C) ---- [Jupiter]
                    |
                 [Venus](D)

Legend: (C) = Cassia, (D) = Darrow, etc.
        * = Outpost present

Fleet Positions:
  Tribune Cassia is at Asteroid Belt
  Legate Darrow is at Venus Orbit
```

### 2. Player Dashboard
Access via option 8 during your turn:

```
╔═══════════════════════════════════════════════════════════════╗
║          Tribune Cassia's Command Dashboard                   ║
╚═══════════════════════════════════════════════════════════════╝

┌─── STATISTICS ───────────────────────────────────────────────┐
│ Level:  3 - Tribune         Health: 120/120                  │
│ XP: 185/300                 AP Cap: 31                       │
│ Damage Bonus: +2            Scan Range: Normal               │
└──────────────────────────────────────────────────────────────┘

┌─── CONTROLLED SECTORS ───────────────────────────────────────┐
│ • Earth Orbit              [OUTPOST]                         │
│ • Asteroid Belt                                              │
└──────────────────────────────────────────────────────────────┘

┌─── ABILITIES ────────────────────────────────────────────────┐
│ • Move Fleet (5 AP)          • Attack (12 damage, 8 AP)      │
│ • Scan Sector (3 AP)         • Build Outpost (10 AP)         │
│ • Reinforce (15 AP) - Heal 20 HP                             │
└──────────────────────────────────────────────────────────────┘
```

### 3. Player Comparison
Shown at game end or via Export Views:

```
╔═══════════════════════════════════════════════════════════════╗
║                    PLAYER COMPARISON                          ║
╚═══════════════════════════════════════════════════════════════╝

                    Player 1         vs         Player 2
─────────────────────────────────────────────────────────────────
Name:                   Cassia                     Darrow
Rank:                  Tribune                      Legate
Level:                       3                           5
Health:                    120                         135
AP Cap:                     31                          36
Damage:                     +2                          +5
Sectors:                     2                           2

Health Bars:
Cassia          [████████████████████] 100%
Darrow          [████████████████████] 100%
```

## HTML Export

### Accessing HTML Views
During your turn, choose option 9 to export views:
```
9) Export Views (free)
```

This creates two HTML files:
- `player_cassia_view.html`
- `player_darrow_view.html`

### HTML Features
The exported HTML includes:
- **Retro Terminal Theme**: Green text on black background
- **Player Statistics**: Level, health, XP, AP cap
- **Controlled Sectors**: List with outpost indicators
- **Solar System Map**: Visual representation
- **Recent Events**: Last 10 game events
- **Responsive Design**: Works on mobile devices

### Opening HTML Files
1. Find the files in your game directory
2. Double-click to open in browser
3. Or right-click → Open with → Your preferred browser

## Database Preparation

### Current: JSON Files
- `game_state.json` - Main save file
- `player_0_view.dat` - Player 1 view data
- `player_1_view.dat` - Player 2 view data

### Future: MySQL Database
The game includes `mysql_schema.sql` with tables for:
- Game states
- Players (with password hashes)
- Sectors and ownership
- Event logs
- Player views
- Game statistics

### Migration Path
1. Current: File-based (JsonDatabase)
2. Future: MySQL (MySqlDatabase)
3. The Database trait allows easy switching

## Security Features

### Password Protection
- SHA-256 hashing (no plaintext storage)
- Optional passwords (can play without)
- Per-turn verification
- Failed password = skip turn

### View Protection
- Each player's HTML export is separate
- Future: Password-protected web views
- Database ready for user authentication

## Use Cases

### Local Play
- Both players at same computer
- Take turns with password protection
- View dashboards during turn
- Export HTML after game

### Remote Sharing
- Export HTML views
- Email to opponent
- Host on website
- View on any device

### Future Web Mode
- MySQL backend
- Real-time updates
- Secure login system
- Multiple concurrent games

## Tips

1. **Set memorable passwords** - You need them every turn
2. **Export views regularly** - Track your progress
3. **Check the map** - See territory control at a glance
4. **Use dashboards** - Free action for full stats
5. **Share HTML files** - Show off your victories

## Troubleshooting

### Forgot Password?
- Delete `game_state.json` to start over
- Or edit the file (advanced users only)

### Can't See HTML Styles?
- Make sure JavaScript is enabled
- Try a different browser
- Check file isn't corrupted

### Export Failed?
- Check write permissions
- Ensure enough disk space
- Try running as administrator 