import { TILE_SIZE } from '../utils/boardGeometry';

export default function DragOverlay({ isDragging, type, mousePos }) {
  // If the user isn't actively holding a piece, don't mount anything to the DOM
  if (!isDragging || !type) return null;

  const isWhite = type === 'W';
  const ringColor = isWhite ? 'bg-slate-800' : 'bg-neutral-300';
  const innerColor = isWhite ? 'bg-neutral-300' : 'bg-slate-700';

  return (
    <div
      className="fixed top-0 left-0 pointer-events-none z-50 opacity-60 mix-blend-screen"
      style={{
        width: `${TILE_SIZE}px`,
        height: `${TILE_SIZE}px`,
        // Center the piece directly underneath the cursor crosshair offset
        transform: `translate3d(${mousePos.x - TILE_SIZE / 2}px, ${mousePos.y - TILE_SIZE / 2}px, 0px)`,
      }}
    >
      {/* Mirroring the exact aesthetic design parameters of our Piece.jsx layer */}
      <div className={`w-12 h-12 rounded-full flex items-center justify-center shadow-2xl ${ringColor}`}>
        <div className={`w-10 h-10 rounded-full ${innerColor}`} />
      </div>
    </div>
  );
}