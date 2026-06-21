import type { Coordinate } from '../domain/types';

export const GridGeometry = {
  /**
   * Translates grid indexes to absolute pixel coordinates mapping the true center of a square.
   * Inverts the Y-axis calculation so that row 0 represents the bottom visual row.
   */
  matrixToPixels(row: number, col: number, boardSize: number): { x: number; y: number } {
    const tileSize = boardSize / 8;
    return {
      x: col * tileSize + tileSize / 2,
      y: (7 - row) * tileSize + tileSize / 2
    };
  },

  /**
   * Translates grid indexes to the exact top-left pixel corner of a cell.
   * Ideal for positioning HTML piece wrappers or computing bounding boxes.
   */
  matrixToTileTopLeft(row: number, col: number, boardSize: number): { x: number; y: number } {
    const tileSize = boardSize / 8;
    return {
      x: col * tileSize,
      y: (7 - row) * tileSize
    };
  },

  /**
   * Converts raw canvas bounding pixel hits back into a row/column grid intersection.
   * Crucial for intercepting where a pointer tap down or drag drop occurs.
   */
  pixelsToMatrix(x: number, y: number, boardSize: number): Coordinate {
    const tileSize = boardSize / 8;
    const col = Math.floor(x / tileSize);
    const row = 7 - Math.floor(y / tileSize);
    
    return {
      row: Math.max(0, Math.min(7, row)),
      col: Math.max(0, Math.min(7, col))
    };
  },

  /**
   * Translates coordinates to standard chess notation strings.
   */
  matrixToAlgebraic(row: number, col: number): string {
    const files = "ABCDEFGH";
    return `${files[col]}${row}`;
  }
};