# Gameplay Guide

## Starting a Game

```bash
./play.sh new
# Enter player names and passwords
# Choose map size
```

## Taking Turns

```bash
./play.sh
# Enter password
# Take actions
# Turn ends when AP runs out
```

## Commands

**Actions (cost AP):**
- `move <sector>` - Move to adjacent sector (5 AP)
- `attack` - Attack enemy in same sector (8 AP)
- `scan <sector>` - Reveal sector info (3 AP)
- `build` - Build outpost (10 AP)
- `reinforce` - Heal 20 HP (15 AP, level 3+)

**Other:**
- `status` - Show details
- `map` - Show map
- `end` - End turn early
- `help` - Show help

## Turn Flow

1. Player 1 takes turn → saves
2. Player 2 takes turn → saves
3. Repeat until someone wins

## Tips

- Level 4 unlocks Command Centers (needed to capture sectors)
- Scan before moving
- Save AP by ending turn early
- Check turn status: `./play.sh status`

## Security

- Each player has a password-protected account
- Game state is saved after every action
- Only the active player can take actions
- Failed authentication attempts are limited

## Example Turn Flow

```
Player 1 logs in → Authenticates → Takes actions → Turn ends
↓
Game waits for Player 2
↓
Player 2 logs in → Authenticates → Takes actions → Turn ends
↓
Game waits for Player 1
↓
(Repeat until victory)
```

## Troubleshooting

- **"It's not your turn!"** - The other player needs to complete their turn first
- **"Authentication failed!"** - Check your password (case-sensitive)
- **"No saved game found"** - Start a new game with `./play.sh new`

## Advanced Features (Coming Soon)

- Turn time limits
- Email notifications when it's your turn
- Spectator mode for completed games
- Tournament brackets for multiple games

---

Remember: In Interstellar Command, victory comes not from speed of clicking, but depth of strategy! 