import Piece from './Piece';

export default function PieceLayer({ boardState, boardSize }) {
  return (
    <div 
      className="absolute top-0 left-0 pointer-events-none"
      style={{ width: `${boardSize}px`, height: `${boardSize}px` }}
    >
      {boardState.map((rowArray, rowIndex) =>
        rowArray.map((cell, colIndex) => {
          if (!cell) return null;

          return (
            <Piece
              key={`${rowIndex}-${colIndex}`}
              type={cell}
              row={rowIndex}
              col={colIndex}
              boardSize={boardSize}
            />
          );
        })
      )}
    </div>
  );
}