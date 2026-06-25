export type PlayerColor = 'W' | 'B';

export const PlayerIndex = {
  WHITE: 0,
  BLACK: 1
} as const;

export type PlayerIndex = typeof PlayerIndex[keyof typeof PlayerIndex];

export const GameModeType = {
  STRICT: 'STRICT',
  CASUAL: 'CASUAL',
  ANALYSIS: 'ANALYSIS'
} as const;

export type GameModeType = typeof GameModeType[keyof typeof GameModeType];

export interface Coordinate {
  row: number;
  col: number;
}

export interface Move {
  from: Coordinate;
  to: Coordinate;
  isCapture: boolean;
}

export interface EngineCandidateMove extends Move {
  scoreValue: number;
  scoreLabel: string;
}

export interface EvaluationProgress {
  candidates: EngineCandidateMove[];
  depthReached: number;
  nodesExplored: number;
  branchingFactor: number;
  pv: Move[];
}

export type BoardMatrixState = (PlayerColor | null)[][];

export interface GameSnapshot {
  board: BoardMatrixState;
  currentPlayer: PlayerIndex;
  lastMove: Move | null;
}

// todo: move to higher place
export interface PlayerData {
  id: string;
  name: string;
  rating: number;
  ratingChange: number;
  avatarUrl?: string;
  isUser: boolean;
}