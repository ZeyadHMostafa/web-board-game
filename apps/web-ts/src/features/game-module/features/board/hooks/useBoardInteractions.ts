import React, { useState, useRef } from 'react';
import { useGame } from '../../../context/GameContext';
import { GridGeometry } from '../../../utils/gridGeometry';
import type { Coordinate, PlayerColor } from '../../../domain/types';

interface UseBoardInteractionsProps {
  boardRef: React.RefObject<HTMLDivElement | null>;
}

export const useBoardInteractions = ({ boardRef }: UseBoardInteractionsProps) => {
  const { board, selectedCoords, executeMove, selectPiece, currentPlayer } = useGame();
  
  const [draggedPiece, setDraggedPiece] = useState<PlayerColor | null>(null);
  const [mousePosition, setMousePosition] = useState({ x: 0, y: 0 });
  const [dragSource, setDragSource] = useState<Coordinate | null>(null);
  
  const activeDragStart = useRef<Coordinate | null>(null);

  const getEventCoordinates = (clientX: number, clientY: number): Coordinate | null => {
    if (!boardRef.current) return null;
    const rect = boardRef.current.getBoundingClientRect();
    const x = clientX - rect.left;
    const y = clientY - rect.top;
    return GridGeometry.pixelsToMatrix(x, y, rect.width);
  };

  const handlePointerDown = (e: React.PointerEvent<HTMLDivElement>) => {
    const coords = getEventCoordinates(e.clientX, e.clientY);
    if (!coords) return;

    const piece = board[coords.row]?.[coords.col];
    const playerColorToken = currentPlayer === 0 ? 'W' : 'B';

    activeDragStart.current = coords;

    if (piece === playerColorToken) {
      e.currentTarget.setPointerCapture(e.pointerId);
      setDraggedPiece(piece);
      setDragSource(coords);
      setMousePosition({ x: e.clientX, y: e.clientY });
    }
  };

  const handlePointerMove = (e: React.PointerEvent<HTMLDivElement>) => {
    if (!activeDragStart.current) return;
    setMousePosition({ x: e.clientX, y: e.clientY });
  };

  const handlePointerUp = (e: React.PointerEvent<HTMLDivElement>) => {
    if (!activeDragStart.current) return;
    
    e.currentTarget.releasePointerCapture(e.pointerId);

    const targetCoords = getEventCoordinates(e.clientX, e.clientY);
    const from = activeDragStart.current;

    if (targetCoords) {
      const isDragAction = from.row !== targetCoords.row || from.col !== targetCoords.col;

      if (isDragAction) {
        executeMove(from, targetCoords);
      } else {
        const isAlreadySelected = selectedCoords !== null && 
          selectedCoords.row === from.row && 
          selectedCoords.col === from.col;

        if (isAlreadySelected) {
          selectPiece(null);
        } else if (selectedCoords) {
          const moved = executeMove(selectedCoords, targetCoords);
          if (!moved) {
            selectPiece(targetCoords);
          }
        } else {
          selectPiece(targetCoords);
        }
      }
    }

    activeDragStart.current = null;
    setDraggedPiece(null);
    setDragSource(null);
  };

  return {
    handlePointerDown,
    handlePointerMove,
    handlePointerUp,
    draggedPiece,
    mousePosition,
    dragSource
  };
};