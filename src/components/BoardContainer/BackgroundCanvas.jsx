import { useEffect, useRef } from 'react';
import { BOARD_SIZE, TILE_SIZE, BoardGeometry } from '../../utils/boardGeometry';
import { CanvasDrawers } from '../../utils/canvasDrawers';

export default function BackgroundCanvas() {
  const canvasRef = useRef(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    CanvasDrawers.clear(ctx, BOARD_SIZE);

    // 1. Render the classic alternating checkerboard grid squares
    for (let row = 0; row < 8; row++) {
      for (let col = 0; col < 8; col++) {
        // Use the geometry engine to determine top-left boundaries
        const { x, y } = BoardGeometry.matrixToTileTopLeft(row, col);
        
        const isDark = (row + col) % 2 === 1;
        ctx.fillStyle = isDark ? '#1e293b' : '#334155'; // Slate-800 vs Slate-700
        ctx.fillRect(x, y, TILE_SIZE, TILE_SIZE);
      }
    }

    // 2. Configure font styles for board border notations
    ctx.font = 'bold 11px monospace';
    ctx.textBaseline = 'middle';

    const files = "ABCDEFGH";

    // 3. Render Algebraic Annotations (Files A-H & Ranks 1-8)
    for (let i = 0; i < 8; i++) {
      // ---- Draw Files (A-H) along the bottom edge of each column ----
      // Positioned near the bottom-right corner of each square in the bottom visual row
      const fileX = i * TILE_SIZE + TILE_SIZE - 8;
      const fileY = BOARD_SIZE - 8;
      
      // Alternate text color based on the underlying tile color for high readability
      const isFileSquareDark = (0 + i) % 2 === 1; 
      ctx.fillStyle = isFileSquareDark ? '#64748b' : '#475569'; // Slate-500 vs Slate-600
      ctx.textAlign = 'right';
      ctx.fillText(files[i], fileX, fileY);

      // ---- Draw Ranks (1-8) along the left edge of each row ----
      // Positioned near the top-left corner of each square in the first visual column
      const rankX = 6;
      const rankY = i * TILE_SIZE + 8;
      
      // Calculate matrix row index based on visual top-down index to get the rank string
      const matrixRow = 7 - i;
      const rankLabel = BoardGeometry.matrixToAlgebraic(matrixRow, 0).substring(1);

      const isRankSquareDark = (matrixRow + 0) % 2 === 1;
      ctx.fillStyle = isRankSquareDark ? '#64748b' : '#475569';
      ctx.textAlign = 'left';
      ctx.fillText(rankLabel, rankX, rankY);
    }
  }, []);

  return (
    <canvas
      ref={canvasRef}
      width={BOARD_SIZE}
      height={BOARD_SIZE}
      className="absolute top-0 left-0"
    />
  );
}