use interstellar_command::core::game::GameState;
use interstellar_command::persistence;

fn main() {
    println!("Welcome to Interstellar Command");
    println!("A point-based strategy game inspired by Red Rising\n");

    // Attempt to load existing state; otherwise, create new
    let mut state = match persistence::load() {
        Ok(s) => {
            println!("Resuming saved game from turn {}", s.turn_number);
            s
        }
        Err(_) => GameState::new(),
    };
    
    // Run the main loop
    state.run_turn_loop();
} 