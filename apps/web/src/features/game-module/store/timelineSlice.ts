import type { StateCreator } from 'zustand';
import type { GameStoreState } from './useGameStore';
import type { GameSnapshot } from '../domain/types';
import { createInitialPosition } from '../domain/rules';

export interface TimelineState {
  history: GameSnapshot[];
  currentIndex: number;
}

export interface TimelineActions {
  appendSnapshot: (newSnapshot: GameSnapshot) => void;
  jumpToHistoryIndex: (index: number) => void;
  resetTimeline: () => void;
  getCurrentSnapshot: () => GameSnapshot;
}

export type TimelineSlice = TimelineState & TimelineActions;

export const createTimelineSlice: StateCreator<
  GameStoreState,
  [],
  [],
  TimelineSlice
> = (set, get) => ({
  history: [
    {
      board: createInitialPosition(),
      currentPlayer: 0,
      lastMove: null,
    },
  ],
  currentIndex: 0,

  appendSnapshot: (newSnapshot) => {
    set((state) => {
      const cleanHistory = state.history.slice(0, state.currentIndex + 1);
      return {
        history: [...cleanHistory, newSnapshot],
        currentIndex: state.currentIndex + 1,
      };
    });
  },

  jumpToHistoryIndex: (index) => {
    const { history, clearSelection, setLiveEval, triggerEngineMove, triggerLiveEvaluation, fetchLegalMoves } = get();
    if (index >= 0 && index < history.length) {
      const targetSnapshot = history[index];
      set({ currentIndex: index, currentPlayer: targetSnapshot.currentPlayer });
      clearSelection();
      setLiveEval(null);
      fetchLegalMoves().then(() => {
        triggerEngineMove();
        triggerLiveEvaluation();
      });
    }
  },

  resetTimeline: () => {
    const { clearSelection, setLiveEval, triggerEngineMove, triggerLiveEvaluation, fetchLegalMoves } = get();
    set({
      history: [
        {
          board: createInitialPosition(),
          currentPlayer: 0,
          lastMove: null,
        },
      ],
      currentIndex: 0,
      currentPlayer: 0
    });
    clearSelection();
    setLiveEval(null);
    fetchLegalMoves().then(() => {
      triggerEngineMove();
      triggerLiveEvaluation();
    });
  },

  getCurrentSnapshot: () => {
    const { history, currentIndex } = get();
    return history[currentIndex];
  }
});