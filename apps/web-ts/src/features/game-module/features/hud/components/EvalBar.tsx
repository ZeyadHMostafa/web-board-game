import React from 'react';
import { useGameStore } from '../../../store/useGameStore';

export const EvalBar: React.FC = () => {
  const liveEval = useGameStore((state) => state.liveEval);
  const gameEnded = useGameStore((state) => state.gameEnded);
  const history = useGameStore((state) => state.history);
  const currentIndex = useGameStore((state) => state.currentIndex);

  const currentSnapshot = history[currentIndex] || { currentPlayer: 0 };
  const currentPlayer = currentSnapshot.currentPlayer;


  if (gameEnded || !liveEval || liveEval.candidates.length === 0) {
    return <div className="h-1.5 w-full bg-border-muted transition-all duration-300" />;
  }
  
  const topMoveScore = liveEval.candidates[0].scoreValue

  const whitePercentage = Math.tanh(topMoveScore*[1,-1][currentPlayer]/100)*50+50

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