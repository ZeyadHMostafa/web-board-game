import React, { useState, useEffect } from 'react';
import {useGame} from '../../../context/GameContext';
import {PlayerIndex} from '../../../domain/types';

interface ClockProps {
  initialSeconds?: number;
}

export const GameTimer: React.FC<ClockProps> = ({ initialSeconds = 600 }) => {
  const { currentPlayer, gameEnded } = useGame();
  const [whiteTime, setWhiteTime] = useState(initialSeconds);
  const [blackTime, setBlackTime] = useState(initialSeconds);

  useEffect(() => {
    if (gameEnded) return;

    const timer = setInterval(() => {
      if (currentPlayer === PlayerIndex.WHITE) {
        setWhiteTime((prev) => Math.max(0, prev - 1));
      } else {
        setBlackTime((prev) => Math.max(0, prev - 1));
      }
    }, 1000);

    return () => clearInterval(timer);
  }, [currentPlayer, gameEnded]);

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  return (
    <div className="flex items-center gap-3 font-mono text-xs tracking-wider">
      <div className={`px-2 py-0.5 rounded border transition-colors ${
        currentPlayer === PlayerIndex.WHITE && !gameEnded
          ? 'bg-piece-white-fill text-piece-black-fill border-piece-white-ring font-bold' 
          : 'bg-hud-card text-text-muted border-border-muted'
      }`}>
        W: {formatTime(whiteTime)}
      </div>
      <div className={`px-2 py-0.5 rounded border transition-colors ${
        currentPlayer === PlayerIndex.BLACK && !gameEnded
          ? 'bg-piece-black-fill text-piece-white-fill border-piece-black-ring font-bold' 
          : 'bg-hud-card text-text-muted border-border-muted'
      }`}>
        B: {formatTime(blackTime)}
      </div>
    </div>
  );
};

export default GameTimer;