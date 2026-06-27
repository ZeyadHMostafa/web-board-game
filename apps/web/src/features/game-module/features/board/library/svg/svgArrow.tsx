import React from 'react';
import type { Coordinate } from '../../../../domain/types';
import { GridGeometry } from '../../../../utils/gridGeometry';

interface SvgArrowProps {
  from: Coordinate;
  to: Coordinate;
  color: string;
  index: number;
  total: number;
}

export const SvgArrow: React.FC<SvgArrowProps> = ({ from, to, color, index, total }) => {
  const start = GridGeometry.matrixToVectorCenter(from.row, from.col);
  const end = GridGeometry.matrixToVectorCenter(to.row, to.col);
  
  const intensity = (1 - index / total) * 0.8 + 0.2;
  const strokeWidth = 8 * intensity;
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
      <circle cx={start.x} cy={start.y} r={6} fill={vectorSilhouette} />
      <circle cx={end.x} cy={end.y} r={6} fill={color} />
    </g>
  );
};

export default SvgArrow;