use serde::{Serialize, Deserialize};
use crate::core::player::Player;
use crate::core::turn::{TurnManager, TurnPhase};
use crate::map::sector::Sector;
use crate::{ai, persistence};
use crate::visualization::Visualizer;
use std::io::{self, Write};
use std::fs;

// Action costs in AP
const MOVE_COST: u8 = 5;
const ATTACK_COST: u8 = 8;
const SCAN_COST: u8 = 3;
const BUILD_COST: u8 = 10;
const REINFORCE_COST: u8 = 15;  // Level 3+ ability
const SABOTAGE_COST: u8 = 12;   // Level 5+ ability
const ORBITAL_STRIKE_COST: u8 = 20; // Level 7+ ability

// Experience rewards
const XP_MOVE: u32 = 10;
const XP_ATTACK: u32 = 25;
const XP_SCAN: u32 = 5;
const XP_BUILD: u32 = 30;
const XP_CAPTURE: u32 = 50;
const XP_KILL_BONUS: u32 = 200;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub players: [Player; 2],      // two players
    pub sectors: Vec<Sector>,      // entire solar system map
    pub turn_number: u32,          // counts each round (starting at 1)
    pub current_player: u8,        // index 0 or 1
    pub game_over: bool,           // true once someone wins or quits
    pub event_log: Vec<String>,    // AI flavor text entries per turn
    pub turn_manager: TurnManager, // NEW: Manages asynchronous turns
}

impl GameState {
    pub fn new() -> Self {
        println!("\n{}", "+".to_string() + &"=".repeat(68) + "+");
        println!("|{:^68}|", "INTERSTELLAR COMMAND");
        println!("|{:^68}|", "A Terminal Strategy Game");
        println!("+{}+\n", "=".repeat(68));
        
        // Choose map size
        println!("Select map size:");
        println!("1) Tactical (8 sectors) - Quick game");
        println!("2) Strategic (17 sectors) - Full game");
        print!("\nChoice: ");
        io::stdout().flush().unwrap();
        
        let mut map_choice = String::new();
        io::stdin().read_line(&mut map_choice).unwrap();
        
        let sectors = match map_choice.trim() {
            "1" => {
                println!("\nLoading Tactical Map...");
                crate::map::galaxy::Galaxy::create_tactical_map()
            }
            _ => {
                println!("\nLoading Strategic Map...");
                crate::map::galaxy::Galaxy::create_default_map()
            }
        };
        
        let map_size = sectors.len();
        
        // Prompt for player names and passwords
        print!("\nPlayer 1, enter your name: ");
        io::stdout().flush().unwrap();
        let mut player1_name = String::new();
        io::stdin().read_line(&mut player1_name).unwrap();
        let player1_name = player1_name.trim().to_string();
        
        print!("Player 1, set a password (or press Enter for none): ");
        io::stdout().flush().unwrap();
        let player1_pass = match rpassword::read_password() {
            Ok(pass) => pass,
            Err(_) => String::new(),
        };
        
        println!(); // Space between players
        
        print!("Player 2, enter your name: ");
        io::stdout().flush().unwrap();
        let mut player2_name = String::new();
        io::stdin().read_line(&mut player2_name).unwrap();
        let player2_name = player2_name.trim().to_string();
        
        print!("Player 2, set a password (or press Enter for none): ");
        io::stdout().flush().unwrap();
        let player2_pass = match rpassword::read_password() {
            Ok(pass) => pass,
            Err(_) => String::new(),
        };
        
        // Determine starting positions based on map size
        let (start1, start2) = if map_size <= 8 {
            (0, 7)  // Opposite corners for tactical map
        } else {
            (0, 16) // Sol System vs Deep Space Relay for strategic map
        };
        
        // Create players starting in different sectors
        let mut player1 = Player::new(0, player1_name, start1);
        let mut player2 = Player::new(1, player2_name, start2);
        
        // Set passwords if provided
        if !player1_pass.is_empty() {
            player1.set_password(&player1_pass);
            println!("Password set for Player 1");
        }
        if !player2_pass.is_empty() {
            player2.set_password(&player2_pass);
            println!("Password set for Player 2");
        }
        
        let players = [player1, player2];
        
        println!("\n{}", "-".repeat(70));
        println!("Starting new campaign...");
        println!("- {} begins at {}", players[0].name, sectors[start1 as usize].name);
        println!("- {} begins at {}", players[1].name, sectors[start2 as usize].name);
        println!("{}\n", "-".repeat(70));
        
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        GameState {
            players,
            sectors,
            turn_number: 1,
            current_player: 0,
            game_over: false,
            event_log: vec!["The solar empire awaits conquest...".to_string()],
            turn_manager: TurnManager::new(),
        }
    }
    
    pub fn new_with_names(player1_name: String, player2_name: String) -> Self {
        println!("\n{}", "+".to_string() + &"=".repeat(68) + "+");
        println!("|{:^68}|", "INTERSTELLAR COMMAND");
        println!("|{:^68}|", "A Terminal Strategy Game");
        println!("+{}+\n", "=".repeat(68));
        
        // Choose map size
        println!("Select map size:");
        println!("1) Tactical (8 sectors) - Quick game");
        println!("2) Strategic (17 sectors) - Full game");
        print!("\nChoice: ");
        io::stdout().flush().unwrap();
        
        let mut map_choice = String::new();
        io::stdin().read_line(&mut map_choice).unwrap();
        
        let sectors = match map_choice.trim() {
            "1" => {
                println!("\nLoading Tactical Map...");
                crate::map::galaxy::Galaxy::create_tactical_map()
            }
            _ => {
                println!("\nLoading Strategic Map...");
                crate::map::galaxy::Galaxy::create_default_map()
            }
        };
        
        let map_size = sectors.len();
        
        // Determine starting positions based on map size
        let (start1, start2) = if map_size <= 8 {
            (0, 7)  // Opposite corners for tactical map
        } else {
            (0, 16) // Sol System vs Deep Space Relay for strategic map
        };
        
        // Create players starting in different sectors
        let player1 = Player::new(0, player1_name, start1);
        let player2 = Player::new(1, player2_name, start2);
        
        let players = [player1, player2];
        
        GameState {
            players,
            sectors,
            turn_number: 1,
            current_player: 0,
            game_over: false,
            event_log: vec!["The solar empire awaits conquest...".to_string()],
            turn_manager: TurnManager::new(),
        }
    }
    
