import RenderedPiece from './BoardContainer/RenderedPiece';

export default function DragOverlay({ isDragging, type, mousePos, boardSize }) {
  if (!isDragging || !type) return null;

  const tileSize = boardSize / 8;
  const snappedTile = Math.floor(tileSize);

  return (
    <div
      className="fixed top-0 left-0 pointer-events-none z-50 opacity-60 mix-blend-screen"
      style={{
        width: `${snappedTile}px`,
        height: `${snappedTile}px`,
        transform: `translate3d(${mousePos.x - snappedTile / 2}px, ${mousePos.y - snappedTile / 2}px, 0px)`,
      }}
    >
      <RenderedPiece type={type} tileSize={tileSize} />
    </div>
  );
}