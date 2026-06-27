import React, { useState } from 'react';
import Navigation from '../components/Navigation'
import LeftMenu from '../components/LeftMenu';

interface LayoutProps {
  children: React.ReactNode;
}

export const MainLayout: React.FC<LayoutProps> = ({ children }) => {
  const [isLeftMenuOpen, setIsLeftMenuOpen] = useState<boolean>(false);

  const toggleLeftMenu = () => {
    setIsLeftMenuOpen((prev) => !prev);
  };

  const closeLeftMenu = () => {
    setIsLeftMenuOpen(false);
  };

  return (
    <div className="w-screen h-screen flex flex-col short-height:flex-row overflow-hidden bg-app-bg text-text-main selection:bg-selection-bg">
      {/* Primary Application Navigation Shell */}
      <div className="w-full z-50 h-14 shrink-0 short-height:w-16 short-height:h-full">
        <Navigation 
          onToggleLeftMenu={toggleLeftMenu} 
          isLeftMenuOpen={isLeftMenuOpen} 
        />
      </div>

      {/* Global Content Viewport Frame */}
      <div className="flex-1 flex flex-row min-w-0 min-h-0 relative">
        {/* Left Interactive Sidebar Utility Overlay / Dock */}
        <div 
          className={`
            fixed top-14 bottom-0 left-0 z-40 w-[260px] 
            transition-transform duration-200 ease-in-out
            short-height:top-0 short-height:left-16
            xl:static xl:transform-none xl:z-0 xl:h-full
            ${isLeftMenuOpen ? 'translate-x-0' : '-translate-x-full xl:translate-x-0'}
          `}
        >
          <LeftMenu />
        </div>

        {/* Global Structural Overlay Mask for Mobile Dimming */}
        {isLeftMenuOpen && (
          <div 
            onClick={closeLeftMenu}
            className="fixed inset-0 z-30 bg-app-bg/40 xl:hidden"
          />
        )}

        {/* Primary Page Canvas Area */}
        <main className="flex-1 min-w-0 min-h-0 relative bg-app-bg">
          {children}
        </main>
      </div>
    </div>
  );
};

export default MainLayout;