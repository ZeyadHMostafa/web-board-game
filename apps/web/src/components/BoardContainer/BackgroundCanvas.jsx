import { useEffect, useRef } from 'react';
import { BoardGeometry } from '../../utils/boardGeometry';
import { CanvasDrawers } from '../../utils/canvasDrawers';

export default function BackgroundCanvas({ boardSize }) {
  const canvasRef = useRef(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    const tileSize = boardSize / 8;
    CanvasDrawers.clear(ctx, boardSize);

    for (let row = 0; row < 8; row++) {
      for (let col = 0; col < 8; col++) {
        const { x, y } = BoardGeometry.matrixToTileTopLeft(row, col, boardSize);
        
        const isDark = (row + col) % 2 === 1;
        ctx.fillStyle = isDark ? '#1e293b' : '#334155';
        ctx.fillRect(x, y, tileSize, tileSize);
      }
    }

    const dynamicFontSize = Math.max(9, Math.floor(boardSize / 46));
    ctx.font = `bold ${dynamicFontSize}px monospace`;
    ctx.textBaseline = 'middle';

    const files = "ABCDEFGH";
    const padding = Math.max(4, boardSize * 0.015);

    for (let i = 0; i < 8; i++) {
      const fileX = i * tileSize + tileSize - padding;
      const fileY = boardSize - padding;
      
      const isFileSquareDark = (0 + i) % 2 === 1; 
      ctx.fillStyle = isFileSquareDark ? '#64748b' : '#475569'; 
      ctx.textAlign = 'right';
      ctx.fillText(files[i], fileX, fileY);

      const rankX = padding;
      const rankY = i * tileSize + padding;
      
      const matrixRow = 7 - i;
      const rankLabel = BoardGeometry.matrixToAlgebraic(matrixRow, 0).substring(1);

      const isRankSquareDark = (matrixRow + 0) % 2 === 1;
      ctx.fillStyle = isRankSquareDark ? '#64748b' : '#475569';
      ctx.textAlign = 'left';
      ctx.fillText(rankLabel, rankX, rankY);
    }
  }, [boardSize]);

  return (
    <canvas
      ref={canvasRef}
      width={boardSize}
      height={boardSize}
      className="absolute top-0 left-0"
    />
  );
}