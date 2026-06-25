import React from 'react';
import { useGame } from '../../../context/GameContext';

export const TimelineNavigation: React.FC = () => {
  const { currentIndex, history, jumpToHistoryIndex, gameEnded } = useGame();

  const isAtStart = currentIndex === 0;
  const isAtLive = currentIndex === history.length - 1;

  return (
    <div className="flex items-center gap-1 bg-hud-card border border-border-muted p-1 rounded-lg">
      {/* Step Back One Move */}
      <button
        onClick={() => jumpToHistoryIndex(currentIndex - 1)}
        disabled={isAtStart}
        className="rounded text-text-muted hover:text-text-main hover:bg-surface-card disabled:opacity-20 transition-colors cursor-pointer flex items-center justify-center"
        title="Step Backward"
      >
        <span className="material-icons text-lg">chevron_left</span>
      </button>

      {/* Step Forward One Move */}
      <button
        onClick={() => jumpToHistoryIndex(currentIndex + 1)}
        disabled={isAtLive || gameEnded}
        className="rounded text-text-muted hover:text-text-main hover:bg-surface-card disabled:opacity-20 transition-colors cursor-pointer flex items-center justify-center"
        title="Step Forward"
      >
        <span className="material-icons text-lg">chevron_right</span>
      </button>

      {/* Jump Directly to Present/Live State */}
      <button
        onClick={() => jumpToHistoryIndex(history.length - 1)}
        disabled={isAtLive}
        className={`p-1 rounded text-xs font-bold transition-colors cursor-pointer flex items-center justify-center h-7 px-2 border ${
          isAtLive
            ? 'bg-accent-primary/20 border-accent-primary text-accent-glow'
            : 'bg-surface-card border-border-muted text-text-muted hover:text-text-main hover:border-text-muted/40'
        }`}
        title="Jump to Live"
      >
        <span className="material-icons text-sm mr-1">skip_next</span>
      </button>
    </div>
  );
};

export default TimelineNavigation;