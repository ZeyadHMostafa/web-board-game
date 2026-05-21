import { useEffect, useRef } from 'react';
import { CanvasDrawers } from '../../utils/canvasDrawers';
import { EngineAdapterMock } from '../../utils/engineAdapterMock';

export default function ControlCanvas({ boardState, showControl, boardSize }) {
  const canvasRef = useRef(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    CanvasDrawers.clear(ctx, boardSize);

    if (!showControl) return;

    const controlMap = EngineAdapterMock.getControlMap(boardState);

    for (let row = 0; row < 8; row++) {
      for (let col = 0; col < 8; col++) {
        const totalControl = controlMap[row][col];
        
        CanvasDrawers.drawControlText(ctx, row, col, totalControl, boardSize);
      }
    }
  }, [boardState, showControl, boardSize]);

  return (
    <canvas
      ref={canvasRef}
      width={boardSize}
      height={boardSize}
      className="absolute top-0 left-0 pointer-events-none z-20"
    />
  );
}