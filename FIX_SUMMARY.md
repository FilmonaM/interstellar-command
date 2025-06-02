# Fix Summary

## Issues Fixed

### 1. Pacing Issues (AP Cap)
- **Problem**: Starting with 25 AP at level 1 was too much
- **Solution**: Reduced starting AP to 15 and made progression more gradual:
  - Level 1: 15 AP
  - Level 2: 17 AP (+2)
  - Level 3: 19 AP (+2)
  - Level 4: 21 AP (+2) - Critical level for Command Center
  - Level 5: 23 AP (+2)
  - Level 6: 25 AP (+2)
  - Level 7: 28 AP (+3)
  - Level 8: 30 AP (+2)
  - Level 9: 33 AP (+3)
  - Level 10: 35 AP (+2)

### 2. ASCII Map Display Issues
- **Problem**: Map was misaligned and too complex with box-drawing characters
- **Solution**: Completely rewrote the map display:
  - Uses simple ASCII characters: `+ - |`
  - Clean grid layout for larger maps
  - Simplified sector control display: `P1/P2/--`
  - Better alignment and consistent formatting
  - Three map layouts: 5-sector tactical, 8-sector tactical, 17-sector strategic

### 3. HTML Export Issues
- **Problem**: HTML files used fancy CSS and Unicode that didn't convert well with html2text
- **Solution**: 
  - Now exports both `.txt` and `.html` files
  - Plain text format that works perfectly with html2text
  - Simple monospace HTML with minimal styling
  - No Unicode characters or complex CSS
  - Password protection removed for simpler text output

### 4. Box Drawing Characters
- **Problem**: Box-drawing characters (╔═╗║├┤└┘) throughout the interface
- **Solution**: Replaced all with simple ASCII:
  - `╔═╗` → `+=+`
  - `║` → `|`
  - `└┘` → `++`
  - All menus now use simple `+ - |` characters

### 5. Enhanced play.sh Script
- **Problem**: No way to check game status without logging in
- **Solution**: Added `./play.sh status` command that shows:
  - Current turn number
  - Which player's turn it is
  - Turn phase if available
  - No password required

## New Features Added

1. **Better Status Display**: Cleaner formatting throughout
2. **Text Export**: Player views now export as plain `.txt` files
3. **Gradual Progression**: More balanced AP progression curve
4. **Cleaner Maps**: Three distinct map layouts with clear visualization

## Files Modified

- `src/core/player.rs` - AP progression changes
- `src/visualization.rs` - Complete rewrite of map and export functions
- `src/main.rs` - Replaced box-drawing characters
- `src/core/game.rs` - Replaced box-drawing characters
- `play.sh` - Added status command
- `HOW_TO_PLAY.md` - Updated AP progression table

## Testing

The game now:
- Compiles without errors
- Displays clean ASCII art without Unicode
- Exports text-friendly files
- Has better early-game pacing
- Shows maps that align properly 