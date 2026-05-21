import { useEffect, useRef } from 'react';
import { BOARD_SIZE } from '../../utils/boardGeometry';
import { CanvasDrawers } from '../../utils/canvasDrawers';

export default function AssistCanvas({ moves = [], showAssist = false }) {
  const canvasRef = useRef(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    
    // 1. Always reset pixels before processing state shifts
    CanvasDrawers.clear(ctx, BOARD_SIZE);

    // 2. Conditional check for early exit
    if (!showAssist || moves.length === 0) return;

    // 3. Draw using high-level readability abstractions
    const maxMovesToShow = Math.min(moves.length, 5);
    CanvasDrawers.renderAssistOverlay(ctx, moves, maxMovesToShow);

  }, [moves, showAssist]);

  return (
    <canvas
      ref={canvasRef}
      width={BOARD_SIZE}
      height={BOARD_SIZE}
      className="absolute top-0 left-0 pointer-events-none mix-blend-screen z-10"
    />
  );
}