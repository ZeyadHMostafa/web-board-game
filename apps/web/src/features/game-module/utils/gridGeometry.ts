import type { Coordinate } from '../domain/types';

export const GridGeometry = {
  /**
   * Translates matrix indexes to the top-left corner within the 800x800 vector space.
   * Ideal for positioning on-grid elements or cell layout wrappers.
   */
  matrixToVectorTopLeft(row: number, col: number): { x: number; y: number } {
    return {
      x: col * 100,
      y: (7 - row) * 100
    };
  },

  /**
   * Translates matrix indexes to the exact center within the 800x800 vector space.
   * Ideal for cross-grid layouts like drawing tactical lines and assist vectors.
   */
  matrixToVectorCenter(row: number, col: number): { x: number; y: number } {
    return {
      x: col * 100 + 50,
      y: (7 - row) * 100 + 50
    };
  },

  /**
   * Converts raw pointer event offsets back into a row/column grid intersection.
   * Uses the live DOM bounding width at the moment of interaction.
   */
  pixelsToMatrix(x: number, y: number, currentBoardSize: number): Coordinate {
    const tileSize = currentBoardSize / 8;
    const col = Math.floor(x / tileSize);
    const row = 7 - Math.floor(y / tileSize);
    
    return {
      row: Math.max(0, Math.min(7, row)),
      col: Math.max(0, Math.min(7, col))
    };
  },

  /**
   * Translates matrix coordinates to standard algebraic notation.
   */
  matrixToAlgebraic(row: number, col: number): string {
    const files = 'ABCDEFGH';
    return `${files[col]}${row}`;
  }
};