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
    // Structural guard: ensure target coordinates are within grid limits
    if (to.row < 0 || to.row > 7 || to.col < 0 || to.col > 7) return false;
    
    // Fetch the structural piece from the matrix source index
    const piece = BoardMatrix.getPiece(board, from.row, from.col);
    if (!piece) return false;
    console.log(currentPlayer, piece);

    // Turn enforcement check: ensure player is only manipulation assets belonging to them
    if (currentPlayer === 0 && piece !== 'W') return false; // White turn can only move 'W'
    if (currentPlayer === 1 && piece !== 'B') return false; // Black turn can only move 'B'

    return true;
  },

  /**
   * Mock AI move evaluator: Generates static candidate move indicators 
   * so we can see the vector paths drawing on the Assist Canvas layer
   */
  getMockAIAssistMoves(currentPlayer) {
    // If it's White's turn (Player 0), suggest a couple forward moves
    if (currentPlayer === 0) {
      return [
        { from: { row: 1, col: 3 }, to: { row: 3, col: 3 }, rating: 8.5 },
        { from: { row: 1, col: 4 }, to: { row: 2, col: 4 }, rating: 6.2 }
      ];
    }
    
    // If it's Black's turn (Player 1), suggest alternative mock responses
    return [
      { from: { row: 6, col: 2 }, to: { row: 4, col: 2 }, rating: 7.9 },
      { from: { row: 6, col: 5 }, to: { row: 5, col: 5 }, rating: 5.1 }
    ];
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
  }
};