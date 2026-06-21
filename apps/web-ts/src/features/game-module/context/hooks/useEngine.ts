import { useState, useEffect, useRef, useCallback } from 'react';
import type { GameSnapshot, PlayerIndex } from '../../domain/types';
import type { GameEngineClient } from '../../services/engine/workerClient';
import type { AiEngineConfiguration } from '../../domain/configurations';

export const useEngine = (
  targetPlayer: PlayerIndex,
  initialIsAuto: boolean,
  initialConfig: AiEngineConfiguration,
  currentSnapshot: GameSnapshot,
  engineClient: GameEngineClient | null,
  gameEnded: boolean
) => {
  // Localized mutable state allows the HUD to adjust these mid-match
  const [isAuto, setIsAuto] = useState(initialIsAuto);
  const [engineConfig, setEngineConfig] = useState<AiEngineConfiguration>(initialConfig);

  const isCalculatingRef = useRef(false);

  // Reset the calculation lock whenever the turn leaves this specific player
  useEffect(() => {
    if (currentSnapshot.currentPlayer !== targetPlayer) {
      isCalculatingRef.current = false;
    }
  }, [currentSnapshot.currentPlayer, targetPlayer]);

  useEffect(() => {
    if (!engineClient || gameEnded) return;

    // The brain only wakes up if it's currently automated AND it is its specific turn
    if (currentSnapshot.currentPlayer === targetPlayer && isAuto && !isCalculatingRef.current) {
      isCalculatingRef.current = true;
      
      engineClient.requestAIMove(
        currentSnapshot.board, 
        targetPlayer, 
        engineConfig
      );
    }
  }, [currentSnapshot, targetPlayer, isAuto, engineConfig, engineClient, gameEnded]);

  const toggleAuto = useCallback(() => {
    setIsAuto(prev => !prev);
  }, []);

  const updateConfig = useCallback((updates: Partial<AiEngineConfiguration>) => {
    setEngineConfig(prev => ({ ...prev, ...updates }));
  }, []);

  // Expose the state and setters so the HUD panels can wire up toggle switches and sliders
  return {
    isAuto,
    engineConfig,
    toggleAuto,
    updateConfig
  };
};