import React from 'react';
import BaseSvgLayer from './BaseSvgLayer';

interface BoardBackgroundSvgLayerProps {
  boardSize: number;
}

export const BoardBackgroundSvgLayer: React.FC<BoardBackgroundSvgLayerProps> = ({ boardSize }) => {
  const tileSize = boardSize / 8;
  const squares: React.ReactNode[] = [];

  // Generate the 32 alternating light squares across the 8x8 matrix grid
  for (let row = 0; row < 8; row++) {
    for (let col = 0; col < 8; col++) {
      // Light tile condition: row and col sum must be even
      if ((row + col) % 2 === 0) {
        squares.push(
          <rect
            key={`tile-${row}-${col}`}
            x={col * tileSize}
            y={(row) * tileSize}
            width={tileSize}
            height={tileSize}
            fill="var(--color-tile-light)"
          />
        );
      }
    }
  }

  return (
    <BaseSvgLayer zIndex={0}>
      {/* 1. Large base backing sheet representing all dark squares */}
      <rect 
        width={boardSize} 
        height={boardSize} 
        fill="var(--color-tile-dark)" 
      />
      
      {/* 2. Layer the 32 specific light square vector paths */}
      {squares}
    </BaseSvgLayer>
  );
};

export default BoardBackgroundSvgLayer;