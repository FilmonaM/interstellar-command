-- MySQL Schema for Interstellar Command
-- Ready for future migration from JSON to MySQL

CREATE DATABASE IF NOT EXISTS interstellar_command;
USE interstellar_command;

-- Game states table
CREATE TABLE game_states (
    id INT AUTO_INCREMENT PRIMARY KEY,
    turn_number INT NOT NULL,
    current_player TINYINT NOT NULL,
    game_over BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_game_over (game_over),
    INDEX idx_updated (updated_at)
);

-- Players table
CREATE TABLE players (
    id INT AUTO_INCREMENT PRIMARY KEY,
    game_id INT NOT NULL,
    player_id TINYINT NOT NULL,
    name VARCHAR(50) NOT NULL,
    password_hash VARCHAR(64),
    health INT NOT NULL,
    ap_cap TINYINT NOT NULL,
    ap_remaining TINYINT NOT NULL,
    current_sector TINYINT NOT NULL,
    level TINYINT NOT NULL DEFAULT 1,
    experience INT NOT NULL DEFAULT 0,
    rank VARCHAR(30) NOT NULL DEFAULT 'Legionnaire',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (game_id) REFERENCES game_states(id) ON DELETE CASCADE,
    INDEX idx_game_player (game_id, player_id),
    INDEX idx_name (name)
);

-- Sectors table
CREATE TABLE sectors (
    id INT AUTO_INCREMENT PRIMARY KEY,
    game_id INT NOT NULL,
    sector_id TINYINT NOT NULL,
    name VARCHAR(50) NOT NULL,
    owner_id TINYINT,
    has_outpost BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (game_id) REFERENCES game_states(id) ON DELETE CASCADE,
    INDEX idx_game_sector (game_id, sector_id),
    INDEX idx_owner (owner_id)
);

-- Sector adjacency table
CREATE TABLE sector_adjacency (
    id INT AUTO_INCREMENT PRIMARY KEY,
    game_id INT NOT NULL,
    sector_id TINYINT NOT NULL,
    adjacent_sector_id TINYINT NOT NULL,
    FOREIGN KEY (game_id) REFERENCES game_states(id) ON DELETE CASCADE,
    INDEX idx_adjacency (game_id, sector_id)
);

-- Sector visibility table
CREATE TABLE sector_visibility (
    id INT AUTO_INCREMENT PRIMARY KEY,
    game_id INT NOT NULL,
    sector_id TINYINT NOT NULL,
    player_id TINYINT NOT NULL,
    visible_since TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (game_id) REFERENCES game_states(id) ON DELETE CASCADE,
    INDEX idx_visibility (game_id, sector_id, player_id)
);

-- Event logs table
CREATE TABLE event_logs (
    id INT AUTO_INCREMENT PRIMARY KEY,
    game_id INT NOT NULL,
    turn_number INT NOT NULL,
    event_text TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (game_id) REFERENCES game_states(id) ON DELETE CASCADE,
    INDEX idx_game_turn (game_id, turn_number)
);

-- Player views table (for password-protected dashboards)
CREATE TABLE player_views (
    id INT AUTO_INCREMENT PRIMARY KEY,
    game_id INT NOT NULL,
    player_id TINYINT NOT NULL,
    view_type VARCHAR(20) NOT NULL, -- 'dashboard', 'html', 'stats'
    view_data LONGTEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (game_id) REFERENCES game_states(id) ON DELETE CASCADE,
    INDEX idx_player_view (game_id, player_id, view_type)
);

-- Game statistics table for analytics
CREATE TABLE game_statistics (
    id INT AUTO_INCREMENT PRIMARY KEY,
    game_id INT NOT NULL,
    player_id TINYINT NOT NULL,
    turn_number INT NOT NULL,
    sectors_controlled INT DEFAULT 0,
    total_damage_dealt INT DEFAULT 0,
    total_damage_taken INT DEFAULT 0,
    actions_taken INT DEFAULT 0,
    xp_gained INT DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (game_id) REFERENCES game_states(id) ON DELETE CASCADE,
    INDEX idx_stats (game_id, player_id, turn_number)
);

-- Create views for easier querying

-- Active games view
CREATE VIEW active_games AS
SELECT 
    g.id,
    g.turn_number,
    g.current_player,
    p1.name as player1_name,
    p2.name as player2_name,
    p1.level as player1_level,
    p2.level as player2_level,
    p1.health as player1_health,
    p2.health as player2_health,
    g.updated_at
FROM game_states g
JOIN players p1 ON g.id = p1.game_id AND p1.player_id = 0
JOIN players p2 ON g.id = p2.game_id AND p2.player_id = 1
WHERE g.game_over = FALSE
ORDER BY g.updated_at DESC;

-- Player rankings view
CREATE VIEW player_rankings AS
SELECT 
    p.name,
    p.rank,
    p.level,
    COUNT(DISTINCT p.game_id) as games_played,
    SUM(CASE WHEN g.game_over = TRUE AND p.health > 0 THEN 1 ELSE 0 END) as wins,
    MAX(p.level) as highest_level,
    SUM(s.sectors_controlled) as total_sectors_controlled
FROM players p
JOIN game_states g ON p.game_id = g.id
LEFT JOIN game_statistics s ON p.game_id = s.game_id AND p.player_id = s.player_id
GROUP BY p.name
ORDER BY wins DESC, highest_level DESC; 