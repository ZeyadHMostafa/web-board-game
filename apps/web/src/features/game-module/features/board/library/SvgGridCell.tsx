import React from 'react';
import { GridGeometry } from '../../../utils/gridGeometry';

interface SvgGridCellProps {
  row: number;
  col: number;
  children: React.ReactNode;
}

export const SvgGridCell: React.FC<SvgGridCellProps> = ({ row, col, children }) => {
  const { x, y } = GridGeometry.matrixToVectorTopLeft(row, col);

  return (
    <g transform={`translate(${x}, ${y})`}>
      {children}
    </g>
  );
};