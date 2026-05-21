import { useEffect, useRef } from 'react';
import { CanvasDrawers } from '../../utils/canvasDrawers';

export default function SelectionCanvas({ selectedCoords, validMoves, boardState, boardSize }) {
  const canvasRef = useRef(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    CanvasDrawers.clear(ctx, boardSize);

    // Render selection box if a piece is active
    if (selectedCoords) {
      CanvasDrawers.drawSelectionHighlight(ctx, selectedCoords.row, selectedCoords.col, boardSize);
    }

    // Render individual destination dots or target rings
    if (validMoves && validMoves.length > 0) {
      for (const moveTarget of validMoves) {
        CanvasDrawers.drawValidMoveDot(ctx, moveTarget.row, moveTarget.col, boardState, boardSize);
      }
    }
  }, [selectedCoords, validMoves, boardState, boardSize]);

  return (
    <canvas
      ref={canvasRef}
      width={boardSize}
      height={boardSize}
      className="absolute top-0 left-0 pointer-events-none"
    />
  );
}