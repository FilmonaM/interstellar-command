use crate::core::game::GameState;
use std::fs;

pub struct Visualizer;

impl Visualizer {
    /// Generate ASCII map of the solar system
    pub fn generate_map(state: &GameState) -> String {
        let mut map = String::new();
        
        map.push_str("\n═══════════════════════════════════════════════════════════════\n");
        map.push_str("                    SOLAR SYSTEM MAP                           \n");
        map.push_str("═══════════════════════════════════════════════════════════════\n\n");
        
        // Map layout
        map.push_str("    [Earth]");
        if state.sectors[0].owner.is_some() {
            let owner = state.sectors[0].owner.unwrap();
            map.push_str(&format!("({})", &state.players[owner as usize].name[0..1]));
        } else {
            map.push_str("   ");
        }
        
        map.push_str(" ---- [Mars]");
        if state.sectors[1].owner.is_some() {
            let owner = state.sectors[1].owner.unwrap();
            map.push_str(&format!("({})", &state.players[owner as usize].name[0..1]));
        } else {
            map.push_str("   ");
        }
        
        map.push_str(" ---- [Asteroid]");
        if state.sectors[2].owner.is_some() {
            let owner = state.sectors[2].owner.unwrap();
            map.push_str(&format!("({})", &state.players[owner as usize].name[0..1]));
        } else {
            map.push_str("   ");
        }
        
        map.push_str(" ---- [Jupiter]");
        if state.sectors[4].owner.is_some() {
            let owner = state.sectors[4].owner.unwrap();
            map.push_str(&format!("({})", &state.players[owner as usize].name[0..1]));
        } else {
            map.push_str("   ");
        }
        
        map.push_str("\n");
        map.push_str("                    |\n");
        map.push_str("                 [Venus]");
        if state.sectors[3].owner.is_some() {
            let owner = state.sectors[3].owner.unwrap();
            map.push_str(&format!("({})", &state.players[owner as usize].name[0..1]));
        } else {
            map.push_str("   ");
        }
        
        map.push_str("\n\n");
        map.push_str("Legend: (C) = Cassia, (D) = Darrow, etc.\n");
        map.push_str("        * = Outpost present\n");
        
        // Fleet positions
        map.push_str("\nFleet Positions:\n");
        for player in &state.players {
            let sector = &state.sectors[player.current_sector as usize];
            map.push_str(&format!("  {} {} is at {}\n", player.rank, player.name, sector.name));
        }
        
        map
    }
    
