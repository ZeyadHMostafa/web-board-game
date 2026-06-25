import React from 'react';
import { useGame } from '../../../context/GameContext';
import BaseSvgLayer from './BaseSvgLayer';
import { SvgGridCell } from '../library/SvgGridCell';

export const LastMoveLayer: React.FC = () => {
  const { history, currentIndex} = useGame();
  if (history.length == 1) return;
  const lastMove = history[currentIndex].lastMove;

  if (!lastMove) return null;

  return (
    <BaseSvgLayer zIndex={1}>
      <SvgGridCell row={lastMove.from.row} col={lastMove.from.col}>
        <rect width={100} height={100} fill="var(--color-highlight-from)" />
      </SvgGridCell>
      <SvgGridCell row={lastMove.to.row} col={lastMove.to.col}>
        <rect width={100} height={100} fill="var(--color-highlight-to)" />
      </SvgGridCell>
    </BaseSvgLayer>
  );
};

export default LastMoveLayer;