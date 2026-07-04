import React from 'react';

export interface GameAction {
  id: string;
  label: string;
  icon: string;
  danger?: boolean;
}

interface QuickActionsMenuProps {
  onActionSelect: (actionId: string) => void;
  disabled?: boolean;
}

const VISIBLE_ACTIONS: GameAction[] = [
  { id: 'draw', label: 'OFFER DRAW', icon: 'handshake' },
  { id: 'resign', label: 'RESIGN', icon: 'flag', danger: true },
];

const OVERFLOW_ACTIONS: GameAction[] = [
  { id: 'takeback', label: 'REQUEST TAKEBACK', icon: 'restore' },
  { id: 'abort', label: 'ABORT GAME', icon: 'gavel', danger: true },
];

export const QuickActionsMenu: React.FC<QuickActionsMenuProps> = ({
  onActionSelect,
  disabled = false
}) => {
  return (
    <div className="flex items-center gap-1 bg-hud-card border border-border-muted p-1 rounded-lg">
      {/* Primary Visible Actions Row */}
      {VISIBLE_ACTIONS.map((action) => (
        <button
          key={action.id}
          disabled={disabled}
          onClick={() => onActionSelect(action.id)}
          className={`h-7 px-2 rounded font-mono text-[10px] font-bold uppercase tracking-wider flex items-center justify-center gap-1.5 border transition-colors cursor-pointer disabled:opacity-20 disabled:cursor-not-allowed ${
            action.danger
              ? 'bg-indicator-capture/10 border-indicator-capture/30 text-indicator-capture hover:bg-indicator-capture/20 hover:border-indicator-capture/50'
              : 'bg-surface-card border-border-muted text-text-muted hover:text-text-main hover:border-text-muted/40'
          }`}
          title={action.label}
        >
          <span className="material-icons text-sm">{action.icon}</span>
        </button>
      ))}

      {/* Overflow Menu Anchor Dropdown */}
      <div className="relative group/menu inline-block">
        <button
          disabled={disabled}
          className="h-7 w-7 rounded bg-surface-card border border-border-muted text-text-muted hover:text-text-main hover:border-text-muted/40 disabled:opacity-20 disabled:cursor-not-allowed flex items-center justify-center cursor-pointer transition-colors"
          title="More Actions"
        >
          <span className="material-icons text-base">more_vert</span>
        </button>

        {/* Dropdown Overlay Layer */}
        <div className="absolute right-0 bottom-full mb-1.5 z-40 hidden group-focus-within/menu:block hover:block w-48 bg-surface-card border border-border-muted rounded-lg shadow-xl py-1 overflow-hidden">
          {OVERFLOW_ACTIONS.map((action) => (
            <button
              key={action.id}
              disabled={disabled}
              onClick={() => onActionSelect(action.id)}
              className={`w-full px-3 py-2 text-left font-mono text-[11px] font-bold tracking-tight flex items-center gap-2.5 transition-colors cursor-pointer disabled:opacity-20 disabled:cursor-not-allowed border-b border-border-muted/50 last:border-b-0 ${
                action.danger
                  ? 'text-indicator-capture hover:bg-indicator-capture/10'
                  : 'text-text-muted hover:text-text-main hover:bg-hud-card'
              }`}
            >
              <span className="material-icons text-base">{action.icon}</span>
              {action.label}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
};

export default QuickActionsMenu;