// Import the board WASM module
import { process_turn, initial_foods } from './board/pkg/board.js';
import { __wasm } from './board/pkg/board.js';
import { gameParameters, CUSTOM_SEED } from './game-config.js';

let memory = __wasm.memory;

/**
 * Initialize game state
 * @param {string} mode Game mode: "1v1", "4snakes", or "custom"
 * @param {Array} snakeModules List of snake decision functions
 * @param {BigInt|undefined} customSeed Optional custom random seed (64-bit)
 * @returns {Object} Game state
 */
export function initializeGameState(mode = "4snakes", snakeModules, customSeed = CUSTOM_SEED) {
  // Get game parameters from configuration
  const config = gameParameters[mode] || gameParameters["4snakes"];
  let n = config.boardSize;
  let snake_num = config.snakeCount;
  let food_num = config.foodCount;
  let max_rounds = config.maxRounds;
  
  // Initialize snake positions
  let snakes = [...config.initialSnakePositions];

  // Random seed for food generation - generate 64-bit BigInt
  let seed;
  if (customSeed !== undefined) {
    seed = BigInt(customSeed);
  } else {
    // Generate 64-bit random seed using two 32-bit values
    const high32 = BigInt(Math.floor(Math.random() * 0x100000000)) << BigInt(32);
    const low32 = BigInt(Math.floor(Math.random() * 0x100000000));
    seed = high32 | low32;
  }
  
  // Initialize food positions with 64-bit seed
  let foods = initial_foods(n, snakes.flat(), seed, food_num);

  // Limit snake count to not exceed snake modules count
  snake_num = Math.min(snake_num, snakeModules.length);
  snakes = snakes.filter((_, i) => i < snake_num);

  // Initialize scores for each snake
  let scores = new Array(snake_num).fill(0);
  let dead_round = new Array(snake_num).fill(max_rounds);

  let alive = new Array(snake_num).fill(true);
  let alive_snake_num = alive.filter(Boolean).length;
  let alive_snake_index = alive.map((a, i) => a ? i : -1).filter(i => i !== -1);

  // Timing
  let time = new Array(snake_num).fill(0);
  const time_limit = 500; // 500 ms time limit

  return {
    n,
    snake_num,
    food_num,
    max_rounds,
    seed,  // Store the full 64-bit seed
    snakes,
    foods,
    scores,
    dead_round,
    alive,
    alive_snake_num,
    alive_snake_index,
    time,
    time_limit,
    round: 0,
    snakeModules
  };
}

/**
 * Process a game turn
 * @param {Object} gameState Current game state
 * @returns {Object} Updated game state, turn results and any warnings/errors
 */
export function processGameTurn(gameState) {
  const { 
    n, snake_num, food_num, round, snakes, foods, scores, 
    dead_round, alive, alive_snake_index, time, time_limit, snakeModules 
  } = gameState;
  
  let alive_snake_num = gameState.alive_snake_num;
  let warnings = [];
  let errors = [];
  
  // Prepare input data
  const input = prepareBoardInput(gameState, warnings, errors);
  
  // Process turn - now pass 64-bit seed directly
  const output = process_turn(
    input.n,
    input.snakes,
    input.actions,
    input.foods,
    input.seed
  );

  // Update game state from output
  const status = new Int32Array(memory.buffer, output.status(), alive_snake_num);
  const updatedSnakes = new Int32Array(memory.buffer, output.snakes(), alive_snake_num * 8);
  const updatedFoods = new Int32Array(memory.buffer, output.foods(), food_num * 2);

  // Process snake updates
  let aliveIndex = 0;
  const newAlive = [...alive];
  const newScores = [...scores];
  const newDeadRound = [...dead_round];
  const newSnakes = [...snakes];
  
  for (let i = 0; i < alive_snake_num; i++) {
    if (newAlive[alive_snake_index[i]]) {
      // Process snake status update
      const snakeStatus = status[i];

      if (snakeStatus >= 0) {
        // Snake is still alive, update position
        for (let j = 0; j < 8; j++) {
          newSnakes[aliveIndex][j] = updatedSnakes[i * 8 + j];
        }
        // If snake ate food, update score
        if (snakeStatus === 1) {
          newScores[alive_snake_index[i]] += 1;
        }
        aliveIndex++;
      } else {
        // Snake died this turn
        newAlive[alive_snake_index[i]] = false;
        newDeadRound[alive_snake_index[i]] = round + 1;
      }
    }
    // Dead snakes remain unchanged
  }
  
  // Update indices of alive snakes
  const newAliveSnakeNum = newAlive.filter(Boolean).length;
  const newAliveSnakeIndex = newAlive.map((a, i) => a ? i : -1).filter(i => i !== -1);
  
  // Filter out dead snakes
  const finalSnakes = newSnakes.filter((_, i) => i < newAliveSnakeNum);

  // Update food positions
  const newFoods = [...foods];
  for (let i = 0; i < food_num * 2; i++) {
    newFoods[i] = updatedFoods[i];
  }

  // Return updated game state
  const newGameState = {
    ...gameState,
    round: round + 1,
    snakes: finalSnakes,
    foods: newFoods,
    scores: newScores,
    dead_round: newDeadRound,
    alive: newAlive,
    alive_snake_num: newAliveSnakeNum,
    alive_snake_index: newAliveSnakeIndex
  };

  return {
    gameState: newGameState,
    turnResult: {
      status,
      updatedSnakes,
      updatedFoods
    },
    messages: {
      warnings,
      errors
    }
  };
}

