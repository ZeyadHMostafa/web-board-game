import React from 'react';
import BaseSvgLayer from './BaseSvgLayer';
import SvgArrow from '../library/svg/svgArrow';
import {useGameStore} from '../../../store/useGameStore';

export const AssistLayer: React.FC = () => {

  const liveEval = useGameStore((state) => state.liveEval);
  const showAssist = useGameStore((state) => state.showAssist);
  const config = useGameStore((state) => state.config);

  if (!showAssist || !liveEval?.candidates) {
    return null;
  }

  return (
    <BaseSvgLayer zIndex={4}>
      {liveEval.candidates.slice(0,config?.maxAssistMovesShown??0).map((candidate, index) => (
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