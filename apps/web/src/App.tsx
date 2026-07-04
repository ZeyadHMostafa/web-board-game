import React from 'react';
import GamePage from './pages/GamePage';
import MainLayout from './layouts/MainLayout';

export const App: React.FC = () => {
  return (
    <MainLayout>
      <GamePage />
    </MainLayout>
);
};

export default App;