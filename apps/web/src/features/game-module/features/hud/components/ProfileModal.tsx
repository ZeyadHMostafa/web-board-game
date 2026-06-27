import React from 'react';
import { type PlayerData } from '../../../domain/types';

interface ProfileModalProps {
  isOpen: boolean;
  onClose: () => void;
  player: PlayerData;
}

export const ProfileModal: React.FC<ProfileModalProps> = ({ isOpen, onClose, player }) => {
  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
      {/* Backdrop */}
      <div className="absolute inset-0 bg-app-bg/80 backdrop-blur-sm" onClick={onClose} />
      
      {/* Modal Container */}
      <div className="relative w-full max-w-xs bg-surface-card border border-border-muted rounded-2xl p-6 shadow-2xl overflow-hidden">
        <div className="flex flex-col items-center gap-4">
          <div className="w-20 h-20 rounded-full bg-hud-bg border border-accent-primary flex items-center justify-center overflow-hidden">
            {player.avatarUrl ? (
              <img src={player.avatarUrl} alt={player.name} className="w-full h-full object-cover" />
            ) : (
              <span className="material-icons text-5xl text-accent-glow">person</span>
            )}
          </div>
          <h2 className="text-xl font-bold text-text-main truncate w-full text-center">{player.name}</h2>
          
          <div className="w-full flex flex-col gap-2 mt-2">
            {player.isUser ? (
              <>
                <button className="w-full py-2 bg-accent-primary hover:bg-blue-700 text-white rounded-lg font-bold transition-colors cursor-pointer flex items-center justify-center gap-2">
                  <span className="material-icons text-sm">settings</span>
                  EDIT ACCOUNT
                </button>
                <button className="w-full py-2 bg-hud-card hover:bg-hud-border text-text-muted rounded-lg transition-colors cursor-pointer">
                  VIEW CAREER STATS
                </button>
              </>
            ) : (
              <>
                <button className="w-full py-2 bg-accent-primary hover:bg-blue-700 text-white rounded-lg font-bold transition-colors cursor-pointer flex items-center justify-center gap-2">
                  <span className="material-icons text-sm">person_add</span>
                  ADD FRIEND
                </button>
                <button className="w-full py-2 bg-hud-card hover:bg-hud-border text-indicator-capture rounded-lg transition-colors cursor-pointer flex items-center justify-center gap-2">
                  <span className="material-icons text-sm">report</span>
                  REPORT USER
                </button>
              </>
            )}
          </div>
        </div>

        <button 
          onClick={onClose}
          className="absolute top-4 right-4 text-text-muted hover:text-text-main transition-colors cursor-pointer"
        >
          <span className="material-icons">close</span>
        </button>
      </div>
    </div>
  );
};

export default ProfileModal;