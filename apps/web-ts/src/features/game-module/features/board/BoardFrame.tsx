import React from 'react';
import BoardContainer from './BoardContainer';

export const BoardFrame: React.FC = () => {
  const files = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
  const ranks = ['7', '6', '5', '4', '3', '2', '1', '0'];

  return (
    <div className="h-full max-h-full aspect-square p-1 bg-surface-card border border-border-muted rounded-xl shadow-2xl mx-auto grid grid-cols-[2rem_1fr_2rem] grid-rows-[2rem_1fr_2rem] items-stretch justify-items-stretch select-none font-mono font-bold text-sm text-text-muted box-border overflow-hidden">
        
      {/* Top Left Corner Buffer */}
      <div className="bg-hud-bg/20 rounded-tl-lg border-b border-r border-border-muted/30" />

      {/* Top File Labels Header */}
      <div className="flex justify-between items-center px-1 bg-hud-bg/20 border-b border-border-muted/30">
        {files.map((file) => (
          <span key={`top-${file}`} className="w-full text-center tracking-normal">
            {file}
          </span>
        ))}
      </div>

      {/* Top Right Corner Buffer */}
      <div className="bg-hud-bg/20 rounded-tr-lg border-b border-l border-border-muted/30" />

      {/* Left Vertical Rank Sidebar */}
      <div className="flex flex-col justify-between py-1 bg-hud-bg/10 border-r border-border-muted/30">
        {ranks.map((rank, idx) => (
          <span 
            key={`left-${rank}`} 
            className={`h-full flex items-center justify-center ${idx % 2 === 0 ? 'bg-hud-card/10' : 'bg-transparent'}`}
          >
            {rank}
          </span>
        ))}
      </div>

      {/* Central Interactive Matrix Core Payload */}
      <div className="p-2 bg-app-bg/40 flex items-center justify-center">
        <BoardContainer />
      </div>

      {/* Right Vertical Rank Sidebar */}
      <div className="flex flex-col justify-between py-1 bg-hud-bg/10 border-l border-border-muted/30">
        {ranks.map((rank, idx) => (
          <span 
            key={`right-${rank}`} 
            className={`h-full flex items-center justify-center ${idx % 2 === 0 ? 'bg-hud-card/10' : 'bg-transparent'}`}
          >
            {rank}
          </span>
        ))}
      </div>

      {/* Bottom Left Corner Buffer */}
      <div className="bg-hud-bg/20 rounded-bl-lg border-t border-r border-border-muted/30" />

      {/* Bottom File Labels Footer */}
      <div className="flex justify-between items-center px-1 bg-hud-bg/20 border-t border-border-muted/30">
        {files.map((file) => (
          <span key={`bottom-${file}`} className="w-full text-center tracking-normal">
            {file}
          </span>
        ))}
      </div>

      {/* Bottom Right Corner Buffer */}
      <div className="bg-hud-bg/20 rounded-br-lg border-t border-l border-border-muted/30" />

    </div>
  );
};

export default BoardFrame;