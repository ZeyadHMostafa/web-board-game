import React from 'react';
import BoardFrame from '../features/game-module/features/board/BoardFrame';

export const MainBoard: React.FC = () => {
  return (
    <main className="w-full h-full max-w-5xl max-h-full aspect-square flex items-center justify-center min-h-0 min-w-0">
      <BoardFrame />
    </main>
  );
};

export default MainBoard;