/**
 * Prepare board input data
 * @param {Object} gameState Current game state
 * @param {Array} warnings Array to collect warning messages 
 * @param {Array} errors Array to collect error messages
 * @returns {Object} Board input data
 */
function prepareBoardInput(gameState, warnings = [], errors = []) {
  const { 
    n, max_rounds, round, snakes, foods, food_num, 
    seed, alive_snake_num, alive_snake_index, time, 
    time_limit, snakeModules 
  } = gameState;
  
  // Calculate and add snake decisions
  const actions = [];
  for (let i = 0; i < alive_snake_num; i++) {
    // Collect other alive snakes
    const otherSnakes = [];
    let otherSnakeCount = alive_snake_num - 1;
    
    for (let j = 0; j < alive_snake_num; j++) {
      if (j !== i) {
        otherSnakes.push(...snakes[j]);
      }
    }
    
    // Get snake action
    const remainingRounds = max_rounds - round;
    let action;
    
    try {
      const startTime = performance.now();
      
      // Directly call snake decision function
      action = snakeModules[alive_snake_index[i]](
        n,
        snakes[i],
        otherSnakeCount,
        otherSnakes,
        food_num,
        foods,
        remainingRounds
      );
      
      const elapsedTime = performance.now() - startTime;
      time[alive_snake_index[i]] += elapsedTime;

      // Check if timed out
      if (elapsedTime > time_limit) {
        const warningMsg = `Snake ${alive_snake_index[i] + 1} timed out: ${elapsedTime.toFixed(3)}ms`;
        warnings.push(warningMsg);
        
        // Determine previous direction based on head and neck position
        const [headX, headY, neckX, neckY] = snakes[i].slice(0, 4);
        if (headX === neckX) {
          action = headY > neckY ? 0 : 2;
        } else {
          action = headX > neckX ? 3 : 1;
        }
      }
    } catch (error) {
      // Error handling
      const errorMsg = `⚠️ Snake ${alive_snake_index[i] + 1} fatal error: ${error.message}`;
      errors.push(errorMsg);
      throw error;
    }
    
    actions.push(action);
  }
  
  // Use 64-bit seed directly
  return {
    n: n,
    snakes: new Int32Array(snakes.flat()),
    actions: new Int32Array(actions),
    foods: new Int32Array(foods),
    seed: gameState.seed,
  };
}

/**
 * Check if game is over
 * @param {Object} gameState Current game state
 * @returns {boolean} Whether game is over
 */
export function isGameOver(gameState) {
  return gameState.round >= gameState.max_rounds || gameState.alive.every(s => !s);
}

/**
 * Get final game results
 * @param {Object} gameState Current game state
 * @returns {Object} Game result data
 */
export function getFinalResults(gameState) {
  const { scores, alive, dead_round, time, max_rounds, snake_num } = gameState;
  
  return {
    scores,
    alive,
    dead_round,
    time
  };
}
