import React from 'react';
import {useGame} from '../../../context/GameContext';

export const EvalBar: React.FC = () => {
  const { liveEval, gameEnded } = useGame();

  if (gameEnded || !liveEval || liveEval.candidates.length === 0) {
    return <div className="h-1.5 w-full bg-border-muted transition-all duration-300" />;
  }

  // Use the top candidate's score to calculate proportions.
  // Assuming scoreValue is scaled (e.g., centipawns or direct win probability).
  const topScore = liveEval.candidates[0].scoreValue;
  
  // Map score to a percentage clamping between 5% and 95% so neither side vanishes entirely
  const minPct = 5;
  const maxPct = 95;
  let whitePercentage = 50 + (topScore * 5); // Simple scaling baseline
  whitePercentage = Math.max(minPct, Math.min(maxPct, whitePercentage));

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