    /// Generate player dashboard
    pub fn generate_player_view(state: &GameState, player_id: u8) -> String {
        let player = &state.players[player_id as usize];
        let mut view = String::new();
        
        view.push_str(&format!("\n╔═══════════════════════════════════════════════════════════════╗\n"));
        view.push_str(&format!("║          {} {}'s Command Dashboard             ║\n", player.rank, player.name));
        view.push_str(&format!("╚═══════════════════════════════════════════════════════════════╝\n\n"));
        
        // Stats
        view.push_str(&format!("┌─── STATISTICS ───────────────────────────────────────────────┐\n"));
        view.push_str(&format!("│ Level: {:2} - {:14}     Health: {:3}/{:3}            │\n", 
            player.level, player.rank, player.health, 100 + (player.level as i32 - 1) * 20));
        view.push_str(&format!("│ XP: {:3}/{:3}                      AP Cap: {:2}                  │\n", 
            player.experience, player.level as u32 * 100, player.ap_cap));
        view.push_str(&format!("│ Damage Bonus: +{:2}               Scan Range: {}               │\n", 
            player.get_damage_bonus(), if player.get_scan_range_bonus() > 0 { "Extended" } else { "Normal  " }));
        view.push_str(&format!("└──────────────────────────────────────────────────────────────┘\n\n"));
        
        // Fleet Composition
        view.push_str(&format!("┌─── FLEET COMPOSITION ────────────────────────────────────────┐\n"));
        view.push_str(&format!("│ Scouts: {:2}         Frigates: {:2}      Destroyers: {:2}         │\n",
            player.fleet.scouts, player.fleet.frigates, player.fleet.destroyers));
        view.push_str(&format!("│ Command Centers: {:2}              Combat Strength: {:3}        │\n",
            player.fleet.command_centers, player.fleet.combat_strength()));
        view.push_str(&format!("│ Total Ships: {:2}                                              │\n",
            player.fleet.total_ships()));
        if !player.can_capture_sector() {
            view.push_str(&format!("│ ⚠️  Command Center required to capture sectors (Level 4+)     │\n"));
        }
        view.push_str(&format!("└──────────────────────────────────────────────────────────────┘\n\n"));
        
        // Controlled Sectors
        view.push_str(&format!("┌─── CONTROLLED SECTORS ───────────────────────────────────────┐\n"));
        let mut has_sectors = false;
        for sector in &state.sectors {
            if sector.owner == Some(player_id) {
                has_sectors = true;
                view.push_str(&format!("│ • {:20} ", sector.name));
                if sector.has_outpost {
                    view.push_str("[OUTPOST] ");
                } else {
                    view.push_str("          ");
                }
                view.push_str("                           │\n");
            }
        }
        if !has_sectors {
            view.push_str("│ No sectors under control                                     │\n");
        }
        view.push_str(&format!("└──────────────────────────────────────────────────────────────┘\n\n"));
        
        // Abilities
        view.push_str(&format!("┌─── ABILITIES ────────────────────────────────────────────────┐\n"));
        view.push_str(&format!("│ • Move Fleet (5 AP)          • Attack ({} damage, 8 AP)      │\n", 10 + player.get_damage_bonus()));
        view.push_str(&format!("│ • Scan Sector (3 AP)         • Build Outpost (10 AP)         │\n"));
        if player.level >= 3 {
            view.push_str(&format!("│ • Reinforce (15 AP) - Heal 20 HP                             │\n"));
        }
        if player.level >= 5 {
            view.push_str(&format!("│ • Sabotage (12 AP) - Destroy enemy outpost                   │\n"));
        }
        if player.level >= 7 {
            view.push_str(&format!("│ • Orbital Strike (20 AP) - 30 damage anywhere                │\n"));
        }
        view.push_str(&format!("└──────────────────────────────────────────────────────────────┘\n"));
        
        view
    }
    
