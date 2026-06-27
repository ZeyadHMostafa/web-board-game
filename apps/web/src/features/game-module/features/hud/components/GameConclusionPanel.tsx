import React from 'react';
import {useGameStore} from '../../../store/useGameStore';

export const GameConclusionPanel: React.FC = () => {
  const gameEnded = useGameStore((state) => state.gameEnded);
  const resetGame = useGameStore((state) => state.resetTimeline);

  if (!gameEnded) return null;

  return (
    <div className="absolute inset-0 bg-app-bg/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
      <div className="w-full max-w-xs bg-surface-card border border-indicator-legal/30 rounded-xl p-5 shadow-2xl text-center transform scale-100 transition-transform">
        <span className="material-icons text-indicator-legal text-4xl animate-pulse">
          emoji_events
        </span>
        <h3 className="text-lg font-black text-text-main uppercase tracking-tight mt-2">
          Match Concluded
        </h3>
        <p className="text-xs text-text-muted mt-1">
          All grid modifications and operations are frozen.
        </p>

        <button
          onClick={resetGame}
          className="mt-4 w-full flex items-center justify-center gap-2 py-2 px-4 rounded-lg bg-accent-primary hover:bg-blue-700 text-text-main font-semibold text-xs transition-colors cursor-pointer"
        >
          <span className="material-icons text-sm">refresh</span>
          Reset Workspace Environment
        </button>
      </div>
    </div>
  );
};

export default GameConclusionPanel;