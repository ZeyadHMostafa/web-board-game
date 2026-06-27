import type { StateCreator } from 'zustand';
import type { GameStoreState } from './useGameStore';
import type { EvaluationProgress } from '../domain/types';
import type { FeatureConfiguration } from '../domain/configurations';

export interface EvalState {
  showAssist: boolean;
  liveEval: EvaluationProgress | null;
}

export interface EvalActions {
  setInitialConfig: (config: FeatureConfiguration) => void;
  updateFeatureConfig: (config: FeatureConfiguration) => void;
  toggleAssist: () => void;
  setLiveEval: (progress: EvaluationProgress | null) => void;
  triggerLiveEvaluation: () => void;
}

export type EvalSlice = EvalState & EvalActions;

export const createEvalSlice: StateCreator<
  GameStoreState,
  [],
  [],
  EvalSlice
> = (set, get) => ({
  config: null,
  showAssist: false,
  liveEval: null,

  setInitialConfig: (config) => {
    set({
      config,
      showAssist: config.enableLiveEval,
    });
  },

  updateFeatureConfig: (config) => {
    set({ config });
    if (!config.enableLiveEval) {
      set({ liveEval: null });
    } else {
      get().triggerLiveEvaluation();
    }
  },

  toggleAssist: () => {
    set((state) => {
      const nextShowAssist = !state.showAssist;
      if (!nextShowAssist) {
        return { showAssist: nextShowAssist, liveEval: null };
      }
      return { showAssist: nextShowAssist };
    });

    if (get().showAssist) {
      get().triggerLiveEvaluation();
    }
  },

  setLiveEval: (progress) => {
    set({ liveEval: progress });
  },

  triggerLiveEvaluation: () => {
    const { engineClient, gameEnded, history, currentIndex, config, showAssist } = get();
    if (!engineClient || !config || gameEnded) return;

    if (!config.enableLiveEval || !showAssist) {
      set({ liveEval: null });
      return;
    }
    console.log("live eval triggered")
    const currentSnapshot = history[currentIndex];
    engineClient.requestLiveEvaluation(
      currentSnapshot.board,
      currentSnapshot.currentPlayer,
      config
    );
  }
});