    /// Export player view to HTML
    pub fn export_player_html(state: &GameState, player_id: u8) -> Result<String, Box<dyn std::error::Error>> {
        let player = &state.players[player_id as usize];
        let filename = format!("player_{}_view.html", player.name.to_lowercase());
        
        // Generate content
        let protected_content = format!(r#"
        <div class="header">
            <h1>{} {}</h1>
            <p>Turn {} - Solar Command Interface</p>
        </div>
        
        <div class="section">
            <h2>Statistics</h2>
            <div class="stats">
                <div>Level: {} - {}</div>
                <div>Health: {}/{}</div>
                <div>Experience: {}/{}</div>
                <div>Action Points: {}</div>
                <div>Damage Bonus: +{}</div>
                <div>Scan Range: {}</div>
            </div>
        </div>
        
        <div class="section">
            <h2>Fleet Composition</h2>
            <div class="fleet">
                <div>⚡ Scouts: {}</div>
                <div>⚔️ Frigates: {}</div>
                <div>💥 Destroyers: {}</div>
                <div>🏛️ Command Centers: {}</div>
                <div><strong>Total Ships: {}</strong></div>
                <div><strong>Combat Strength: {}</strong></div>
            </div>
        </div>
        
        <div class="section">
            <h2>Controlled Sectors</h2>
            <ul>
                {}
            </ul>
        </div>
        
        <div class="section">
            <h2>Solar System Map</h2>
            <pre class="map">{}</pre>
        </div>
        
        <div class="section">
            <h2>Recent Events</h2>
            <ul>
                {}
            </ul>
        </div>"#,
            player.rank, player.name,
            state.turn_number,
            player.level, player.rank,
            player.health, 100 + (player.level as i32 - 1) * 20,
            player.experience, player.level as u32 * 100,
            player.ap_cap,
            player.get_damage_bonus(),
            if player.get_scan_range_bonus() > 0 { "Extended" } else { "Normal" },
            player.fleet.scouts,
            player.fleet.frigates,
            player.fleet.destroyers,
            player.fleet.command_centers,
            player.fleet.total_ships(),
            player.fleet.combat_strength(),
            state.sectors.iter()
                .filter(|s| s.owner == Some(player_id))
                .map(|s| format!("<li>{}{}</li>", 
                    s.name, 
                    if s.has_outpost { " <span class='outpost'>[OUTPOST]</span>" } else { "" }))
                .collect::<Vec<_>>()
                .join("\n                "),
            Self::generate_map(state).replace("\n", "\n"),
            state.event_log.iter()
                .rev()
                .take(10)
                .map(|e| format!("<li>{}</li>", e))
                .collect::<Vec<_>>()
                .join("\n                ")
        );
        
        // Create password-protected HTML
        let html = format!(r#"<!DOCTYPE html>
<html>
<head>
    <title>{} {}'s Command Dashboard - Password Protected</title>
    <style>
        body {{
            background-color: #0a0a0a;
            color: #00ff00;
            font-family: 'Courier New', monospace;
            padding: 20px;
        }}
        .container {{
            max-width: 800px;
            margin: 0 auto;
        }}
        .header {{
            text-align: center;
            border: 2px solid #00ff00;
            padding: 20px;
            margin-bottom: 20px;
        }}
        .section {{
            border: 1px solid #00ff00;
            padding: 15px;
            margin-bottom: 15px;
        }}
        .stats {{
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 10px;
        }}
        .fleet {{
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 10px;
            margin: 10px 0;
        }}
        .fleet div {{
            padding: 5px;
            background: rgba(0, 255, 0, 0.05);
            border: 1px solid rgba(0, 255, 0, 0.3);
        }}
        .fleet strong {{
            color: #ffff00;
        }}
        .map {{
            text-align: center;
            font-size: 14px;
            line-height: 1.5;
        }}
        h1, h2 {{
            color: #00ff00;
        }}
        .warning {{
            color: #ff0000;
        }}
        .outpost {{
            color: #ffff00;
        }}
        #passwordForm {{
            text-align: center;
            margin-top: 100px;
        }}
        #passwordInput {{
            background-color: #0a0a0a;
            color: #00ff00;
            border: 1px solid #00ff00;
            padding: 10px;
            font-family: 'Courier New', monospace;
            font-size: 16px;
        }}
        #submitButton {{
            background-color: #0a0a0a;
            color: #00ff00;
            border: 2px solid #00ff00;
            padding: 10px 20px;
            font-family: 'Courier New', monospace;
            font-size: 16px;
            cursor: pointer;
            margin-left: 10px;
        }}
        #submitButton:hover {{
            background-color: #00ff00;
            color: #0a0a0a;
        }}
        #errorMessage {{
            color: #ff0000;
            margin-top: 20px;
            display: none;
        }}
        #content {{
            display: none;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div id="passwordForm">
            <h1>🔒 Classified Command Interface</h1>
            <p>This view is password protected</p>
            <p>Enter your player password to access:</p>
            <br>
            <input type="password" id="passwordInput" placeholder="Enter password">
            <button id="submitButton" onclick="checkPassword()">Access</button>
            <div id="errorMessage">❌ Invalid password</div>
        </div>
        
        <div id="content">
            {}
        </div>
    </div>
    
    <script>
        // SHA-256 hash function
        async function sha256(message) {{
            const msgBuffer = new TextEncoder().encode(message);
            const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer);
            const hashArray = Array.from(new Uint8Array(hashBuffer));
            const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
            return hashHex;
        }}
        
        // Stored password hash
        const storedHash = '{}';
        
        async function checkPassword() {{
            const input = document.getElementById('passwordInput').value;
            const errorMsg = document.getElementById('errorMessage');
            
            // No password set allows empty input
            if (storedHash === '' && input === '') {{
                showContent();
                return;
            }}
            
            // Verify hash
            const inputHash = await sha256(input);
            
            if (inputHash === storedHash || (storedHash === '' && input === '')) {{
                showContent();
            }} else {{
                errorMsg.style.display = 'block';
                document.getElementById('passwordInput').value = '';
            }}
        }}
        
        function showContent() {{
            document.getElementById('passwordForm').style.display = 'none';
            document.getElementById('content').style.display = 'block';
        }}
        
        // Enter key support
        document.getElementById('passwordInput').addEventListener('keypress', function(event) {{
            if (event.key === 'Enter') {{
                checkPassword();
            }}
        }});
        
        // Auto-focus
        window.onload = function() {{
            document.getElementById('passwordInput').focus();
        }};
    </script>
