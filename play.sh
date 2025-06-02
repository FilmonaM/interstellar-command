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
        # Extract turn info from JSON using grep and sed
        TURN=$(grep -o '"turn_number":[0-9]*' game_state.json | sed 's/.*://')
        CURRENT=$(grep -o '"current_player":[0-9]*' game_state.json | sed 's/.*://')
        
        # Try to get player names
        PLAYER1=$(grep -o '"name":"[^"]*"' game_state.json | head -1 | sed 's/.*:"\(.*\)"/\1/')
        PLAYER2=$(grep -o '"name":"[^"]*"' game_state.json | head -2 | tail -1 | sed 's/.*:"\(.*\)"/\1/')
        
        echo ""
        echo "GAME STATUS"
        echo "-----------"
        echo "Turn: $TURN"
        
        if [ "$CURRENT" = "0" ]; then
            echo "Waiting for: $PLAYER1 to take their turn"
        else
            echo "Waiting for: $PLAYER2 to take their turn"
        fi
        
        # Check turn phase if available
        PHASE=$(grep -o '"phase":"[^"]*"' game_state.json | tail -1 | sed 's/.*:"\(.*\)"/\1/')
        if [ ! -z "$PHASE" ]; then
            echo "Turn phase: $PHASE"
        fi
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