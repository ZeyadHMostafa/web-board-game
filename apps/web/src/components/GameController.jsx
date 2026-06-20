import { useState, useCallback, useMemo, useRef, useEffect } from 'react';
import { useGameState } from '../hooks/useGameState';
import { useHotkeys } from '../hooks/useHotkeys';
import { BoardMatrix } from '../utils/boardMatrix';
import { useWorkerOrchestrator } from '../hooks/useWorkerOrchestrator';
import BoardContainer from './BoardContainer/BoardContainer';
import HUD from './HUD/HUD';
import DragOverlay from './DragOverlay';

export default function GameController() {
  const gameState = useGameState();

  useWorkerOrchestrator(gameState);

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
    selectedCoords,
    validMoves,
    selectPiece,
    setShowAssist,
    setShowControl,
    executeMove,
    resetGame,
    togglePlayerAuto,
    stepBackward,
    stepForward
  } = gameState;

  const [draggedPiece, setDraggedPiece] = useState(null);
  const [mousePosition, setMousePosition] = useState({ x: 0, y: 0 });
  
  const [sharedBoardSize, setSharedBoardSize] = useState(512);
  const boardWrapperRef = useRef(null);

  useEffect(() => {
    if (!boardWrapperRef.current) return;

    const resizeObserver = new ResizeObserver((entries) => {
      for (let entry of entries) {
        const width = entry.contentRect.width;
        if (width > 0) {
          const snappedWidth = Math.floor(width / 8) * 8;
          if (Math.abs(sharedBoardSize - snappedWidth) >= 8) {
            setSharedBoardSize(snappedWidth);
          }
        }
      }
    });

    resizeObserver.observe(boardWrapperRef.current);
    return () => resizeObserver.disconnect();
  }, [sharedBoardSize]);

  const hotkeyMap = useMemo(() => ({
    'F2': () => setShowAssist(prev => !prev),
    'F3': () => setShowControl(prev => !prev),
    'q':  () => togglePlayerAuto(0),
    'a':  () => togglePlayerAuto(1),
    'r':  () => resetGame(),
    'ArrowLeft':  () => stepBackward(),
    'ArrowRight': () => stepForward(),
    's':  () => {
      console.log("Single simulation tick requested via 'S' hotkey.");
    }
  }), [setShowAssist, setShowControl, togglePlayerAuto, resetGame, stepBackward, stepForward]);

  useHotkeys(hotkeyMap);

  const handleGlobalMouseMove = useCallback((event) => {
    if (!draggedPiece) return;
    setMousePosition({ x: event.clientX, y: event.clientY });
  }, [draggedPiece]);

  const handleMoveAttempt = useCallback( async (fromCoords, toCoords) => {
    await executeMove(fromCoords, toCoords);
  }, [executeMove]);

  /**
   * Handles individual, static cell selection clicks
   */
  const handleSquareClick = useCallback(async (coords) => {
    if (autoPlayers[currentPlayer]) return;

    // Determine if the clicked cell matches an option inside our pre-calculated targets array
    const isTargetingValidMove = validMoves.some(
      move => move.row === coords.row && move.col === coords.col
    );

    if (selectedCoords && isTargetingValidMove) {
      // User has a piece active and clicked an authorized destination square -> Execute!
      await executeMove(selectedCoords, coords);
    } else {
      // Otherwise, evaluate changing the current active selection coordinate focus
      selectPiece(coords);
    }
  }, [selectedCoords, validMoves, executeMove, selectPiece, autoPlayers, currentPlayer]);

  const handleDragStartIntercept = (coords, clientX, clientY) => {
    if (autoPlayers[currentPlayer]) return;

    const piece = BoardMatrix.getPiece(board, coords.row, coords.col);
    if (piece && piece == ['W','B'][currentPlayer]) {
      setDraggedPiece(piece);
      setMousePosition({ x: clientX, y: clientY });
      
      // Mirror drag targets visually by forcing selection hooks to match the dragged origin
      selectPiece(coords);
    } else {
      console.log(piece)
    }
  };

  const handleGlobalMouseUpIntercept = () => {
    setDraggedPiece(null);
  };

  return (
    <div 
      onMouseMove={handleGlobalMouseMove}
      onMouseUp={handleGlobalMouseUpIntercept}
      className="min-h-dvh w-full flex flex-col lg:flex-row items-center justify-center gap-6 p-4 md:p-8 bg-slate-950 overflow-x-hidden"
    >
      <div 
        ref={boardWrapperRef}
        className="w-full max-w-[min(100vw-2rem,100dvh-4rem)] lg:max-w-[min(100vw-26rem,100dvh-4rem)] aspect-square flex items-center justify-center shrink-0"
      >
        <BoardContainer
          boardState={board}
          showAssist={showAssist}
          showControl={showControl}
          assistMoves={assistMoves}
          lastMove={lastMove}
          selectedCoords={selectedCoords}
          validMoves={validMoves}
          onMoveAttempt={handleMoveAttempt}
          onDragStart={handleDragStartIntercept}
          onSquareClick={handleSquareClick}
        />
      </div>

      <div 
        className="w-full max-w-[min(100vw-2rem,512px)] lg:w-80 shrink-0"
        style={{
          height: window.innerWidth >= 1024 ? `${sharedBoardSize}px` : 'auto'
        }}
      >
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

      <DragOverlay
        isDragging={!!draggedPiece}
        type={draggedPiece}
        mousePos={mousePosition}
        boardSize={sharedBoardSize}
      />
    </div>
  );
}