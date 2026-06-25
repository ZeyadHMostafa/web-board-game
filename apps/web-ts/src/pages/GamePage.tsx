import React from 'react';
import GameModuleRoot from '../features/game-module/GameModuleRoot';

export const GamePage: React.FC = () => {
  return (
    <div className="w-full h-full flex flex-col landscape:flex-row min-h-0 min-w-0 overflow-hidden relative">
      <GameModuleRoot/>
    </div>
  );
};

export default GamePage;