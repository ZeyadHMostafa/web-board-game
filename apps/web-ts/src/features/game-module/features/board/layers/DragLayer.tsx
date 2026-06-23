import React from 'react';
import type {PlayerColor} from '../../../domain/types';
import SvgPiece from '../library/svg/svgPiece';

interface DragLayerProps {
  type: PlayerColor | null;
  mousePos: { x: number; y: number };
}

export const DragLayer: React.FC<DragLayerProps> = ({ type, mousePos }) => {
  if (!type) return null;

  return (
    <div
      className="fixed top-0 left-0 pointer-events-none z-50 opacity-70 mix-blend-screen w-[64px] h-[64px]"
      style={{
        transform: `translate3d(${mousePos.x - 32}px, ${mousePos.y - 32}px, 0px)`,
      }}
    >
      <svg viewBox="0 0 100 100" className="w-full h-full">
        <SvgPiece type={type} />
      </svg>
    </div>
  );
};

export default DragLayer;