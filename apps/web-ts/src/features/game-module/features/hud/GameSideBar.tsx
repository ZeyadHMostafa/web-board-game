import React, { useState } from 'react';
import EngineConfigCard from './components/EngineConfigCard';
import DisplayLayersCard from './components/DisplayLayersCard';
import MoveScorer from './components/MoveScorer';
import {useGameStore} from '../../store/useGameStore';

type ActiveTab = 'analysis' | 'configuration';

interface TabItem {
  id: ActiveTab;
  icon: string;
  accessibilityLabel: string;
}

export const GameSideBar: React.FC = () => {
  const [activeTab, setActiveTab] = useState<ActiveTab>('analysis');
  const currentPlayer = useGameStore((state) => state.currentPlayer);
  const gameEnded = useGameStore((state) => state.gameEnded);

  {/* Icon-Only Navigation Control Schema */}
  const tabSchema: TabItem[] = [
    { id: 'analysis', icon: 'analytics', accessibilityLabel: 'Analysis Workspace' },
    { id: 'configuration', icon: 'tune', accessibilityLabel: 'System Parameters' }
  ];

  return (
    <div className="w-full h-full flex flex-col min-h-0 bg-surface-card overflow-hidden">  
      {/* Primary Scrollable Content Viewport */}
      <div className="flex-1 min-h-0 overflow-y-auto p-4 space-y-4">
        {activeTab === 'analysis' ? (
          <>
            {/* System Status State Panel */}
            <div className="p-3 bg-hud-card/30 border border-border-muted rounded-xl flex items-center justify-between shrink-0">
              <span className="text-[10px] font-bold text-text-muted tracking-wider uppercase">
                Active Sub-State:
              </span>
              <span className={`text-xs font-mono font-black tracking-wide ${
                gameEnded 
                  ? 'text-indicator-capture' 
                  : currentPlayer === 0 ? 'text-text-main' : 'text-text-muted'
              }`}>
                {gameEnded ? 'TERMINATED' : currentPlayer === 0 ? 'WHITE_TURN' : 'BLACK_TURN'}
              </span>
            </div>

            {/* Core Calculations Interface */}
            <MoveScorer />
          </>
        ) : (
          <>
            {/* Configuration Interface Sub-Blocks */}
            <div className="space-y-1">
              <h3 className="text-[10px] font-bold text-text-muted uppercase tracking-wider px-1">
                Automation Framework
              </h3>
              <EngineConfigCard />
            </div>

            <div className="space-y-1 pt-2">
              <h3 className="text-[10px] font-bold text-text-muted uppercase tracking-wider px-1">
                Overlay Parameters
              </h3>
              <DisplayLayersCard />
            </div>
          </>
        )}
      </div>

      {/* Minimalist Icon-Based Tab Bar */}
      <div className="flex bg-hud-bg border-t border-border-muted p-1 shrink-0 h-12">
        {tabSchema.map((tab) => {
          const isSelected = activeTab === tab.id;
          return (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              aria-label={tab.accessibilityLabel}
              className={`flex-1 flex items-center justify-center rounded-md transition-colors cursor-pointer ${
                isSelected
                  ? 'bg-surface-card text-accent-glow border border-border-muted'
                  : 'text-text-muted hover:text-text-main'
              }`}
            >
              <span className="material-icons text-xl">{tab.icon}</span>
            </button>
          );
        })}
      </div>
    </div>
  );
};

export default GameSideBar;