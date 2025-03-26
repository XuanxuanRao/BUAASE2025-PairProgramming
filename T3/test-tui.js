// Import game engine
import { 
  initializeGameState, 
  processGameTurn, 
  isGameOver, 
  getFinalResults 
} from './snake-engine.js';
import blessed from 'blessed';

// Import configuration
import {
  GAME_MODE,
  snakeModules,
  SNAKE_DISPLAY_CONFIG,
  GAME_SYMBOLS,
  CUSTOM_SEED
} from './game-config.js';

// Test parameters
const TEST_MODE = GAME_MODE;

// Use Unicode characters to distinguish different snakes, no color dependency
const SNAKE_SYMBOLS = SNAKE_DISPLAY_CONFIG.SYMBOLS;

// Backup color scheme, better display if terminal supports color
const SNAKE_COLORS = SNAKE_DISPLAY_CONFIG.COLORS;

// Game element symbols
const SYMBOLS = GAME_SYMBOLS;

// Initialize game state
let gameState = initializeGameState(TEST_MODE, snakeModules, CUSTOM_SEED);
let autoPlayInterval = null;
let autoPlaySpeed = 500; // ms between turns in auto mode
// Add storage for persistent messages
let persistentMessages = {
  warnings: [],
  errors: []
};
const MAX_PERSISTENT_MESSAGES = 10; // Keep up to 10 messages

// Detect if terminal supports color
const supportsColor = process.env.FORCE_COLOR !== '0' && 
                       process.env.TERM !== 'dumb' && 
                       process.stdout.isTTY;

// Create blessed screen
const screen = blessed.screen({
  smartCSR: true,
  title: 'Snake Game Visualization',
  fullUnicode: true // Enable full Unicode support
});

// Calculate required width and height based on grid size
const gridWidth = gameState.n * 4 + 3;  // Each cell width is 2, plus borders
const gridHeight = gameState.n * 2 + 3;  // Each cell height is 1, plus borders

// Create game board box
const boardBox = blessed.box({
  top: 0,
  left: 0,
  width: gridWidth,
  height: gridHeight,
  tags: true,
  border: {
    type: 'line'
  },
  style: {
    border: {
      fg: 'white'
    },
    fg: 'white',
    bg: 'black'
  }
});

// Create info panel
const infoBox = blessed.box({
  top: 0,
  left: gridWidth + 1,
  width: 30,
  height: '100%',
  tags: true,
  border: {
    type: 'line'
  },
  style: {
    border: {
      fg: 'white'
    },
    fg: 'white'
  }
});

// Create command panel
const commandBox = blessed.box({
  bottom: 0,
  left: 0,
  width: gridWidth + 1,
  height: 7,
  tags: true,
  border: {
    type: 'line'
  },
  content: ' SPACE: Next Step \n A: Auto Play \n S: Stop \n C: Clear Messages \n Q: Quit',
  style: {
    border: {
      fg: 'white'
    },
    fg: 'white'
  }
});

// Add components to screen
screen.append(boardBox);
screen.append(infoBox);
screen.append(commandBox);

// Key bindings
screen.key(['q', 'C-c'], function() {
  stopAutoPlay();
  return process.exit(0);
});

screen.key('space', function() {
  stopAutoPlay();
  processTurn();
});

screen.key('a', function() {
  startAutoPlay();
});

screen.key('s', function() {
  stopAutoPlay();
});

// Key binding to clear messages
screen.key('c', function() {
  persistentMessages = { warnings: [], errors: [] };
  updateInfo();
});

// Function to process a single turn
function processTurn() {
  if (isGameOver(gameState)) {
    updateInfo("Game Over!");
    return;
  }
  
  try {
    // Process one turn
    const { gameState: newGameState, messages } = processGameTurn(gameState);
    gameState = newGameState;
    
    // Update display
    renderBoard();
    
    // Store the new messages in our persistent messages store
    if (messages) {
      if (messages.warnings.length > 0) {
        // Add round number to each warning
        const warningsWithRound = messages.warnings.map(
          w => `R${gameState.round}: ${w}`
        );
        persistentMessages.warnings = [
          ...persistentMessages.warnings,
          ...warningsWithRound
        ].slice(-MAX_PERSISTENT_MESSAGES); // Keep only the most recent ones
      }
      
      if (messages.errors.length > 0) {
        // Add round number to each error
        const errorsWithRound = messages.errors.map(
          e => `R${gameState.round}: ${e}`
        );
        persistentMessages.errors = [
          ...persistentMessages.errors,
          ...errorsWithRound
        ].slice(-MAX_PERSISTENT_MESSAGES); // Keep only the most recent ones
      }
    }
    
    // Update the info panel with our persistent messages
    updateInfo();
  } catch (error) {
    // Store the error message in persistent errors
    persistentMessages.errors.push(`R${gameState.round}: ${error.message}`);
    updateInfo();
  }
}

