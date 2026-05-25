import { useState, useCallback, useEffect, useMemo } from 'react';
import { EngineAdapterMock } from '../utils/engineAdapterMock';
import { BoardMatrix } from '../utils/boardMatrix';

export function useGameState() {
  // 1. Unified History Timeline Architecture
  const [history, setHistory] = useState(() => [
    {
      board: EngineAdapterMock.getInitialBoard(),
      currentPlayer: 0, // 0 = White, 1 = Black
      lastMove: null
    }
  ]);
  const [currentIndex, setCurrentIndex] = useState(0);
  const [gameEnded, setGameEnded] = useState(false);

  // Derive active snapshot data straight from our current timeline pointer index
  const currentSnapshot = history[currentIndex];
  const board = currentSnapshot.board;
  const currentPlayer = currentSnapshot.currentPlayer;
  const lastMove = currentSnapshot.lastMove;

  // 2. Active Selection States
  const [selectedCoords, setSelectedCoords] = useState(null);

  // 3. Overlay Visibility toggles
  const [showAssist, setShowAssist] = useState(false);
  const [showControl, setShowControl] = useState(false);

  // 4. Automation engine states
  const [autoPlayers, setAutoPlayers] = useState([false, false]); // [Player0Auto, Player1Auto]

  // 5. AI calculation tracking
  const [assistMoves, setAssistMoves] = useState(() => 
    EngineAdapterMock.getMockAIAssistMoves(board, 0)
  );

  /**
   * Computes the available valid targets for the currently selected piece.
   * Recomputes whenever the selection changes, the active board changes, or the player shifts.
   */
  const validMoves = useMemo(() => {
    return EngineAdapterMock.getValidMovesForPiece(board, selectedCoords, currentPlayer);
  }, [board, selectedCoords, currentPlayer]);

  /**
   * Safe action executor to translate a piece structurally across the grid matrix
   */
  const executeMove = useCallback((from, to) => {
    if (gameEnded) return false;

    if (!EngineAdapterMock.isValidMove(board, from, to, currentPlayer)) return false;

    const piece = BoardMatrix.getPiece(board, from.row, from.col);

    let updatedBoard = BoardMatrix.setPiece(board, from.row, from.col, null);
    updatedBoard = BoardMatrix.setPiece(updatedBoard, to.row, to.col, piece);

    const nextPlayer = 1 - currentPlayer;

    // Timeline branching constraint: slice off any future history frames if the user 
    // traveled backward in time and then executed a brand new distinct move.
    const cleanHistory = history.slice(0, currentIndex + 1);

    const newSnapshot = {
      board: updatedBoard,
      currentPlayer: nextPlayer,
      lastMove: { from, to }
    };

    setHistory([...cleanHistory, newSnapshot]);
    setCurrentIndex(cleanHistory.length);
    setAssistMoves(EngineAdapterMock.getMockAIAssistMoves(updatedBoard, nextPlayer));
    
    // Clear selection state upon successful resolution of a move action
    setSelectedCoords(null);
    
    return true;
  }, [board, currentPlayer, gameEnded, history, currentIndex]);

  /**
   * Updates or cancels the active piece coordinate selection.
   */
  const selectPiece = useCallback((coords) => {
    if (gameEnded) return;

    if (!coords) {
      setSelectedCoords(null);
      return;
    }

    const piece = BoardMatrix.getPiece(board, coords.row, coords.col);
    const friendlyPiece = currentPlayer === 0 ? 'W' : 'B';

    if (piece === friendlyPiece) {
      setSelectedCoords(coords);
    } else {
      setSelectedCoords(null);
    }
  }, [board, currentPlayer, gameEnded]);

  /**
   * Timeline History Navigation Methods (Back / Forward)
   */
  const jumpToTimelineIndex = useCallback((index) => {
    if (index < 0 || index >= history.length) return;
    
    // Safety check: force automation to turn off if traveling backward in time
    // to prevent the AI loop from auto-generating subsequent moves on past states.
    setAutoPlayers([false, false]);
    setSelectedCoords(null);
    
    setCurrentIndex(index);
    setAssistMoves(EngineAdapterMock.getMockAIAssistMoves(history[index].board, history[index].currentPlayer));
  }, [history]);

  const stepBackward = useCallback(() => {
    jumpToTimelineIndex(currentIndex - 1);
  }, [currentIndex, jumpToTimelineIndex]);

  const stepForward = useCallback(() => {
    jumpToTimelineIndex(currentIndex + 1);
  }, [currentIndex, jumpToTimelineIndex]);

  /**
   * Automation Game Loop Side-Effect
   * Watches player turns and schedules mock AI responses when automation is active.
   */
  useEffect(() => {
    if (gameEnded) return;

    const isCurrentAuto = autoPlayers[currentPlayer];
    if (!isCurrentAuto) return;

    let isCurrentTurnActive = true;

    const aiTimer = setTimeout(() => {
      if (!isCurrentTurnActive) return;

      const candidates = EngineAdapterMock.getMockAIAssistMoves(board, currentPlayer);
      if (candidates && candidates.length > 0) {
        const topMove = candidates[0];
        executeMove(topMove.from, topMove.to);
      }
    }, 800);

    return () => {
      isCurrentTurnActive = false;
      clearTimeout(aiTimer);
    };
  }, [currentPlayer, autoPlayers, gameEnded, executeMove, board]);

  /**
   * Resets the entire ecosystem state back to default parameters
   */
  const resetGame = useCallback(() => {
    setHistory([
      {
        board: EngineAdapterMock.getInitialBoard(),
        currentPlayer: 0,
        lastMove: null
      }
    ]);
    setCurrentIndex(0);
    setGameEnded(false);
    setSelectedCoords(null);
    setAssistMoves(EngineAdapterMock.getMockAIAssistMoves(EngineAdapterMock.getInitialBoard(), 0));
  }, []);

  /**
   * Helper toggles for system automation modifiers
   */
  const togglePlayerAuto = useCallback((playerIndex) => {
    setAutoPlayers(prev => {
      const updated = [...prev];
      updated[playerIndex] = !updated[playerIndex];
      return updated;
    });
  }, []);

  return {
    board,
    currentPlayer,
    gameEnded,
    showAssist,
    showControl,
    autoPlayers,
    assistMoves,
    lastMove,
    historyLength: history.length,
    currentTimelineIndex: currentIndex,
    selectedCoords,
    validMoves,
    setSelectedCoords,
    selectPiece,
    setShowAssist,
    setShowControl,
    executeMove,
    resetGame,
    togglePlayerAuto,
    stepBackward,
    stepForward
  };
}