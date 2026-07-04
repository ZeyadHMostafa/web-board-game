import type { StateCreator } from 'zustand';
import type { GameStoreState } from './useGameStore';
import { AI_LEVEL_PRESETS, type AiLevel } from '../domain/configurations';

export interface EngineActor {
  isAuto: boolean;
  currentLevel: AiLevel;
  isCalculating: boolean; 
}

export interface EngineState {
  whiteEngine: EngineActor;
  blackEngine: EngineActor;
}

export interface EngineActions {
  setupInitialEngines: (whiteAuto: boolean, blackAuto: boolean, defaultLevel: AiLevel) => void;
  toggleEngineAuto: (playerIndex: 0 | 1) => void;
  setEngineAiLevel: (playerIndex: 0 | 1, level: AiLevel) => void;
  triggerEngineMove: () => void;
}

export type EngineSlice = EngineState & EngineActions;

export const createEngineSlice: StateCreator<
  GameStoreState, 
  [], 
  [], 
  EngineSlice
> = (set, get) => {
  const getKey = (idx: 0 | 1) => idx === 0 ? 'whiteEngine' as const : 'blackEngine' as const;

  return {
    whiteEngine: { isAuto: false, currentLevel: 'COMPETITOR' as AiLevel, isCalculating: false },
    blackEngine: { isAuto: false, currentLevel: 'COMPETITOR' as AiLevel, isCalculating: false },

    setupInitialEngines: (whiteAuto, blackAuto, defaultLevel) => {
      set({
        whiteEngine: { isAuto: whiteAuto, currentLevel: defaultLevel, isCalculating: false },
        blackEngine: { isAuto: blackAuto, currentLevel: defaultLevel, isCalculating: false }
      });
    },

    toggleEngineAuto: (playerIndex) => {
      const key = getKey(playerIndex);
      set((state) => ({
        [key]: { ...state[key], isAuto: !state[key].isAuto }
      }));
      get().triggerEngineMove();
    },

    setEngineAiLevel: (playerIndex, level) => {
      const key = getKey(playerIndex);
      set((state) => ({
        [key]: { ...state[key], currentLevel: level }
      }));
      get().triggerEngineMove();
    },

    triggerEngineMove: () => {
      const { engineClient, gameEnded, history, currentIndex, whiteEngine, blackEngine } = get();
      if (!engineClient || gameEnded) return;

      const currentSnapshot = history[currentIndex];
      const activePlayer = currentSnapshot.currentPlayer; 
      const targetKey = getKey(activePlayer);

      let nextWhiteCalculating = whiteEngine.isCalculating;
      let nextBlackCalculating = blackEngine.isCalculating;

      if (activePlayer !== 0 && whiteEngine.isCalculating) nextWhiteCalculating = false;
      if (activePlayer !== 1 && blackEngine.isCalculating) nextBlackCalculating = false;

      if (nextWhiteCalculating !== whiteEngine.isCalculating || nextBlackCalculating !== blackEngine.isCalculating) {
        set((state) => ({
          whiteEngine: { ...state.whiteEngine, isCalculating: nextWhiteCalculating },
          blackEngine: { ...state.blackEngine, isCalculating: nextBlackCalculating }
        }));
      }

      const freshEngineState = activePlayer === 0 
        ? { ...whiteEngine, isCalculating: nextWhiteCalculating }
        : { ...blackEngine, isCalculating: nextBlackCalculating };

      if (freshEngineState.isAuto && !freshEngineState.isCalculating) {
        set((state) => ({
          [targetKey]: { ...state[targetKey], isCalculating: true }
        }));

        engineClient.requestAIMove(
          currentSnapshot.board,
          activePlayer,
          AI_LEVEL_PRESETS[freshEngineState.currentLevel]
        );
      }
    }
  };
};