import { useEffect } from 'react';
import type { GameSnapshot } from '../../domain/types';
import type { GameEngineClient } from '../../services/engine/workerClient';
import type { FeatureConfiguration } from '../../domain/configurations';

export const useEval = (
  currentSnapshot: GameSnapshot,
  config: FeatureConfiguration,
  engineClient: GameEngineClient | null,
  gameEnded: boolean,
  clearEval: () => void
) => {
  useEffect(() => {
    // If the config toggles the feature off mid-game, instantly wipe the visual UI state
    if (!engineClient || !config.enableLiveEval) {
      clearEval();
      return;
    }

    if (gameEnded) return;

    // By reacting to the snapshot itself, this automatically recalculates
    // when a user clicks "undo" or jumps through the move history ledger.
    engineClient.requestLiveEvaluation(
      currentSnapshot.board, 
      currentSnapshot.currentPlayer, 
      config
    );

  }, [currentSnapshot, config, engineClient, gameEnded, clearEval]);
};