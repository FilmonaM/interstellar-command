#!/usr/bin/env python3
"""
Simulation of new AP-based Interstellar Command gameplay
"""

print("Welcome to Interstellar Command")
print("A point-based strategy game inspired by Red Rising\n")
print("Starting new game...\n")

print("Player 1, enter your name: Cassia")
print("Player 2, enter your name: Darrow")

print("\n--- Turn 1: Captain Cassia ---")
print("\nHealth: 100   AP: 25/25   Current Sector: Earth Orbit")
print("Controlled Sectors: ")

print("\nChoose an action (AP remaining: 25)")
print("  1) Move Fleet (cost 5 AP)")
print("  2) Attack Enemy (cost 8 AP)")
print("  3) Scan Sector (cost 3 AP)")
print("  4) Build Outpost (cost 10 AP)")
print("  5) End Turn")
print("  6) Quit Game")
print("Enter action number: 3")

print("\nSelect sector to scan:")
print("  0 - Earth Orbit (current)")
print("  1 - Mars Orbit")
print("Enter sector ID to scan: 1")

print("\nScan results for Mars Orbit:")
print("  WARNING: Enemy fleet detected!")
print("  Status: Unclaimed")
print("Darrow scanned Mars Orbit")

print("\nHealth: 100   AP: 22/25   Current Sector: Earth Orbit")
print("\nChoose an action (AP remaining: 22)")
print("Enter action number: 1")

print("\nCurrent location: Earth Orbit")
print("Adjacent sectors:")
print("  1 - Mars Orbit")
print("Enter target sector ID (or -1 to cancel): 1")

print("Moved fleet to Mars Orbit")
print("Cassia moved fleet to Mars Orbit")

print("\nHealth: 100   AP: 17/25   Current Sector: Mars Orbit")
print("\nChoose an action (AP remaining: 17)")
print("Enter action number: 2")

print("Cassia attacks Darrow for 10 damage!")
print("Darrow's health: 90")

print("\nAI: The Martian Senate declares neutrality in recent skirmishes.")

print("\n--- Turn 1: Captain Darrow ---")
print("\nHealth: 90   AP: 25/25   Current Sector: Mars Orbit")
print("Controlled Sectors: ")

print("\nChoose an action (AP remaining: 25)")
print("Enter action number: 2")

print("Darrow attacks Cassia for 10 damage!")
print("Cassia's health: 90")

print("\nHealth: 90   AP: 17/25   Current Sector: Mars Orbit")
print("\nChoose an action (AP remaining: 17)")
print("Enter action number: 1")

print("\nCurrent location: Mars Orbit")
print("Adjacent sectors:")
print("  0 - Earth Orbit")
print("  2 - Asteroid Belt")
print("  3 - Venus Orbit")
print("Enter target sector ID: 3")

print("Moved fleet to Venus Orbit")
print("You have captured Venus Orbit!")

print("\nHealth: 90   AP: 12/25   Current Sector: Venus Orbit")
print("Controlled Sectors: Venus Orbit")

print("\nChoose an action (AP remaining: 12)")
print("Enter action number: 4")

print("Outpost built in Venus Orbit!")
print("Darrow built an outpost in Venus Orbit")

print("\nAI: A solar flare disrupts communications on Venus.")

print("\n... Game continues ...")
print("\n--- Turn 5: Captain Cassia ---")
print("[System] Cassia AP cap increased to 30!")
print("[System] Darrow AP cap increased to 30!")

print("\nHealth: 50   AP: 30/30   Current Sector: Asteroid Belt")
print("Controlled Sectors: Earth Orbit, Asteroid Belt (with outpost)")

print("\n... Several turns later ...")

print("\nDarrow attacks Cassia for 10 damage!")
print("Cassia's health: 0")
print("\nCassia has been eliminated! Darrow wins!")

print("\n=== GAME OVER ===")
print("Game ended on turn 12")
print("\nEvent Log:")
print("  - The solar empire awaits conquest...")
print("  - Cassia scanned Mars Orbit")
print("  - Cassia moved fleet to Mars Orbit") 
print("  - Cassia engaged Darrow in combat at Mars Orbit")
print("  - AI: The Martian Senate declares neutrality in recent skirmishes.")
print("  - Darrow engaged Cassia in combat at Mars Orbit")
print("  - Darrow moved fleet to Venus Orbit")
print("  - Darrow captured Venus Orbit")
print("  - Darrow built an outpost in Venus Orbit")
print("  - AI: A solar flare disrupts communications on Venus.")
print("  - ... (more events)")
print("  - Darrow destroyed Cassia's fleet. Victory!") 