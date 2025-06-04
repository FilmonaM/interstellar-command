// Sector map visualization
class SectorMap {
    constructor(canvas) {
        this.canvas = canvas;
        this.ctx = canvas.getContext('2d');
        this.sectors = [];
        this.selectedSector = null;
        this.sectorSize = 90;
        this.padding = 10;
        
        // Colors
        this.colors = {
            neutral: '#000000',
            controlled: '#003300',
            enemyControlled: '#330000',
            border: '#00ff00',
            enemyBorder: '#ff0000',
            text: '#00ff00',
            enemyText: '#ff0000',
            selectedBorder: '#ffff00'
        };
        
        this.setupEventListeners();
    }
    
    setupEventListeners() {
        this.canvas.addEventListener('click', (e) => {
            const rect = this.canvas.getBoundingClientRect();
            const x = e.clientX - rect.left;
            const y = e.clientY - rect.top;
            this.handleClick(x, y);
        });
        
        this.canvas.addEventListener('mousemove', (e) => {
            const rect = this.canvas.getBoundingClientRect();
            const x = e.clientX - rect.left;
            const y = e.clientY - rect.top;
            this.handleHover(x, y);
        });
    }
    
    updateSectors(sectorData) {
        this.sectors = sectorData;
        this.draw();
    }
    
    draw() {
        // Clear canvas
        this.ctx.fillStyle = '#000000';
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
        
        // Draw grid lines
        this.ctx.strokeStyle = '#004400';
        this.ctx.lineWidth = 1;
        
        // Vertical lines
        for (let i = 0; i <= 4; i++) {
            const x = i * (this.sectorSize + this.padding) + this.padding / 2;
            this.ctx.beginPath();
            this.ctx.moveTo(x, 0);
            this.ctx.lineTo(x, this.canvas.height);
            this.ctx.stroke();
        }
        
        // Horizontal lines
        for (let i = 0; i <= 4; i++) {
            const y = i * (this.sectorSize + this.padding) + this.padding / 2;
            this.ctx.beginPath();
            this.ctx.moveTo(0, y);
            this.ctx.lineTo(this.canvas.width, y);
            this.ctx.stroke();
        }
        
        // Draw sectors
        this.sectors.forEach(sector => {
            this.drawSector(sector);
        });
        
        // Draw selected sector highlight
        if (this.selectedSector) {
            const sector = this.sectors.find(s => s.id === this.selectedSector);
            if (sector) {
                this.highlightSector(sector);
            }
        }
    }
    
    drawSector(sector) {
        const x = sector.position[0] * (this.sectorSize + this.padding) + this.padding;
        const y = sector.position[1] * (this.sectorSize + this.padding) + this.padding;
        
        // Determine ownership
        const isControlled = sector.controlled_by !== null;
        const isOwnSector = sector.controlled_by === window.gameClient?.playerId;
        
        // Background
        if (isControlled) {
            this.ctx.fillStyle = isOwnSector ? this.colors.controlled : this.colors.enemyControlled;
        } else {
            this.ctx.fillStyle = this.colors.neutral;
        }
        this.ctx.fillRect(x, y, this.sectorSize, this.sectorSize);
        
        // Border
        this.ctx.strokeStyle = isControlled && !isOwnSector ? this.colors.enemyBorder : this.colors.border;
        this.ctx.lineWidth = 2;
        this.ctx.strokeRect(x, y, this.sectorSize, this.sectorSize);
        
        // Text color
        this.ctx.fillStyle = isControlled && !isOwnSector ? this.colors.enemyText : this.colors.text;
        this.ctx.font = '14px monospace';
        
        // Sector name
        this.ctx.fillText(sector.name, x + 5, y + 20);
        
        // Ship count
        if (sector.ship_count > 0) {
            this.ctx.font = '12px monospace';
            this.ctx.fillText(`Ships: ${sector.ship_count}`, x + 5, y + 40);
        }
        
        // Garrison indicator
        if (sector.has_garrison) {
            this.ctx.fillText('[G]', x + 5, y + 55);
        }
        
        // Control indicator
        if (isControlled) {
            this.ctx.font = '10px monospace';
            this.ctx.fillText(isOwnSector ? '[ALLIED]' : '[ENEMY]', x + 5, y + 75);
        }
    }
    
    highlightSector(sector) {
        const x = sector.position[0] * (this.sectorSize + this.padding) + this.padding;
        const y = sector.position[1] * (this.sectorSize + this.padding) + this.padding;
        
        this.ctx.strokeStyle = this.colors.selectedBorder;
        this.ctx.lineWidth = 3;
        this.ctx.strokeRect(x - 2, y - 2, this.sectorSize + 4, this.sectorSize + 4);
    }
    
    handleClick(mouseX, mouseY) {
        const sector = this.getSectorAtPosition(mouseX, mouseY);
        
        if (sector) {
            this.selectedSector = sector.id;
            this.draw();
            
            // Update sector info
            const infoDiv = document.getElementById('sector-info');
            infoDiv.textContent = `Selected: ${sector.name} (${sector.id})`;
            
            // Add sector ID to command input for convenience
            const commandInput = document.getElementById('command-input');
            if (commandInput.value.endsWith(' ')) {
                commandInput.value += sector.id;
            }
        }
    }
    
    handleHover(mouseX, mouseY) {
        const sector = this.getSectorAtPosition(mouseX, mouseY);
        this.canvas.style.cursor = sector ? 'pointer' : 'crosshair';
    }
    
    getSectorAtPosition(mouseX, mouseY) {
        for (const sector of this.sectors) {
            const x = sector.position[0] * (this.sectorSize + this.padding) + this.padding;
            const y = sector.position[1] * (this.sectorSize + this.padding) + this.padding;
            
            if (mouseX >= x && mouseX <= x + this.sectorSize &&
                mouseY >= y && mouseY <= y + this.sectorSize) {
                return sector;
            }
        }
        return null;
    }
}

// Initialize map when page loads
window.addEventListener('DOMContentLoaded', () => {
    const canvas = document.getElementById('sector-map');
    const map = new SectorMap(canvas);
    
    // Attach to game client
    if (window.gameClient) {
        window.gameClient.map = map;
    }
}); 