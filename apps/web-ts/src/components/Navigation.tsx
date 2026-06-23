import React from 'react';

interface NavigationProps {
  onToggleLeftMenu: () => void;
  isLeftMenuOpen: boolean;
}

export const Navigation: React.FC<NavigationProps> = ({
  onToggleLeftMenu,
  isLeftMenuOpen
}) => {
  return (
    /* The navbar layout is simple: fill the space given by the grid.
       In desktop it will naturally stretch wide; in mobile landscape it turns into a sidebar column. */
    <nav className="w-full h-full bg-surface-card border-b border-border-muted p-4 flex flex-row items-center justify-between landscape:max-md:flex-col landscape:max-md:justify-start landscape:max-md:gap-8 landscape:max-md:border-b-0 landscape:max-md:border-r">
      {/* Brand / Toggle Section */}
      <div className="flex items-center gap-3 landscape:max-md:flex-col landscape:max-md:gap-6">
        <button
          onClick={onToggleLeftMenu}
          className="xl:hidden flex items-center justify-center p-1.5 rounded-lg border border-border-muted bg-hud-card hover:bg-hud-bg text-text-main cursor-pointer shrink-0"
          aria-label="Toggle Side Menu"
        >
          <span className="material-icons text-xl">
            {isLeftMenuOpen ? 'menu_open' : 'menu'}
          </span>
        </button>

        {/* Simple text hide token using standard visibility fallback */}
        <span className="font-bold text-sm tracking-wider block landscape:max-md:hidden">
          ABSTRACT ORBITAL
        </span>
        <span className="hidden landscape:max-md:inline material-icons text-accent-glow text-2xl">
          language
        </span>
      </div>

      {/* Navigation Links */}
      <div className="flex items-center gap-4 text-xs font-medium max-md:landscape:flex-col max-md:landscape:gap-6 lg:gap-6 lg:text-sm">
        {/* Link 1: Matchmaking */}
        <span className="text-text-muted cursor-not-allowed flex items-center gap-1 pb-1">
          {/* Icon: Always visible on mobile landscape, hidden on standard desktop layout if preferred, or kept for consistency */}
          <span className="material-icons text-xl lg:hidden max-md:landscape:inline">
            group
          </span>
          {/* Text: Hidden on mobile landscape view, visible everywhere else */}
          <span className="max-md:landscape:hidden hidden sm:inline">
            Matchmaking
          </span>
        </span>

        {/* Link 2: Scoreboard */}
        <span className="text-text-muted cursor-not-allowed flex items-center gap-1 pb-1">
          <span className="material-icons text-xl lg:hidden max-md:landscape:inline">
            leaderboard
          </span>
          <span className="max-md:landscape:hidden hidden sm:inline">
            Scoreboard
          </span>
        </span>

        {/* Link 3: Game Arena (Active) */}
        <span className="text-accent-glow border-b-2 border-accent-primary pb-1 px-1 cursor-pointer landscape:max-md:border-b-0 landscape:max-md:border-l-2 landscape:max-md:pl-2 landscape:max-md:pb-0 flex items-center gap-1">
          <span className="material-icons text-xl lg:hidden max-md:landscape:inline">
            videogame_asset
          </span>
          <span className="max-md:landscape:hidden hidden sm:inline">
            Game Arena
          </span>
        </span>
      </div>
    </nav>
  );
};

export default Navigation;
