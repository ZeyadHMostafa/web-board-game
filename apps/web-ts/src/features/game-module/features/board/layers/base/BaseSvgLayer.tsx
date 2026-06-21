import React from 'react';

interface BaseSvgLayerProps {
  boardSize: number;
  zIndex: number;
  children: React.ReactNode;
}

export const BaseSvgLayer: React.FC<BaseSvgLayerProps> = ({
  boardSize,
  zIndex,
  children
}) => {
  return (
    <svg
      width={boardSize}
      height={boardSize}
      viewBox={`0 0 ${boardSize} ${boardSize}`}
      style={{ zIndex }}
      className="absolute top-0 left-0 pointer-events-none select-none overflow-visible"
    >
      {children}
    </svg>
  );
};

export default BaseSvgLayer;