#!/bin/bash

echo "=== Fixing all compilation errors ==="

# Navigate to the game directory
cd /home/gameplayer/interstellar-command

# Fix 1: Ensure the getter method exists in player.rs
echo "Fixing player.rs..."
if ! grep -q "pub fn get_password_hash" src/core/player.rs; then
    # Add the getter method before the last closing brace
    sed -i '/^}$/i\    pub fn get_password_hash(&self) -> Option<&String> {\n        self.password_hash.as_ref()\n    }' src/core/player.rs
fi

# Fix 2: Replace all direct password_hash access with getter in visualization.rs
echo "Fixing visualization.rs password access..."
sed -i 's/player\.password_hash\.as_ref()/player.get_password_hash()/g' src/visualization.rs

# Fix 3: Fix the self vs Self issue
echo "Fixing self vs Self in visualization.rs..."
sed -i 's/self\.generate_connections_html/Self::generate_connections_html/g' src/visualization.rs
sed -i 's/self\.generate_sectors_html/Self::generate_sectors_html/g' src/visualization.rs
sed -i 's/self\.generate_fleets_html/Self::generate_fleets_html/g' src/visualization.rs

# Fix 4: Add proper type annotations for f64
echo "Fixing f64 type issues..."
# Fix the positions array
sed -i 's/(200\.0, 400\.0)/(200.0_f64, 400.0_f64)/g' src/visualization.rs
sed -i 's/(500\.0, 400\.0)/(500.0_f64, 400.0_f64)/g' src/visualization.rs
sed -i 's/(800\.0, 400\.0)/(800.0_f64, 400.0_f64)/g' src/visualization.rs
sed -i 's/(500\.0, 600\.0)/(500.0_f64, 600.0_f64)/g' src/visualization.rs
sed -i 's/(1000\.0, 400\.0)/(1000.0_f64, 400.0_f64)/g' src/visualization.rs

# Fix the method signatures to remove &self where not needed
echo "Fixing method signatures..."
sed -i 's/fn generate_connections_html(&self, state: &GameState)/fn generate_connections_html(state: \&GameState)/g' src/visualization.rs
sed -i 's/fn generate_sectors_html(&self, state: &GameState)/fn generate_sectors_html(state: \&GameState)/g' src/visualization.rs
sed -i 's/fn generate_fleets_html(&self, state: &GameState)/fn generate_fleets_html(state: \&GameState)/g' src/visualization.rs

# Fix 5: Remove unused imports
echo "Cleaning up unused imports..."
sed -i '/^use crate::core::player::Player;$/d' src/visualization.rs
sed -i '/^use crate::map::sector::Sector;$/d' src/visualization.rs
sed -i '/^use std::io::Write;$/d' src/visualization.rs

# Add back only if needed
if ! grep -q "use std::f64;" src/visualization.rs; then
    sed -i '5i use std::f64;' src/visualization.rs
fi

# Clean and rebuild
echo "Cleaning previous build..."
cargo clean

echo "Building with release mode..."
cargo build --release

echo "=== Build complete! ==="
if [ $? -eq 0 ]; then
    echo "✓ Build successful!"
    echo "Run the game with: ./target/release/interstellar_command"
else
    echo "✗ Build failed. Check the error messages above."
fi 