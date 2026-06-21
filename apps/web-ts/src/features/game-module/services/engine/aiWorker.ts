import init, { WasmEngine } from '../../../../wasm/core_engine';
import type {Move} from '../../domain/types';

interface WasmMove {
  from_square: number;
  to_square: number;
  is_capture: boolean;
}

interface WasmCandidate {
  current_move: WasmMove;
  score_value: number;
  score_label: string;
}

interface WasmProgressUpdate {
  candidates: WasmCandidate[];
  depth_reached: number;
  nodes_explored: number;
  branching_factor: number;
  pv: WasmMove[];
}

let wasmInitialized = false;

async function ensureWasmReady() {
  if (!wasmInitialized) {
    await init();
    wasmInitialized = true;
  }
}

function matrixToBitboards(board: (string | null)[][]) {
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

function bitIndexToCoord(idx: number) {
  return {
    row: Math.floor(idx / 8),
    col: idx % 8
  };
}

let cachedBoardKey = '';
let memoizedMovesCache: Move[] = [];

function computeBoardCacheKey(board: (string | null)[][], currentPlayer: number): string {
  // Simple deterministic string representation for the cache lock
  return JSON.stringify(board) + '-' + currentPlayer;
}

self.onmessage = async (e: MessageEvent) => {
  const { type, board, currentPlayer, config, id} = e.data;

  try {
    await ensureWasmReady();

    const bitboards = matrixToBitboards(board);
    const wasmState = {
      ...bitboards,
      active_player: currentPlayer === 0 ? 'P1' : 'P2'
    };

    if (type === 'COMPUTE_ALL_LEGAL_MOVES') {
      const currentKey = computeBoardCacheKey(board, currentPlayer);
      
      if (memoizedMovesCache.length > 0 && cachedBoardKey === currentKey) {
        self.postMessage({ type: 'ALL_LEGAL_MOVES_READY', moves: memoizedMovesCache, id });
        return;
      }

      // Generate fresh from WASM
      const rawMoves = WasmEngine.generate_legal_moves(wasmState);
      
      memoizedMovesCache = rawMoves.map((m: WasmMove) => ({
        from: bitIndexToCoord(m.from_square),
        to: bitIndexToCoord(m.to_square),
        isCapture: m.is_capture
      }));
      cachedBoardKey = currentKey;

      self.postMessage({ type: 'ALL_LEGAL_MOVES_READY', moves: memoizedMovesCache, id });
      return;
    }

    if (type === 'COMPUTE_AI_MOVE') {
      const rawMove: WasmMove = WasmEngine.compute_ai_move(
        wasmState,
        config.minDepth || 1,
        config.maxDepth || 3,
        config.temp !== undefined ? config.temp : 0.2,
        config.bThresh !== undefined ? config.bThresh : 20
      );

      self.postMessage({
        type: 'AI_MOVE_READY',
        move: {
          from: bitIndexToCoord(rawMove.from_square),
          to: bitIndexToCoord(rawMove.to_square),
          isCapture: rawMove.is_capture
        }
      });
    }

    if (type === 'COMPUTE_LIVE_EVAL') {
      const rawProgress: WasmProgressUpdate = WasmEngine.compute_evaluation_progress(
        wasmState,
        config.minDepth || 1,
        config.maxDepth || 4
      );

      const sortedCandidates = rawProgress.candidates
        .map((c: WasmCandidate) => ({
          from: bitIndexToCoord(c.current_move.from_square),
          to: bitIndexToCoord(c.current_move.to_square),
          isCapture: c.current_move.is_capture,
          scoreValue: c.score_value,
          scoreLabel: c.score_label
        }))
        .sort((a, b) => b.scoreValue - a.scoreValue);

      const parsedPv = rawProgress.pv.map((m: WasmMove) => ({
        from: bitIndexToCoord(m.from_square),
        to: bitIndexToCoord(m.to_square),
        isCapture: m.is_capture
      }));

      self.postMessage({
        type: 'EVAL_PROGRESS_UPDATE',
        progress: {
          candidates: sortedCandidates,
          depthReached: rawProgress.depth_reached,
          nodesExplored: rawProgress.nodes_explored,
          branchingFactor: rawProgress.branching_factor,
          pv: parsedPv
        }
      });
    }

  } catch (error) {
    self.postMessage({
      type: 'ENGINE_ERROR',
      error: error instanceof Error ? error.message : String(error),
      id
    });
  }
};