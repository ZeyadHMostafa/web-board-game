import React from 'react';
import { useGame } from '../../../context/GameContext';
import BaseSvgLayer from './BaseSvgLayer';
import { SvgGridCell } from '../library/SvgGridCell';
import SvgPiece from '../library/svg/svgPiece';
import type {Coordinate} from '../../../domain/types';

interface PieceLayerProps {
  dragSource: Coordinate | null;
}

export const PieceLayer: React.FC<PieceLayerProps> = ({ dragSource }) => {
  const { board } = useGame();

  return (
    <BaseSvgLayer zIndex={30}>
      {board.map((rowArray, rowIndex) =>
        rowArray.map((cell, colIndex) => {
          if (!cell) return null;

          // Check if this specific piece is currently being lifted by the drag overlay
          const isBeingDragged = dragSource !== null && 
                                 dragSource.row === rowIndex && 
                                 dragSource.col === colIndex;

          if (isBeingDragged) return null;

          return (
            <SvgGridCell key={`${rowIndex}-${colIndex}`} row={rowIndex} col={colIndex}>
              <SvgPiece type={cell} />
            </SvgGridCell>
          );
        })
      )}
    </BaseSvgLayer>
  );
};

export default PieceLayer;