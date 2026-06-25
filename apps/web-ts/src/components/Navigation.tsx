import React from 'react';

interface NavigationProps {
  onToggleLeftMenu: () => void;
  isLeftMenuOpen: boolean;
}

interface NavigationItem {
  id: string;
  label: string;
  icon: string;
  isActive?: boolean;
  isDisabled?: boolean;
}

export const Navigation: React.FC<NavigationProps> = ({
  onToggleLeftMenu,
  isLeftMenuOpen
}) => {
  {/* Structured Navigation Model Array */}
  const navItems: NavigationItem[] = [
    {
      id: 'matchmaking',
      label: 'Matchmaking',
      icon: 'group',
      isDisabled: true
    },
    {
      id: 'scoreboard',
      label: 'Scoreboard',
      icon: 'leaderboard',
      isDisabled: true
    },
    {
      id: 'arena',
      label: 'Game Arena',
      icon: 'videogame_asset',
      isActive: true
    }
  ];

  return (
    <nav className="w-full h-full bg-surface-card border-b border-border-muted px-4 flex flex-row items-center justify-between short-height:flex-col short-height:justify-start short-height:py-6 short-height:px-0 short-height:gap-8 short-height:border-b-0 short-height:border-r">
      {/* Brand Context & Control Actions Row */}
      <div className="flex items-center gap-3 short-height:flex-col short-height:gap-6">
        <button
          onClick={onToggleLeftMenu}
          className="xl:hidden flex items-center justify-center p-1.5 rounded-lg border border-border-muted bg-hud-card hover:bg-hud-bg text-text-main cursor-pointer shrink-0"
          aria-label="Toggle Side Menu"
        >
          <span className="material-icons text-xl">
            {isLeftMenuOpen ? 'menu_open' : 'menu'}
          </span>
        </button>

        {/* Global Operational Identity Logo Mark */}
        <span className="font-bold text-sm tracking-wider block short-height:hidden">
          ABSTRACT ORBITAL
        </span>
        <span className="hidden short-height:inline material-icons text-accent-glow text-2xl">
          language
        </span>
      </div>

      {/* Dynamic Data-Driven Navigation Loop */}
      <div className="flex items-center gap-4 text-xs font-medium short-height:flex-col short-height:gap-6 lg:gap-6 lg:text-sm">
        {navItems.map((item) => {
          if (item.isActive) {
            return (
              <span
                key={item.id}
                className="text-accent-glow border-b-2 border-accent-primary pb-1 px-1 cursor-pointer short-height:border-b-0 short-height:border-l-2 short-height:pl-2 short-height:pb-0 flex items-center gap-1"
              >
                <span className="material-icons text-xl lg:hidden short-height:inline">
                  {item.icon}
                </span>
                <span className="short-height:hidden hidden sm:inline">
                  {item.label}
                </span>
              </span>
            );
          }

          return (
            <span
              key={item.id}
              className={`flex items-center gap-1 pb-1 ${
                item.isDisabled 
                  ? 'text-text-muted cursor-not-allowed' 
                  : 'text-text-muted hover:text-text-main cursor-pointer'
              }`}
            >
              <span className="material-icons text-xl lg:hidden short-height:inline">
                {item.icon}
              </span>
              <span className="short-height:hidden hidden sm:inline">
                {item.label}
              </span>
            </span>
          );
        })}
      </div>
    </nav>
  );
};

export default Navigation;