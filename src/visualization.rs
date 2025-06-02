use crate::core::game::GameState;
use std::fs;

pub struct Visualizer;

impl Visualizer {
    /// Generate ASCII map of the solar system
    pub fn generate_map(state: &GameState) -> String {
        let mut map = String::new();
        
        // Simpler, cleaner map header
        map.push_str("\n+---------------------------- SECTOR MAP ----------------------------+\n");
        
        if state.sectors.len() <= 5 {
            // Tactical 5-sector map - clean visual layout
            map.push_str("|                                                                    |\n");
            map.push_str("|     [0] Earth ---- [1] Mars ---- [2] Asteroid ---- [4] Jupiter    |\n");
            map.push_str("|                        |                                           |\n");
            map.push_str("|                    [3] Venus                                       |\n");
            map.push_str("|                                                                    |\n");
        } else if state.sectors.len() <= 8 {
            // Tactical 8-sector map
            map.push_str("|                                                                    |\n");
            map.push_str("|  [0] Sol ------ [1] Inner ------ [2] Mars ------ [3] Belt        |\n");
            map.push_str("|     |              |                 |               |             |\n");
            map.push_str("|  [4] Luna      [5] Venus         [6] Phobos     [7] Ceres        |\n");
            map.push_str("|                                                                    |\n");
        } else {
            // Strategic map - grid layout
            map.push_str("|  INNER SYSTEM:                    |  OUTER SYSTEM:                 |\n");
            map.push_str("|  [0] Sol         [4] Luna         |  [8] Jupiter    [12] Callisto  |\n");
            map.push_str("|  [1] Inner       [5] Venus        |  [9] Io         [13] Ganymede  |\n");
            map.push_str("|  [2] Mars        [6] Phobos       |  [10] Europa    [14] Saturn    |\n");
            map.push_str("|  [3] Belt        [7] Ceres        |  [11] Belt II   [15] Titan     |\n");
            map.push_str("|                                   |  [16] Deep Space                |\n");
        }
        
        map.push_str("+--------------------------------------------------------------------+\n");
        
        // Sector ownership status
        map.push_str("| SECTOR CONTROL:                                                    |\n");
        
        let mut control_lines = Vec::new();
        let mut current_line = String::from("| ");
        
        for (i, sector) in state.sectors.iter().enumerate() {
            let mut status = format!("[{}] ", i);
            
            // Add ownership indicator
            match sector.owner {
                Some(0) => status.push_str(&format!("P1")),
                Some(1) => status.push_str(&format!("P2")),
                None => status.push_str("--"),
                Some(_) => status.push_str("??"), // Handle any other player IDs
            }
            
            // Add outpost indicator
            if sector.has_outpost {
                status.push_str("*");
            } else {
                status.push_str(" ");
            }
            
            // Check if adding this would exceed line length
            if current_line.len() + status.len() + 2 > 66 {
                // Pad current line and save it
                while current_line.len() < 67 {
                    current_line.push(' ');
                }
                current_line.push('|');
                control_lines.push(current_line);
                current_line = String::from("| ");
            }
            
            current_line.push_str(&status);
            current_line.push_str("  ");
        }
        
        // Add last line
        if current_line.len() > 2 {
            while current_line.len() < 67 {
                current_line.push(' ');
            }
            current_line.push('|');
            control_lines.push(current_line);
        }
        
        for line in control_lines {
            map.push_str(&line);
            map.push('\n');
        }
        
        map.push_str("+--------------------------------------------------------------------+\n");
        
        // Fleet positions
        map.push_str("| FLEET POSITIONS:                                                   |\n");
        for player in &state.players {
            let sector_name = &state.sectors[player.current_sector as usize].name;
            let fleet_line = format!("| {} {} at [{}] {} - {} ships", 
                player.rank, 
                player.name,
                player.current_sector,
                sector_name,
                player.fleet.total_ships()
            );
            
            // Pad to consistent width
            let mut padded_line = fleet_line;
            while padded_line.len() < 67 {
                padded_line.push(' ');
            }
            padded_line.push('|');
            map.push_str(&padded_line);
            map.push('\n');
        }
        
        map.push_str("+--------------------------------------------------------------------+\n");
        map.push_str("| Legend: P1/P2 = Owner, * = Outpost, -- = Neutral                  |\n");
        map.push_str("+--------------------------------------------------------------------+\n");
        
        map
    }
    