    pub fn calculate_sector_distance(&self, from: u8, to: u8) -> u8 {
        if from == to {
            return 0;
        }
        
        // Simple BFS to find shortest path
        use std::collections::{VecDeque, HashSet};
        
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        
        queue.push_back((from, 0));
        visited.insert(from);
        
        while let Some((current, distance)) = queue.pop_front() {
            if current == to {
                return distance;
            }
            
            let sector = &self.sectors[current as usize];
            for &adjacent in &sector.adjacent {
                if !visited.contains(&adjacent) {
                    visited.insert(adjacent);
                    queue.push_back((adjacent, distance + 1));
                }
            }
        }
        
        // If no path found, return maximum distance
        u8::MAX
    }
    
    pub fn run_single_turn(&mut self) -> Result<(), String> {
        // Get current player ID
        let player_id = self.turn_manager.current_turn.active_player;
        
        // Check if this player can act
        if !self.turn_manager.can_player_act(player_id) {
            return Err("It's not your turn!".to_string());
        }
        
        // If turn hasn't started yet, start it
        if self.turn_manager.current_turn.phase == TurnPhase::WaitingForPlayer {
            self.turn_manager.current_turn.start();
            self.players[player_id as usize].reset_ap();
        }
        
        Ok(())
    }
    
    pub fn run_turn_loop(&mut self) {
        // Clear screen for better visual experience
        if cfg!(target_os = "windows") {
            let _ = std::process::Command::new("cmd").args(&["/C", "cls"]).status();
        } else {
            let _ = std::process::Command::new("clear").status();
        }
        
        while !self.game_over {
            let pid = self.current_player as usize;
            
            // New turn banner
            println!("\n{}", "+".to_string() + &"=".repeat(68) + "+");
            println!("|{:^68}|", format!("TURN {} - {} PHASE", self.turn_number, 
                if pid == 0 { "PLAYER 1" } else { "PLAYER 2" }));
            println!("+{}+\n", "=".repeat(68));
            
            // Password verification for turn start
            if self.turn_number > 1 {  // Skip password on first turn since they just set it
                println!("+-- AUTHENTICATION ------------------------------------------------+");
                
                let mut authenticated = false;
                for attempt in 1..=2 {
                    print!("| {}, enter password (attempt {}/2): ", self.players[pid].name, attempt);
                    io::stdout().flush().unwrap();
                    let password = match rpassword::read_password() {
                        Ok(pass) => pass,
                        Err(_) => {
                            println!("| Error reading password. Try again.                               |");
                            continue;
                        }
                    };
                    
                    if self.players[pid].verify_password(&password) {
                        println!("| [OK] Authentication successful!                                  |");
                        authenticated = true;
                        break;
                    } else if attempt == 1 {
                        println!("| Incorrect password. One more attempt remaining.                 |");
                    }
                }
                
                if !authenticated {
                    println!("| Authentication failed! Turn forfeited.                          |");
                    println!("+-----------------------------------------------------------------+");
                    println!("\nPassing control to opponent in 3 seconds...");
                    self.current_player = 1 - self.current_player;
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    
                    // Clear screen for next player
                    if cfg!(target_os = "windows") {
                        let _ = std::process::Command::new("cmd").args(&["/C", "cls"]).status();
                    } else {
                        let _ = std::process::Command::new("clear").status();
                    }
                    continue;
                }
                
                println!("+-----------------------------------------------------------------+\n");
            }
            
            self.players[pid].reset_ap();
            
            // Player overview section
            println!("+-- COMMANDER OVERVIEW --------------------------------------------+");
            println!("| {} {} (Level {})                                          |", 
                self.players[pid].rank, 
                self.players[pid].name,
                self.players[pid].level
            );
            println!("+-----------------------------------------------------------------+");
            
            // Show map
            println!("\n▼ STRATEGIC MAP ▼");
            println!("{}", Visualizer::generate_map(self));
            
            // Action phase
            loop {
                println!("\n{}", "-".repeat(70));
                self.display_status();
                println!("{}", "-".repeat(70));
                self.display_menu();
                
                print!("\nSelect action: ");
                io::stdout().flush().unwrap();
                let mut choice = String::new();
                io::stdin().read_line(&mut choice).unwrap();
                
                println!(); // Add space after input
                
                let should_continue = match choice.trim() {
                    "1" => { self.move_action(pid); true }
                    "2" => { self.attack_action(pid); true }
                    "3" => { self.scan_action(pid); true }
                    "4" => { self.build_action(pid); true }
                    "5" => { self.reinforce_action(pid); true }
                    "6" => { self.sabotage_action(pid); true }
                    "7" => { self.orbital_strike_action(pid); true }
                    "8" => { self.view_dashboard(pid); true }
                    "9" => { self.export_views(); true }
                    "10" => { self.open_interactive_map(); true }
                    "11" => { self.open_instructions(); true }
                    "12" => {
                        println!("Ending turn. {} AP will be available next turn.", 
                            self.players[pid].ap_remaining);
                        false
                    }
                    "13" => {
                        self.game_over = true;
                        println!("{} has forfeited the match!", self.players[pid].name);
                        self.event_log.push(format!("{} {} abandoned the campaign.", 
                            self.players[pid].rank, self.players[pid].name));
                        false
                    }
                    _ => {
                        println!("Invalid selection. Choose a number from the menu.");
                        true
                    }
                };
                
                if self.players[pid].ap_remaining == 0 && should_continue {
                    println!("\nAction Points depleted!");
                    println!("Turn will end in 3 seconds...");
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    break;
                }
                
                if !should_continue || self.game_over {
                    break;
                }
            }
            
            // End-of-turn resolution
            if !self.game_over {
                println!("\n{}", "+".to_string() + &"=".repeat(68) + "+");
                println!("|{:^68}|", "TURN SUMMARY");
                println!("+{}+", "=".repeat(68));
                
                // Check for level up
                let old_level = self.players[pid].level;
                self.players[pid].check_level_up();
                if self.players[pid].level > old_level {
                    println!("\nPROMOTION!");
                    println!("{} has been promoted to Level {} - {}!", 
                        self.players[pid].name, 
                        self.players[pid].level,
                        self.players[pid].rank
                    );
                    self.event_log.push(format!("{} achieved rank of {}!", 
                        self.players[pid].name, self.players[pid].rank));
                }
                
                // Optional AI event (only if available and not disabled)
                if !std::env::var("DISABLE_AI").is_ok() {
                    let ai_event = ai::generate_event(self);
                    if !ai_event.is_empty() {
                        println!("\nSTELLAR NEWS NETWORK");
                        println!("\"{}\"", ai_event);
                        self.event_log.push(ai_event);
                    }
                }
                
                // Save game
                print!("\nSaving campaign progress...");
                io::stdout().flush().unwrap();
                match persistence::save(self) {
                    Ok(_) => println!(" Success!"),
                    Err(e) => println!(" Warning: Save failed - {}", e),
                }
                
                self.turn_number += 1;
                self.current_player = 1 - self.current_player;
                
                println!("\n{}", "-".repeat(70));
                println!("Turn complete. Prepare for player changeover.");
                println!("Press Enter when {} is ready...", 
                    self.players[self.current_player as usize].name);
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                
                // Clear screen for next player
                if cfg!(target_os = "windows") {
                    let _ = std::process::Command::new("cmd").args(&["/C", "cls"]).status();
                } else {
                    let _ = std::process::Command::new("clear").status();
                }
            }
        }
        
        // Game over
        println!("\n{}", "+".to_string() + &"=".repeat(68) + "+");
        println!("|{:^68}|", "CAMPAIGN CONCLUDED");
        println!("+{}+", "=".repeat(68));
        println!("\nFinal Turn: {}", self.turn_number);
        
        // Show final comparison
        println!("{}", Visualizer::generate_stats_comparison(self));
        
        println!("\nCAMPAIGN CHRONICLE");
        println!("{}", "-".repeat(70));
        for (i, event) in self.event_log.iter().enumerate() {
            println!("Turn {:2}: {}", i + 1, event);
        }
        
        // Export final views
        println!("\n{}", "-".repeat(70));
        println!("Generating final reports...");
        self.export_views();
        
        println!("\n{}", "-".repeat(70));
        println!("Thank you for playing Interstellar Command!");
        println!("Your campaign data has been preserved.");
        println!("{}", "-".repeat(70));
    }
    
