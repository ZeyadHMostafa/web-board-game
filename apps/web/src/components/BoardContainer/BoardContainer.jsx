import { useState, useRef, useEffect } from 'react';
import { BoardGeometry } from '../../utils/boardGeometry';
import BackgroundCanvas from './BackgroundCanvas';
import HighlightCanvas from './HighlightCanvas';
import AssistCanvas from './AssistCanvas';
import ControlCanvas from './ControlCanvas';
import SelectionCanvas from './SelectionCanvas';
import PieceLayer from './PieceLayer';

export default function BoardContainer({ 
  boardState, 
  showAssist, 
  showControl,
  assistMoves, 
  lastMove,
  selectedCoords,
  validMoves,
  onMoveAttempt,
  onDragStart,
  onSquareClick
}) {
  const boardRef = useRef(null);
  const [boardSize, setBoardSize] = useState(512);
  const [activeDragStart, setActiveDragStart] = useState(null);

  useEffect(() => {
    if (!boardRef.current) return;

    const resizeObserver = new ResizeObserver((entries) => {
      for (let entry of entries) {
        const width = entry.contentRect.width;
        if (width > 0 && Math.abs(boardSize - width) > 1) {
          setBoardSize(width);
        }
      }
    });

    resizeObserver.observe(boardRef.current);
    return () => resizeObserver.disconnect();
  }, [boardSize]);

  const handleMouseDown = (event) => {
    if (!boardRef.current) return;

    const rect = boardRef.current.getBoundingClientRect();
    const clickX = event.clientX - rect.left;
    const clickY = event.clientY - rect.top;

    const cellCoords = BoardGeometry.pixelsToMatrix(clickX, clickY, boardSize);
    setActiveDragStart(cellCoords);
    
    if (onDragStart) {
      onDragStart(cellCoords, event.clientX, event.clientY);
    }
  };

  const handleMouseUp = (event) => {
    if (!activeDragStart || !boardRef.current) return;

    const rect = boardRef.current.getBoundingClientRect();
    const releaseX = event.clientX - rect.left;
    const releaseY = event.clientY - rect.top;

    const targetCoords = BoardGeometry.pixelsToMatrix(releaseX, releaseY, boardSize);

    if (activeDragStart.row !== targetCoords.row || activeDragStart.col !== targetCoords.col) {
      // It was an intentional drag action across different squares
      onMoveAttempt(activeDragStart, targetCoords);
    } else {
      // The cursor began and finished on the exact same tile, signaling a standard discrete click
      if (onSquareClick) {
        onSquareClick(targetCoords);
      }
    }

    setActiveDragStart(null);
  };

  return (
    <div
      ref={boardRef}
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseUp}
      className="relative w-full max-w-[min(100vw-2rem,100dvh-4rem)] min-w-[280px] aspect-square shadow-2xl border border-slate-700/50 rounded-md overflow-hidden bg-slate-900 mx-auto"
    >
      <BackgroundCanvas boardSize={boardSize} />
      <HighlightCanvas lastMove={lastMove} boardSize={boardSize} />
      <AssistCanvas moves={assistMoves} showAssist={showAssist} boardSize={boardSize} />
      <ControlCanvas boardState={boardState} showControl={showControl} boardSize={boardSize} />
      
      {/* Visual rendering of piece selections and potential moves */}
      <SelectionCanvas 
        selectedCoords={selectedCoords} 
        validMoves={validMoves} 
        boardState={boardState} 
        boardSize={boardSize} 
      />
      
      <PieceLayer boardState={boardState} boardSize={boardSize} />
    </div>
  );
}