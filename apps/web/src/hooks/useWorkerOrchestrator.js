import { useEffect, useRef } from 'react';

const PLAYER_CONFIGS = {
  0: { minDepth: 1, maxDepth: 5, temp: 0.2, bThresh: 30 }, // White
  1: { minDepth: 1, maxDepth: 4, temp: 0.6, bThresh: 50 }  // Black
};

export function useWorkerOrchestrator(gameState) {
  const {
    board,
    currentPlayer,
    autoPlayers,
    gameEnded,
    showAssist,
    executeMove,
    setAssistMoves,
    setLiveProgress
  } = gameState;

  const workerRef = useRef(null);

  // 1. Initialize Worker Lifecycle
  useEffect(() => {
    workerRef.current = new Worker(
      new URL('../workers/aiWorker.js', import.meta.url),
      { type: 'module' }
    );

    workerRef.current.onmessage = (e) => {
      const { type, move, progress, error } = e.data;

      if (error) {
        console.error("Wasm Engine Thread Error:", error);
        return;
      }

      if (type === 'AI_MOVE_READY') {
        executeMove(move.from, move.to);
      }

      if (type === 'EVAL_PROGRESS_UPDATE') {
        setLiveProgress(progress);
        
        // Transform the evaluation scores directly into the UI's assist format
        const topMoves = progress.candidates
          .sort((a, b) => b.scoreValue - a.scoreValue)
          .slice(0, 3)
          .map(c => ({
            from: c.from,
            to: c.to,
            rating: parseFloat(Math.min(Math.max((c.scoreValue / 100) + 5.0, 1.0), 10.0).toFixed(1))
          }));
        
        setAssistMoves(topMoves);
      }
    };

    return () => {
      if (workerRef.current) workerRef.current.terminate();
    };
  }, [executeMove, setAssistMoves, setLiveProgress]);

  // 2. Dispatch Tasks based on Game Mutations
  useEffect(() => {
    if (gameEnded || !workerRef.current) return;

    const isCurrentAuto = autoPlayers[currentPlayer];
    const currentConfig = PLAYER_CONFIGS[currentPlayer];

    if (isCurrentAuto) {
      setLiveProgress(null);
      
      const timer = setTimeout(() => {
        workerRef.current.postMessage({
          type: 'COMPUTE_AI_MOVE',
          board,
          currentPlayer,
          config: currentConfig
        });
      }, 500);

      return () => clearTimeout(timer);
    } else if (showAssist) {
      workerRef.current.postMessage({
        type: 'COMPUTE_LIVE_EVAL',
        board,
        currentPlayer,
        config: currentConfig
      });
    }
  }, [board, currentPlayer, autoPlayers, gameEnded, showAssist, setLiveProgress]);
}