import React from 'react';
import { useGame } from '../../../context/GameContext';
import BaseSvgLayer from './BaseSvgLayer';
import { SvgGridCell } from '../library/SvgGridCell';
import { GridGeometry } from '../../../utils/gridGeometry';

export const SelectionLayer: React.FC = () => {
  const { board, selectedCoords, validMoves } = useGame();

  return (
    <BaseSvgLayer zIndex={20}>
      {selectedCoords && (
        <SvgGridCell row={selectedCoords.row} col={selectedCoords.col}>
          <rect
            x={8}
            y={8}
            width={84}
            height={84}
            fill="var(--color-selection-bg)"
            stroke="var(--color-accent-glow)"
            strokeWidth={4}
            rx={4}
          />
        </SvgGridCell>
      )}

      {validMoves?.map((moveTarget) => {
        const isCapture = board[moveTarget.row]?.[moveTarget.col] !== null;
        const { x, y } = GridGeometry.matrixToVectorCenter(moveTarget.row, moveTarget.col);

        return isCapture ? (
          <circle
            key={`valid-capture-${moveTarget.row}-${moveTarget.col}`}
            cx={x}
            cy={y}
            r={42}
            fill="none"
            stroke="var(--color-indicator-capture)"
            strokeWidth={6}
            opacity={0.7}
          />
        ) : (
          <circle
            key={`valid-target-${moveTarget.row}-${moveTarget.col}`}
            cx={x}
            cy={y}
            r={12}
            fill="var(--color-indicator-legal)"
            opacity={0.6}
          />
        );
      })}
    </BaseSvgLayer>
  );
};

export default SelectionLayer;