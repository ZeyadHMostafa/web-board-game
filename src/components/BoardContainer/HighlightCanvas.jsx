import { useEffect, useRef } from 'react';
import { CanvasDrawers } from '../../utils/canvasDrawers';

export default function HighlightCanvas({ lastMove, boardSize }) {
  const canvasRef = useRef(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    CanvasDrawers.clear(ctx, boardSize);

    if (!lastMove) return;

    CanvasDrawers.drawTileHighlight(ctx, lastMove.from.row, lastMove.from.col, 'rgba(14, 110, 206, 0.18)', boardSize); 
    CanvasDrawers.drawTileHighlight(ctx, lastMove.to.row, lastMove.to.col, 'rgba(36, 215, 251, 0.28)', boardSize);
  }, [lastMove, boardSize]);

  return (
    <canvas
      ref={canvasRef}
      width={boardSize}
      height={boardSize}
      className="absolute top-0 left-0 pointer-events-none"
    />
  );
}