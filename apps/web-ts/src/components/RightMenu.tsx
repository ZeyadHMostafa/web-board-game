import React from 'react';

export const RightMenu: React.FC = () => {
  // Mock move data array for cleaner rendering layout
  const moves = [
    { id: '01', white: 'e2-e4', black: 'd7-d5' },
    { id: '02', white: 'exd5', black: 'Qxd5' },
    { id: '03', white: 'Nc3', black: 'Qa5' },
    { id: '04', white: 'd2-d4', black: 'Nf6' },
  ];

  return (
    <div className="w-full h-full flex flex-col bg-surface-card">
      {/* Telemetry Header */}
      <div className="p-4 border-b border-border-muted bg-hud-bg flex items-center gap-2 shrink-0">
        <span className="material-icons text-text-muted text-sm">history</span>
        <h2 className="font-bold text-xs text-text-main uppercase tracking-wider">
          Telemetry Log
        </h2>
      </div>

      {/* Scrollable Move Log Body */}
      <div className="flex-1 overflow-y-auto p-4 space-y-2 font-mono text-xs text-text-muted">
        {moves.map((move) => (
          <div 
            key={move.id} 
            className="flex justify-between p-2 rounded bg-hud-card/50 border border-border-muted"
          >
            <span>{move.id}. {move.white}</span>
            <span className="text-text-main">{move.black}</span>
          </div>
        ))}
      </div>
    </div>
  );
};

export default RightMenu;