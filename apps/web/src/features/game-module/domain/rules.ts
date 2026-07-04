import type { BoardMatrixState, PlayerColor } from './types';

/**
 * Generates a clean starting 8x8 matrix layout with default player positions.
 * Row 0 and 1 represent White, Row 6 and 7 represent Black.
 */
export const createInitialPosition = (): BoardMatrixState => {
  const matrix: BoardMatrixState = Array(8).fill(null).map(() => Array(8).fill(null));

  // Initialize white pieces
  for (let r = 0; r < 2; r++) {
    for (let c = 0; c < 8; c++) {
      matrix[r][c] = 'W';
    }
  }

  // Initialize black pieces
  for (let r = 6; r < 8; r++) {
    for (let c = 0; c < 8; c++) {
      matrix[r][c] = 'B';
    }
  }

  return matrix;
};

/**
 * Returns a new immutably cloned matrix with the updated piece token placement.
 */
export const setPiece = (
  board: BoardMatrixState, 
  row: number, 
  col: number, 
  piece: PlayerColor | null
): BoardMatrixState => {
  const nextBoard = board.map(innerRow => [...innerRow]);
  if (row >= 0 && row < 8 && col >= 0 && col < 8) {
    nextBoard[row][col] = piece;
  }
  return nextBoard;
};