use interstellar_command::core::game::GameState;
use interstellar_command::core::actions;
use interstellar_command::core::turn::TurnPhase;
use interstellar_command::persistence;
use interstellar_command::visualization::Visualizer;
use std::io::{self, Write};

fn main() {
    // Check command line arguments
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 && args[1] == "new" {
        // Start a new game
        start_new_game();
    } else {
        // Resume existing game
        resume_game();
    }
}

fn start_new_game() {
    println!("\n{}", "╔".to_string() + &"═".repeat(68) + "╗");
    println!("║{:^68}║", "NEW CAMPAIGN");
    println!("╚{}╝\n", "═".repeat(68));
    
    // Prompt for player names and passwords
    print!("Player 1 name: ");
    io::stdout().flush().unwrap();
    let mut player1_name = String::new();
    io::stdin().read_line(&mut player1_name).unwrap();
    let player1_name = player1_name.trim().to_string();
    
    print!("Player 1 password: ");
    io::stdout().flush().unwrap();
    let mut player1_pass = String::new();
    io::stdin().read_line(&mut player1_pass).unwrap();
    let player1_pass = player1_pass.trim().to_string();
    
    println!();
    
    print!("Player 2 name: ");
    io::stdout().flush().unwrap();
    let mut player2_name = String::new();
    io::stdin().read_line(&mut player2_name).unwrap();
    let player2_name = player2_name.trim().to_string();
    
    print!("Player 2 password: ");
    io::stdout().flush().unwrap();
    let mut player2_pass = String::new();
    io::stdin().read_line(&mut player2_pass).unwrap();
    let player2_pass = player2_pass.trim().to_string();
    
    // Create new game state
    let mut state = GameState::new();
    
    // Set player names and passwords
    state.players[0].name = player1_name.clone();
    state.players[0].set_password(&player1_pass);
    
    state.players[1].name = player2_name.clone();
    state.players[1].set_password(&player2_pass);
    
    println!("\nStarting campaign...");
    println!("{} starts at {}", state.players[0].name, state.sectors[state.players[0].current_sector as usize].name);
    println!("{} starts at {}", state.players[1].name, state.sectors[state.players[1].current_sector as usize].name);
    
    // Save initial state
    match persistence::save(&state) {
        Ok(_) => println!("\nGame saved. Players can now take turns."),
        Err(e) => {
            eprintln!("Failed to save: {}", e);
            return;
        }
    }
}

fn resume_game() {
    // Load existing state
    let mut state = match persistence::load() {
        Ok(s) => s,
        Err(_) => {
            println!("No saved game found. Start new: cargo run new");
            return;
        }
    };
    
    // Clear screen
    clear_screen();
    
    // Show banner
    println!("\n{}", "╔".to_string() + &"═".repeat(68) + "╗");
    println!("║{:^68}║", format!("TURN {} - CYCLE {}", 
        state.turn_manager.current_turn.number,
        (state.turn_manager.current_turn.number + 1) / 2
    ));
    println!("╚{}╝\n", "═".repeat(68));
    
    // Show turn status
    println!("Turn Status: {}", state.turn_manager.get_turn_summary());
    
    // Check whose turn it is
    let active_player_id = state.turn_manager.current_turn.active_player;
    let active_player = &state.players[active_player_id as usize];
    
    // Handle completed turns
    if state.turn_manager.current_turn.phase == TurnPhase::Complete {
        println!("\n{}'s turn is complete.", active_player.name);
        println!("Waiting for {} to take their turn.", 
            state.players[(1 - active_player_id) as usize].name);
        return;
    }
    
    // Authenticate player
    print!("Password for {}: ", active_player.name);
    io::stdout().flush().unwrap();
    
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();
    
    if !state.players[active_player_id as usize].verify_password(password) {
        println!("Wrong password.");
        return;
    }
    
    println!("Authenticated.\n");
    
    // Start turn if needed
    if let Err(e) = state.run_single_turn() {
        eprintln!("Error starting turn: {}", e);
        return;
    }
    
    // Run the player's turn
    run_player_turn(&mut state, active_player_id);
}

