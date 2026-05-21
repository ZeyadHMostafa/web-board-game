import { BoardMatrix } from './boardMatrix';

export const EngineAdapterMock = {
  /**
   * Generates a clean, starting 2D board matrix layout using our BoardMatrix utility
   */
  getInitialBoard() {
    return BoardMatrix.createInitialPosition();
  },

  /**
   * Rule validator: Inspects indices and turns to prevent illegal or out-of-turn actions
   * @param {Array} board - The current 8x8 structural board matrix state
   * @param {Object} from - Grid source coordinates { row, col }
   * @param {Object} to - Grid target coordinates { row, col }
   * @param {number} currentPlayer - Active player code identifier (0 = White, 1 = Black)
   * @returns {boolean} True if the action adheres to the basic turn validation rules
   */
  isValidMove(board, from, to, currentPlayer) {
    // 1. Boundary Guard: Ensure target coordinates are within grid limits
    if (to.row < 0 || to.row > 7 || to.col < 0 || to.col > 7) return false;
    
    // 2. Identity Guard: Fetch the piece from the matrix source index
    const piece = BoardMatrix.getPiece(board, from.row, from.col);
    if (!piece) return false;

    // 3. Turn Enforcement: Ensure player is only manipulating assets belonging to them
    const friendlyPiece = currentPlayer === 0 ? 'W' : 'B';
    if (piece !== friendlyPiece) return false;

    // 4. No Self-Sabotage: Cannot land on a square occupied by your own piece
    const targetPiece = BoardMatrix.getPiece(board, to.row, to.col);
    if (targetPiece === friendlyPiece) return false;

    // 5. Stationary Guard: Moving to the exact same square is invalid
    if (from.row === to.row && from.col === to.col) return false;

    // 6. Evaluate Movement Variants
    if (this._checkDiagonalMove(board, from, to, friendlyPiece)) return true;
    if (this._checkPivotalMove(board, from, to, friendlyPiece)) return true;

    return false;
  },

  /**
   * Validates diagonal jumping over continuous chains of friendly pieces
   */
  _checkDiagonalMove(board, from, to, friendlyPiece) {
    const rowDiff = to.row - from.row;
    const colDiff = to.col - from.col;

    // Must be a strict diagonal line
    if (Math.abs(rowDiff) !== Math.abs(colDiff)) return false;

    const steps = Math.abs(rowDiff);
    const rowDir = rowDiff > 0 ? 1 : -1;
    const colDir = colDiff > 0 ? 1 : -1;

    if (steps === 1) {
      return false; // Single-step diagonals are not valid moves in this game
    }

    // Scan every intermediary square along the diagonal line
    for (let i = 1; i < steps; i++) {
      const checkRow = from.row + i * rowDir;
      const checkCol = from.col + i * colDir;
      const stepPiece = BoardMatrix.getPiece(board, checkRow, checkCol);

      // Every single jumped tile MUST contain a friendly piece
      if (stepPiece !== friendlyPiece) return false;
    }

    return true;
  },
/**
   * Validates 90/180/270 degree orbital rotations around adjacent friendly pieces.
   * Checks both Clockwise and Counter-Clockwise paths to ensure blocker detection is accurate.
   */
  _checkPivotalMove(board, from, to, friendlyPiece) {
    // Define relative orthogonal neighbors (Up, Down, Left, Right)
    const neighbors = [
      { row: -1, col: 0 },
      { row: 1, col: 0 },
      { row: 0, col: -1 },
      { row: 0, col: 1 }
    ];

    // Check all 4 possible adjacent pivot spots
    for (const n of neighbors) {
      const pivotRow = from.row + n.row;
      const pivotCol = from.col + n.col;

      // Ensure pivot coordinates stay within board boundaries
      if (pivotRow < 0 || pivotRow > 7 || pivotCol < 0 || pivotCol > 7) continue;

      // A valid pivot point must contain a friendly piece
      if (BoardMatrix.getPiece(board, pivotRow, pivotCol) !== friendlyPiece) continue;

      // Calculate relative vector from the pivot center to our starting tile
      const startRelRow = from.row - pivotRow;
      const startRelCol = from.col - pivotCol;

      // Test all geometric landing transformations
      // Note: A 270° Clockwise landing spot is the same as a 90° Counter-Clockwise spot.
      const rotationTargets = [
        { 
          // 90° Clockwise / -270° Counter-Clockwise target
          r: -startRelCol, c: startRelRow, 
          cwDegrees: 90, ccwDegrees: 270 
        },
        { 
          // 180° Half-turn target (same path length either way)
          r: -startRelRow, c: -startRelCol, 
          cwDegrees: 180, ccwDegrees: 180 
        },
        { 
          // 270° Clockwise / -90° Counter-Clockwise target
          r: startRelCol,  c: -startRelRow, 
          cwDegrees: 270, ccwDegrees: 90 
        }
      ];

      for (const target of rotationTargets) {
        const expectedRow = pivotRow + target.r;
        const expectedCol = pivotCol + target.c;

        // If this rotation mapping matches our intended destination coordinates
        if (to.row === expectedRow && to.col === expectedCol) {
          
          // Path Option A: Check the Clockwise sweep path
          const cwClear = this._isPathClear(board, pivotRow, pivotCol, startRelRow, startRelCol, target.cwDegrees, true);
          if (cwClear) return true;

          // Path Option B: Check the Counter-Clockwise sweep path
          const ccwClear = this._isPathClear(board, pivotRow, pivotCol, startRelRow, startRelCol, target.ccwDegrees, false);
          if (ccwClear) return true;
        }
      }
    }

    return false;
  },

  /**
   * Sweeps sequential 90-degree quadrant steps either Clockwise or Counter-Clockwise
   */
  _isPathClear(board, pivotRow, pivotCol, startRelRow, startRelCol, totalDegrees, isClockwise) {
    const steps = totalDegrees / 90;
    let currentRelRow = startRelRow;
    let currentRelCol = startRelCol;

    for (let step = 0; step < steps; step++) {
      // Calculate next orthogonal vector based on rotation direction
      // CW:  (row, col) -> (-col, row)
      // CCW: (row, col) -> (col, -row)
      const nextRelRow = isClockwise ? -currentRelCol : currentRelCol;
      const nextRelCol = isClockwise ? currentRelRow : -currentRelRow;

      const targetRow = pivotRow + nextRelRow;
      const targetCol = pivotCol + nextRelCol;

      // 1. Check the 90-degree landing/corner tile
      // Exception: The ultimate destination tile of the complete move can hold an enemy
      if (step < steps - 1) {
        if (targetRow < 0 || targetRow > 7 || targetCol < 0 || targetCol > 7) return false;
        if (BoardMatrix.getPiece(board, targetRow, targetCol) !== null) return false;
      }

      // 2. Check the inner diagonal tile being crossed through during the arc swing
      const diagonalRow = pivotRow + currentRelRow + nextRelRow;
      const diagonalCol = pivotCol + currentRelCol + nextRelCol;

      if (diagonalRow >= 0 && diagonalRow <= 7 && diagonalCol >= 0 && diagonalCol <= 7) {
        if (BoardMatrix.getPiece(board, diagonalRow, diagonalCol) !== null) return false;
      }

      // Cycle parameters forward for the next quadrant step
      currentRelRow = nextRelRow;
      currentRelCol = nextRelCol;
    }

    return true;
  },
/**
   * Mock evluator for AI 
   * Evaluates the board dynamically to find up to 3 legal moves for the current player.
   * Scans local tile radii and tests them against the rules engine.
   */
  getMockAIAssistMoves(board, currentPlayer) {
    const validMoves = [];
    const friendlyPiece = currentPlayer === 0 ? 'W' : 'B';

    // 1. Scan the 8x8 matrix to find any squares containing the active player's pieces
    for (let r = 0; r < 8; r++) {
      for (let c = 0; c < 8; c++) {
        // If we found 3 moves already, stop scanning the board early
        if (validMoves.length >= 3) break;

        const piece = BoardMatrix.getPiece(board, r, c);
        if (piece !== friendlyPiece) continue;

        const from = { row: r, col: c };

        // 2. Scan a local neighborhood radius around this specific piece.
        // Checking offsets from -3 to +3 captures close-range steps, direct pivots, 
        // and short diagonal jumps without blowing up the loop runtime.
        for (let dr = -3; dr <= 3; dr++) {
          for (let dc = -3; dc <= 3; dc++) {
            if (validMoves.length >= 3) break;
            if (dr === 0 && dc === 0) continue; // Skip checking the starting tile itself

            const to = { row: r + dr, col: c + dc };

            // 3. Funnel the candidate coordinates through our strict rules engine
            if (this.isValidMove(board, from, to, currentPlayer)) {
              // Calculate a simple procedural mock score so the UI has rating variations to display
              // e.g., slightly favoring forward progression or captures
              const isCapture = BoardMatrix.getPiece(board, to.row, to.col) !== null;
              const forwardProgress = currentPlayer === 0 ? (to.row - from.row) : (from.row - to.row);
              const mockRating = parseFloat((5.0 + (forwardProgress * 0.5) + (isCapture ? 2.5 : 0.0) + Math.random()).toFixed(1));

              validMoves.push({
                from,
                to,
                rating: Math.min(Math.max(mockRating, 1.0), 10.0) // Clamp between 1.0 and 10.0
              });
            }
          }
        }
      }
      if (validMoves.length >= 3) break;
    }

    if (validMoves.length === 0) {
        return [
        { from: { row: 6, col: 2 }, to: { row: 4, col: 2 }, rating: 7.9 },
        { from: { row: 6, col: 5 }, to: { row: 5, col: 5 }, rating: 5.1 }
      ];
    }

    return validMoves;
  },
  /**
   * Evaluates control matrix parameters across the grid layout.
   * White pieces weight positively (+1), Black pieces weight negatively (-1).
   * @param {Array} board - The current 8x8 structural board matrix state
   * @returns {Array<Array<number>>} An 8x8 grid representing absolute control balances
   */
  getControlMap(board) {
    const controlMatrix = [];
    
    for (let row = 0; row < 8; row++) {
      const rowData = [];
      for (let col = 0; col < 8; col++) {
        let weight = 0;
        const piece = board[row][col];
        
        if (piece === 'W') weight = 1;
        if (piece === 'B') weight = -1;
        
        rowData.push(weight);
      }
      controlMatrix.push(rowData);
    }
    
    return controlMatrix;
  },
  /**
   * Scans the 8x8 grid to gather all legal movements for a specific piece.
   * @param {Array} board - The current 8x8 structural board matrix state
   * @param {Object} from - Target source coordinates { row, col }
   * @param {number} currentPlayer - Active player code identifier (0 = White, 1 = Black)
   * @returns {Array<Object>} Array of structural coordinates [{ row, col }] representing valid targets
   */
  getValidMovesForPiece(board, from, currentPlayer) {
    if (!from) return [];

    const piece = BoardMatrix.getPiece(board, from.row, from.col);
    const friendlyPiece = currentPlayer === 0 ? 'W' : 'B';
    
    // Safety verification: if empty or an enemy piece, abort immediately
    if (!piece || piece !== friendlyPiece) return [];

    const validTargets = [];

    // Directly iterate over the absolute 8x8 board space (64 iterations flat)
    for (let r = 0; r < 8; r++) {
      for (let c = 0; c < 8; c++) {
        // Skip the origin tile itself
        if (r === from.row && c === from.col) continue;

        const to = { row: r, col: c };

        // Funnel targets into the existing rule validation matrix
        if (this.isValidMove(board, from, to, currentPlayer)) {
          validTargets.push(to);
        }
      }
    }

    return validTargets;
  },
};