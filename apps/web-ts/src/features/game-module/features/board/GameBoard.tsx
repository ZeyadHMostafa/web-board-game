import React from 'react';
import BoardContainer from './BoardContainer';

export const MainBoard: React.FC = () => {
  const files = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
  const ranks = ['7', '6', '5', '4', '3', '2', '1', '0'];

  return (
    <div className="w-full h-auto max-w-full max-h-full aspect-square m-auto min-h-0 min-w-0">
      {/* grid-rows to put the 2rem track at the bottom */}
      <div className="h-full max-h-full mx-auto aspect-square bg-surface-card border border-border-muted rounded-xl shadow-2xl p-1 select-none font-mono font-bold text-sm text-text-muted overflow-hidden box-border grid grid-cols-[1rem_1fr] grid-rows-[1fr_1rem]">
        {/* Left Vertical Rank Sidebar */}
        <div className="flex flex-col justify-between py-1 bg-hud-bg/10 border-r border-border-muted/30">
          {ranks.map((rank, idx) => (
            <span
              key={`left-${rank}`}
              className={`h-full flex items-center justify-left ${idx % 2 === 0 ? 'bg-hud-card/10' : 'bg-transparent'}`}
            >
              {rank}
            </span>
          ))}
        </div>

        {/* CENTRAL PAYLOAD SLOT */}
        <div className="bg-app-bg/40 relative rounded-tr-md">
          <BoardContainer />
        </div>

        {/* Bottom-Left Corner Buffer */}
        <div className="bg-hud-bg/20 rounded-bl-lg border-t border-r border-border-muted/30" />

        {/* Bottom File Labels Footer */}
        <div className="flex justify-between items-bottom bg-hud-bg/20 border-t border-border-muted/30">
          {files.map((file) => (
            <span
              key={`bottom-${file}`}
              className="w-full text-center tracking-normal"
            >
              {file}
            </span>
          ))}
        </div>
      </div>
    </div>
  );
};

export default MainBoard;
