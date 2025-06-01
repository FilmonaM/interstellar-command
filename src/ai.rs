use crate::core::game::GameState;

/// AI Event Generator
/// 
/// Provides atmospheric flavor text between turns.
/// Works in three modes: stub (default), Ollama integration, or disabled.
/// Set DISABLE_AI=1 to disable all events.
pub fn generate_event(state: &GameState) -> String {
    // Check if AI should be disabled
    if std::env::var("DISABLE_AI").is_ok() {
        return String::new();
    }
    
    // Try Ollama if available
    if let Ok(event) = try_ollama_event(state) {
        return event;
    }
    
    // Fall back to pre-written events
    generate_stub_event(state)
}

/// Attempt to generate event using Ollama API
fn try_ollama_event(state: &GameState) -> Result<String, Box<dyn std::error::Error>> {
    // Check if Ollama is running
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .build()?;
    
    // Build context from game state
    let p1 = &state.players[0];
    let p2 = &state.players[1];
    
    let p1_sectors = state.sectors.iter().filter(|s| s.owner == Some(0)).count();
    let p2_sectors = state.sectors.iter().filter(|s| s.owner == Some(1)).count();
    
    let recent_event = state.event_log.last()
        .map(|e| e.as_str())
        .unwrap_or("The campaign has begun");
    
    let prompt = format!(
        "You are a narrator for Interstellar Command, a space strategy game inspired by Red Rising. \
        Generate ONE brief atmospheric event (under 20 words) based on this situation:\n\
        Turn {}: {} {} (Level {}, {} ships) controls {} sectors. \
        {} {} (Level {}, {} ships) controls {} sectors.\n\
        Recent: {}\n\
        Create flavor text about the solar empire, NOT game mechanics. Be dramatic and evocative.",
        state.turn_number,
        p1.rank, p1.name, p1.level, p1.fleet.total_ships(), p1_sectors,
        p2.rank, p2.name, p2.level, p2.fleet.total_ships(), p2_sectors,
        recent_event
    );
    
    // Request to Ollama API
    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": "llama2",
            "prompt": prompt,
            "stream": false,
            "options": {
                "temperature": 0.8,
                "num_predict": 50,
                "stop": [".", "!", "?"]
            }
        }))
        .send()?;
    
    if response.status().is_success() {
        let body: serde_json::Value = response.json()?;
        if let Some(text) = body["response"].as_str() {
            let cleaned = text.trim()
                .split('\n').next().unwrap_or("")
                .trim_end_matches('.')
                .trim();
            
            if !cleaned.is_empty() && cleaned.len() < 100 {
                return Ok(format!("{}.", cleaned));
            }
        }
    }
    
    Err("Ollama not available".into())
}

/// Generate fallback event when Ollama is not available
fn generate_stub_event(state: &GameState) -> String {
    let events = [
        "The Martian Senate debates new trade regulations affecting the asteroid belt.",
        "Solar flares from Sol temporarily disrupt long-range communications.",
        "Rumors spread of ancient technology discovered in the Jovian moons.",
        "Pirates have been spotted near the outer rim territories.",
        "The Venusian nobility hosts a grand celebration, distracting local defenses.",
        "A prototype warp gate activates briefly near Saturn before shutting down.",
        "Asteroid miners report unusual energy readings from deep space.",
        "The Earth Coalition announces new military funding initiatives.",
    ];
    
    // Return empty after showing all events once
    if state.turn_number > events.len() as u32 {
        return String::new();
    }
    
    events[(state.turn_number as usize - 1) % events.len()].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ollama_connection() {
        let client = reqwest::blocking::Client::new();
        match client.get("http://localhost:11434/api/tags").send() {
            Ok(response) => {
                println!("Ollama is running!");
                if let Ok(body) = response.text() {
                    println!("Available models: {}", body);
                }
            }
            Err(_) => {
                println!("Ollama is not running. Install and start with 'ollama serve'");
            }
        }
    }
}

// Future expansion: connect to Ollama or other LLM
// pub async fn generate_event_with_llm(state: &GameState) -> String {
//     let prompt = format!(
//         "Turn {}: {}
//         Recently, these events occurred: {:?}
//         Provide a short AI-generated rumor or reaction.",
//         state.turn_number,
//         state.players[state.current_player as usize].name,
//         state.event_log.iter().rev().take(3).collect::<Vec<_>>()
//     );
//     // Send prompt to Ollama and return its response text
//     // If Ollama is unreachable, fallback to the static list
// } 