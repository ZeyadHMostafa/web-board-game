import { GridGeometry } from '../../../utils/gridGeometry';

export const CanvasDrawers = {
  clear(ctx: CanvasRenderingContext2D, size: number): void {
    ctx.clearRect(0, 0, size, size);
  },

  drawTileHighlight(ctx: CanvasRenderingContext2D, row: number, col: number, color: string, boardSize: number): void {
    const { x, y } = GridGeometry.matrixToTileTopLeft(row, col, boardSize);
    const tileSize = boardSize / 8;
    ctx.fillStyle = color;
    ctx.fillRect(x, y, tileSize, tileSize);
  },

  drawSelectionRing(ctx: CanvasRenderingContext2D, row: number, col: number, boardSize: number): void {
    const { x, y } = GridGeometry.matrixToTileTopLeft(row, col, boardSize);
    const tileSize = boardSize / 8;
    const padding = tileSize * 0.08;

    ctx.strokeStyle = 'rgba(96, 165, 250, 0.85)';
    ctx.lineWidth = Math.max(2, boardSize / 256);
    ctx.lineJoin = 'round';
    
    ctx.strokeRect(x + padding, y + padding, tileSize - padding * 2, tileSize - padding * 2);
    ctx.fillStyle = 'rgba(96, 165, 250, 0.08)';
    ctx.fillRect(x + padding, y + padding, tileSize - padding * 2, tileSize - padding * 2);
  },

  drawValidMoveIndicator(ctx: CanvasRenderingContext2D, row: number, col: number, isCapture: boolean, boardSize: number): void {
    const { x, y } = GridGeometry.matrixToPixels(row, col, boardSize);
    const tileSize = boardSize / 8;
    const scaleFactor = boardSize / 512;

    if (isCapture) {
      ctx.strokeStyle = 'rgba(248, 113, 113, 0.6)';
      ctx.lineWidth = 3 * scaleFactor;
      ctx.beginPath();
      ctx.arc(x, y, (tileSize / 2) - (6 * scaleFactor), 0, Math.PI * 2);
      ctx.stroke();
    } else {
      ctx.fillStyle = 'rgba(16, 185, 129, 0.5)';
      ctx.beginPath();
      ctx.arc(x, y, 6 * scaleFactor, 0, Math.PI * 2);
      ctx.fill();
    }
  }
};