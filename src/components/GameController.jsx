import { useState, useCallback, useMemo } from 'react';
import { useGameState } from '../hooks/useGameState';
import { useHotkeys } from '../hooks/useHotkeys';
import { BoardMatrix } from '../utils/boardMatrix';
import { BoardGeometry } from '../utils/boardGeometry';
import BoardContainer from './BoardContainer/BoardContainer';
import HUD from './HUD/HUD';
import DragOverlay from './DragOverlay';

export default function GameController() {
  // 1. Mount our unified core simulation engine hook with history properties
  const {
    board,
    currentPlayer,
    gameEnded,
    showAssist,
    showControl,
    autoPlayers,
    assistMoves,
    lastMove,
    historyLength,
    currentTimelineIndex,
    setShowAssist,
    setShowControl,
    executeMove,
    resetGame,
    togglePlayerAuto,
    stepBackward,
    stepForward
  } = useGameState();

  // 2. Local mouse tracking states specifically for our floating Drag Overlay layer
  const [draggedPiece, setDraggedPiece] = useState(null); // stores piece string type ('W'/'B')
  const [mousePosition, setMousePosition] = useState({ x: 0, y: 0 });

  // 3. Register global keyboard shortcuts using our useHotkeys hook
  // Wrapped in useMemo to prevent unnecessary key re-binding cycles
  const hotkeyMap = useMemo(() => ({
    'F2': () => setShowAssist(prev => !prev),
    'F3': () => setShowControl(prev => !prev),
    'q':  () => togglePlayerAuto(0),
    'a':  () => togglePlayerAuto(1),
    'r':  () => resetGame(),
    'ArrowLeft':  () => stepBackward(),
    'ArrowRight': () => stepForward(),
    's':  () => {
      // Step once command: if we had an AI step ready, we would call it here
      console.log("Single simulation tick requested via 'S' hotkey.");
    }
  }), [setShowAssist, setShowControl, togglePlayerAuto, resetGame, stepBackward, stepForward]);

  useHotkeys(hotkeyMap);

  // 4. Global cursor tracking event handlers
  const handleGlobalMouseMove = useCallback((event) => {
    if (!draggedPiece) return;
    setMousePosition({ x: event.clientX, y: event.clientY });
  }, [draggedPiece]);

  /**
   * Catches piece picking triggers passing up from the board assembly grid
   */
  const handleMoveAttempt = useCallback((fromCoords, toCoords) => {
    executeMove(fromCoords, toCoords);
  }, [executeMove]);

  // Intercepting click states directly from the matrix representation to handle visual dragging previews
  const handleBoardMouseDownIntercept = (event) => {
    // If the simulation is running automation, lock out player inputs
    if (autoPlayers[currentPlayer]) return;

    // We can infer what piece was picked up based on where the user pressed
    // The BoardContainer handles actual grid indexing, we just use this to prime the cursor overlay
    const boardRef = event.currentTarget.getBoundingClientRect();
    const clickX = event.clientX - boardRef.left;
    const clickY = event.clientY - boardRef.top;
    
    const targetCoords = BoardGeometry.pixelsToMatrix(clickX, clickY);
    const piece = BoardMatrix.getPiece(board, targetCoords.row, targetCoords.col);

    if (piece) {
      setDraggedPiece(piece);
      setMousePosition({ x: event.clientX, y: event.clientY });
    }
  };

  const handleGlobalMouseUpIntercept = () => {
    setDraggedPiece(null);
  };

  return (
    <div 
      onMouseMove={handleGlobalMouseMove}
      onMouseUp={handleGlobalMouseUpIntercept}
      className="h-auto min-h-screen w-full flex flex-col lg:flex-row items-center justify-center gap-8 lg:gap-12 bg-slate-950 p-4 md:p-8"
    >
      {/* Central Visual Board Layout Assembly Stack */}
      <div onMouseDown={handleBoardMouseDownIntercept} className="h-fit w-fit shrink-0">
        <BoardContainer
          boardState={board}
          showAssist={showAssist}
          showControl={showControl}
          assistMoves={assistMoves}
          lastMove={lastMove}
          onMoveAttempt={handleMoveAttempt}
        />
      </div>

      {/* Control Panel Information Hub Sidebar Layer */}
      <div className="w-full max-w-[512px] lg:w-80 shrink-0">
        <HUD
          currentPlayer={currentPlayer}
          gameEnded={gameEnded}
          showAssist={showAssist}
          showControl={showControl}
          autoPlayers={autoPlayers}
          assistMoves={assistMoves}
          historyLength={historyLength}
          currentTimelineIndex={currentTimelineIndex}
          onToggleAssist={() => setShowAssist(prev => !prev)}
          onToggleControl={() => setShowControl(prev => !prev)}
          onToggleAuto={togglePlayerAuto}
          onStepBackward={stepBackward}
          onStepForward={stepForward}
        />
      </div>

      {/* Independent Performance Optimized Float Render Tracker Layer */}
      <DragOverlay
        isDragging={!!draggedPiece}
        type={draggedPiece}
        mousePos={mousePosition}
      />
    </div>
  );
}