import { useState, useCallback, useEffect } from 'react';
import { EngineAdapterMock } from '../utils/engineAdapterMock';
import { BoardMatrix } from '../utils/boardMatrix';

export function useGameState() {
  // 1. Unified History Timeline Architecture
  // Each history snapshot stores: the board matrix, the active player at that turn, and the lastMove coords
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

  // 2. Overlay Visibility toggles
  const [showAssist, setShowAssist] = useState(false);
  const [showControl, setShowControl] = useState(false);

  // 3. Automation engine states
  const [autoPlayers, setAutoPlayers] = useState([false, false]); // [Player0Auto, Player1Auto]

  // 4. AI calculation tracking
  const [assistMoves, setAssistMoves] = useState(() => 
    EngineAdapterMock.getMockAIAssistMoves(0)
  );

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
    setAssistMoves(EngineAdapterMock.getMockAIAssistMoves(nextPlayer));
    
    return true;
  }, [board, currentPlayer, gameEnded, history, currentIndex]);

  /**
   * Timeline History Navigation Methods (Back / Forward)
   */
  const jumpToTimelineIndex = useCallback((index) => {
    if (index < 0 || index >= history.length) return;
    
    // Safety check: force automation to turn off if traveling backward in time
    // to prevent the AI loop from auto-generating subsequent moves on past states.
    setAutoPlayers([false, false]);
    
    setCurrentIndex(index);
    setAssistMoves(EngineAdapterMock.getMockAIAssistMoves(history[index].currentPlayer));
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
    
    // If automation is not enabled for the active player, do absolutely nothing
    if (!isCurrentAuto) return;

    // Race condition token
    let isCurrentTurnActive = true;

    // Simulate AI "thinking" time (800ms delay)
    const aiTimer = setTimeout(() => {
      // If the user disabled auto, shifted turns, or reset during the window, drop the execution
      if (!isCurrentTurnActive) return;

      const candidates = EngineAdapterMock.getMockAIAssistMoves(currentPlayer);
      if (candidates && candidates.length > 0) {
        // Pick the top-rated candidate move
        const topMove = candidates[0];
        executeMove(topMove.from, topMove.to);
      }
    }, 800);

    // CLEANUP: If turn changes or auto-play is toggled off before the timer ends, clear it
    return () => {
      isCurrentTurnActive = false;
      clearTimeout(aiTimer);
    };
  }, [currentPlayer, autoPlayers, gameEnded, executeMove]);

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
    setAssistMoves(EngineAdapterMock.getMockAIAssistMoves(0));
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
    lastMove,             // Exposed for our new HighlightCanvas layer
    historyLength: history.length, // Exposed for tracking movement bounds
    currentTimelineIndex: currentIndex,
    setShowAssist,
    setShowControl,
    executeMove,
    resetGame,
    togglePlayerAuto,
    stepBackward,         // Exposed for HUD timeline controls
    stepForward           // Exposed for HUD timeline controls
  };
}