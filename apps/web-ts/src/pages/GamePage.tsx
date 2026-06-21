import React from 'react';
import Navigation from '../components/Navigation';
import GameModuleRoot from '../features/game-module/GameModuleRoot';

export const GamePage: React.FC = () => {
  return (
    <div className="w-full min-h-screen bg-app-bg flex flex-col overflow-hidden">
      <Navigation />
      <main className="flex-1 w-full flex flex-col items-center justify-center min-h-0">
        <GameModuleRoot initialMode="ANALYSIS" />
      </main>
    </div>
  );
};

export default GamePage;
