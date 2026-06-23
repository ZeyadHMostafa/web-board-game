import React from 'react';
import { useGame } from '../../../context/GameContext';
import BaseSvgLayer from './BaseSvgLayer';
import SvgArrow from '../library/svg/svgArrow';

export const AssistLayer: React.FC = () => {
  const { liveEval, config, showAssist } = useGame();

  if (!showAssist || !liveEval?.candidates) {
    return null;
  }

  return (
    <BaseSvgLayer zIndex={40}>
      {liveEval.candidates.slice(0,config.maxAssistMovesShown).map((candidate, index) => (
        <SvgArrow
          key={`assist-arrow-${candidate.from.row}-${candidate.from.col}-${candidate.to.row}-${candidate.to.col}`}
          from={candidate.from}
          to={candidate.to}
          color="var(--color-accent-primary)"
          index={index}
          total={liveEval.candidates.length}
        />
      ))}
    </BaseSvgLayer>
  );
};

export default AssistLayer;