import React, { useState } from 'react';
import {useGame} from '../features/game-module/context/GameContext';
import EngineConfigCard from '../features/game-module/features/hud/components/EngineConfigCard';
import DisplayLayersCard from '../features/game-module/features/hud/components/DisplayLayersCard';
import MoveScorer from '../features/game-module/features/hud/components/MoveScorer';

export const RightMenu: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'telemetry' | 'system'>('telemetry');
  const { currentPlayer, gameEnded } = useGame();

  return (
    <div className="w-full h-full flex flex-col min-h-0 bg-surface-card border-l border-border-muted">  
      {/* Sub-header Dynamic Tracking Tab Bar (Pinned - will not scroll away) */}
      <div className="flex bg-hud-bg border-b border-border-muted p-1 shrink-0">
        <button
          onClick={() => setActiveTab('telemetry')}
          className={`flex-1 flex items-center justify-center gap-1.5 py-2.5 text-[11px] font-bold tracking-wider uppercase rounded-md transition-colors cursor-pointer ${
            activeTab === 'telemetry'
              ? 'bg-surface-card text-accent-glow border border-border-muted'
              : 'text-text-muted hover:text-text-main'
          }`}
        >
          <span className="material-icons text-sm">analytics</span>
          Telemetry
        </button>
        <button
          onClick={() => setActiveTab('system')}
          className={`flex-1 flex items-center justify-center gap-1.5 py-2.5 text-[11px] font-bold tracking-wider uppercase rounded-md transition-colors cursor-pointer ${
            activeTab === 'system'
              ? 'bg-surface-card text-accent-glow border border-border-muted'
              : 'text-text-muted hover:text-text-main'
          }`}
        >
          <span className="material-icons text-sm">tune</span>
          Control
        </button>
      </div>

      {/* Main Panel Content Container
        - flex-1 lets it absorb all remaining vertical space.
        - overflow-y-auto handles content exceeding that vertical footprint safely.
        - min-h-0 prevents nested flex components from expanding past the viewport boundary.
      */}
      <div className="flex-1 min-h-0 overflow-y-auto p-4 space-y-4 scrollbar-thin scrollbar-thumb-border-muted">
        {activeTab === 'telemetry' ? (
          <>
            {/* Context Status Readout Card */}
            <div className="p-3 bg-hud-card/30 border border-border-muted rounded-xl flex items-center justify-between shrink-0">
              <span className="text-[10px] font-bold text-text-muted tracking-wider uppercase">Active Sub-State:</span>
              <span className={`text-xs font-mono font-black tracking-wide ${
                gameEnded 
                  ? 'text-indicator-capture' 
                  : currentPlayer === 0 ? 'text-text-main' : 'text-text-muted'
              }`}>
                {gameEnded ? 'TERMINATED' : currentPlayer === 0 ? 'WHITE_TURN' : 'BLACK_TURN'}
              </span>
            </div>

            {/* Engine Analysis Vectors */}
            <MoveScorer />
          </>
        ) : (
          <>
            {/* AI Control Configurations */}
            <div className="space-y-1">
              <h3 className="text-[10px] font-bold text-text-muted uppercase tracking-wider px-1">
                Automation Framework
              </h3>
              <EngineConfigCard />
            </div>

            {/* Visibility Settings Layer Block */}
            <div className="space-y-1 pt-2">
              <h3 className="text-[10px] font-bold text-text-muted uppercase tracking-wider px-1">
                Overlay Parameters
              </h3>
              <DisplayLayersCard />
            </div>
          </>
        )}
      </div>
    </div>
  );
};

export default RightMenu;