    fn display_status(&self) {
        let player = &self.players[self.current_player as usize];
        let sector = &self.sectors[player.current_sector as usize];
        
        println!("+-- STATUS --------------------------------------------------------+");
        println!("| Health: {:3} HP    AP: {:2}/{}    XP: {:3}/{} (Level {})          |", 
            player.health, 
            player.ap_remaining, 
            player.ap_cap, 
            player.experience,
            player.level as u32 * 100,
            player.level
        );
        println!("| Location: {:25} Rank: {:15} |", 
            sector.name,
            player.rank
        );
        println!("+-----------------------------------------------------------------+");
        
        // Show fleet composition
        println!("\n+-- FLEET COMPOSITION ---------------------------------------------+");
        println!("| Scouts: {}  Frigates: {}  Destroyers: {}  Command Centers: {}        |",
            player.fleet.scouts, player.fleet.frigates, 
            player.fleet.destroyers, player.fleet.command_centers
        );
        println!("| Total Ships: {}    Combat Strength: {}                           |",
            player.fleet.total_ships(), player.fleet.combat_strength()
        );
        if !player.can_capture_sector() {
            println!("| [!] Need Command Center to capture sectors (unlocks at Level 4) |");
        }
        println!("+-----------------------------------------------------------------+");
        
        // Show controlled sectors
        let controlled: Vec<String> = self.sectors.iter()
            .filter(|s| s.owner == Some(self.current_player))
            .map(|s| {
                if s.has_outpost {
                    format!("{} [OUT]", s.name)
                } else {
                    s.name.clone()
                }
            })
            .collect();
        
        if !controlled.is_empty() {
            println!("\nControlled Territories: {}", controlled.join(" • "));
        }
    }
    
    fn display_menu(&self) {
        let player = &self.players[self.current_player as usize];
        
        println!("\n+-- COMMAND OPTIONS -----------------------------------------------+");
        println!("| AP Remaining: {:2}                                                 |", player.ap_remaining);
        println!("+-----------------------------------------------------------------+");
        println!("|  1) Move Fleet .................. {} AP                          |", MOVE_COST);
        println!("|  2) Attack Enemy ................ {} AP (deals {} damage)        |", 
            ATTACK_COST, 10 + player.get_damage_bonus());
        println!("|  3) Scan Sector ................. {} AP                          |", SCAN_COST);
        println!("|  4) Build Outpost ............... {} AP                         |", BUILD_COST);
        
        if player.level >= 3 {
            println!("|  5) Reinforce Fleet ............. {} AP (heals 20 HP)          |", REINFORCE_COST);
        }
        if player.level >= 5 {
            println!("|  6) Sabotage Outpost ............ {} AP                         |", SABOTAGE_COST);
        }
        if player.level >= 7 {
            println!("|  7) Orbital Strike .............. {} AP (30 damage, ranged)    |", ORBITAL_STRIKE_COST);
        }
        
        println!("+-----------------------------------------------------------------+");
        println!("|  8) View Dashboard .............. FREE                           |");
        println!("|  9) Export Game Data ............ FREE                           |");
        println!("| 10) Open Interactive Map ........ FREE (opens in browser)       |");
        println!("| 11) View Game Manual ............ FREE (opens in browser)       |");
        println!("| 12) End Turn .................... Saves remaining AP             |");
        println!("| 13) Quit Game ................... Forfeit the match              |");
        println!("+-----------------------------------------------------------------+");
    }
    
