import React from 'react';
import BoardContainer from '../features/game-module/features/board/BoardContainer';

export const MainBoard: React.FC = () => {
  const files = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
  const ranks = ['7', '6', '5', '4', '3', '2', '1', '0'];

  return (
    <div className="w-full h-auto max-w-full max-h-full aspect-square m-auto min-h-0 min-w-0">
      
      <div className="h-full max-h-full mx-auto aspect-square bg-surface-card border border-border-muted rounded-xl shadow-2xl p-1 select-none font-mono font-bold text-sm text-text-muted overflow-hidden box-border grid grid-cols-[2rem_1fr] grid-rows-[2rem_1fr]">
        {/* Top-Left Corner Buffer */}
        <div className="bg-hud-bg/20 rounded-tl-lg border-b border-r border-border-muted/30" />

        {/* Top File Labels Header */}
        <div className="flex justify-between items-center px-1 bg-hud-bg/20 border-b border-border-muted/30">
          {files.map((file) => (
            <span key={`top-${file}`} className="w-full text-center tracking-normal">
              {file}
            </span>
          ))}
        </div>

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

        {/* CENTRAL PAYLOAD SLOT */}
        <div className="bg-app-bg/40 p-2 relative">
          <BoardContainer />
        </div>

      </div>
    </div>
  );
};

export default MainBoard;