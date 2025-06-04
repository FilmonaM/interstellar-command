#!/bin/bash

# Quick update script for Interstellar Command

echo "Updating Interstellar Command..."

# Pull latest changes
git pull origin main

# Build the new version
cd backend
echo "Building new version..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

cd ..
echo "Update complete! Now run ./run.sh to start the server" 