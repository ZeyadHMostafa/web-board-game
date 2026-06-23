import React, { useState } from 'react';
import Navigation from '../components/Navigation';
import Footer from '../components/Footer';
import GameProvider from '../features/game-module/context/GameProvider';
import LeftMenu from '../components/LeftMenu';
import MainBoard from '../components/GameBoard';
import RightMenu from '../components/RightMenu';

export const GamePage: React.FC = () => {
  const [isLeftMenuOpen, setIsLeftMenuOpen] = useState(false);

  const toggleLeftMenu = () => {
    setIsLeftMenuOpen(!isLeftMenuOpen);
  };

  return (
    <GameProvider mode="ANALYSIS">
      <div className={`
        holy-grail-grid bg-app-bg text-text-main select-none
        ${isLeftMenuOpen ? 'left-menu-active' : ''}
      `}>
        
        {/* Navigation Section */}
        <div className="[grid-area:nav] z-50">
          <Navigation 
            onToggleLeftMenu={toggleLeftMenu} 
            isLeftMenuOpen={isLeftMenuOpen} 
          />
        </div>

        {/* Left Side Menu Section */}
        <div className={`left-menu-container ${isLeftMenuOpen ? 'is-open' : ''}`}>
          <LeftMenu />
        </div>

        {/* Central Viewport Grid Core */}
        <div className="[grid-area:main] min-h-0 min-w-0 flex items-center justify-center p-4">
          <MainBoard />
        </div>

        {/* Right Side Menu Section */}
        <div className="[grid-area:right] h-full w-full min-h-0 flex flex-col overflow-hidden">
          <RightMenu />
        </div>

        {/* Footer Status Bar Section */}
        <div className="[grid-area:foot] z-30">
          <Footer />
        </div>

      </div>
    </GameProvider>
  );
};

export default GamePage;