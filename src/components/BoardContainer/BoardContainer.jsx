import { useState, useRef } from 'react';
import { BOARD_SIZE, BoardGeometry } from '../../utils/boardGeometry';
import BackgroundCanvas from './BackgroundCanvas';
import HighlightCanvas from './HighlightCanvas';
import AssistCanvas from './AssistCanvas';
import ControlCanvas from './ControlCanvas';
import PieceLayer from './PieceLayer';

export default function BoardContainer({ 
  boardState, 
  showAssist, 
  showControl,
  assistMoves, 
  lastMove, // Added Prop
  onMoveAttempt 
}) {
  const boardRef = useRef(null);
  
  // Track the active piece coordination state locally during drag events
  const [activeDragStart, setActiveDragStart] = useState(null);

  /**
   * Captures the initial grid index when the user presses down on a tile
   */
  const handleMouseDown = (event) => {
    if (!boardRef.current) return;

    // Determine the click position relative to the board's top-left origin
    const rect = boardRef.current.getBoundingClientRect();
    const clickX = event.clientX - rect.left;
    const clickY = event.clientY - rect.top;

    // Convert pixel coordinates to standard grid row and col indices
    const cellCoords = BoardGeometry.pixelsToMatrix(clickX, clickY);
    setActiveDragStart(cellCoords);
  };

  /**
   * Evaluates the destination grid index when the user releases the mouse
   */
  const handleMouseUp = (event) => {
    if (!activeDragStart || !boardRef.current) return;

    const rect = boardRef.current.getBoundingClientRect();
    const releaseX = event.clientX - rect.left;
    const releaseY = event.clientY - rect.top;

    const targetCoords = BoardGeometry.pixelsToMatrix(releaseX, releaseY);

    // If the piece was dropped on a different tile, attempt the move
    if (activeDragStart.row !== targetCoords.row || activeDragStart.col !== targetCoords.col) {
      onMoveAttempt(activeDragStart, targetCoords);
    }

    // Reset the drag tracker state
    setActiveDragStart(null);
  };

  return (
    <div
      ref={boardRef}
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseUp}
      className="relative shadow-2xl border border-slate-700/50 rounded-md overflow-hidden bg-slate-900"
      style={{
        width: `${BOARD_SIZE}px`,
        height: `${BOARD_SIZE}px`,
      }}
    >
      {/* Layer 1: The Static Grid Backdrop */}
      <BackgroundCanvas />

      {/* Layer 1.5: Visual highlights tracking the absolute coordinates of the most recent move */}
      <HighlightCanvas lastMove={lastMove} />

      {/* Layer 2: The Vector Vector HUD Overlay Path Engine */}
      <AssistCanvas moves={assistMoves} showAssist={showAssist} />

      {/* Layer 3: The Text-Based Control Map Overlay */}
      <ControlCanvas boardState={boardState} showControl={showControl} />

      {/* Layer 4: The DOM-managed Piece Objects */}
      <PieceLayer boardState={boardState} />
    </div>
  );
}