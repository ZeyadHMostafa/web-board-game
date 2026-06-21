import { useState, useEffect, useCallback } from 'react';
import { setPiece } from '../../domain/rules';
import type { GameSnapshot, Coordinate, Move } from '../../domain/types';
import type { GameEngineClient } from '../../services/engine/workerClient';

export const useGameState = (
  currentSnapshot: GameSnapshot,
  appendSnapshot: (snapshot: GameSnapshot) => void,
  engineClient: GameEngineClient | null
) => {
  const [allLegalMoves, setAllLegalMoves] = useState<Move[]>([]);
  const [gameEnded, setGameEnded] = useState(false);

  // Synchronize React state with the asynchronous external WASM thread
  useEffect(() => {
    let isActive = true;

    if (!engineClient) return;

    engineClient.requestAllLegalMoves(
      currentSnapshot.board, 
      currentSnapshot.currentPlayer
    ).then((moves: Move[]) => {
      if (!isActive) return;
      
      setAllLegalMoves(moves);
      // If the active player has zero legal moves, the game is over (Checkmate/Stalemate)
      setGameEnded(moves.length === 0);
    }).catch((error: Error) => {
      console.error("WASM Evaluation Error:", error);
    });

    return () => {
      // Prevent race conditions if the user undoes a move before the worker replies
      isActive = false;
    };
  }, [currentSnapshot, engineClient]);

  const executeMove = useCallback((from: Coordinate, to: Coordinate) => {
    if (gameEnded) return false;

    const { board, currentPlayer } = currentSnapshot;
    const piece = board[from.row][from.col];
    
    if (!piece) return false;

    // Strict validation against the worker's source of truth
    const isLegal = allLegalMoves.some(
      (move) => move.to.row === to.row && move.to.col === to.col && move.from.row === from.row && move.from.col === from.col
    );

    if (!isLegal) return false;

    const targetSquare = board[to.row][to.col];
    const isCapture = targetSquare !== null;

    let updatedBoard = setPiece(board, from.row, from.col, null);
    updatedBoard = setPiece(updatedBoard, to.row, to.col, piece);

    const nextPlayer = currentPlayer === 0 ? 1 : 0;

    // Push the resolved state to the Timeline ledger
    appendSnapshot({
      board: updatedBoard,
      currentPlayer: nextPlayer,
      lastMove: { from, to, isCapture }
    });

    return true;
  }, [gameEnded, currentSnapshot, allLegalMoves, appendSnapshot]);

  return {
    allLegalMoves,
    gameEnded,
    executeMove
  };
};