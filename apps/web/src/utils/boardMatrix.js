/**
 * Utility Object to manage an isolated 8x8 Board Matrix State
 */
export const BoardMatrix = {
  /**
   * Initializes an empty 8x8 matrix or maps a custom arrangement
   * W = Player 1, B = Player 2, null = Empty Tile
   */
  createEmpty() {
    return Array(8).fill(null).map(() => Array(8).fill(null));
  },

  createInitialPosition() {
    const grid = this.createEmpty();
    
    // Rows 0 and 1: Player 1 (White / Bottom side)
    for (let r = 0; r < 2; r++) {
      for (let c = 0; c < 8; c++) {
        grid[r][c] = 'W';
      }
    }
    
    // Rows 6 and 7: Player 2 (Black / Top side)
    for (let r = 6; r < 8; r++) { // Adjusted layout matching your configuration bits
      for (let c = 0; c < 8; c++) {
        grid[r][c] = 'B';
      }
    }

    return grid;
  },

  /**
   * Safe Getter/Setter matrix wrappers
   */
  getPiece(grid, row, col) {
    if (row >= 0 && row < 8 && col >= 0 && col < 8) {
      return grid[row][col];
    }
    return null;
  },

  setPiece(grid, row, col, piece) {
    // Return a new shallow matrix copy to adhere to React immutability rules
    const newGrid = grid.map(innerRow => [...innerRow]);
    if (row >= 0 && row < 8 && col >= 0 && col < 8) {
      newGrid[row][col] = piece;
    }
    return newGrid;
  },

  /**
   * Deep clone helper to cleanly break React state memory references
   */
  clone(grid) {
    return grid.map(row => [...row]);
  }
};