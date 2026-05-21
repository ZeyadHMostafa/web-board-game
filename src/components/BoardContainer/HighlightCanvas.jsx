import { useEffect, useRef } from 'react';
import { BOARD_SIZE } from '../../utils/boardGeometry';
import { CanvasDrawers } from '../../utils/canvasDrawers';

export default function HighlightCanvas({ lastMove }) {
  const canvasRef = useRef(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    CanvasDrawers.clear(ctx, BOARD_SIZE);

    // If no moves have been recorded yet (start of game), leave the canvas transparent
    if (!lastMove) return;

    // Highlight the source square ('from') with a soft alpha tint
    CanvasDrawers.drawTileHighlight(ctx, lastMove.from.row, lastMove.from.col, 'rgba(14, 110, 206, 0.18)'); // Subtle slate glow
    
    // Highlight the destination square ('to') with a slightly stronger matching alpha accent tint
    CanvasDrawers.drawTileHighlight(ctx, lastMove.to.row, lastMove.to.col, 'rgba(36, 215, 251, 0.28)'); // Amber accent highlight
  }, [lastMove]);

  return (
    <canvas
      ref={canvasRef}
      width={BOARD_SIZE}
      height={BOARD_SIZE}
      className="absolute top-0 left-0 pointer-events-none"
    />
  );
}