    /// Generate player dashboard
    pub fn generate_player_view(state: &GameState, player_id: u8) -> String {
        let player = &state.players[player_id as usize];
        let mut view = String::new();
        
        view.push_str("\n");
        view.push_str(&format!("+{:-^68}+\n", format!(" {} {}'s Command Dashboard ", player.rank, player.name)));
        view.push_str("\n");
        
        // Stats
        view.push_str("STATISTICS:\n");
        view.push_str(&format!("  Level: {} - {}     Health: {}/{}\n", 
            player.level, player.rank, player.health, 100 + (player.level as i32 - 1) * 20));
        view.push_str(&format!("  XP: {}/{}     AP Cap: {}\n", 
            player.experience, player.level as u32 * 100, player.ap_cap));
        view.push_str(&format!("  Damage Bonus: +{}     Scan Range: {}\n", 
            player.get_damage_bonus(), if player.get_scan_range_bonus() > 0 { "Extended" } else { "Normal" }));
        view.push_str("\n");
        
        // Fleet Composition
        view.push_str("FLEET COMPOSITION:\n");
        view.push_str(&format!("  Scouts: {}     Frigates: {}     Destroyers: {}\n",
            player.fleet.scouts, player.fleet.frigates, player.fleet.destroyers));
        view.push_str(&format!("  Command Centers: {}     Combat Strength: {}\n",
            player.fleet.command_centers, player.fleet.combat_strength()));
        view.push_str(&format!("  Total Ships: {}\n", player.fleet.total_ships()));
        if !player.can_capture_sector() {
            view.push_str("  Note: Command Center required to capture sectors (Level 4+)\n");
        }
        view.push_str("\n");
        
        // Controlled Sectors
        view.push_str("CONTROLLED SECTORS:\n");
        let mut has_sectors = false;
        for sector in &state.sectors {
            if sector.owner == Some(player_id) {
                has_sectors = true;
                if sector.has_outpost {
                    view.push_str(&format!("  * {} [OUTPOST]\n", sector.name));
                } else {
                    view.push_str(&format!("  * {}\n", sector.name));
                }
            }
        }
        if !has_sectors {
            view.push_str("  No sectors under control\n");
        }
        view.push_str("\n");
        
        // Abilities
        view.push_str("ABILITIES:\n");
        view.push_str(&format!("  * Move Fleet (5 AP)\n"));
        view.push_str(&format!("  * Attack ({} damage, 8 AP)\n", 10 + player.get_damage_bonus()));
        view.push_str(&format!("  * Scan Sector (3 AP)\n"));
        view.push_str(&format!("  * Build Outpost (10 AP)\n"));
        if player.level >= 3 {
            view.push_str(&format!("  * Reinforce (15 AP) - Heal 20 HP\n"));
        }
        if player.level >= 5 {
            view.push_str(&format!("  * Sabotage (12 AP) - Destroy enemy outpost\n"));
        }
        if player.level >= 7 {
            view.push_str(&format!("  * Orbital Strike (20 AP) - 30 damage anywhere\n"));
        }
        view.push_str("\n");
        
        view
    }
    
