import { BoardGeometry } from './boardGeometry';
import { BoardMatrix } from './boardMatrix';

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
  drawVectorLine(ctx, fromPixels, toPixels, color, boardSize) {
    ctx.lineCap = 'round';
    
    const scaleFactor = boardSize / 512;

    // 1. Thick white shadow silhouette for contrast
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.9)';
    ctx.lineWidth = 9 * scaleFactor;
    ctx.beginPath();
    ctx.moveTo(fromPixels.x, fromPixels.y);
    ctx.lineTo(toPixels.x, toPixels.y);
    ctx.stroke();

    // 2. Main colored path core
    ctx.strokeStyle = color;
    ctx.lineWidth = 5 * scaleFactor;
    ctx.beginPath();
    ctx.moveTo(fromPixels.x, fromPixels.y);
    ctx.lineTo(toPixels.x, toPixels.y);
    ctx.stroke();
  },

  /**
   * Draws a dual-ring anchor point node at a given location
   */
  drawAnchorNode(ctx, pixels, color, boardSize) {
    const scaleFactor = boardSize / 512;

    // Outer white rim
    ctx.fillStyle = '#ffffff';
    ctx.beginPath();
    ctx.arc(pixels.x, pixels.y, 8 * scaleFactor, 0, Math.PI * 2);
    ctx.fill();

    // Inner colored core
    ctx.fillStyle = color;
    ctx.beginPath();
    ctx.arc(pixels.x, pixels.y, 5 * scaleFactor, 0, Math.PI * 2);
    ctx.fill();
  },

  /**
   * High-level orchestrator that takes an abstract move array and maps it visually
   */
  renderAssistOverlay(ctx, moves, displayLimit, boardSize) {
    for (let i = 0; i < displayLimit; i++) {
      const move = moves[i];
      
      const startPixels = BoardGeometry.matrixToPixels(move.from.row, move.from.col, boardSize);
      const endPixels = BoardGeometry.matrixToPixels(move.to.row, move.to.col, boardSize);

      const intensity = (1 - i / displayLimit) * 0.8 + 0.2;
      const moveColor = `rgba(147, 197, 253, ${intensity})`;

      this.drawVectorLine(ctx, startPixels, endPixels, moveColor, boardSize);
      this.drawAnchorNode(ctx, startPixels, moveColor, boardSize);
      this.drawAnchorNode(ctx, endPixels, moveColor, boardSize);
    }
  },
  
  /**
   * Draws total tile control integers directly onto the center of each tile
   */
  drawControlText(ctx, row, col, totalControl, boardSize) {
    const { x, y } = BoardGeometry.matrixToPixels(row, col, boardSize);
    const dynamicFontSize = Math.max(10, Math.floor(boardSize / 36));

    ctx.font = `bold ${dynamicFontSize}px monospace`;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';

    if (totalControl > 0) {
      ctx.fillStyle = '#60a5fa'; 
    } else if (totalControl < 0) {
      ctx.fillStyle = '#f87171'; 
    } else {
      ctx.fillStyle = '#94a3b8'; 
    }

    ctx.fillText(totalControl.toString(), x, y);
  },

  /**
   * Draws soft, semi-transparent highlight overlay squares on specific grid tiles
   */
  drawTileHighlight(ctx, row, col, color = 'rgba(251, 191, 36, 0.4)', boardSize) {
    const { x, y } = BoardGeometry.matrixToTileTopLeft(row, col, boardSize);
    const tileSize = boardSize / 8;
    ctx.fillStyle = color;
    ctx.fillRect(x, y, tileSize, tileSize);
  },
  /**
   * Draws a unique highlight or border around the currently selected piece
   */
  drawSelectionHighlight(ctx, row, col, boardSize) {
    const { x, y } = BoardGeometry.matrixToTileTopLeft(row, col, boardSize);
    const tileSize = boardSize / 8;
    const padding = tileSize * 0.08; // 8% inner padding

    // A crisp, clean neon border around the active selection
    ctx.strokeStyle = 'rgba(96, 165, 250, 0.85)'; // slate-400 / blue accent
    ctx.lineWidth = Math.max(2, boardSize / 256);
    ctx.lineJoin = 'round';
    
    ctx.strokeRect(
      x + padding, 
      y + padding, 
      tileSize - padding * 2, 
      tileSize - padding * 2
    );
    
    // Soft inner glow filling
    ctx.fillStyle = 'rgba(96, 165, 250, 0.08)';
    ctx.fillRect(
      x + padding, 
      y + padding, 
      tileSize - padding * 2, 
      tileSize - padding * 2
    );
  },

  /**
   * Draws a clean, non-intrusive indicator dot on a valid target square
   */
  drawValidMoveDot(ctx, row, col, board, boardSize) {
    const { x, y } = BoardGeometry.matrixToPixels(row, col, boardSize);
    const scaleFactor = boardSize / 512;
    const tileSize = boardSize / 8;

    // Check if the destination tile is occupied using BoardMatrix structural parsing
    const targetPiece = BoardMatrix.getPiece(board, row, col);

    if (targetPiece !== null) {
      // CAPTURE INDICATOR: If an enemy piece is on the tile, draw a subtle corner-bracket or outer ring
      ctx.strokeStyle = 'rgba(248, 113, 113, 0.6)'; // soft red capture alert
      ctx.lineWidth = 3 * scaleFactor;
      ctx.beginPath();
      ctx.arc(x, y, (tileSize / 2) - (6 * scaleFactor), 0, Math.PI * 2);
      ctx.stroke();
    } else {
      // EMPTY MOVE INDICATOR: Clean, small translucent radial dot in the center
      ctx.fillStyle = 'rgba(52, 211, 153, 0.5)'; // minty emerald green for clean tracking
      ctx.beginPath();
      ctx.arc(x, y, 6 * scaleFactor, 0, Math.PI * 2);
      ctx.fill();
    }
  },
};