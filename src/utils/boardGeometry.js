export const TILE_SIZE = 64;
export const BOARD_SIZE = TILE_SIZE * 8; // 512px

export const BoardGeometry = {
  /**
   * Converts matrix coordinates to absolute pixel positions (center of the tile)
   * @param {number} row - Grid row (0-7)
   * @param {number} col - Grid column (0-7)
   * @returns {{x: number, y: number}} Pixel coordinates of tile center
   */
  matrixToPixels(row, col) {
    return {
      x: col * TILE_SIZE + TILE_SIZE / 2,
      y: (7 - row) * TILE_SIZE + TILE_SIZE / 2
    };
  },

  /**
   * Converts matrix coordinates to the top-left pixel position of the tile
   * Perfect for positioning absolute DOM elements like Pieces
   */
  matrixToTileTopLeft(row, col) {
    return {
      x: col * TILE_SIZE,
      y: (7 - row) * TILE_SIZE
    };
  },

  /**
   * Converts canvas bounding pixel coordinates back into a grid index
   * Perfect for capturing where a user clicked or dropped a piece
   */
  pixelsToMatrix(x, y) {
    const col = Math.floor(x / TILE_SIZE);
    const row = 7 - Math.floor(y / TILE_SIZE);
    
    // Guard against clicks slightly outside bounds
    return {
      row: Math.max(0, Math.min(7, row)),
      col: Math.max(0, Math.min(7, col))
    };
  },

  /**
   * Translates matrix coordinates to standard chess notation (e.g., row 0, col 0 -> "A8")
   */
  matrixToAlgebraic(row, col) {
    const files = "ABCDEFGH";
    const rank = row;
    return `${files[col]}${rank}`;
  }
};