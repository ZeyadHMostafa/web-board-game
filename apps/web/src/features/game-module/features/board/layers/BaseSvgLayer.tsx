import React from 'react';

interface BaseSvgLayerProps {
  zIndex: number;
  children: React.ReactNode;
}

export const BaseSvgLayer: React.FC<BaseSvgLayerProps> = ({ zIndex, children }) => {
  return (
    <svg
      viewBox="0 0 800 800"
      style={{ zIndex }}
      className="absolute top-0 left-0 w-full h-full pointer-events-none select-none overflow-visible"
    >
      {children}
    </svg>
  );
};

export default BaseSvgLayer;