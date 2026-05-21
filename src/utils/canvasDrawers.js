import { BoardGeometry, TILE_SIZE } from './boardGeometry';

/**
 * Pure utility operations to offload low-level canvas context manipulations.
 */
export const CanvasDrawers = {
  /**
   * Clears the entire canvas viewport safely
   */
  clear(ctx, size) {
    ctx.clearRect(0, 0, size, size);
  },

  /**
   * Draws a composite vector line with a sharp white silhouette backing
   */
  drawVectorLine(ctx, fromPixels, toPixels, color) {
    ctx.lineCap = 'round';

    // 1. Thick white shadow silhouette for contrast
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.9)';
    ctx.lineWidth = 9;
    ctx.beginPath();
    ctx.moveTo(fromPixels.x, fromPixels.y);
    ctx.lineTo(toPixels.x, toPixels.y);
    ctx.stroke();

    // 2. Main colored path core
    ctx.strokeStyle = color;
    ctx.lineWidth = 5;
    ctx.beginPath();
    ctx.moveTo(fromPixels.x, fromPixels.y);
    ctx.lineTo(toPixels.x, toPixels.y);
    ctx.stroke();
  },

  /**
   * Draws a dual-ring anchor point node at a given location
   */
  drawAnchorNode(ctx, pixels, color) {
    // Outer white rim
    ctx.fillStyle = '#ffffff';
    ctx.beginPath();
    ctx.arc(pixels.x, pixels.y, 8, 0, Math.PI * 2);
    ctx.fill();

    // Inner colored core
    ctx.fillStyle = color;
    ctx.beginPath();
    ctx.arc(pixels.x, pixels.y, 5, 0, Math.PI * 2);
    ctx.fill();
  },

  /**
   * High-level orchestrator that takes an abstract move array and maps it visually
   */
  renderAssistOverlay(ctx, moves, displayLimit) {
    for (let i = 0; i < displayLimit; i++) {
      const move = moves[i];
      
      // Translate matrix indices to concrete rendering positions
      const startPixels = BoardGeometry.matrixToPixels(move.from.row, move.from.col);
      const endPixels = BoardGeometry.matrixToPixels(move.to.row, move.to.col);

      // Quality falloff calculation (matches your original Pygame engine look)
      const intensity = (1 - i / displayLimit) * 0.8 + 0.2;
      const moveColor = `rgba(147, 197, 253, ${intensity})`; // Tailwind blue-300 with alpha

      // Execute clear geometric actions
      this.drawVectorLine(ctx, startPixels, endPixels, moveColor);
      this.drawAnchorNode(ctx, startPixels, moveColor);
      this.drawAnchorNode(ctx, endPixels, moveColor);
    }
  },
  
  /**
   * Draws total tile control integers directly onto the center of each tile
   */
  drawControlText(ctx, row, col, totalControl) {
    const { x, y } = BoardGeometry.matrixToPixels(row, col);

    // Styling configurations
    ctx.font = 'bold 14px monospace';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';

    // Highlight text color based on who holds positive control advantages
    if (totalControl > 0) {
      ctx.fillStyle = '#60a5fa'; // Blue advantage (White)
    } else if (totalControl < 0) {
      ctx.fillStyle = '#f87171'; // Red advantage (Black)
    } else {
      ctx.fillStyle = '#94a3b8'; // Balanced slate-400
    }

    ctx.fillText(totalControl.toString(), x, y);
  },

  /**
   * Draws soft, semi-transparent highlight overlay squares on specific grid tiles
   */
  drawTileHighlight(ctx, row, col, color = 'rgba(251, 191, 36, 0.4)') { // Default subtle amber tint
    const { x, y } = BoardGeometry.matrixToTileTopLeft(row, col);
    ctx.fillStyle = color;
    ctx.fillRect(x, y, TILE_SIZE, TILE_SIZE);
  }
};

