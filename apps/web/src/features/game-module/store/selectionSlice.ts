import type { StateCreator } from 'zustand';
import type { GameStoreState } from './useGameStore';
import type { Coordinate, PlayerColor } from '../domain/types';

export interface SelectionState {
  selectedCoords: Coordinate | null;
}

export interface SelectionActions {
  selectPiece: (coords: Coordinate | null) => void;
  clearSelection: () => void;
  getValidMovesForSelection: () => Coordinate[];
}

export type SelectionSlice = SelectionState & SelectionActions;

export const createSelectionSlice: StateCreator<
  GameStoreState,
  [],
  [],
  SelectionSlice
> = (set, get) => ({
  selectedCoords: null,

  clearSelection: () => {
    set({ selectedCoords: null });
  },

  selectPiece: (coords) => {
    const { gameEnded, history, currentIndex, clearSelection } = get();
    if (gameEnded || !coords) {
      clearSelection();
      return;
    }

    const currentSnapshot = history[currentIndex];
    const { board, currentPlayer } = currentSnapshot;
    const piece = board[coords.row][coords.col];
    const friendlyColor: PlayerColor = currentPlayer === 0 ? 'W' : 'B';

    if (piece === friendlyColor) {
      set({ selectedCoords: coords });
    } else {
      clearSelection();
    }
  },

  getValidMovesForSelection: () => {
    const { selectedCoords, allLegalMoves } = get();
    if (!selectedCoords) return [];

    return allLegalMoves
      .filter(move => 
        move.from.row === selectedCoords.row && 
        move.from.col === selectedCoords.col
      )
      .map(move => move.to);
  }
});