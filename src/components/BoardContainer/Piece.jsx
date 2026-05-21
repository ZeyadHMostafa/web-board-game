import { BoardGeometry } from '../../utils/boardGeometry';
import RenderedPiece from './RenderedPiece';

export default function Piece({ type, row, col, boardSize }) {
  const { x, y } = BoardGeometry.matrixToTileTopLeft(row, col, boardSize);
  const tileSize = boardSize / 8;
  const snappedTile = Math.floor(tileSize);

  return (
    <div
      className="absolute pointer-events-auto cursor-grab active:cursor-grabbing"
      style={{
        width: `${snappedTile}px`,
        height: `${snappedTile}px`,
        transform: `translate3d(${Math.floor(x)}px, ${Math.floor(y)}px, 0px)`,
      }}
    >
      <RenderedPiece type={type} tileSize={tileSize} />
    </div>
  );
}