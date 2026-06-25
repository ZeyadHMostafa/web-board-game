import React, { useState } from 'react';
import { type PlayerData } from '../../../domain/types';
import ProfileModal from './ProfileModal';

interface PlayerInfoBlockProps {
  player: PlayerData;
  timeLeft: number;
  isActive: boolean;
}

export const PlayerInfoBlock: React.FC<PlayerInfoBlockProps> = ({
  player,
  timeLeft,
  isActive
}) => {
  const [isModalOpen, setIsModalOpen] = useState(false);

  /**
   * Converts total seconds into a standardized MM:SS format.
   */
  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  const isGain = player.ratingChange >= 0;

  return (
    <div className={`flex flex-col gap-2 p-2 rounded-lg border transition-all duration-300 w-full min-w-0 ${
      isActive 
        ? 'bg-hud-card border-accent-primary/40' 
        : 'bg-transparent border-transparent opacity-80'
    }`}>
      
      {/* Row 1: Player Name*/}
      <div className="w-full min-w-0">
        <span className="text-text-main font-bold truncate text-xs uppercase tracking-tight block">
          {player.name}
        </span>
      </div>

      {/* Row 2 & 3: Profile (Left) and Score/Time Metrics (Right) */}
      <div className="flex items-center gap-2 w-full">
        
        {/* Profile Avatar Trigger*/}
        <div className="relative w-12 h-12 shrink-0">
          <button 
            onClick={() => setIsModalOpen(true)}
            className="w-full h-full rounded-md bg-surface-card border border-border-muted flex items-center justify-center text-text-muted hover:text-accent-glow hover:border-accent-glow transition-colors cursor-pointer overflow-hidden"
          >
            {player.avatarUrl ? (
              <img src={player.avatarUrl} alt={player.name} className="w-full h-full object-cover" />
            ) : (
              <span className="material-icons text-xl">account_circle</span>
            )}
          </button>
        </div>

        {/* Metrics Column: Score*/}
        <div className="flex flex-col items-start justify-center font-mono text-xs leading-none">
          
          {/* Score & Trend Sub-row */}
          <div className="flex items-center">
            <span className="text-text-muted font-semibold">{player.rating}</span>
            <div className={`flex items-center ${isGain ? 'text-indicator-legal' : 'text-indicator-capture'}`}>
              <span className="material-icons text-xs leading-none">
                {isGain ? 'arrow_drop_up' : 'arrow_drop_down'}
              </span>
              <span className="font-bold text-2xs">{Math.abs(player.ratingChange)}</span>
            </div>
          </div>

          {/* Time Remaining Sub-row */}
          <div className={`px-1.5 py-0.5 rounded text-2xs font-bold tracking-tighter border shadow-sm select-none transition-colors ${
            isActive
              ? 'bg-piece-white-fill text-piece-black-fill border-piece-white-ring'
              : 'bg-surface-card text-text-muted border-border-muted'
          }`}>
            {formatTime(timeLeft)}
          </div>
        </div>

      </div>

      {/* Overlay Component Context Portal Anchor */}
      <ProfileModal 
        isOpen={isModalOpen} 
        onClose={() => setIsModalOpen(false)} 
        player={player}
      />
    </div>
  );
};

export default PlayerInfoBlock;