import React from 'react';
import BoardFrame from './features/board/BoardFrame';

export const GameModuleRoot: React.FC<{ isLeftMenuOpen: boolean }> = ({ isLeftMenuOpen }) => {
  return (
    <>
      {/* Left Menu Side Bar */}
      <aside className={`
        [grid-area:left] bg-surface-card border-r border-border-muted flex flex-col h-full overflow-hidden transition-all duration-300
        ${isLeftMenuOpen ? 'w-full' : 'w-0 border-r-0 pointer-events-none'} 
        landscape:hidden lg:flex
      `}>
        {isLeftMenuOpen && (
          <div className="p-4 min-w-[250px]">
            <h2 className="font-bold text-sm text-text-main uppercase">Engine Utilities</h2>
            <p className="text-xs text-text-muted mt-2">Analysis Mode Active.</p>
          </div>
        )}
      </aside>

      {/* Main Game Frame */}
      <div className="[grid-area:main] flex items-center justify-center p-4 min-h-0 min-w-0">
        <main className="w-full max-w-5xl aspect-square max-h-[60vh] landscape:max-h-[85vh] lg:max-h-[calc(100vh-10rem)]">
          <BoardFrame />
        </main>
      </div>

      {/* Right Menu / Telemetry Log */}
      <aside className="[grid-area:right] bg-surface-card border-t landscape:border-t-0 landscape:border-l lg:border-t-0 lg:border-l border-border-muted flex flex-col min-h-0">
        <div className="p-4 border-b border-border-muted bg-hud-bg">
          <h2 className="font-bold text-sm text-text-main uppercase">Telemetry Log</h2>
        </div>
        <div className="flex-1 overflow-y-auto p-4 space-y-2 font-mono text-xs text-text-muted h-[200px] landscape:h-auto">
          {/* Logs map down here cleanly */}
        </div>
      </aside>

      {/* Unified Status Footer */}
      <footer className="[grid-area:footer] bg-surface-card border-t border-border-muted px-4 py-2 flex items-center justify-between text-xs font-mono text-text-muted">
        <div className="flex items-center gap-2">
          <span className="w-2 h-2 rounded-full bg-indicator-legal animate-pulse" />
          SYSTEM LOGGED IN
        </div>
        <span>PING: 24ms</span>
      </footer>
    </>
  );
};

export default GameModuleRoot;
