import React from 'react';

interface SvgPieceProps {
  type: 'W' | 'B';
}

export const SvgPiece: React.FC<SvgPieceProps> = ({ type }) => {
  const isWhite = type === 'W';
  
  const outerFill = isWhite ? 'var(--color-piece-white-ring)' : 'var(--color-piece-black-ring)';
  const innerFill = isWhite ? 'var(--color-piece-white-fill)' : 'var(--color-piece-black-fill)';

  return (
    <g className="drop-shadow-md select-none pointer-events-none">
      <circle 
        cx={50} 
        cy={50} 
        r={37.5} 
        fill={outerFill} 
      />
      <circle 
        cx={50} 
        cy={50} 
        r={31.25} 
        fill={innerFill}
        className="drop-shadow-inner"
      />
    </g>
  );
};

export default SvgPiece;