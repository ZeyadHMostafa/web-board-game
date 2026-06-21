import { useRef, useImperativeHandle, forwardRef } from 'react';

// Explicit type contract for the imperative layout bridge
export interface CanvasLayerHandle {
  redraw: (onDraw: (ctx: CanvasRenderingContext2D) => void) => void;
}

interface BaseCanvasLayerProps {
  boardSize: number;
  zIndex: number;
}

export const BaseCanvasLayer = forwardRef<CanvasLayerHandle, BaseCanvasLayerProps>(
  ({ boardSize, zIndex }, ref) => {
    const canvasRef = useRef<HTMLCanvasElement>(null);

    useImperativeHandle(ref, () => ({
      redraw: (onDraw) => {
        const canvas = canvasRef.current;
        if (!canvas) return;
        const ctx = canvas.getContext('2d');
        if (!ctx) return;
        onDraw(ctx);
      }
    }));

    return (
      <canvas
        ref={canvasRef}
        width={boardSize}
        height={boardSize}
        style={{ zIndex }}
        className="absolute top-0 left-0 pointer-events-none"
      />
    );
  }
);

BaseCanvasLayer.displayName = 'BaseCanvasLayer';
export default BaseCanvasLayer;