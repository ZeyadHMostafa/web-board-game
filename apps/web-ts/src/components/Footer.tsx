import React from 'react';

export const Footer: React.FC = () => {
  return (
    <footer className="w-full bg-surface-card border-t border-border-muted px-4 py-2 flex items-center justify-between font-mono text-xs text-text-muted landscape:border-l landscape:border-t-0 landscape:bg-hud-bg">
      <div className="flex items-center gap-4">
        <span className="flex items-center gap-1.5">
          <span className="w-2 h-2 rounded-full bg-indicator-legal animate-pulse" />
          SYSTEM ONLINE
        </span>
        <span className="hidden sm:inline opacity-60">|</span>
        <span>PING: 24ms</span>
      </div>
      
      <div className="hidden sm:block opacity-40 text-[10px]">
        v2.4.0-ORBITAL
      </div>
    </footer>
  );
};

export default Footer;