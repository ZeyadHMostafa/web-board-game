export const BoardGeometry = {
  /**
   * Converts matrix coordinates to absolute pixel positions (center of the tile)
   * @param {number} row - Grid row (0-7)
   * @param {number} col - Grid column (0-7)
   * @param {number} boardSize - Current responsive pixel size of the board
   * @returns {{x: number, y: number}} Pixel coordinates of tile center
   */
  matrixToPixels(row, col, boardSize) {
    const tileSize = boardSize / 8;
    return {
      x: col * tileSize + tileSize / 2,
      y: (7 - row) * tileSize + tileSize / 2
    };
  },

  /**
   * Converts matrix coordinates to the top-left pixel position of the tile
   * Perfect for positioning absolute DOM elements like Pieces
   */
  matrixToTileTopLeft(row, col, boardSize) {
    const tileSize = boardSize / 8;
    return {
      x: col * tileSize,
      y: (7 - row) * tileSize
    };
  },

  /**
   * Converts canvas bounding pixel coordinates back into a grid index
   * Perfect for capturing where a user clicked or dropped a piece
   */
  pixelsToMatrix(x, y, boardSize) {
    const tileSize = boardSize / 8;
    const col = Math.floor(x / tileSize);
    const row = 7 - Math.floor(y / tileSize);
    
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