    /// Export player view to HTML
    pub fn export_player_html(state: &GameState, player_id: u8) -> Result<String, Box<dyn std::error::Error>> {
        let player = &state.players[player_id as usize];
        let filename = format!("player_{}_view.txt", player.name.to_lowercase());
        
        // Generate plain text content instead of HTML
        let mut content = String::new();
        
        content.push_str("========================================\n");
        content.push_str(&format!("INTERSTELLAR COMMAND - TURN {}\n", state.turn_number));
        content.push_str(&format!("{} {}'s Status Report\n", player.rank, player.name));
        content.push_str("========================================\n\n");
        
        content.push_str("STATISTICS\n");
        content.push_str("----------\n");
        content.push_str(&format!("Level: {} - {}\n", player.level, player.rank));
        content.push_str(&format!("Health: {} / {}\n", player.health, 100 + (player.level as i32 - 1) * 20));
        content.push_str(&format!("Experience: {} / {}\n", player.experience, player.level as u32 * 100));
        content.push_str(&format!("Action Points: {}\n", player.ap_cap));
        content.push_str(&format!("Damage Bonus: +{}\n", player.get_damage_bonus()));
        content.push_str(&format!("Scan Range: {}\n", if player.get_scan_range_bonus() > 0 { "Extended" } else { "Normal" }));
        content.push_str("\n");
        
        content.push_str("FLEET COMPOSITION\n");
        content.push_str("-----------------\n");
        content.push_str(&format!("Scouts: {}\n", player.fleet.scouts));
        content.push_str(&format!("Frigates: {}\n", player.fleet.frigates));
        content.push_str(&format!("Destroyers: {}\n", player.fleet.destroyers));
        content.push_str(&format!("Command Centers: {}\n", player.fleet.command_centers));
        content.push_str(&format!("Total Ships: {}\n", player.fleet.total_ships()));
        content.push_str(&format!("Combat Strength: {}\n", player.fleet.combat_strength()));
        content.push_str("\n");
        
        content.push_str("CONTROLLED SECTORS\n");
        content.push_str("------------------\n");
        let mut has_sectors = false;
        for sector in &state.sectors {
            if sector.owner == Some(player_id) {
                has_sectors = true;
                if sector.has_outpost {
                    content.push_str(&format!("* {} [OUTPOST]\n", sector.name));
                } else {
                    content.push_str(&format!("* {}\n", sector.name));
                }
            }
        }
        if !has_sectors {
            content.push_str("No sectors under control\n");
        }
        content.push_str("\n");
        
        content.push_str("SECTOR MAP\n");
        content.push_str("----------\n");
        content.push_str(&Self::generate_map(state));
        content.push_str("\n");
        
        content.push_str("RECENT EVENTS\n");
        content.push_str("-------------\n");
        for event in state.event_log.iter().rev().take(10) {
            content.push_str(&format!("* {}\n", event));
        }
        
        // Write as plain text file
        fs::write(&filename, &content)?;
        
        // Also create a simple HTML version
        let html_filename = format!("player_{}_view.html", player.name.to_lowercase());
        let html = format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>{} {}'s Command Report</title>
    <style>
        body {{
            font-family: monospace;
            background: #000;
            color: #0f0;
            padding: 20px;
            white-space: pre-wrap;
        }}
    </style>
</head>
<body>{}</body>
</html>"#,
            player.rank, player.name,
            content.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")
        );
        
