#!/bin/bash

# Interstellar Command - Game Runner

echo "INTERSTELLAR COMMAND"
echo "===================="
echo

# Check if this is a new game
if [ "$1" == "new" ]; then
    echo "Starting new campaign..."
    cargo run new
elif [ "$1" == "help" ]; then
    echo "Usage:"
    echo "  ./play.sh         - Resume game"
    echo "  ./play.sh new     - Start new"
    echo "  ./play.sh status  - Check status"
    echo "  ./play.sh log     - View history"
elif [ "$1" == "status" ]; then
    if [ -f "game_state.json" ]; then
        echo "Checking game status..."
        turn=$(grep -o '"number":[0-9]*' game_state.json | head -1 | cut -d: -f2)
        active=$(grep -o '"active_player":[0-9]*' game_state.json | head -1 | cut -d: -f2)
        phase=$(grep -o '"phase":"[^"]*"' game_state.json | head -1 | cut -d: -f3 | tr -d '"')
        
        echo "Turn: $turn"
        echo "Active: Player $((active + 1))"
        echo "Phase: $phase"
    else
        echo "No game in progress."
    fi
elif [ "$1" == "log" ]; then
    if [ -f "game_state.json" ]; then
        echo "Recent events:"
        grep -o '"event_log":\[[^]]*\]' game_state.json | sed 's/","/\n/g' | tail -10
    else
        echo "No game in progress."
    fi
else
    # Resume game
    if [ -f "game_state.json" ]; then
        cargo run
    else
        echo "No saved game."
        echo "Start new: ./play.sh new"
    fi
fi 