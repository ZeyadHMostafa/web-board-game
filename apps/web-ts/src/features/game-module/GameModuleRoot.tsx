import React, {useEffect, useState, useRef} from 'react';

interface GameModuleRootProps {
  initialMode?: 'STRICT' | 'CASUAL' | 'ANALYSIS';
}

export const GameModuleRoot: React.FC<GameModuleRootProps> = ({
  initialMode = 'ANALYSIS'
}) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const [boardSize, setBoardSize] = useState<number>(512);

  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;

    const observer = new ResizeObserver((entries) => {
      for (const entry of entries) {
        const {width, height} = entry.contentRect;
        const minDimension = Math.min(width, height, window.innerHeight * 0.7);
        // Grid rule: Snap size values to clean multiples of 8
        const snappedSize = Math.max(256, Math.floor(minDimension / 8) * 8);
        setBoardSize(snappedSize);
      }
    });

    observer.observe(container);
    return () => observer.disconnect();
  }, []);

  return (
    <div className="w-full min-h-screen flex flex-col items-center justify-center bg-app-bg p-6 text-text-main">
      <header className="mb-4 text-center">
        <h1 className="text-2xl font-bold tracking-wider">GAME ENGINE CORE</h1>
        <p className="text-xs text-text-muted mt-1">
          Mode: {initialMode} | Locked Resolution: {boardSize}px
        </p>
      </header>

      <div
        ref={containerRef}
        className="w-full flex-1 max-w-5xl flex items-center justify-center min-h-0"
      >
        {/* Structural Gateway Target View Placeholder */}
        <div
          className="bg-surface-card border border-border-muted rounded-xl flex items-center justify-center shadow-2xl transition-all duration-200"
          style={{width: boardSize, height: boardSize}}
        >
          <span className="text-sm text-text-muted font-mono">
            [Module Target Frame Ready]
          </span>
        </div>
      </div>
    </div>
  );
};

export default GameModuleRoot;