fn run_player_turn(state: &mut GameState, player_id: u8) {
    // Show map and status
    println!("\nSTRATEGIC MAP");
    println!("{}", Visualizer::generate_map(state));
    
    loop {
        // Display current status
        println!("\n{}", "▬".repeat(70));
        display_player_status(state, player_id);
        println!("{}", "▬".repeat(70));
        
        // Show available actions
        display_action_menu(state, player_id);
        
        // Get player input
        print!("\n> Enter command (or 'help' for options): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        // Handle special commands
        match input.to_lowercase().as_str() {
            "help" => {
                show_help();
                continue;
            }
            "status" => {
                show_detailed_status(state, player_id);
                continue;
            }
            "map" => {
                println!("\n{}", Visualizer::generate_map(state));
                continue;
            }
            "end" | "done" => {
                end_turn(state, player_id);
                break;
            }
            "quit" => {
                if confirm_quit() {
                    state.game_over = true;
                    state.event_log.push(format!("{} forfeited the match.", 
                        state.players[player_id as usize].name));
                    save_and_exit(state);
                    break;
                }
                continue;
            }
            _ => {}
        }
        
        // Try to parse as an action
        match actions::parse_action(input, state, player_id) {
            Ok(action) => {
                // Check if player has enough AP
                if !state.players[player_id as usize].can_perform(action.cost()) {
                    println!("[X] Not enough AP! (Need {} AP, have {})", 
                        action.cost(), 
                        state.players[player_id as usize].ap_remaining);
                    continue;
                }
                
                // Validate action
                if let Err(e) = action.validate(state, player_id) {
                    println!("[X] Invalid action: {}", e);
                    continue;
                }
                
                // Execute action
                match action.execute(state, player_id) {
                    Ok(result) => {
                        println!("\n[OK] {}", result);
                        
                        // Record action in turn log
                        state.turn_manager.current_turn.record_action(
                            input.split_whitespace().next().unwrap_or("unknown"),
                            &result,
                            action.cost()
                        );
                        
                        // Auto-save after each action
                        if let Err(e) = persistence::save(state) {
                            eprintln!("Warning: Failed to save game: {}", e);
                        }
                    }
                    Err(e) => {
                        println!("[X] Action failed: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("[X] {}", e);
            }
        }
        
        // Check if out of AP
        if state.players[player_id as usize].ap_remaining == 0 {
            println!("\n[!] Action Points depleted! Turn ending...");
            end_turn(state, player_id);
            break;
        }
        
        // Check if game is over
        if state.game_over {
            handle_game_over(state);
            break;
        }
    }
}

fn display_player_status(state: &GameState, player_id: u8) {
    let player = &state.players[player_id as usize];
    let sector = &state.sectors[player.current_sector as usize];
    
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║ {} {} (Level {})                                                  ║", player.rank, player.name, player.level);
    println!("║ Health: {} HP │ AP: {}/{} │ XP: {}/{} │ Fleet: {} ships        ║", 
        player.health, player.ap_remaining, player.ap_cap, 
        player.experience, player.level as u32 * 100,
        player.fleet.total_ships());
    println!("║ Location: {} │ Rank: {}                                          ║", 
        sector.name, player.rank);
    println!("╚══════════════════════════════════════════════════════════════════╝");
}

fn display_action_menu(state: &GameState, player_id: u8) {
    let player = &state.players[player_id as usize];
    
    println!("\n┌─ Actions ─────────────────────────┬─ Commands ───────────────────┐");
    println!("│ move <sector>  - Move fleet (5 AP) │ status - Show details        │");
    println!("│ attack         - Attack enemy (8 AP)│ map    - Show map            │");
    println!("│ scan <sector>  - Scan sector (3 AP)│ end    - End turn            │");
    println!("│ build          - Build outpost (10)│ help   - Show help           │");
    
    if player.level >= 3 {
        println!("│ reinforce      - Heal fleet (15 AP)│                              │");
    } else {
        println!("│                                    │                              │");
    }
    
    println!("└────────────────────────────────────┴──────────────────────────────┘");
}

fn show_help() {
    println!("\n╔════════════════════════════════ HELP ════════════════════════════╗");
    println!("║ Actions cost AP. Your turn ends when you run out.                ║");
    println!("╟───────────────────────────────────────────────────────────────────╢");
    println!("║ move <sector> - Move to adjacent sector                           ║");
    println!("║ attack - Attack enemy in your sector                              ║");
    println!("║ scan <sector> - Reveal sector info                                ║");
    println!("║ build - Build outpost (must control sector)                       ║");
    println!("╟───────────────────────────────────────────────────────────────────╢");
    println!("║ Type 'end' to end turn early and save AP.                        ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");
}

fn show_detailed_status(state: &GameState, player_id: u8) {
    println!("{}", Visualizer::generate_player_view(state, player_id));
}

fn end_turn(state: &mut GameState, player_id: u8) {
    let player = &state.players[player_id as usize];
    
    println!("\n╔═══════════════════════ TURN COMPLETE ════════════════════════════╗");
    println!("║ Actions taken: {}                                                  ║", state.turn_manager.current_turn.actions_taken.len());
    println!("║ AP remaining: {} (saved for next turn)                             ║", player.ap_remaining);
    println!("╚═══════════════════════════════════════════════════════════════════╝");
    
    // Mark turn as complete
    state.turn_manager.current_turn.complete();
    
    // Check for level up
    let old_level = state.players[player_id as usize].level;
    state.players[player_id as usize].check_level_up();
    if state.players[player_id as usize].level > old_level {
        println!("\nPROMOTION!");
        println!("You have been promoted to Level {} - {}!", 
            state.players[player_id as usize].level,
            state.players[player_id as usize].rank);
    }
    
    // Advance to next turn
    state.turn_manager.advance_turn();
    state.turn_number = state.turn_manager.current_turn.number;
    state.current_player = state.turn_manager.current_turn.active_player;
    
    // Save game
    save_and_exit(state);
    
    println!("\n{} can now take their turn.", 
        state.players[state.current_player as usize].name);
}

fn save_and_exit(state: &GameState) {
    print!("\nSaving game... ");
    io::stdout().flush().unwrap();
    
    match persistence::save(state) {
        Ok(_) => println!("Success!"),
        Err(e) => eprintln!("Failed: {}", e),
    }
}

fn confirm_quit() -> bool {
    print!("Are you sure you want to forfeit the match? (y/n): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().to_lowercase() == "y"
}

fn handle_game_over(state: &GameState) {
    println!("\n{}", "╔".to_string() + &"═".repeat(68) + "╗");
    println!("║{:^68}║", "CAMPAIGN CONCLUDED");
    println!("╚{}╝", "═".repeat(68));
    
    // Determine winner
    let winner = if state.players[0].is_alive() { 0 } else { 1 };
    println!("\n{} is victorious!", state.players[winner].name);
    
    println!("\nFinal Statistics:");
    for (i, player) in state.players.iter().enumerate() {
        println!("\n{}: Level {} {}", player.name, player.level, player.rank);
        println!("  Final HP: {}", player.health);
        println!("  Sectors controlled: {}", 
            state.sectors.iter().filter(|s| s.owner == Some(i as u8)).count());
    }
    
    // Export final game data
    println!("\nExporting final game data...");
    match Visualizer::export_player_html(state, 0) {
        Ok(f) => println!("Player 1 view: {}", f),
        Err(e) => eprintln!("Export failed: {}", e),
    }
    match Visualizer::export_player_html(state, 1) {
        Ok(f) => println!("Player 2 view: {}", f),
        Err(e) => eprintln!("Export failed: {}", e),
    }
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = std::process::Command::new("cmd").args(&["/C", "cls"]).status();
    } else {
        let _ = std::process::Command::new("clear").status();
    }
} 