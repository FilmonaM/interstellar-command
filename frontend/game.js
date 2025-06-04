// Main game client
class GameClient {
    constructor() {
        this.ws = null;
        this.playerId = null;
        this.connected = false;
        this.terminal = new Terminal();
        this.map = null; // Will be initialized when map.js loads
        
        this.initializeUI();
    }
    
    initializeUI() {
        // Command input
        const commandInput = document.getElementById('command-input');
        commandInput.addEventListener('keydown', (e) => {
            if (e.key === 'Enter' && this.connected) {
                const command = e.target.value.trim();
                if (command) {
                    this.sendCommand(command);
                    e.target.value = '';
                }
            }
        });
        
        // Quick command buttons
        document.querySelectorAll('.quick-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                const cmd = btn.dataset.cmd;
                commandInput.value = cmd;
                commandInput.focus();
            });
        });
        
        // Connect button
        document.getElementById('connect-btn').addEventListener('click', () => {
            this.handleConnect();
        });
        
        // Test player buttons
        document.querySelectorAll('.test-player-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                this.connectAsPlayer(btn.dataset.id);
            });
        });
        
        // Check for saved player ID
        const savedPlayerId = localStorage.getItem('playerId');
        if (savedPlayerId) {
            document.getElementById('player-id-input').value = savedPlayerId;
        }
    }
    
    async handleConnect() {
        const playerIdInput = document.getElementById('player-id-input').value.trim();
        const playerName = document.getElementById('player-name-input').value.trim();
        
        if (playerIdInput) {
            // Connect with existing ID
            this.connectAsPlayer(playerIdInput);
        } else if (playerName) {
            // Register new player
            try {
                const response = await fetch('/api/register', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ name: playerName })
                });
                
                if (response.ok) {
                    const data = await response.json();
                    this.terminal.print(data.message, 'success-message');
                    this.connectAsPlayer(data.player_id);
                }
            } catch (error) {
                this.terminal.print('Failed to register: ' + error.message, 'error-message');
            }
        } else {
            alert('Please enter either a Player ID or a Commander Name');
        }
    }
    
    connectAsPlayer(playerId) {
        this.playerId = playerId;
        localStorage.setItem('playerId', playerId);
        
        // Hide login modal
        document.getElementById('login-modal').classList.add('hidden');
        
        // Connect WebSocket
        this.connectWebSocket();
    }
    
    connectWebSocket() {
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const wsUrl = `${protocol}//${window.location.host}/ws/${this.playerId}`;
        
        this.terminal.print('Connecting to command server...', 'system-message');
        
        this.ws = new WebSocket(wsUrl);
        
        this.ws.onopen = () => {
            this.connected = true;
            this.terminal.print('Connected to Interstellar Command', 'success-message');
            document.getElementById('command-input').focus();
        };
        
        this.ws.onmessage = (event) => {
            const data = JSON.parse(event.data);
            this.handleServerMessage(data);
        };
        
        this.ws.onclose = () => {
            this.connected = false;
            this.terminal.print('Connection lost. Attempting to reconnect...', 'error-message');
            setTimeout(() => this.connectWebSocket(), 3000);
        };
        
        this.ws.onerror = (error) => {
            this.terminal.print('Connection error', 'error-message');
        };
    }
    
    handleServerMessage(data) {
        switch(data.type) {
            case 'game_update':
            case 'command_result':
                // Update player stats
                if (data.player) {
                    this.updatePlayerStats(data.player);
                }
                
                // Update sector map
                if (data.sectors && this.map) {
                    this.map.updateSectors(data.sectors);
                }
                
                // Show message
                if (data.message) {
                    this.terminal.print(data.message);
                }
                break;
                
            case 'player_update':
                if (data.player) {
                    this.updatePlayerStats(data.player);
                }
                break;
                
            case 'cycle_update':
                if (data.message) {
                    this.terminal.print(data.message, 'success-message');
                }
                if (data.sectors && this.map) {
                    this.map.updateSectors(data.sectors);
                }
                break;
                
            case 'error':
                this.terminal.print(data.message, 'error-message');
                break;
        }
    }
    
    updatePlayerStats(player) {
        document.getElementById('ap').textContent = player.ap;
        document.getElementById('max-ap').textContent = player.max_ap;
        document.getElementById('credits').textContent = player.credits;
        document.getElementById('level').textContent = player.level;
        document.getElementById('ship-count').textContent = player.ship_count;
    }
    
    sendCommand(command) {
        if (!this.connected) {
            this.terminal.print('Not connected to server', 'error-message');
            return;
        }
        
        // Echo command
        this.terminal.print(`$ ${command}`, 'command-echo');
        
        // Send to server
        this.ws.send(JSON.stringify({
            type: 'command',
            content: command
        }));
    }
}

// Terminal handler
class Terminal {
    constructor() {
        this.output = document.getElementById('terminal-output');
        this.maxLines = 100;
    }
    
    print(text, className = '') {
        const line = document.createElement('div');
        line.className = 'terminal-line ' + className;
        
        // Handle multi-line text
        if (text.includes('\n')) {
            line.innerHTML = text.split('\n').map(l => 
                l.replace(/\s/g, '&nbsp;')
            ).join('<br>');
        } else {
            line.textContent = text;
        }
        
        this.output.appendChild(line);
        
        // Limit lines
        while (this.output.children.length > this.maxLines) {
            this.output.removeChild(this.output.firstChild);
        }
        
        // Scroll to bottom
        this.output.scrollTop = this.output.scrollHeight;
    }
    
    clear() {
        this.output.innerHTML = '';
    }
}

// Initialize game when page loads
window.addEventListener('DOMContentLoaded', () => {
    window.gameClient = new GameClient();
}); 