// Render the board
function renderBoard() {
  const { n, foods, food_num, snakes, alive_snake_index } = gameState;
  
  // Create 2D board
  const board = Array(n).fill().map(() => Array(n).fill({ type: 'empty' }));
  
  // Place foods
  for (let i = 0; i < food_num; i++) {
    const x = foods[i * 2] - 1;     // Convert to 0-index
    const y = n - foods[i * 2 + 1];  // 反转y坐标使其从底部开始
    if (x >= 0 && x < n && y >= 0 && y < n) {
      board[y][x] = { type: 'food' };
    }
  }
  
  // Place snakes
  for (let i = 0; i < snakes.length; i++) {
    const snake = snakes[i];
    const snakeIndex = alive_snake_index[i];
    
    for (let j = 0; j < snake.length; j += 2) {
      const x = snake[j] - 1;      
      const y = n - snake[j + 1];   // Reverse the y-coordinate so that it starts at the bottom
      
      if (x >= 0 && x < n && y >= 0 && y < n) {
        // First segment is head
        const type = (j === 0) ? 'head' : 'body';
        board[y][x] = { type, snakeIndex };
      }
    }
  }
  
  // Create grid display
  let content = '';
  
  // Draw top border with T-junctions
  content += SYMBOLS.CORNER_TL;
  for (let x = 0; x < n; x++) {
    content += SYMBOLS.HORIZONTAL + SYMBOLS.HORIZONTAL + SYMBOLS.HORIZONTAL;
    if (x < n - 1) content += SYMBOLS.T_DOWN;
  }
  content += SYMBOLS.CORNER_TR + '\n';
  
  // Draw board content with grid lines
  for (let y = 0; y < n; y++) {
    // Draw cell content row
    content += SYMBOLS.VERTICAL; // Left border
    
    for (let x = 0; x < n; x++) {
      const cell = board[y][x];
      let cellContent;
      
      // Render based on cell content
      if (cell.type === 'head') {
        const symbol = SNAKE_SYMBOLS[cell.snakeIndex].head;
        
        // Try to use color with fallback
        if (supportsColor) {
          cellContent = `{${SNAKE_COLORS[cell.snakeIndex]}-fg}${symbol}{/}`;
        } else {
          cellContent = symbol;
        }
      } else if (cell.type === 'body') {
        const symbol = SNAKE_SYMBOLS[cell.snakeIndex].body;
        
        if (supportsColor) {
          cellContent = `{${SNAKE_COLORS[cell.snakeIndex]}-fg}${symbol}{/}`;
        } else {
          cellContent = symbol;
        }
      } else if (cell.type === 'food') {
        if (supportsColor) {
          cellContent = `{red-fg}${SYMBOLS.FOOD}{/}`;
        } else {
          cellContent = SYMBOLS.FOOD;
        }
      } else {
        cellContent = SYMBOLS.EMPTY;
      }
      
      // Add proper padding to make cells consistent
      content += ' ' + cellContent + ' ';
      // Add vertical divider between cells
      if (x < n - 1) content += SYMBOLS.VERTICAL;
    }
    
    content += SYMBOLS.VERTICAL + '\n'; // Right border
    
    // Add horizontal divider between rows
    if (y < n - 1) {
      content += SYMBOLS.T_RIGHT; // Left T-junction
      for (let x = 0; x < n; x++) {
        content += SYMBOLS.HORIZONTAL + SYMBOLS.HORIZONTAL + SYMBOLS.HORIZONTAL;
        // Intersection point
        if (x < n - 1) {
          content += SYMBOLS.CROSS;
        }
      }
      content += SYMBOLS.T_LEFT + '\n'; // Right T-junction
    }
  }
  
  // Draw bottom border with T-up junctions
  content += SYMBOLS.CORNER_BL;
  for (let x = 0; x < n; x++) {
    content += SYMBOLS.HORIZONTAL + SYMBOLS.HORIZONTAL + SYMBOLS.HORIZONTAL;
    if (x < n - 1) content += SYMBOLS.T_UP;
  }
  content += SYMBOLS.CORNER_BR;
  
  boardBox.setContent(content);
  screen.render();
}

// Update info panel
function updateInfo(gameOverMessage = '') {
  const { round, max_rounds, alive_snake_num, snake_num, scores, alive, dead_round, time, seed } = gameState;
  
  let content = `Round: ${round}/${max_rounds}\n\n`;
  content += `Alive: ${alive_snake_num}/${snake_num}\n`;
  // Format the BigInt seed in hexadecimal for better display
  content += `Seed: 0x${seed.toString(16).padStart(16, '0')}\n\n`;
  
  content += 'Scores:\n';
  for (let i = 0; i < snake_num; i++) {
    // Add the same symbol as in the board for identification
    const symbol = SNAKE_SYMBOLS[i].head;
    const status = alive[i] ? 'Alive' : `Dead R${dead_round[i]}`;
    
    content += `Snake${symbol}: ${scores[i]} (${status})\n`;
    content += `Time: ${time[i].toFixed(3)}ms\n`;
  }
  
  // Add legend explanation
  content += '\nLegend:\n';
  for (let i = 0; i < Math.min(snake_num, 4); i++) {
    content += `${SNAKE_SYMBOLS[i].head}: Snake${i+1} Head\n`;
  }
  content += `${SNAKE_SYMBOLS[0].body}: Snake Body\n`;
  content += `${SYMBOLS.FOOD}: Food\n`;
  
  // Add persistent messages section
  if (persistentMessages.warnings.length > 0 || persistentMessages.errors.length > 0) {
    content += '\nStatus Messages:\n';
    
    if (persistentMessages.warnings.length > 0) {
      content += `Warnings:\n`;
      persistentMessages.warnings.forEach(w => {
        content += `- ${w}\n`;
      });
    }
    
    if (persistentMessages.errors.length > 0) {
      content += `Errors:\n`;
      persistentMessages.errors.forEach(e => {
        content += `- ${e}\n`;
      });
    }
  }
  
  // Add game over message if provided
  if (gameOverMessage) {
    content += `\n${gameOverMessage}\n`;
  }
  
  infoBox.setContent(content);
  screen.render();
}

function startAutoPlay() {
  if (autoPlayInterval) return;
  
  autoPlayInterval = setInterval(() => {
    if (!isGameOver(gameState)) {
      processTurn();
    } else {
      stopAutoPlay();
    }
  }, autoPlaySpeed);
}

function stopAutoPlay() {
  if (autoPlayInterval) {
    clearInterval(autoPlayInterval);
    autoPlayInterval = null;
  }
}

// Initial render
renderBoard();
updateInfo('Press SPACE to start');


