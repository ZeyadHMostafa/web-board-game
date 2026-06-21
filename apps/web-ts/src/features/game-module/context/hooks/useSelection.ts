import { useState, useCallback, useMemo } from 'react';
import type { GameSnapshot, Coordinate, Move, PlayerColor } from '../../domain/types';

export const useSelection = (
  currentSnapshot: GameSnapshot,
  allLegalMoves: Move[],
  gameEnded: boolean
) => {
  const [selectedCoords, setSelectedCoords] = useState<Coordinate | null>(null);

  const clearSelection = useCallback(() => {
    setSelectedCoords(null);
  }, []);

  const selectPiece = useCallback((coords: Coordinate | null) => {
    if (gameEnded || !coords) {
      clearSelection();
      return;
    }

    const { board, currentPlayer } = currentSnapshot;
    const piece = board[coords.row][coords.col];
    const friendlyColor: PlayerColor = currentPlayer === 0 ? 'W' : 'B';

    if (piece === friendlyColor) {
      setSelectedCoords(coords);
    } else {
      clearSelection();
    }
  }, [currentSnapshot, gameEnded, clearSelection]);

  const validMovesForSelection = useMemo(() => {
    if (!selectedCoords) return [];

    return allLegalMoves
      .filter(move => 
        move.from.row === selectedCoords.row && 
        move.from.col === selectedCoords.col
      )
      .map(move => move.to);
  }, [selectedCoords, allLegalMoves]);

  return {
    selectedCoords,
    validMovesForSelection,
    selectPiece,
    clearSelection
  };
};