    fn view_dashboard(&self, pid: usize) {
        println!("{}", Visualizer::generate_player_view(self, pid as u8));
        println!("\nPress Enter to continue...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
    
    fn export_views(&self) {
        println!("\nExporting player views...");
        
        // Export HTML views
        match Visualizer::export_player_html(self, 0) {
            Ok(filename) => println!("Player 1 view exported to: {}", filename),
            Err(e) => eprintln!("Failed to export Player 1 view: {}", e),
        }
        
        match Visualizer::export_player_html(self, 1) {
            Ok(filename) => println!("Player 2 view exported to: {}", filename),
            Err(e) => eprintln!("Failed to export Player 2 view: {}", e),
        }
        
        // Show comparison
        println!("{}", Visualizer::generate_stats_comparison(self));
    }
    
    fn move_action(&mut self, pid: usize) {
        if !self.players[pid].can_perform(MOVE_COST) {
            println!("Not enough AP for Move (need {} AP)", MOVE_COST);
            return;
        }
        
        let current_sector_id = self.players[pid].current_sector;
        let current_sector = &self.sectors[current_sector_id as usize];
        
        println!("\nCurrent location: {}", current_sector.name);
        println!("Adjacent sectors:");
        for &adj_id in &current_sector.adjacent {
            let adj_sector = &self.sectors[adj_id as usize];
            println!("  {} - {}", adj_id, adj_sector.name);
        }
        
        print!("Enter target sector ID (or -1 to cancel): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if let Ok(target_id) = input.trim().parse::<i8>() {
            if target_id == -1 {
                println!("Move cancelled.");
                return;
            }
            
            let target_id = target_id as u8;
            if current_sector.is_adjacent(target_id) {
                self.players[pid].current_sector = target_id;
                self.players[pid].ap_remaining -= MOVE_COST;
                self.players[pid].gain_experience(XP_MOVE);
                
                let target_name = self.sectors[target_id as usize].name.clone();
                println!("Fleet moved to {} (+{} XP)", target_name, XP_MOVE);
                self.event_log.push(format!("{} {} moved fleet to {}", 
                    self.players[pid].rank, self.players[pid].name, target_name));
                
                // Mark sector as visible
                if !self.sectors[target_id as usize].visible_to.contains(&(pid as u8)) {
                    self.sectors[target_id as usize].visible_to.push(pid as u8);
                }
                
                // Auto-capture if unowned AND player has command center
                if self.sectors[target_id as usize].owner.is_none() {
                    if self.players[pid].can_capture_sector() {
                        self.sectors[target_id as usize].capture(pid as u8);
                        self.players[pid].gain_experience(XP_CAPTURE);
                        println!("Sector captured! (+{} XP)", XP_CAPTURE);
                        println!("Your Command Center has established control.");
                        self.event_log.push(format!("{} {} captured {}", 
                            self.players[pid].rank, self.players[pid].name, target_name));
                    } else {
                        println!("Cannot capture sector - no Command Center in fleet!");
                        println!("(Command Centers unlock at Level 4)");
                    }
                }
            } else {
                println!("Cannot move there - sector {} is not adjacent.", target_id);
            }
        } else {
            println!("Invalid input.");
        }
    }
    
    fn attack_action(&mut self, pid: usize) {
        if !self.players[pid].can_perform(ATTACK_COST) {
            println!("Not enough AP for Attack (need {} AP)", ATTACK_COST);
            return;
        }
        
        let current_sector = self.players[pid].current_sector;
        let enemy_id = 1 - pid;
        
        if self.players[enemy_id].current_sector == current_sector {
            self.players[pid].ap_remaining -= ATTACK_COST;
            self.players[pid].gain_experience(XP_ATTACK);
            
            let base_damage = 10;
            let damage_bonus = self.players[pid].get_damage_bonus();
            let total_damage = base_damage + damage_bonus;
            
            self.players[enemy_id].take_damage(total_damage);
            
            let attacker_name = format!("{} {}", self.players[pid].rank, self.players[pid].name);
            let target_name = format!("{} {}", self.players[enemy_id].rank, self.players[enemy_id].name);
            let sector_name = self.sectors[current_sector as usize].name.clone();
            
            println!("{} attacks {} for {} damage! (+{} XP)", 
                attacker_name, target_name, total_damage, XP_ATTACK);
            if damage_bonus > 0 {
                println!("  (Base: {} + Level bonus: {})", base_damage, damage_bonus);
            }
            println!("{}'s health: {}", target_name, self.players[enemy_id].health);
            
            self.event_log.push(format!("{} engaged {} in combat at {}", 
                attacker_name, target_name, sector_name));
            
            if !self.players[enemy_id].is_alive() {
                self.game_over = true;
                self.players[pid].gain_experience(XP_KILL_BONUS);
                println!("\n{} has been eliminated! {} wins! (+{} XP)", 
                    target_name, attacker_name, XP_KILL_BONUS);
                self.event_log.push(format!("{} destroyed {}'s fleet. Victory!", 
                    attacker_name, target_name));
            }
        } else {
            println!("No enemy fleet in this sector.");
        }
    }
    
    fn scan_action(&mut self, pid: usize) {
        if !self.players[pid].can_perform(SCAN_COST) {
            println!("Not enough AP for Scan (need {} AP)", SCAN_COST);
            return;
        }
        
        let current_sector_id = self.players[pid].current_sector;
        let current_sector = &self.sectors[current_sector_id as usize];
        let scan_bonus = self.players[pid].get_scan_range_bonus();
        
        println!("\nSelect sector to scan:");
        println!("  {} - {} (current)", current_sector_id, current_sector.name);
        
        // Show adjacent sectors
        for &adj_id in &current_sector.adjacent {
            let adj_sector = &self.sectors[adj_id as usize];
            println!("  {} - {}", adj_id, adj_sector.name);
        }
        
        // Level 5+ can scan further
        if scan_bonus > 0 {
            println!("\n[Extended Range - Level 5+ bonus]:");
            for &adj_id in &current_sector.adjacent {
                let adj_sector = &self.sectors[adj_id as usize];
                for &far_id in &adj_sector.adjacent {
                    if far_id != current_sector_id && !current_sector.adjacent.contains(&far_id) {
                        let far_sector = &self.sectors[far_id as usize];
                        println!("  {} - {} (extended range)", far_id, far_sector.name);
                    }
                }
            }
        }
        
        print!("Enter sector ID to scan: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if let Ok(scan_id) = input.trim().parse::<u8>() {
            let can_scan = scan_id == current_sector_id || 
                          current_sector.is_adjacent(scan_id) ||
                          (scan_bonus > 0 && self.can_scan_extended(current_sector_id, scan_id));
                          
            if can_scan {
                self.players[pid].ap_remaining -= SCAN_COST;
                self.players[pid].gain_experience(XP_SCAN);
                
                // Mark as visible
                if !self.sectors[scan_id as usize].visible_to.contains(&(pid as u8)) {
                    self.sectors[scan_id as usize].visible_to.push(pid as u8);
                }
                
                let scanned_sector = &self.sectors[scan_id as usize];
                println!("\nScan results for {}: (+{} XP)", scanned_sector.name, XP_SCAN);
                
                // Check for enemy presence
                let enemy_id = 1 - pid;
                if self.players[enemy_id].current_sector == scan_id {
                    println!("  WARNING: Enemy fleet detected!");
                } else {
                    println!("  No enemy presence detected.");
                }
                
                // Show ownership
                match scanned_sector.owner {
                    Some(owner) if owner == pid as u8 => {
                        println!("  Status: Under your control");
                        if scanned_sector.has_outpost {
                            println!("  Outpost: Present");
                        }
                    }
                    Some(_) => {
                        println!("  Status: Enemy controlled");
                        if scanned_sector.has_outpost {
                            println!("  Enemy Outpost: Detected");
                        }
                    }
                    None => println!("  Status: Unclaimed"),
                }
                
                self.event_log.push(format!("{} {} scanned {}", 
                    self.players[pid].rank, self.players[pid].name, scanned_sector.name));
            } else {
                println!("Cannot scan that sector - it's out of range.");
            }
        } else {
            println!("Invalid input.");
        }
    }
    
    fn build_action(&mut self, pid: usize) {
        if !self.players[pid].can_perform(BUILD_COST) {
            println!("Not enough AP for Build (need {} AP)", BUILD_COST);
            return;
        }
        
        let current_sector = self.players[pid].current_sector as usize;
        
        if self.sectors[current_sector].owner == Some(pid as u8) {
            if self.sectors[current_sector].has_outpost {
                println!("This sector already has an outpost.");
            } else {
                self.players[pid].ap_remaining -= BUILD_COST;
                self.players[pid].gain_experience(XP_BUILD);
                self.sectors[current_sector].has_outpost = true;
                
                let sector_name = self.sectors[current_sector].name.clone();
                println!("Outpost built in {}! (+{} XP)", sector_name, XP_BUILD);
                self.event_log.push(format!("{} {} built an outpost in {}", 
                    self.players[pid].rank, self.players[pid].name, sector_name));
            }
        } else {
            println!("Cannot build here - you don't control this sector.");
        }
    }
    
    fn reinforce_action(&mut self, pid: usize) {
        if self.players[pid].level < 3 {
            println!("Reinforce requires Level 3+");
            return;
        }
        
        if !self.players[pid].can_perform(REINFORCE_COST) {
            println!("Not enough AP for Reinforce (need {} AP)", REINFORCE_COST);
            return;
        }
        
        self.players[pid].ap_remaining -= REINFORCE_COST;
        let heal_amount = 20;
        self.players[pid].health += heal_amount;
        if self.players[pid].health > 100 + (self.players[pid].level as i32 - 1) * 20 {
            self.players[pid].health = 100 + (self.players[pid].level as i32 - 1) * 20;
        }
        self.players[pid].gain_experience(15);
        
        println!("Fleet reinforced! Healed {} HP. Current health: {} (+15 XP)", 
            heal_amount, self.players[pid].health);
        self.event_log.push(format!("{} {} reinforced their fleet", 
            self.players[pid].rank, self.players[pid].name));
    }
    
    fn sabotage_action(&mut self, pid: usize) {
        if self.players[pid].level < 5 {
            println!("Sabotage requires Level 5+");
            return;
        }
        
        if !self.players[pid].can_perform(SABOTAGE_COST) {
            println!("Not enough AP for Sabotage (need {} AP)", SABOTAGE_COST);
            return;
        }
        
        let current_sector = self.players[pid].current_sector as usize;
        let enemy_id = (1 - pid) as u8;
        
        if self.sectors[current_sector].owner == Some(enemy_id) && self.sectors[current_sector].has_outpost {
            self.players[pid].ap_remaining -= SABOTAGE_COST;
            self.players[pid].gain_experience(40);
            self.sectors[current_sector].has_outpost = false;
            
            let sector_name = self.sectors[current_sector].name.clone();
            println!("Enemy outpost in {} destroyed! (+40 XP)", sector_name);
            self.event_log.push(format!("{} {} sabotaged the outpost in {}", 
                self.players[pid].rank, self.players[pid].name, sector_name));
        } else {
            println!("No enemy outpost in this sector to sabotage.");
        }
    }
    
    fn orbital_strike_action(&mut self, pid: usize) {
        if self.players[pid].level < 7 {
            println!("Orbital Strike requires Level 7+");
            return;
        }
        
        if !self.players[pid].can_perform(ORBITAL_STRIKE_COST) {
            println!("Not enough AP for Orbital Strike (need {} AP)", ORBITAL_STRIKE_COST);
            return;
        }
        
        println!("\nSelect target sector for orbital strike:");
        for (i, sector) in self.sectors.iter().enumerate() {
            if self.sectors[i].visible_to.contains(&(pid as u8)) {
                println!("  {} - {}", i, sector.name);
            }
        }
        
        print!("Enter target sector ID: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if let Ok(target_id) = input.trim().parse::<usize>() {
            if target_id < self.sectors.len() && self.sectors[target_id].visible_to.contains(&(pid as u8)) {
                self.players[pid].ap_remaining -= ORBITAL_STRIKE_COST;
                self.players[pid].gain_experience(50);
                
                let enemy_id = 1 - pid;
                if self.players[enemy_id].current_sector == target_id as u8 {
                    self.players[enemy_id].take_damage(30);
                    println!("Orbital strike hit enemy fleet for 30 damage! (+50 XP)");
                    println!("Enemy health: {}", self.players[enemy_id].health);
                    
                    if !self.players[enemy_id].is_alive() {
                        self.game_over = true;
                        self.players[pid].gain_experience(XP_KILL_BONUS);
                        println!("\nEnemy eliminated by orbital strike! You win! (+{} XP)", XP_KILL_BONUS);
                    }
                } else {
                    println!("Orbital strike hit empty space! No damage dealt. (+50 XP)");
                }
                
                self.event_log.push(format!("{} {} launched orbital strike on {}", 
                    self.players[pid].rank, self.players[pid].name, self.sectors[target_id].name));
            } else {
                println!("Invalid target - must be a visible sector.");
            }
        } else {
            println!("Invalid input.");
        }
    }
    
    fn can_scan_extended(&self, from_sector: u8, target_sector: u8) -> bool {
        // Check if target is 2 hops away
        let current = &self.sectors[from_sector as usize];
        for &adj_id in &current.adjacent {
            let adj_sector = &self.sectors[adj_id as usize];
            if adj_sector.adjacent.contains(&target_sector) {
                return true;
            }
        }
        false
    }
    
    fn open_interactive_map(&self) {
        println!("Generating interactive map...");
        
        let html = self.generate_interactive_map_html();
        let filename = "interstellar_map.html";
        
        match fs::write(filename, html) {
            Ok(_) => {
                println!("Map generated successfully!");
                
                // Try to open in default browser
                #[cfg(target_os = "windows")]
                {
                    let _ = std::process::Command::new("cmd")
                        .args(&["/C", "start", filename])
                        .spawn();
                }
                #[cfg(target_os = "macos")]
                {
                    let _ = std::process::Command::new("open")
                        .arg(filename)
                        .spawn();
                }
                #[cfg(target_os = "linux")]
                {
                    let _ = std::process::Command::new("xdg-open")
                        .arg(filename)
                        .spawn();
                }
                
                println!("Map opened in your browser (or open {} manually)", filename);
            }
            Err(e) => println!("Failed to generate map: {}", e),
        }
    }
    
    fn generate_interactive_map_html(&self) -> String {
        let player = &self.players[self.current_player as usize];
        
        format!(r#"<!DOCTYPE html>
<html>
<head>
    <title>Interstellar Command - Strategic Map</title>
    <style>
        body {{
            background: #000;
            color: #0f0;
            font-family: 'Courier New', monospace;
            margin: 0;
            padding: 20px;
            overflow: hidden;
        }}
        .map-container {{
            position: relative;
            width: 100vw;
            height: 100vh;
            background: radial-gradient(circle at center, #001100 0%, #000 100%);
        }}
        .sector {{
            position: absolute;
            width: 120px;
            height: 120px;
            border: 2px solid #0f0;
            border-radius: 50%;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            cursor: pointer;
            transition: all 0.3s;
            background: rgba(0, 255, 0, 0.1);
        }}
        .sector:hover {{
            transform: scale(1.1);
            background: rgba(0, 255, 0, 0.2);
            box-shadow: 0 0 20px #0f0;
        }}
        .sector.owned-0 {{
            border-color: #00f;
            background: rgba(0, 0, 255, 0.2);
        }}
        .sector.owned-1 {{
            border-color: #f00;
            background: rgba(255, 0, 0, 0.2);
        }}
        .sector.current {{
            animation: pulse 2s infinite;
        }}
        @keyframes pulse {{
            0% {{ box-shadow: 0 0 10px currentColor; }}
            50% {{ box-shadow: 0 0 30px currentColor, 0 0 50px currentColor; }}
            100% {{ box-shadow: 0 0 10px currentColor; }}
        }}
        .connection {{
            position: absolute;
            height: 2px;
            background: #0f0;
            opacity: 0.3;
            transform-origin: left center;
        }}
        .sector-name {{
            font-weight: bold;
            font-size: 14px;
        }}
        .sector-info {{
            font-size: 11px;
            margin-top: 5px;
        }}
        .legend {{
            position: fixed;
            top: 20px;
            right: 20px;
            background: rgba(0, 0, 0, 0.8);
            border: 1px solid #0f0;
            padding: 20px;
        }}
        .fleet-icon {{
            font-size: 20px;
            margin: 5px 0;
        }}
        h1 {{
            text-align: center;
            color: #0f0;
            text-shadow: 0 0 10px #0f0;
        }}
    </style>
</head>
<body>
    <h1>INTERSTELLAR COMMAND - TURN {}</h1>
    <div class="map-container" id="map">
        <!-- Connections -->
        <div class="connection" style="left: 250px; top: 259px; width: 200px; transform: rotate(0deg);"></div>
        <div class="connection" style="left: 455px; top: 259px; width: 200px; transform: rotate(0deg);"></div>
        <div class="connection" style="left: 455px; top: 290px; width: 141px; transform: rotate(45deg);"></div>
        <div class="connection" style="left: 660px; top: 259px; width: 200px; transform: rotate(0deg);"></div>
        
        <!-- Sectors -->
        {}</div>
    
    <div class="legend">
        <h3>Commander {}</h3>
        <p>Rank: {}</p>
        <p>Fleet Strength: {}</p>
        <p>Ships: {} total</p>
        <hr>
        <p style="color: #00f;">● Player 1 Territory</p>
        <p style="color: #f00;">● Player 2 Territory</p>
        <p style="color: #0f0;">● Neutral Territory</p>
        <p>[X] Your Location</p>
        <p>[O] Outpost Present</p>
    </div>
    
    <script>
        // Add click handlers for sectors
        document.querySelectorAll('.sector').forEach(sector => {{
            sector.addEventListener('click', function() {{
                const name = this.querySelector('.sector-name').textContent;
                const info = this.dataset.info;
                alert('Sector: ' + name + '\\n\\n' + info);
            }});
        }});
        
        // Animate connections
        let hue = 0;
        setInterval(() => {{
            hue = (hue + 1) % 360;
            document.querySelectorAll('.connection').forEach(conn => {{
                conn.style.filter = `hue-rotate(${{hue}}deg)`;
            }});
        }}, 50);
    </script>
</body>
</html>"#,
            self.turn_number,
            self.generate_sector_html(),
            player.name,
            player.rank,
            player.fleet.combat_strength(),
            player.fleet.total_ships()
        )
    }
    
    fn generate_sector_html(&self) -> String {
        let positions = vec![
            (200, 200), // Earth
            (400, 200), // Mars  
            (600, 200), // Asteroid Belt
            (500, 350), // Venus
            (800, 200), // Jupiter
        ];
        
        let mut html = String::new();
        
        for (i, sector) in self.sectors.iter().enumerate() {
            let (x, y) = positions[i];
            let mut class = "sector".to_string();
            
            if let Some(owner) = sector.owner {
                class.push_str(&format!(" owned-{}", owner));
            }
            
            if self.players[self.current_player as usize].current_sector == i as u8 {
                class.push_str(" current");
            }
            
            let mut info = format!("Sector ID: {}\\n", i);
            if let Some(owner) = sector.owner {
                info.push_str(&format!("Owner: {}\\n", self.players[owner as usize].name));
            } else {
                info.push_str("Owner: None\\n");
            }
            if sector.has_outpost {
                info.push_str("Outpost: Present");
            }
            
            // Check for enemy presence
            let enemy_id = 1 - self.current_player;
            let enemy_here = self.players[enemy_id as usize].current_sector == i as u8;
            
            html.push_str(&format!(
                r#"<div class="{}" style="left: {}px; top: {}px;" data-info="{}">
                    <div class="sector-name">{}</div>
                    <div class="fleet-icon">{}</div>
                    <div class="sector-info">{}</div>
                </div>"#,
                class, x, y, info,
                sector.name,
                if self.players[self.current_player as usize].current_sector == i as u8 { "[X]" }
                else if enemy_here { "[!]" }
                else { "" },
                if sector.has_outpost { "[O]" } else { "" }
            ));
        }
        
        html
    }
    
    fn open_instructions(&self) {
        println!("Opening game manual...");
        
        let manual = self.generate_game_manual();
        let filename = "interstellar_manual.html";
        
        match fs::write(filename, manual) {
            Ok(_) => {
                println!("Manual generated successfully!");
                
                // Try to open in default browser
                #[cfg(target_os = "windows")]
                {
                    let _ = std::process::Command::new("cmd")
                        .args(&["/C", "start", filename])
                        .spawn();
                }
                #[cfg(target_os = "macos")]
                {
                    let _ = std::process::Command::new("open")
                        .arg(filename)
                        .spawn();
                }
                #[cfg(target_os = "linux")]
                {
                    let _ = std::process::Command::new("xdg-open")
                        .arg(filename)
                        .spawn();
                }
                
                println!("Manual opened in your browser (or open {} manually)", filename);
            }
            Err(e) => println!("Failed to generate manual: {}", e),
        }
    }
    
    fn generate_game_manual(&self) -> String {
        format!(r#"<!DOCTYPE html>
<html>
<head>
    <title>Interstellar Command - COMPLETE GAME MANUAL</title>
    <style>
        body {{
            background: #0a0a0a;
            color: #0f0;
            font-family: 'Courier New', monospace;
            padding: 40px;
            max-width: 1000px;
            margin: 0 auto;
            line-height: 1.6;
        }}
        h1, h2, h3 {{
            color: #0f0;
            text-shadow: 0 0 10px #0f0;
        }}
        .section {{
            background: rgba(0, 255, 0, 0.05);
            border: 1px solid #0f0;
            padding: 20px;
            margin: 20px 0;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            margin: 20px 0;
        }}
        th, td {{
            border: 1px solid #0f0;
            padding: 10px;
            text-align: left;
        }}
        th {{
            background: rgba(0, 255, 0, 0.2);
        }}
        .fleet-table {{
            background: rgba(0, 0, 255, 0.1);
        }}
        code {{
            background: #000;
            padding: 2px 5px;
            border: 1px solid #0f0;
        }}
        .tip {{
            background: rgba(255, 255, 0, 0.1);
            border-left: 4px solid #ff0;
            padding: 10px;
            margin: 10px 0;
        }}
    </style>
</head>
<body>
    <h1>INTERSTELLAR COMMAND - COMPLETE GAME MANUAL</h1>
    
    <div class="section">
        <h2>Fleet System & Military Ranks</h2>
        <p>Your fleet grows with your rank! You start with a single Frigate and build your armada as you level up.</p>
        
        <h3>Ship Types</h3>
        <table>
            <tr>
                <th>Ship Type</th>
                <th>Combat Strength</th>
                <th>Special Ability</th>
            </tr>
            <tr>
                <td>Scout</td>
                <td>5</td>
                <td>Extended scan range, fast movement</td>
            </tr>
            <tr>
                <td>Frigate</td>
                <td>10</td>
                <td>Balanced combat vessel</td>
            </tr>
            <tr>
                <td>Destroyer</td>
                <td>20</td>
                <td>Heavy firepower</td>
            </tr>
            <tr>
                <td>Command Center</td>
                <td>15</td>
                <td>Can capture and control sectors</td>
            </tr>
        </table>
        
        <div class="tip">
            <strong>Important:</strong> You cannot capture sectors until Level 4 when you receive your first Command Center!
        </div>
        
        <h3>Fleet Growth by Level</h3>
        <table class="fleet-table">
            <tr>
                <th>Level</th>
                <th>Rank</th>
                <th>New Ships</th>
                <th>Total Fleet</th>
            </tr>
            <tr>
                <td>1</td>
                <td>Legionnaire</td>
                <td>1 Frigate (starter)</td>
                <td>1 ship</td>
            </tr>
            <tr>
                <td>2</td>
                <td>Centurion</td>
                <td>+1 Scout</td>
                <td>2 ships</td>
            </tr>
            <tr>
                <td>3</td>
                <td>Tribune</td>
                <td>+1 Frigate</td>
                <td>3 ships</td>
            </tr>
            <tr>
                <td>4</td>
                <td>Prefect</td>
                <td>+1 Command Center</td>
                <td>4 ships</td>
            </tr>
            <tr>
                <td>5</td>
                <td>Legate</td>
                <td>+1 Destroyer, +1 Scout</td>
                <td>6 ships</td>
            </tr>
            <tr>
                <td>10</td>
                <td>Archsovereign</td>
                <td>Massive fleet bonus!</td>
                <td>20+ ships</td>
            </tr>
        </table>
    </div>
    
    <div class="section">
        <h2>Game Controls</h2>
        <h3>Action Points (AP) System</h3>
        <p>Every action costs AP. Plan your turns carefully!</p>
        <ul>
            <li><strong>Move Fleet:</strong> 5 AP - Relocate to adjacent sector</li>
            <li><strong>Attack Enemy:</strong> 8 AP - Engage hostile fleet</li>
            <li><strong>Scan Sector:</strong> 3 AP - Reveal enemy positions</li>
            <li><strong>Build Outpost:</strong> 10 AP - Fortify controlled sector</li>
            <li><strong>Reinforce (Lv 3+):</strong> 15 AP - Heal 20 HP</li>
            <li><strong>Sabotage (Lv 5+):</strong> 12 AP - Destroy enemy outpost</li>
            <li><strong>Orbital Strike (Lv 7+):</strong> 20 AP - Long-range attack</li>
        </ul>
    </div>
    
    <div class="section">
        <h2>Strategic Map</h2>
        <p>The solar system consists of 5 key sectors:</p>
        <pre>
    [Earth] ---- [Mars] ---- [Asteroid Belt] ---- [Jupiter]
                   |
                [Venus]
        </pre>
        <p>Control the center (Mars) for maximum strategic flexibility!</p>
    </div>
    
    <div class="section">
        <h2>Victory Strategies</h2>
        <h3>Early Game (Levels 1-3)</h3>
        <ul>
            <li>Focus on gaining XP through movement and scanning</li>
            <li>You CANNOT capture sectors yet - just explore!</li>
            <li>Avoid combat until you have health advantage</li>
        </ul>
        
        <h3>Mid Game (Levels 4-6)</h3>
        <ul>
            <li>Level 4 is crucial - you get your first Command Center!</li>
            <li>Start capturing neutral sectors immediately</li>
            <li>Build outposts for defensive advantage</li>
        </ul>
        
        <h3>Late Game (Levels 7-10)</h3>
        <ul>
            <li>Use Orbital Strike to attack from safety</li>
            <li>Your massive fleet provides overwhelming force</li>
            <li>Multiple Command Centers allow rapid expansion</li>
        </ul>
    </div>
    
    <div class="section">
        <h2>AI Integration (Optional)</h2>
        <h3>Using Ollama for Dynamic Events</h3>
        <p>To enable AI-generated events:</p>
        <ol>
            <li>Install Ollama: <code>curl -fsSL https://ollama.ai/install.sh | sh</code></li>
            <li>Pull a model: <code>ollama pull llama2</code> or <code>ollama pull mistral</code></li>
            <li>Start Ollama: <code>ollama serve</code></li>
            <li>The game will automatically detect and use Ollama on port 11434</li>
        </ol>
        <p>To disable AI events: <code>DISABLE_AI=1 cargo run</code></p>
    </div>
    
    <div class="section">
        <h2>Keyboard Commands</h2>
        <table>
            <tr><th>Key</th><th>Action</th></tr>
            <tr><td>1-7</td><td>Combat actions (cost AP)</td></tr>
            <tr><td>8</td><td>View detailed dashboard</td></tr>
            <tr><td>9</td><td>Export game data</td></tr>
            <tr><td>10</td><td>Open this interactive map</td></tr>
            <tr><td>11</td><td>View this manual</td></tr>
            <tr><td>12</td><td>End turn</td></tr>
            <tr><td>13</td><td>Quit game</td></tr>
        </table>
    </div>
    
    <p style="text-align: center; margin-top: 40px;">
        <em>May your fleets sail swift through the void!</em>
    </p>
</body>
</html>"#)
    }
} 