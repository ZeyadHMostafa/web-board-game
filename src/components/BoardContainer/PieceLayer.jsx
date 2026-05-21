import Piece from './Piece';
import { BOARD_SIZE } from '../../utils/boardGeometry';

export default function PieceLayer({ boardState }) {
  return (
    <div 
      className="absolute top-0 left-0 pointer-events-none"
      style={{ width: `${BOARD_SIZE}px`, height: `${BOARD_SIZE}px` }}
    >
      {boardState.map((rowArray, rowIndex) =>
        rowArray.map((cell, colIndex) => {
          // If the tile is null, don't mount any component here
          if (!cell) return null;

          return (
            <Piece
              key={`${rowIndex}-${colIndex}`}
              type={cell}
              row={rowIndex}
              col={colIndex}
            />
          );
        })
      )}
    </div>
  );
}