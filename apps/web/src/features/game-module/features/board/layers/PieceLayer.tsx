import React from 'react';
import BaseSvgLayer from './BaseSvgLayer';
import { SvgGridCell } from '../library/SvgGridCell';
import SvgPiece from '../library/svg/svgPiece';
import type {Coordinate} from '../../../domain/types';
import {useGameStore} from '../../../store/useGameStore';

interface PieceLayerProps {
  dragSource: Coordinate | null;
}

export const PieceLayer: React.FC<PieceLayerProps> = ({ dragSource }) => {
  const history = useGameStore((state) => state.history);
  const currentIndex = useGameStore((state) => state.currentIndex);
  const board = history[currentIndex]?.board;

  return (
    <BaseSvgLayer zIndex={3}>
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