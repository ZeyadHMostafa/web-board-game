import type { StateCreator } from 'zustand';
import type { GameStoreState } from './useGameStore';
import type { Coordinate, Move, EvaluationProgress } from '../domain/types';
import { setPiece } from '../domain/rules';
import { GameEngineClient } from '../services/engine/workerClient';

export interface GameStateState {
  engineClient: GameEngineClient | null;
  allLegalMoves: Move[];
  gameEnded: boolean;
  liveEval: EvaluationProgress | null;
  currentPlayer: 0 | 1;
}

export interface GameStateActions {
  initEngine: () => void;
  terminateEngine: () => void;
  fetchLegalMoves: () => Promise<void>;
  executeMove: (from: Coordinate, to: Coordinate) => boolean;
  setLiveEval: (progress: EvaluationProgress | null) => void;
}

export type GameStateSlice = GameStateState & GameStateActions;


export const createGameStateSlice: StateCreator<
  GameStoreState,
  [],
  [],
  GameStateSlice
> = (set, get) => ({
  engineClient: null,
  allLegalMoves: [],
  gameEnded: false,
  liveEval: null,
  currentPlayer: 0,

  initEngine: () => {
    if (get().engineClient) return;
    get().terminateEngine();
    const { mode } = get();
    if (!mode) return;
    
    const client = new GameEngineClient({
      onMoveReady: (move) => {
        const { currentPlayer, whiteEngine, blackEngine } = get();
        const activeEngine = currentPlayer === 0 ? whiteEngine : blackEngine;

        const engineKey = currentPlayer === 0 ? 'whiteEngine' : 'blackEngine';
        set((state) => ({
          [engineKey]: { ...state[engineKey], isCalculating: false }
        }));
        
        if (!activeEngine.isAuto) {
          console.warn(`Engine returned a move for player ${currentPlayer}, but they are no longer Auto.`);
        return
        }
        
        get().executeMove(move.from, move.to);
      },
      onEvaluationUpdate: (progress) => {
        get().setLiveEval(progress);
      },
      onError: (err) => console.error("Engine Worker Error:", err)
    });

    set({ engineClient: client });
    get().triggerLiveEvaluation();
    get().fetchLegalMoves();
  },

  terminateEngine: () => {
    const { engineClient } = get();
    if (engineClient) {
      engineClient.terminate();
      set({ engineClient: null });
    }
  },

  fetchLegalMoves: async () => {
    const { engineClient, history, currentIndex } = get();
    if (!engineClient) return;

    const currentSnapshot = history[currentIndex];

    try {
      const moves = await engineClient.requestAllLegalMoves(
        currentSnapshot.board,
        currentSnapshot.currentPlayer
      );
      
      set({ 
        allLegalMoves: moves,
        gameEnded: moves.length === 0 
      });
    } catch (error) {
      console.error("WASM Evaluation Error:", error);
    }
  },

  executeMove: (from, to) => {
    const { gameEnded, allLegalMoves, history, currentIndex, appendSnapshot, fetchLegalMoves, clearSelection, triggerEngineMove, triggerLiveEvaluation } = get();
    
    if (gameEnded) return false;

    const currentSnapshot = history[currentIndex];
    const { board, currentPlayer } = currentSnapshot;
    const piece = board[from.row][from.col];

    if (!piece) return false;

    const isLegal = allLegalMoves.some(
      (move) => 
        move.to.row === to.row && 
        move.to.col === to.col && 
        move.from.row === from.row && 
        move.from.col === from.col
    );

    if (!isLegal) return false;

    const targetSquare = board[to.row][to.col];
    const isCapture = targetSquare !== null;

    let updatedBoard = setPiece(board, from.row, from.col, null);
    updatedBoard = setPiece(updatedBoard, to.row, to.col, piece);

    const nextPlayer = currentPlayer === 0 ? 1 : 0;

    appendSnapshot({
      board: updatedBoard,
      currentPlayer: nextPlayer,
      lastMove: { from, to, isCapture }
    });

    set({ currentPlayer: nextPlayer });

    clearSelection();

    fetchLegalMoves().then(() => {
      triggerEngineMove();
      triggerLiveEvaluation();
    });

    return true;
  },

  setLiveEval: (progress) => {
    set({ liveEval: progress });
  }
});