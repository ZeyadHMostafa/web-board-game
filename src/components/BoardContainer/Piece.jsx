import { TILE_SIZE, BoardGeometry } from '../../utils/boardGeometry';

export default function Piece({ type, row, col }) {
  // Translate the 2D array coordinates into concrete absolute pixel limits
  const { x, y } = BoardGeometry.matrixToTileTopLeft(row, col);

  // Setup visual characteristics depending on the owner
  const isWhite = type === 'W';
  
  // inner circle colors mapping to your original PLAYER_COLORS theme
  const ringColor = isWhite ? 'bg-slate-800' : 'bg-neutral-300';
  const innerColor = isWhite ? 'bg-neutral-300' : 'bg-slate-700';

  return (
    <div
      className="absolute flex items-center justify-center transition-all duration-300 ease-out pointer-events-auto cursor-grab active:cursor-grabbing"
      style={{
        width: `${TILE_SIZE}px`,
        height: `${TILE_SIZE}px`,
        // Use transform3d for hardware-accelerated slide transitions
        transform: `translate3d(${x}px, ${y}px, 0px)`,
      }}
    >
      {/* Outer Circle Ring */}
      <div className={`w-12 h-12 rounded-full flex items-center justify-center shadow-md ${ringColor}`}>
        {/* Inner Circle Accent */}
        <div className={`w-10 h-10 rounded-full shadow-inner ${innerColor}`} />
      </div>
    </div>
  );
}