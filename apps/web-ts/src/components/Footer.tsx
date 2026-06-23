import React from 'react';
import EvalBar from '../features/game-module/features/hud/components/EvalBar';
import GameTimer from '../features/game-module/features/hud/components/GameTimer';
import TimelineNavigation from '../features/game-module/features/hud/components/TimeLineNavigation';

export const Footer: React.FC = () => {
  return (
    <div className="w-full h-full flex flex-col bg-surface-card border-t border-border-muted shrink-0 landscape:max-landscape-max:border-t-0 landscape:max-landscape-max:border-l landscape:max-landscape-max:bg-hud-bg overflow-hidden">
      {/* Real-time Engine Advantage Ratio Strip - Stays horizontal on top of the footer panel */}
      <EvalBar />

      {/* Primary Interaction Ribbon Deck */}
      <footer className="w-full h-full px-4 py-2 flex flex-col sm:flex-row landscape:max-landscape-max:flex-col items-center justify-between gap-3 font-mono text-xs">
        {/* Left Side Section: Timing Telemetry loops */}
        <div className="flex items-center gap-4 shrink-0">
          <GameTimer />
        </div>

        {/* Right Side Section: Media Timeline Playback Tape */}
        <div className="w-full sm:w-auto flex justify-end shrink-0">
          <TimelineNavigation />
        </div>

      </footer>
    </div>
  );
};

export default Footer;