import React from 'react';

interface BaseHtmlGridLayerProps {
  boardSize: number;
  zIndex: number;
  children: React.ReactNode;
}

export const BaseHtmlGridLayer: React.FC<BaseHtmlGridLayerProps> = ({
  boardSize,
  zIndex,
  children
}) => {
  return (
    <div
      style={{ 
        width: `${boardSize}px`, 
        height: `${boardSize}px`,
        zIndex 
      }}
      className="absolute top-0 left-0 pointer-events-none select-none"
    >
      {children}
    </div>
  );
};

export default BaseHtmlGridLayer;