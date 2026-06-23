import { useState, useCallback, useMemo } from 'react';
import type { GameSnapshot } from '../../domain/types';
import { createInitialPosition } from '../../domain/rules';

export const useTimeline = () => {
  const [history, setHistory] = useState<GameSnapshot[]>(() => [{
    board: createInitialPosition(),
    currentPlayer: 0,
    lastMove: null
  }]);
  
  const [currentIndex, setCurrentIndex] = useState(0);

  /**
   * Memoized to prevent downstream renders unless the temporal pointer shifts.
   */
  const currentSnapshot = useMemo(
    () => history[currentIndex], 
    [history, currentIndex]
  );

  /**
   * Appends a new frame to the ledger. 
   * If the user has navigated back in time, this slices off obsolete alternative futures.
   */
  const appendSnapshot = useCallback((newSnapshot: GameSnapshot) => {
    setHistory(prevHistory => {
      const cleanHistory = prevHistory.slice(0, currentIndex + 1);
      return [...cleanHistory, newSnapshot];
    });
    setCurrentIndex(prevIndex => prevIndex + 1);
  }, [currentIndex]);

  const jumpToHistoryIndex = useCallback((index: number) => {
    if (index >= 0 && index < history.length) {
      setCurrentIndex(index);
    }
  }, [history.length]);

  const resetTimeline = useCallback(() => {
    setHistory([{
      board: createInitialPosition(),
      currentPlayer: 0,
      lastMove: null
    }]);
    setCurrentIndex(0);
  }, []);

  return {
    currentSnapshot,
    history: history,
    currentIndex,
    appendSnapshot,
    jumpToHistoryIndex,
    resetTimeline
  };
};