import React from 'react';
import type { Coordinate } from '../../../domain/types';
import { GridGeometry } from '../../../utils/gridGeometry';

interface SvgArrowProps {
  from: Coordinate;
  to: Coordinate;
  color: string;
  boardSize: number;
  index: number;
  total: number;
}

export const SvgArrow: React.FC<SvgArrowProps> = ({ from, to, color, boardSize, index, total }) => {
  const start = GridGeometry.matrixToPixels(from.row, from.col, boardSize);
  const end = GridGeometry.matrixToPixels(to.row, to.col, boardSize);
  
  const scale = boardSize / 512;
  const intensity = (1 - index / total) * 0.8 + 0.2;
  const strokeWidth = 5 * scale * intensity;

  const vectorSilhouette = 'var(--color-text-main)';

  return (
    <g opacity={intensity}>
      <line 
        x1={start.x} y1={start.y} x2={end.x} y2={end.y}
        stroke={vectorSilhouette} strokeWidth={strokeWidth * 1.8} strokeLinecap="round"
      />
      <line 
        x1={start.x} y1={start.y} x2={end.x} y2={end.y}
        stroke={color} strokeWidth={strokeWidth} strokeLinecap="round"
      />
      <circle cx={start.x} cy={start.y} r={4 * scale} fill={vectorSilhouette} />
      <circle cx={end.x} cy={end.y} r={4 * scale} fill={color} />
    </g>
  );
};

interface SvgPieceProps {
  type: 'W' | 'B';
  tileSize: number;
}

export const SvgPiece: React.FC<SvgPieceProps> = ({ type, tileSize }) => {
  const isWhite = type === 'W';
  const center = tileSize / 2;
  
  const outerRadius = (tileSize * 0.75) / 2;
  const innerRadius = (tileSize * 0.625) / 2;

  const outerFill = isWhite ? 'var(--color-piece-white-ring)' : 'var(--color-piece-black-ring)';
  const innerFill = isWhite ? 'var(--color-piece-white-fill)' : 'var(--color-piece-black-fill)';

  return (
    <svg 
      width={tileSize} 
      height={tileSize} 
      viewBox={`0 0 ${tileSize} ${tileSize}`}
      className="drop-shadow-md select-none pointer-events-none"
    >
      <circle 
        cx={center} 
        cy={center} 
        r={outerRadius} 
        fill={outerFill} 
      />
      <circle 
        cx={center} 
        cy={center} 
        r={innerRadius} 
        fill={innerFill}
        className="drop-shadow-inner"
      />
    </svg>
  );
};