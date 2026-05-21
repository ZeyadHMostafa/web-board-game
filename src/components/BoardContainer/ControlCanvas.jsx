import { useEffect, useRef } from 'react';
import { BOARD_SIZE } from '../../utils/boardGeometry';
import { CanvasDrawers } from '../../utils/canvasDrawers';
import { EngineAdapterMock } from '../../utils/engineAdapterMock';

export default function ControlCanvas({ boardState, showControl }) {
  const canvasRef = useRef(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    CanvasDrawers.clear(ctx, BOARD_SIZE);

    // Structural guard: do not proceed with drawing routines if layer is hidden
    if (!showControl) return;

    // Fetch the structural values directly from our decoupled engine adapter utility
    const controlMap = EngineAdapterMock.getControlMap(boardState);

    // Loop through the matrix rows and columns to draw the evaluation metrics
    for (let row = 0; row < 8; row++) {
      for (let col = 0; col < 8; col++) {
        const totalControl = controlMap[row][col];
        
        // Execute the draw utility method to display integers onto the canvas center
        CanvasDrawers.drawControlText(ctx, row, col, totalControl);
      }
    }
  }, [boardState, showControl]);

  return (
    <canvas
      ref={canvasRef}
      width={BOARD_SIZE}
      height={BOARD_SIZE}
      className="absolute top-0 left-0 pointer-events-none z-20"
    />
  );
}