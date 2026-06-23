import React from 'react';
import {useGame} from '../../../context/GameContext';

export const EvalBar: React.FC = () => {
  const { liveEval, gameEnded } = useGame();

  if (gameEnded || !liveEval || liveEval.candidates.length === 0) {
    return <div className="h-1.5 w-full bg-border-muted transition-all duration-300" />;
  }
  
  const whitePercentage = Math.tanh(liveEval.candidates[0].scoreValue/100)*50+50

  return (
    <div 
      className="w-full h-1.5 flex bg-piece-black-fill overflow-hidden transition-all duration-300"
      style={{ '--white-bias': `${whitePercentage}%` } as React.CSSProperties}
    >
      <div 
        className="h-full bg-piece-white-fill transition-[width] duration-500 ease-out"
        style={{ width: 'var(--white-bias)' }}
        title={`White Advantage: ${whitePercentage.toFixed(0)}%`}
      />
      <div className="h-full flex-1 bg-piece-black-fill" />
    </div>
  );
};

export default EvalBar;