import React, { useState, useEffect, useRef, useCallback } from 'react';
import { GameContext } from './GameContext';
import { MODE_REGISTRY, AI_LEVELS, AI_LEVEL_PRESETS, type AiLevel } from '../domain/configurations';
import { GameEngineClient } from '../services/engine/workerClient';
import type { GameModeType, Coordinate, EvaluationProgress } from '../domain/types';

import { useTimeline } from './hooks/useTimeline';
import { useGameState } from './hooks/useGameState';
import { useSelection } from './hooks/useSelection';
import { useEngine } from './hooks/useEngine';
import { useEval } from './hooks/useEval';

export const GameProvider: React.FC<{ mode: GameModeType; children: React.ReactNode }> = ({ mode, children }) => {
  const config = MODE_REGISTRY[mode];
  
  const [engineClient, setEngineClient] = useState<GameEngineClient | null>(null);
  const [liveEval, setLiveEval] = useState<EvaluationProgress | null>(null);
  const [showAssist, setShowAssist] = useState<boolean>(config.enableLiveEval);

  // Dynamic state allowing UI modifications mid-game
  const [whiteLevel, setWhiteLevel] = useState<AiLevel>(AI_LEVELS.COMPETITOR);
  const [blackLevel, setBlackLevel] = useState<AiLevel>(AI_LEVELS.COMPETITOR);
  
  const executeMoveRef = useRef<(from: Coordinate, to: Coordinate) => boolean>(null);
  const clearSelectionRef = useRef<() => void>(null);

  useEffect(() => {
    const client = new GameEngineClient({
      onMoveReady: (move) => {
        if (executeMoveRef.current) {
          executeMoveRef.current(move.from, move.to);
        }
      },
      onEvaluationUpdate: (progress) => {
        setLiveEval(progress);
      },
      onError: (err) => console.error(err)
    });

    setEngineClient(client);

    return () => {
      client.terminate();
    };
  }, [mode]);

  const clearEval = useCallback(() => setLiveEval(null), []);

  const timeline = useTimeline();
  const gameState = useGameState(timeline.currentSnapshot, timeline.appendSnapshot, engineClient);
  const selection = useSelection(timeline.currentSnapshot, gameState.allLegalMoves, gameState.gameEnded);

  // Synchronize mutable refs with the latest closures from the domain hooks
  useEffect(() => {
    clearSelectionRef.current = selection.clearSelection;
  }, [selection.clearSelection]);

  useEffect(() => {
    executeMoveRef.current = (from: Coordinate, to: Coordinate) => {
      const success = gameState.executeMove(from, to);
      if (success && clearSelectionRef.current) {
        clearSelectionRef.current();
      }
      return success;
    };
  }, [gameState.executeMove, gameState]);

  // Exposes a wrapped execution handler for the UI that also clears visual selections
  const handleExecuteMove = useCallback((from: Coordinate, to: Coordinate) => {
    if (executeMoveRef.current) {
      return executeMoveRef.current(from, to);
    }
    return false;
  }, []);

  const handleToggleAssist = useCallback(() => {
    setShowAssist((prev) => !prev);
  }, []);

  const handleJumpToHistory = useCallback((index: number) => {
    timeline.jumpToHistoryIndex(index);
    selection.clearSelection();
    clearEval();
  }, [timeline, selection, clearEval]);

  const handleResetGame = useCallback(() => {
    timeline.resetTimeline();
    selection.clearSelection();
    clearEval();
  }, [timeline, selection, clearEval]);

  // Hook receives modified preset objects reactively
  const whiteEngine = useEngine(
    0, 
    config.autoPlayers[0], 
    AI_LEVEL_PRESETS[whiteLevel], 
    timeline.currentSnapshot, 
    engineClient, 
    gameState.gameEnded
  );

  const blackEngine = useEngine(
    1, 
    config.autoPlayers[1], 
    AI_LEVEL_PRESETS[blackLevel], 
    timeline.currentSnapshot, 
    engineClient, 
    gameState.gameEnded
  );

  useEval(
    timeline.currentSnapshot, 
    config, 
    engineClient, 
    gameState.gameEnded, 
    clearEval
  );

  return (
    <GameContext.Provider value={{
      board: timeline.currentSnapshot.board,
      currentPlayer: timeline.currentSnapshot.currentPlayer,
      gameEnded: gameState.gameEnded,
      config,
      historyLength: timeline.historyLength,
      currentIndex: timeline.currentIndex,
      liveEval,
      selectedCoords: selection.selectedCoords,
      validMoves: selection.validMovesForSelection,
      showAssist,
      toggleAssist: handleToggleAssist,
      selectPiece: selection.selectPiece,
      executeMove: handleExecuteMove,
      jumpToHistoryIndex: handleJumpToHistory,
      resetGame: handleResetGame,
      whiteEngine: {
        isAuto: whiteEngine.isAuto,
        toggleAuto: whiteEngine.toggleAuto,
        currentLevel: whiteLevel,
        setAiLevel: setWhiteLevel
      },
      blackEngine: {
        isAuto: blackEngine.isAuto,
        toggleAuto: blackEngine.toggleAuto,
        currentLevel: blackLevel,
        setAiLevel: setBlackLevel
      }
    }}>
      {children}
    </GameContext.Provider>
  );
};

export default GameProvider;