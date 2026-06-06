import init, { WasmEngine } from 'core-engine';
import { BoardMatrix } from './boardMatrix';

// Cache to hold the generated legal moves for the current active board position state
let memoizedMovesCache = null;
let cachedBoardKey = "";

/**
 * Compresses an 8x8 structural board matrix into binary BigInt components.
 * Matches bit orientations utilized internally by the core engine.
 */
export function matrixToBitboards(board) {
  let p1Pieces = 0n;
  let p2Pieces = 0n;

  for (let r = 0; r < 8; r++) {
    for (let c = 0; c < 8; c++) {
      const piece = board[r][c];
      if (!piece) continue;

      const bitIdx = BigInt(r * 8 + c);
      if (piece === 'W') {
        p1Pieces |= (1n << bitIdx);
      } else if (piece === 'B') {
        p2Pieces |= (1n << bitIdx);
      }
    }
  }

  return {
    p1_pieces: p1Pieces,
    p2_pieces: p2Pieces
  };
}

/**
 * Translates structural BigInt parameters back into an 8x8 board grid matrix.
 */
export function bitboardsToMatrix(p1Pieces, p2Pieces) {
  const matrix = Array(8).fill(null).map(() => Array(8).fill(null));
  const p1 = BigInt(p1Pieces);
  const p2 = BigInt(p2Pieces);

  for (let idx = 0; idx < 64; idx++) {
    const bitmask = 1n << BigInt(idx);
    const r = Math.floor(idx / 8);
    const c = idx % 8;

    if ((p1 & bitmask) !== 0n) {
      matrix[r][c] = 'W';
    } else if ((p2 & bitmask) !== 0n) {
      matrix[r][c] = 'B';
    }
  }

  return matrix;
}

/**
 * Translates flat 1D indices (0-63) into structural matrix row/column tracking coordinates.
 */
export function bitIndexToCoord(idx) {
  return {
    row: Math.floor(idx / 8),
    col: idx % 8
  };
}

/**
 * Generates a unique string hash of the board configuration to handle cache validation.
 */
function computeBoardCacheKey(board, currentPlayer) {
  let key = `${currentPlayer}:`;
  for (let r = 0; r < 8; r++) {
    for (let c = 0; c < 8; c++) {
      if (board[r][c]) key += `${r}${c}${board[r][c]}`;
    }
  }
  return key;
}

/**
 * Lazy loads the full legal move spectrum from WASM bitboards and memoizes the result.
 */
async function fetchAndMemoizeMoves(board, currentPlayer) {
  const currentKey = computeBoardCacheKey(board, currentPlayer);
  if (memoizedMovesCache && cachedBoardKey === currentKey) {
    return memoizedMovesCache;
  }

  await init();
  const bitboards = matrixToBitboards(board);
  const wasmState = {
    ...bitboards,
    active_player: currentPlayer === 0 ? 'P1' : 'P2'
  };

  try {
    const allMoves = WasmEngine.generate_legal_moves(wasmState);
    memoizedMovesCache = allMoves.map(m => ({
      from: bitIndexToCoord(m.from_square),
      to: bitIndexToCoord(m.to_square),
      isCapture: m.is_capture
    }));
    cachedBoardKey = currentKey;
  } catch {
    memoizedMovesCache = [];
  }

  return memoizedMovesCache;
}

export const EngineAdapterMock = {
  /**
   * Generates a clean, starting 2D board matrix layout using our BoardMatrix utility.
   */
  getInitialBoard() {
    return BoardMatrix.createInitialPosition();
  },

  /**
   * Fast rule validator: Verifies an intended structural movement path by matching 
   * against the pre-compiled memoized valid move registry.
   */
  async isValidMove(board, from, to, currentPlayer) {
    if (!from || !to) return false;
    const moves = await fetchAndMemoizeMoves(board, currentPlayer);
    return moves.some(m => 
      m.from.row === from.row && m.from.col === from.col &&
      m.to.row === to.row && m.to.col === to.col
    );
  },

  /**
   * Collects localized tactical engine moves along with mock rating indicators 
   * to display within the UI player assistance indicators.
   */
  async getMockAIAssistMoves(board, currentPlayer) {
    const moves = await fetchAndMemoizeMoves(board, currentPlayer);
    
    // Process up to 3 candidate moves for visual presentation
    const parsedAssistMoves = moves.slice(0, 3).map(m => {
      const forwardProgress = currentPlayer === 0 ? (m.to.row - m.from.row) : (m.from.row - m.to.row);
      const rating = parseFloat(
        Math.min(
          Math.max(5.0 + (forwardProgress * 0.5) + (m.isCapture ? 2.5 : 0.0) + Math.random(), 1.0),
          10.0
        ).toFixed(1)
      );

      return {
        from: m.from,
        to: m.to,
        rating
      };
    });

    if (parsedAssistMoves.length === 0) {
      return [
        { from: { row: 6, col: 2 }, to: { row: 4, col: 2 }, rating: 7.9 },
        { from: { row: 6, col: 5 }, to: { row: 5, col: 5 }, rating: 5.1 }
      ];
    }

    return parsedAssistMoves;
  },

  /**
   * Evaluates piece concentration across the grid layout to build positional visual charts.
   * White pieces weight positively (+1), Black pieces weight negatively (-1).
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
   * Scans the board state to filter out targets matching a specified source coordinate.
   */
  async getValidMovesForPiece(board, from, currentPlayer) {
    if (!from) return [];
    const moves = await fetchAndMemoizeMoves(board, currentPlayer);
    return moves
      .filter(m => m.from.row === from.row && m.from.col === from.col)
      .map(m => m.to);
  }
};