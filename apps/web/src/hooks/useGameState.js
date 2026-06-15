import { useState, useCallback } from 'react';
import { EngineAdapterMock } from '../utils/engineAdapterMock';
import { BoardMatrix } from '../utils/boardMatrix';

export function useGameState() {
  const [history, setHistory] = useState(() => [
    {
      board: EngineAdapterMock.getInitialBoard(),
      currentPlayer: 0, // 0 = White, 1 = Black
      lastMove: null
    }
  ]);
  const [currentIndex, setCurrentIndex] = useState(0);
  const [gameEnded, setGameEnded] = useState(false);

  // Active snapshot projections
  const currentSnapshot = history[currentIndex];
  const board = currentSnapshot.board;
  const currentPlayer = currentSnapshot.currentPlayer;
  const lastMove = currentSnapshot.lastMove;

  // Active selection states
  const [selectedCoords, setSelectedCoords] = useState(null);

  // Overlay toggles
  const [showAssist, setShowAssist] = useState(false);
  const [showControl, setShowControl] = useState(false);

  // Automation flags [Player0Auto, Player1Auto]
  const [autoPlayers, setAutoPlayers] = useState([false, false]);

  // AI-Assisted top move evaluation updates (fed dynamically by the controller)
  const [assistMoves, setAssistMoves] = useState([]);
  const [liveProgress, setLiveProgress] = useState(null);

  // Instant piece movement target resolution
  const [validMoves, setValidMoves] = useState([]);

  /**
   * Triggers an update for the selected piece's valid moves.
   */
  const selectPiece = useCallback(async (coords) => {
    if (gameEnded) return;
    if (!coords) {
      setSelectedCoords(null);
      setValidMoves([]);
      return;
    }

    const piece = BoardMatrix.getPiece(board, coords.row, coords.col);
    const friendlyPiece = currentPlayer === 0 ? 'W' : 'B';

    if (piece === friendlyPiece) {
      setSelectedCoords(coords);
      const moves = await EngineAdapterMock.getValidMovesForPiece(board, coords, currentPlayer);
      setValidMoves(moves);
    } else {
      setSelectedCoords(null);
      setValidMoves([]);
    }
  }, [board, currentPlayer, gameEnded]);

  /**
   * Local, pure state mutation layer
   */
  const executeMove = useCallback(async (from, to) => {
    if (gameEnded) return false;

    const legalMoves = await EngineAdapterMock.getValidMovesForPiece(board, from, currentPlayer);
  
    const isValid = legalMoves.some(move => move.row === to.row && move.col === to.col);
    
    if (!isValid) {
      return false;
    }

    const piece = BoardMatrix.getPiece(board, from.row, from.col);
    let updatedBoard = BoardMatrix.setPiece(board, from.row, from.col, null);
    updatedBoard = BoardMatrix.setPiece(updatedBoard, to.row, to.col, piece);

    const nextPlayer = 1 - currentPlayer;
    const cleanHistory = history.slice(0, currentIndex + 1);

    const newSnapshot = {
      board: updatedBoard,
      currentPlayer: nextPlayer,
      lastMove: { from, to }
    };

    setHistory([...cleanHistory, newSnapshot]);
    setCurrentIndex(cleanHistory.length);
    setSelectedCoords(null);
    setValidMoves([]);
    
    return true;
  }, [board, currentPlayer, gameEnded, history, currentIndex]);

  const jumpToTimelineIndex = useCallback((index) => {
    if (index < 0 || index >= history.length) return;
    setAutoPlayers([false, false]);
    setSelectedCoords(null);
    setValidMoves([]);
    setCurrentIndex(index);
  }, [history]);

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
    setValidMoves([]);
    setAssistMoves([]);
    setLiveProgress(null);
  }, []);

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
    liveProgress,
    lastMove,
    historyLength: history.length,
    currentTimelineIndex: currentIndex,
    selectedCoords,
    validMoves,
    selectPiece,
    setShowAssist,
    setShowControl,
    executeMove,
    resetGame,
    togglePlayerAuto,
    setAssistMoves,
    setLiveProgress,
    stepBackward: () => jumpToTimelineIndex(currentIndex - 1),
    stepForward: () => jumpToTimelineIndex(currentIndex + 1)
  };
}