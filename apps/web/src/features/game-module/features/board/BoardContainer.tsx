import React, { useRef } from 'react';
import CheckerBoardBackgroundLayer from './layers/CheckerBoardBackgroundLayer';
import MoveHistoryLayer from './layers/LastMoveLayer';
import IntentSelectionLayer from './layers/SelectionLayer';
import PieceLayer from './layers/PieceLayer';
import AssistLayer from './layers/AssistLayer';
import { useBoardInteractions } from './hooks/useBoardInteractions';
import DragLayer from './layers/DragLayer';

export const BoardContainer: React.FC = () => {
  const boardRef = useRef<HTMLDivElement>(null);
  
  const { 
    handlePointerDown, 
    handlePointerMove, 
    handlePointerUp, 
    draggedPiece, 
    mousePosition,
    dragSource
  } = useBoardInteractions({ boardRef });

  return (
    <>
      <div
        ref={boardRef}
        onPointerDown={handlePointerDown}
        onPointerMove={handlePointerMove}
        onPointerUp={handlePointerUp}
        className="relative w-full h-full shadow-2xl border border-border-muted rounded-md overflow-hidden bg-surface-card touch-none select-none"
      >
        <CheckerBoardBackgroundLayer boardSize={800} />
        <MoveHistoryLayer />
        <IntentSelectionLayer />
        <PieceLayer dragSource={dragSource}/>
        <AssistLayer />
      </div>

      <DragLayer type={draggedPiece} mousePos={mousePosition} />
    </>
  );
};

export default BoardContainer;