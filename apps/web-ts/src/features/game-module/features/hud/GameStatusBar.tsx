import React from 'react';
import { useGame } from '../../context/GameContext';
import EvalBar from './components/EvalBar';
import PlayerInfoBlock from './components/PlayerInfoBlock';
import TimelineNavigation from './components/TimeLineNavigation';
import QuickActionsMenu from './components/QuickActionsMenu';
import type {PlayerData} from '../../domain/types';

interface GameStatusBarProps {
  onDrawerToggle?: () => void;
  isDrawerExpanded?: boolean;
}

export const GameStatusBar: React.FC<GameStatusBarProps> = ({
  onDrawerToggle,
  isDrawerExpanded
}) => {
  // TODO: add these to game context
  const { 
    // whitePlayer, 
    // blackPlayer, 
    // whiteTimeLeft, 
    // blackTimeLeft, 
    currentPlayer,
    gameEnded,
    // executeGameAction
  } = useGame();

  const MOCK_WHITE_PLAYER: PlayerData = {
    id: 'usr-1',
    name: 'Grandmaster_Alex',
    rating: 2450,
    ratingChange: 12,
    isUser: true,
    avatarUrl: undefined // Left undefined to test the Material Icon fallback glyph
  };
  const whitePlayer = MOCK_WHITE_PLAYER

  const MOCK_BLACK_PLAYER: PlayerData = {
    id: 'usr-2',
    name: 'Deep_Neural_Net',
    rating: 2510,
    ratingChange: -8,
    isUser: false,
    avatarUrl: undefined
  };
  const blackPlayer = MOCK_BLACK_PLAYER
  const whiteTimeLeft = 412
  const blackTimeLeft = 285

  const handleActionSelect = (_actionId: string) => {
    // executeGameAction?.(actionId);
  };

  const handleStatusBarClick = () => {
    if (window.innerHeight > window.innerWidth) {
      onDrawerToggle?.();
    }
  };

  return (
    <div className="w-full flex flex-col bg-surface-card border-b border-border-muted landscape:border-b-0 landscape:border-t shrink-0 select-none">
      
      {/* Primary Interaction Ribbon Deck - Structured via a Vertical Stack */}
      <div 
        onClick={handleStatusBarClick}
        className="w-full flex flex-col gap-2 p-3 cursor-default portrait:cursor-pointer portrait:bg-hud-bg"
      >

        {/* Real-time Engine Advantage Ratio Strip */}
        <div className="w-full h-1.5 shrink-0 landscape:hidden">
          <EvalBar />
        </div>
        
        {/* Players */}
        <div className="flex flex-row gap-2">
          {/* Opponent Profile Card (Black Player) */}
          <div className="w-full" onClick={(e) => e.stopPropagation()}>
            <PlayerInfoBlock 
              player={blackPlayer} 
              timeLeft={blackTimeLeft} 
              isActive={currentPlayer === 1 && !gameEnded} 
            />
          </div>

          {/* User Profile Card (White Player) */}
          <div className="w-full" onClick={(e) => e.stopPropagation()}>
            <PlayerInfoBlock 
              player={whitePlayer} 
              timeLeft={whiteTimeLeft} 
              isActive={currentPlayer === 0 && !gameEnded} 
            />
          </div>

        </div>

        {/* Middle Row: Inline Action Tools & Navigation Timeline Controls */}
        <div className="w-full flex items-center justify-between gap-2 border-y border-border-muted/30 py-1.5">
          <QuickActionsMenu onActionSelect={handleActionSelect} />
          
          {/* Dynamic Portrait Drawer State Indicator Icon */}
          <div className="hidden portrait:block text-text-muted leading-none">
            <span className="material-icons text-base select-none align-middle">
              {isDrawerExpanded ? 'keyboard_arrow_down' : 'keyboard_arrow_up'}
            </span>
          </div>

          <TimelineNavigation />
        </div>

      </div>

      {/* Real-time Engine Advantage Ratio Strip */}
      <div className="w-full h-1.5 shrink-0 portrait:hidden">
        <EvalBar />
      </div>
    </div>
  );
};

export default GameStatusBar;