        fs::write(&html_filename, html)?;
        Ok(filename)
    }
    
    /// Generate statistics comparison
    pub fn generate_stats_comparison(state: &GameState) -> String {
        let mut chart = String::new();
        
        chart.push_str("\n+----------------------- PLAYER COMPARISON ------------------------+\n");
        chart.push_str("|                    Player 1         vs         Player 2          |\n");
        chart.push_str("+------------------------------------------------------------------+\n");
        
        let p1 = &state.players[0];
        let p2 = &state.players[1];
        
        chart.push_str(&format!("| Name:          {:>15}     vs     {:<15} |\n", p1.name, p2.name));
        chart.push_str(&format!("| Rank:          {:>15}     vs     {:<15} |\n", p1.rank, p2.rank));
        chart.push_str(&format!("| Level:         {:>15}     vs     {:<15} |\n", p1.level, p2.level));
        chart.push_str(&format!("| Health:        {:>15}     vs     {:<15} |\n", p1.health, p2.health));
        chart.push_str(&format!("| AP Cap:        {:>15}     vs     {:<15} |\n", p1.ap_cap, p2.ap_cap));
        chart.push_str(&format!("| Damage:        {:>15}     vs     {:<15} |\n", 
            format!("+{}", p1.get_damage_bonus()), format!("+{}", p2.get_damage_bonus())));
        
        // Sector control
        let p1_sectors = state.sectors.iter().filter(|s| s.owner == Some(0)).count();
        let p2_sectors = state.sectors.iter().filter(|s| s.owner == Some(1)).count();
        chart.push_str(&format!("| Sectors:       {:>15}     vs     {:<15} |\n", p1_sectors, p2_sectors));
        
        chart.push_str("+------------------------------------------------------------------+\n");
        
        // Health bars
        chart.push_str("| Health Bars:                                                     |\n");
        chart.push_str(&format!("| {:14} [", p1.name));
        let p1_bar_len = (p1.health as f32 / (100.0 + (p1.level as f32 - 1.0) * 20.0) * 20.0) as usize;
        for i in 0..20 {
            if i < p1_bar_len { chart.push('#'); } else { chart.push('-'); }
        }
        chart.push_str(&format!("] {}%", (p1.health as f32 / (100.0 + (p1.level as f32 - 1.0) * 20.0) * 100.0) as i32));
        
        // Pad to consistent width
        while chart.lines().last().unwrap_or("").len() < 67 {
            chart.push(' ');
        }
        chart.push_str("|\n");
        
        chart.push_str(&format!("| {:14} [", p2.name));
        let p2_bar_len = (p2.health as f32 / (100.0 + (p2.level as f32 - 1.0) * 20.0) * 20.0) as usize;
        for i in 0..20 {
            if i < p2_bar_len { chart.push('#'); } else { chart.push('-'); }
        }
        chart.push_str(&format!("] {}%", (p2.health as f32 / (100.0 + (p2.level as f32 - 1.0) * 20.0) * 100.0) as i32));
        
        // Pad to consistent width
        while chart.lines().last().unwrap_or("").len() < 67 {
            chart.push(' ');
        }
        chart.push_str("|\n");
        
        chart.push_str("+------------------------------------------------------------------+\n");
        
        chart
    }
    
    /// Generate interactive HTML map
    pub fn generate_interactive_map(state: &GameState) -> Result<String, Box<dyn std::error::Error>> {
        let filename = "interstellar_map.html";
        
        let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Interstellar Command - Strategic Map</title>
    <style>
        body {{
            margin: 0;
            padding: 0;
            background: #000;
            color: #0ff;
            font-family: 'Courier New', monospace;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            overflow: hidden;
        }}
        
        .starfield {{
            position: fixed;
            width: 100%;
            height: 100%;
            z-index: -1;
        }}
        
        .star {{
            position: absolute;
            width: 2px;
            height: 2px;
            background: white;
            border-radius: 50%;
            animation: twinkle 3s infinite;
        }}
        
        @keyframes twinkle {{
            0%, 100% {{ opacity: 0.3; }}
            50% {{ opacity: 1; }}
        }}
        
        .map-container {{
            position: relative;
            width: 1200px;
            height: 800px;
            margin: 20px;
        }}
        
        .sector {{
            position: absolute;
            width: 150px;
            height: 150px;
            border: 2px solid #0ff;
            border-radius: 50%;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            text-align: center;
            background: radial-gradient(circle, rgba(0,255,255,0.1) 0%, rgba(0,0,0,0.8) 100%);
            transition: all 0.3s ease;
            cursor: pointer;
        }}
        
        .sector:hover {{
            transform: scale(1.1);
            border-color: #fff;
            box-shadow: 0 0 30px #0ff;
        }}
        
        .sector.owned-0 {{
            border-color: #00ff00;
            background: radial-gradient(circle, rgba(0,255,0,0.2) 0%, rgba(0,0,0,0.8) 100%);
        }}
        
        .sector.owned-1 {{
            border-color: #ff0000;
            background: radial-gradient(circle, rgba(255,0,0,0.2) 0%, rgba(0,0,0,0.8) 100%);
        }}
        
        .sector-name {{
            font-size: 18px;
            font-weight: bold;
            margin-bottom: 5px;
        }}
        
        .sector-info {{
            font-size: 12px;
            opacity: 0.8;
        }}
        
        .connection {{
            position: absolute;
            height: 2px;
            background: linear-gradient(90deg, transparent, #0ff, transparent);
            transform-origin: left center;
            opacity: 0.3;
            animation: pulse 2s infinite;
        }}
        
        @keyframes pulse {{
            0%, 100% {{ opacity: 0.3; }}
            50% {{ opacity: 0.6; }}
        }}
        
        .fleet-marker {{
            position: absolute;
            width: 30px;
            height: 30px;
            border-radius: 50%;
            animation: spin 4s linear infinite;
        }}
        
        .fleet-0 {{
            border: 3px solid #00ff00;
            box-shadow: 0 0 10px #00ff00;
        }}
        
        .fleet-1 {{
            border: 3px solid #ff0000;
            box-shadow: 0 0 10px #ff0000;
        }}
        
        @keyframes spin {{
            from {{ transform: rotate(0deg); }}
            to {{ transform: rotate(360deg); }}
        }}
        
        .legend {{
            position: absolute;
            top: 20px;
            left: 20px;
            background: rgba(0,0,0,0.8);
            border: 1px solid #0ff;
            padding: 20px;
            border-radius: 10px;
        }}
        
        .turn-info {{
            position: absolute;
            top: 20px;
            right: 20px;
            background: rgba(0,0,0,0.8);
            border: 1px solid #0ff;
            padding: 20px;
            border-radius: 10px;
            text-align: right;
        }}
        
        h1 {{
            color: #0ff;
            text-shadow: 0 0 10px #0ff;
            margin: 0 0 10px 0;
        }}
        
        .tooltip {{
            position: absolute;
            background: rgba(0,0,0,0.9);
            border: 1px solid #0ff;
            padding: 10px;
            border-radius: 5px;
            font-size: 12px;
            display: none;
            z-index: 1000;
            pointer-events: none;
        }}
    </style>
</head>
<body>
    <div class="starfield" id="starfield"></div>
    
    <div class="map-container">
        <!-- Connections -->
        {}
        
        <!-- Sectors -->
        {}
        
        <!-- Fleet Markers -->
        {}
        
        <div class="legend">
            <h1>Strategic Map</h1>
            <div style="color: #00ff00">● {} Territory</div>
            <div style="color: #ff0000">● {} Territory</div>
            <div style="color: #0ff">● Neutral Space</div>
            <div style="margin-top: 10px">
                <div>[o] Fleet Position</div>
                <div>[#] Outpost Present</div>
            </div>
        </div>
        
        <div class="turn-info">
            <h1>Turn {}</h1>
            <div>Current Phase: {}</div>
        </div>
    </div>
    
    <div class="tooltip" id="tooltip"></div>
    
    <script>
        // Generate starfield
        const starfield = document.getElementById('starfield');
        for (let i = 0; i < 200; i++) {{
            const star = document.createElement('div');
            star.className = 'star';
            star.style.left = Math.random() * 100 + '%';
            star.style.top = Math.random() * 100 + '%';
            star.style.animationDelay = Math.random() * 3 + 's';
            starfield.appendChild(star);
        }}
        
        // Sector positions
        const sectorPositions = {{
            0: {{ x: 200, y: 400 }},
            1: {{ x: 500, y: 400 }},
            2: {{ x: 800, y: 400 }},
            3: {{ x: 500, y: 600 }},
            4: {{ x: 1000, y: 400 }}
        }};
        
        // Tooltips
        const sectors = document.querySelectorAll('.sector');
        const tooltip = document.getElementById('tooltip');
        
        sectors.forEach(sector => {{
            sector.addEventListener('mouseenter', (e) => {{
                const info = sector.getAttribute('data-tooltip');
                tooltip.innerHTML = info;
                tooltip.style.display = 'block';
            }});
            
            sector.addEventListener('mousemove', (e) => {{
                tooltip.style.left = e.pageX + 10 + 'px';
                tooltip.style.top = e.pageY + 10 + 'px';
            }});
            
            sector.addEventListener('mouseleave', () => {{
                tooltip.style.display = 'none';
            }});
        }});
        
        // Auto-refresh
        setTimeout(() => {{
            location.reload();
        }}, 5000);
    </script>
</body>
</html>"#,
            Self::generate_connections_html(state),
            Self::generate_sectors_html(state),
            Self::generate_fleets_html(state),
            state.players[0].name,
            state.players[1].name,
            state.turn_number,
            state.players[state.current_player as usize].name
        );
        
        fs::write(&filename, html)?;
        
        // Open in browser
        #[cfg(target_os = "windows")]
        std::process::Command::new("cmd")
            .args(&["/C", "start", filename])
            .spawn()?;
            
        #[cfg(target_os = "macos")]
        std::process::Command::new("open")
            .arg(filename)
            .spawn()?;
            
        #[cfg(target_os = "linux")]
        std::process::Command::new("xdg-open")
            .arg(filename)
            .spawn()?;
        
        Ok(filename.to_string())
    }
    
    fn generate_connections_html(_state: &GameState) -> String {
        let mut connections = String::new();
        let positions = vec![
            (200.0_f64, 400.0_f64),  // Earth
            (500.0_f64, 400.0_f64),  // Mars
            (800.0_f64, 400.0_f64),  // Asteroid Belt
            (500.0_f64, 600.0_f64),  // Venus
            (1000.0_f64, 400.0_f64), // Jupiter
        ];
        
        // Sector connections
        let adjacencies = vec![
            (0, 1), // Earth-Mars
            (1, 2), // Mars-Asteroid
            (1, 3), // Mars-Venus
            (2, 4), // Asteroid-Jupiter
        ];
        
        for (from, to) in adjacencies {
            let (x1, y1) = positions[from];
            let (x2, y2) = positions[to];
            let dx = x2 - x1;
            let dy = y2 - y1;
            let length = (dx*dx + dy*dy).sqrt();
            let angle = dy.atan2(dx) * 180.0_f64 / std::f64::consts::PI;
            
            connections.push_str(&format!(
                r#"<div class="connection" style="left: {}px; top: {}px; width: {}px; transform: rotate({}deg);"></div>"#,
                x1 + 75.0, y1 + 75.0, length, angle
            ));
        }
        
        connections
    }
    
    fn generate_sectors_html(state: &GameState) -> String {
        let mut sectors_html = String::new();
        let positions = vec![
            (200, 400),  // Earth
            (500, 400),  // Mars
            (800, 400),  // Asteroid Belt
            (500, 600),  // Venus
            (1000, 400), // Jupiter
        ];
        
        for (i, sector) in state.sectors.iter().enumerate() {
            let (x, y) = positions[i];
            let owner_class = match sector.owner {
                Some(0) => "owned-0",
                Some(1) => "owned-1",
                _ => "",
            };
            
            let owner_name = match sector.owner {
                Some(id) => &state.players[id as usize].name,
                None => "Unclaimed",
            };
            
            let outpost_icon = if sector.has_outpost { "[#]" } else { "" };
            
            let tooltip = format!(
                "Sector: {}<br>Owner: {}<br>Outpost: {}",
                sector.name,
                owner_name,
                if sector.has_outpost { "Yes" } else { "No" }
            );
            
            sectors_html.push_str(&format!(
                r#"<div class="sector {}" style="left: {}px; top: {}px;" data-tooltip="{}">
                    <div class="sector-name">{}</div>
                    <div class="sector-info">{}</div>
                    <div style="font-size: 24px">{}</div>
                </div>"#,
                owner_class, x, y, tooltip, sector.name, owner_name, outpost_icon
            ));
        }
        
        sectors_html
    }
    
    fn generate_fleets_html(state: &GameState) -> String {
        let mut fleets_html = String::new();
        let positions = vec![
            (200, 400),  // Earth
            (500, 400),  // Mars
            (800, 400),  // Asteroid Belt
            (500, 600),  // Venus
            (1000, 400), // Jupiter
        ];
        
        for (i, player) in state.players.iter().enumerate() {
            let sector = player.current_sector as usize;
            if sector < positions.len() {
                let (x, y) = positions[sector];
                let offset = if i == 0 { -20 } else { 20 };
                
                fleets_html.push_str(&format!(
                    r#"<div class="fleet-marker fleet-{}" style="left: {}px; top: {}px;" title="{}'s Fleet"></div>"#,
                    i, x + 60 + offset, y + 60 + offset, player.name
                ));
            }
        }
        
        fleets_html
    }
} 