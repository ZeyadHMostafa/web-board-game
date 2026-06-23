import React from 'react';
import { useGame } from '../../../context/GameContext';

export const TimelineNavigation: React.FC = () => {
  const { currentIndex, history, jumpToHistoryIndex, gameEnded } = useGame();

  // Generate a localized window of frames centered around the current index
  const getVisibleFrames = () => {
    const windowSize = 5; // How many history nodes to display simultaneously
    let start = Math.max(0, currentIndex - Math.floor(windowSize / 2));
    const end = Math.min(history.length - 1, start + windowSize - 1);

    // Adjust window offset if running into right-bound limits
    if (end - start + 1 < windowSize) {
      start = Math.max(0, end - windowSize + 1);
    }

    const frames = [];
    for (let i = start; i <= end; i++) {
      frames.push(i);
    }
    return frames;
  };

  const handleExportHistory = async () => {
    try {
      // Create a simplified text representation of the ledger sequence
      const exportString = `Simulation Ledger Export\nTotal Frames: ${history.length}\nCurrent Frame Pointer: ${currentIndex}\nTimestamp: ${new Date().toISOString()}`;
      await navigator.clipboard.writeText(exportString);
      alert('Simulation log copied to clipboard system.');
    } catch (err) {
      console.error('Failed to export history track data:', err);
    }
  };

  return (
    <div className="flex items-center gap-3 bg-hud-card/40 border border-border-muted p-1.5 rounded-xl font-mono text-xs w-full sm:w-auto">
      {/* Navigation Arrows */}
      <button
        onClick={() => jumpToHistoryIndex(currentIndex - 1)}
        disabled={currentIndex === 0}
        className="p-1 rounded-md hover:bg-surface-card text-text-muted hover:text-text-main disabled:opacity-20 transition-colors cursor-pointer"
      >
        <span className="material-icons text-base">chevron_left</span>
      </button>

      {/* Sliding Frame Strip */}
      <div className="flex items-center gap-1.5 overflow-hidden px-1">
        {getVisibleFrames().map((frameIdx) => {
          const isActive = frameIdx === currentIndex;
          return (
            <button
              key={frameIdx}
              onClick={() => jumpToHistoryIndex(frameIdx)}
              className={`min-w-7 h-7 flex items-center justify-center rounded-md text-[10px] font-bold transition-all border cursor-pointer ${
                isActive
                  ? 'bg-accent-primary border-accent-glow text-text-main shadow-sm scale-105'
                  : 'bg-surface-card/60 border-border-muted text-text-muted hover:text-text-main hover:border-text-muted/40'
              }`}
            >
              F{frameIdx}
            </button>
          );
        })}
      </div>

      <button
        onClick={() => jumpToHistoryIndex(currentIndex + 1)}
        disabled={currentIndex === history.length - 1 || gameEnded}
        className="p-1 rounded-md hover:bg-surface-card text-text-muted hover:text-text-main disabled:opacity-20 transition-colors cursor-pointer"
      >
        <span className="material-icons text-base">chevron_right</span>
      </button>

      <div className="h-4 w-px bg-border-muted mx-1" />

      {/* Utility Action Menu */}
      <button
        onClick={handleExportHistory}
        className="p-1.5 rounded-md bg-surface-card hover:bg-hud-card border border-border-muted text-text-muted hover:text-accent-glow flex items-center gap-1 text-[11px] font-bold tracking-wide transition-colors cursor-pointer"
        title="Copy complete log array configuration parameters to clipboard clipboard"
      >
        <span className="material-icons text-xs">content_copy</span>
        <span className="hidden md:inline">EXPORT</span>
      </button>
    </div>
  );
};

export default TimelineNavigation;