</body>
</html>"#,
            player.rank, player.name,
            protected_content,
            player.get_password_hash().unwrap_or(&String::new())
        );
        
        fs::write(&filename, html)?;
        Ok(filename)
    }
    
    /// Generate statistics comparison
    pub fn generate_stats_comparison(state: &GameState) -> String {
        let mut chart = String::new();
        
        chart.push_str("\n╔═══════════════════════════════════════════════════════════════╗\n");
        chart.push_str("║                    PLAYER COMPARISON                          ║\n");
        chart.push_str("╚═══════════════════════════════════════════════════════════════╝\n\n");
        
        chart.push_str("                    Player 1         vs         Player 2\n");
        chart.push_str("─────────────────────────────────────────────────────────────────\n");
        
        let p1 = &state.players[0];
        let p2 = &state.players[1];
        
        chart.push_str(&format!("Name:           {:>15}             {:>15}\n", p1.name, p2.name));
        chart.push_str(&format!("Rank:           {:>15}             {:>15}\n", p1.rank, p2.rank));
        chart.push_str(&format!("Level:          {:>15}             {:>15}\n", p1.level, p2.level));
        chart.push_str(&format!("Health:         {:>15}             {:>15}\n", p1.health, p2.health));
        chart.push_str(&format!("AP Cap:         {:>15}             {:>15}\n", p1.ap_cap, p2.ap_cap));
        chart.push_str(&format!("Damage:         {:>15}             {:>15}\n", 
            format!("+{}", p1.get_damage_bonus()), format!("+{}", p2.get_damage_bonus())));
        
        // Sector control
        let p1_sectors = state.sectors.iter().filter(|s| s.owner == Some(0)).count();
        let p2_sectors = state.sectors.iter().filter(|s| s.owner == Some(1)).count();
        chart.push_str(&format!("Sectors:        {:>15}             {:>15}\n", p1_sectors, p2_sectors));
        
        // Health bars
        chart.push_str("\nHealth Bars:\n");
        chart.push_str(&format!("{:15} [", p1.name));
        let p1_bar_len = (p1.health as f32 / (100.0 + (p1.level as f32 - 1.0) * 20.0) * 20.0) as usize;
        for i in 0..20 {
            if i < p1_bar_len { chart.push('█'); } else { chart.push('░'); }
        }
        chart.push_str(&format!("] {}%\n", (p1.health as f32 / (100.0 + (p1.level as f32 - 1.0) * 20.0) * 100.0) as i32));
        
        chart.push_str(&format!("{:15} [", p2.name));
        let p2_bar_len = (p2.health as f32 / (100.0 + (p2.level as f32 - 1.0) * 20.0) * 20.0) as usize;
        for i in 0..20 {
            if i < p2_bar_len { chart.push('█'); } else { chart.push('░'); }
        }
        chart.push_str(&format!("] {}%\n", (p2.health as f32 / (100.0 + (p2.level as f32 - 1.0) * 20.0) * 100.0) as i32));
        
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
                <div>⭕ Fleet Position</div>
                <div>🏭 Outpost Present</div>
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
            
            let outpost_icon = if sector.has_outpost { "🏭" } else { "" };
            
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