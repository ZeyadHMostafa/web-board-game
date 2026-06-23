import React from 'react';

export const LeftMenu: React.FC = () => {
  return (
    /* We use h-full and w-full, but allow the visibility to be handled cleanly */
    <div className="w-full h-full flex flex-col bg-surface-card border-r border-border-muted overflow-hidden select-none">
      {/* Header Panel */}
      <div className="p-4 border-b border-border-muted bg-hud-bg flex items-center gap-2 shrink-0">
        <span className="material-icons text-text-muted text-sm">tune</span>
        <h2 className="font-bold text-xs text-text-main uppercase tracking-wider whitespace-nowrap">
          Engine Utilities
        </h2>
      </div>

      {/* Control Content Modules - Locked to a strict internal layout width */}
      <div className="flex-1 overflow-y-auto p-4 space-y-4 font-mono text-xs text-text-muted w-[260px] shrink-0">
        <div className="p-3 rounded-lg border border-border-muted bg-hud-card">
          <p className="text-text-main font-semibold mb-1">Analysis Mode</p>
          <p>Real-time calculation pathing active.</p>
        </div>
        
        <div className="p-3 rounded-lg border border-border-muted bg-hud-card">
          <p className="text-text-main font-semibold mb-1">Evaluation Metric</p>
          <p>Engine parameters uniform.</p>
        </div>
      </div>
    </div>
  );
};

export default LeftMenu;