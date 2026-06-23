import React from 'react';
import { useGame } from '../../../context/GameContext';
import { PlayerIndex } from '../../../domain/types';
import { AI_LEVELS, type AiLevel } from '../../../domain/configurations';

export const EngineConfigCard: React.FC = () => {
  const { whiteEngine, blackEngine } = useGame();

  const engines = [
    {
      id: PlayerIndex.WHITE,
      label: 'White Subsystem',
      state: whiteEngine,
      hotkey: 'Q',
    },
    {
      id: PlayerIndex.BLACK,
      label: 'Black Subsystem',
      state: blackEngine,
      hotkey: 'A',
    },
  ];

  const levels: { value: AiLevel; icon: string }[] = [
    { value: AI_LEVELS.TRAINEE, icon: 'school' },
    { value: AI_LEVELS.COMPETITOR, icon: 'psychology' },
    { value: AI_LEVELS.MASTER, icon: 'military_tech' },
  ];

  return (
    <div className="flex flex-col gap-4">
      {engines.map((engine) => (
        <div 
          key={engine.id} 
          className="p-3.5 rounded-xl border border-border-muted bg-hud-card/40 flex flex-col gap-3"
        >
          {/* Header Row: Toggle & Status Indicator */}
          <div className="flex items-center justify-between">
            <button
              onClick={engine.state.toggleAuto}
              className={`flex items-center gap-2 text-xs font-bold tracking-wider uppercase transition-colors text-left group cursor-pointer ${
                engine.state.isAuto ? 'text-accent-glow' : 'text-text-muted hover:text-text-main'
              }`}
            >
              <span className="material-icons text-sm">
                {engine.state.isAuto ? 'smart_toy' : 'person'}
              </span>
              <span>{engine.label}</span>
              <kbd className="px-1.5 py-0.5 text-[9px] font-mono rounded bg-surface-card border border-border-muted text-text-muted group-hover:text-text-main ml-1">
                {engine.hotkey}
              </kbd>
            </button>

            {/* Visual Operational Bulb */}
            <span className={`h-2 w-2 rounded-full transition-all duration-300 ${
              engine.state.isAuto 
                ? 'bg-indicator-legal shadow-[0_0_8px_var(--color-indicator-legal)] animate-pulse' 
                : 'bg-border-muted'
            }`} />
          </div>

          {/* Difficulty Preset Picker Segment */}
          <div className="grid grid-cols-3 gap-1.5 bg-surface-card/60 p-1 rounded-lg border border-border-muted">
            {levels.map((lvl) => {
              const isSelected = engine.state.currentLevel === lvl.value;
              return (
                <button
                  key={lvl.value}
                  onClick={() => engine.state.setAiLevel(lvl.value)}
                  className={`flex flex-col items-center justify-center py-1.5 px-1 rounded-md transition-all cursor-pointer group ${
                    isSelected
                      ? 'bg-accent-primary text-text-main shadow-md'
                      : 'text-text-muted hover:bg-hud-card hover:text-text-main'
                  }`}
                  title={`Swap node depth matrix parameters to ${lvl.value}`}
                >
                  <span className={`material-icons text-base transition-transform ${
                    isSelected ? 'scale-110' : 'opacity-60 group-hover:opacity-100'
                  }`}>
                    {lvl.icon}
                  </span>
                  <span className="text-[9px] font-mono mt-0.5 tracking-tight uppercase">
                    {lvl.value.slice(0, 4)}
                  </span>
                </button>
              );
            })}
          </div>
        </div>
      ))}
    </div>
  );
};

export default EngineConfigCard;