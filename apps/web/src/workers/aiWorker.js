import init, { WasmEngine } from '../wasm/core_engine';
import { matrixToBitboards, bitIndexToCoord } from '../utils/engineAdapterMock';

let wasmInitialized = false;

/**
 * Synchronizes initialization parameters across the worker execution context.
 */
async function ensureWasmReady() {
  if (!wasmInitialized) {
    await init();
    wasmInitialized = true;
  }
}

self.onmessage = async (e) => {
  const { type, board, currentPlayer, config } = e.data;

  try {
    await ensureWasmReady();

    const bitboards = matrixToBitboards(board);
    const wasmState = {
      ...bitboards,
      active_player: currentPlayer === 0 ? 'P1' : 'P2'
    };

    if (type === 'COMPUTE_AI_MOVE') {
      const rawMove = WasmEngine.compute_ai_move(
        wasmState,
        config.minDepth || 1,
        config.maxDepth || 6,
        config.temp !== undefined ? config.temp : 0.0,
        config.bThresh !== undefined ? config.bThresh : 0
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
      const rawProgress = WasmEngine.compute_evaluation_progress(
        wasmState,
        config.minDepth || 1,
        config.maxDepth || 4
      );

      const parsedCandidates = rawProgress.candidates.map(c => ({
        from: bitIndexToCoord(c.current_move.from_square),
        to: bitIndexToCoord(c.current_move.to_square),
        isCapture: c.current_move.is_capture,
        scoreValue: c.score_value,
        scoreLabel: c.score_label
      }));

      const parsedPv = rawProgress.pv.map(m => ({
        from: bitIndexToCoord(m.from_square),
        to: bitIndexToCoord(m.to_square),
        isCapture: m.is_capture
      }));

      self.postMessage({
        type: 'EVAL_PROGRESS_UPDATE',
        progress: {
          candidates: parsedCandidates,
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
      error: error.toString()
    });
  }
};