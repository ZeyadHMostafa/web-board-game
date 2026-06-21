import { createContext, useContext } from 'react';
import type { 
  BoardMatrixState, 
  PlayerIndex,  
  Coordinate,
  EvaluationProgress
} from '../domain/types';
import type {FeatureConfiguration} from '../domain/configurations';

export interface GameContextType {
  board: BoardMatrixState;
  currentPlayer: PlayerIndex;
  gameEnded: boolean;
  config: FeatureConfiguration;
  historyLength: number;
  currentIndex: number;
  liveEval: EvaluationProgress | null;
  selectedCoords: Coordinate | null;
  validMoves: Coordinate[];
  selectPiece: (coords: Coordinate | null) => void;
  executeMove: (from: Coordinate, to: Coordinate) => boolean;
  jumpToHistoryIndex: (index: number) => void;
  resetGame: () => void;
  
  whiteEngine: { isAuto: boolean; toggleAuto: () => void };
  blackEngine: { isAuto: boolean; toggleAuto: () => void };
}

export const GameContext = createContext<GameContextType | undefined>(undefined);

export const useGame = () => {
  const context = useContext(GameContext);
  if (!context) throw new Error("useGame must be consumed within an active GameProvider");
  return context;
};