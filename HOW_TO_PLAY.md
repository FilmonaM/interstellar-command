# How to Play Interstellar Command

## Overview

Interstellar Command is an **asynchronous turn-based strategy game** designed for two players who take turns at different times. Think of it like play-by-email chess, but through SSH on a shared server.

## The Asynchronous System

### What Makes It Asynchronous?

Unlike traditional games where players sit together, Interstellar Command allows:
- **Player 1** takes their turn, then logs off
- Hours or days later, **Player 2** SSH's in and takes their turn
- The game state persists between sessions
- Each player only needs to be online for their own turn

### How It Works

1. **Starting a New Game**
   ```bash
   ./play.sh new
   ```
   - Both players set up their names and passwords
   - The game saves to `game_state.json`
   - Players can now take turns separately

2. **Taking Your Turn**
   ```bash
   ./play.sh
   ```
   - Enter your password (hidden as you type)
   - Take actions until your AP runs out
   - Game auto-saves after each action
   - Your turn ends, other player is notified

3. **Checking Game Status**
   ```bash
   ./play.sh status
   ```
   - See whose turn it is without logging in
   - Check current turn number and phase

## SSH and Server Setup

### Option 1: Shared Server
Both players SSH into the same server:
```bash
ssh username@server.com
cd /path/to/interstellar-command
./play.sh
```

### Option 2: Personal Server
Host on your own machine:
```bash
# Install and run SSH server
sudo apt install openssh-server  # Ubuntu/Debian
sudo systemctl start sshd

# Give your friend access
sudo adduser player2
# Share your IP address with them
```


## Security Features

- **Password Protection**: Each player has their own password
- **Hidden Input**: Passwords don't show when typing
- **Turn Authentication**: Must enter password to take your turn
- **Auto-save**: Every action is saved immediately

## Action Points System

Your AP (Action Points) increases as you level up:

| Level | Rank          | AP Cap | New Ships                |
|-------|---------------|--------|--------------------------|
| 1     | Legionnaire   | 15     | 1 Frigate (starter)      |
| 2     | Centurion     | 17     | +1 Scout                 |
| 3     | Tribune       | 19     | +1 Frigate               |
| 4     | Prefect       | 21     | +1 Command Center        |
| 5     | Legate        | 23     | +1 Destroyer, +1 Scout   |
| 6     | Praetor       | 25     | +2 Frigates              |
| 7     | Consul        | 28     | +1 Command Center, +1 Destroyer |
| 8     | Imperator     | 30     | +2 Frigates, +1 Destroyer |
| 9     | Sovereign     | 33     | +1 Command Center, +2 Destroyers |
| 10    | Archsovereign | 35     | +2 Scouts, +3 Frigates, +2 Destroyers, +1 Command Center |

## Why No tmux/screen?

You **don't need** tmux or screen because:
- Each turn is a single session
- Game state persists to disk
- No long-running processes
- Connect → Play → Disconnect

However, you *can* use tmux if you want to:
```bash
tmux new -s game
./play.sh
# Detach with Ctrl+B, D
# Reattach with: tmux attach -t game
```

## What's Different Now?

### From Original Design
- **No continuous connection needed** - play when convenient
- **Password-protected turns** - secure authentication
- **Hidden password input** - passwords show as `****`
- **Per-action saves** - never lose progress
- **Turn notifications** - see who needs to play

### From Traditional Games
- **Time flexible** - take your turn whenever
- **Location flexible** - play from any SSH client
- **Persistent state** - pick up exactly where you left off
- **True asynchronous** - no coordination needed

## Example Play Session

```
$ ./play.sh
INTERSTELLAR COMMAND
====================

╔═══════════════════════ TURN 5 - CYCLE 3 ═══════════════════════╗
╚═════════════════════════════════════════════════════════════════╝

Turn Status: Player 1's turn is complete. Waiting for Player 2.

Password for Sarah: ****
Authenticated.

STRATEGIC MAP
[Your turn begins...]
```

## Tips for Asynchronous Play

1. **Set up notifications**: Use the server's mail system to notify when turns are complete
2. **Regular schedule**: Agree on rough turn frequency (daily, every few days, etc.)
3. **Leave notes**: Use the event log to communicate with your opponent
4. **Mobile SSH**: Apps like Termux (Android) or Prompt (iOS) let you play on the go

## Troubleshooting

**"It's not your turn!"**
- The other player needs to finish their turn first
- Check status with `./play.sh status`

**"Wrong password"**
- Passwords are case-sensitive
- No password? Just press Enter

**Connection dropped mid-turn**
- Don't worry! Reconnect and continue
- All actions were saved

**Can't see password**
- This is normal - passwords are hidden for security
- Type carefully and press Enter

## Summary

Interstellar Command brings the strategic depth of classic turn-based games to the modern asynchronous world. No need to coordinate schedules - just SSH in when you have time, take your turn, and let the game handle the rest! 