import { useEffect, useRef } from 'react';
import { CanvasDrawers } from '../../utils/canvasDrawers';

export default function AssistCanvas({ moves = [], showAssist = false, boardSize }) {
  const canvasRef = useRef(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    
    CanvasDrawers.clear(ctx, boardSize);

    if (!showAssist || moves.length === 0) return;

    const maxMovesToShow = Math.min(moves.length, 5);
    CanvasDrawers.renderAssistOverlay(ctx, moves, maxMovesToShow, boardSize);

  }, [moves, showAssist, boardSize]);

  return (
    <canvas
      ref={canvasRef}
      width={boardSize}
      height={boardSize}
      className="absolute top-0 left-0 pointer-events-none mix-blend-screen z-10"
    />
  );
}