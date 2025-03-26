// Import game engine
import { 
  initializeGameState, 
  processGameTurn, 
  isGameOver, 
  getFinalResults 
} from './snake-engine.js';

// Import configuration
import {
  GAME_MODE,
  snakeModules,
  CUSTOM_SEED
} from './game-config.js';

// Test parameters
const TEST_MODE = GAME_MODE;

// Initialize game state
let gameState = initializeGameState(TEST_MODE, snakeModules, CUSTOM_SEED);

// Main simulation loop
console.log(`Starting ${TEST_MODE} mode with ${gameState.snake_num} snakes, board size ${gameState.n}x${gameState.n}, ${gameState.food_num} foods, ${gameState.max_rounds} rounds`);
console.log(`Game seed: 0x${gameState.seed.toString(16).padStart(16, '0')}`)

while (!isGameOver(gameState)) {
  
  try {
    // Process one turn and get any messages
    const { gameState: newGameState, messages } = processGameTurn(gameState);
    gameState = newGameState;
    
    // Display any warnings or errors
    if (messages && messages.warnings.length > 0) {
      console.warn("Warnings in this turn:");
      messages.warnings.forEach(warning => console.warn(`- ${warning}`));
    }
    
    if (messages && messages.errors.length > 0) {
      console.error("Errors in this turn:");
      messages.errors.forEach(error => console.error(`- ${error}`));
    }

    // If necessary, output the intermediate state through the following lines
    // console.log(`\nRound ${gameState.round + 1}/${gameState.max_rounds}:`);
    // console.log("Current board state:");
    // printBoardState(gameState);
  } catch (error) {
    console.error("Game error:", error);
    process.exit(1);
  }
}

// Display functions
function printBoardState(gameState) {
  const { alive, snake_num, scores } = gameState;
  console.log(`Living snakes ${alive.filter(Boolean).length}/${snake_num}: ${alive.join(', ')}`);
  console.log(`Scores: ${scores.join(', ')}`);
}

// Get the final result
const finalResults = getFinalResults(gameState);

// Final results
console.log("\n=== FINAL RESULTS ===");
console.log(`Snake scores:`);
for (let i = 0; i < gameState.snake_num; i++) {
  console.log(`Snake ${i + 1}: ${finalResults.scores[i]} points${finalResults.alive[i] ? " (survived)" : " (died in round " + (finalResults.dead_round[i]) + ")"} spent ${finalResults.time[i].toFixed(3)}ms`);
}
