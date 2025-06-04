#!/bin/bash

# Interstellar Command Server Launcher

echo "==================================="
echo "   INTERSTELLAR COMMAND SERVER"
echo "==================================="

cd backend

# Check if release binary exists
if [ ! -f "target/release/interstellar-backend" ]; then
    echo "Building server (first time setup)..."
    cargo build --release
    if [ $? -ne 0 ]; then
        echo "Build failed! Make sure Rust is installed."
        exit 1
    fi
    echo "Build complete!"
fi

echo "Starting server on port 8080..."
echo "Access the game at http://localhost:8080"
echo "Or http://[your-ip]:8080 from other devices"
echo ""
echo "Press Ctrl+C to stop the server"
echo "-----------------------------------"

# Run the server
./target/release/interstellar-backend 