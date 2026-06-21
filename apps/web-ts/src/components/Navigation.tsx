import React from 'react';

export const Navigation: React.FC = () => {
  return (
    <nav className="w-full bg-surface-card border-b border-border-muted px-6 py-4 flex items-center justify-between z-50">
      <div className="flex items-center gap-2">
        <span className="material-icons text-accent-primary">
          sports_esports
        </span>
        <span className="font-bold tracking-wider text-text-main">
          WASM PLATFORM
        </span>
      </div>
      <div className="flex items-center gap-6 text-sm font-medium">
        <span className="text-text-muted cursor-not-allowed">Matchmaking</span>
        <span className="text-text-muted cursor-not-allowed">Scoreboard</span>
        <span className="text-accent-glow border-b-2 border-accent-primary pb-1 px-1 cursor-pointer">
          Game Arena
        </span>
      </div>
    </nav>
  );
};

export default Navigation;
