import { GameModeType } from './types';

export interface FeatureConfiguration {
  modeType: GameModeType;
  allowTakebacks: boolean;
  enableTimer: boolean;
  enableLiveEval: boolean;
  strictRules: boolean;
  maxEvaluationDepth: number;
  maxAssistMovesShown: number;
  autoPlayers: [boolean, boolean];
  aiEngineConfig: AiEngineConfiguration;
}

export interface AiEngineConfiguration {
  minDepth?: number;
  maxDepth?: number;
  temp?: number;
  bThresh?: number;
}

export const MODE_REGISTRY: Record<GameModeType, FeatureConfiguration> = {
  [GameModeType.STRICT]: {
    modeType: GameModeType.STRICT,
    allowTakebacks: false,
    enableTimer: true,
    enableLiveEval: false,
    strictRules: true,
    maxEvaluationDepth: 0,
    maxAssistMovesShown: 0,
    autoPlayers: [false, false],
    aiEngineConfig: {},
  },
  [GameModeType.CASUAL]: {
    modeType: GameModeType.CASUAL,
    allowTakebacks: true,
    enableTimer: true,
    enableLiveEval: false,
    strictRules: true,
    maxEvaluationDepth: 0,
    maxAssistMovesShown: 0,
    autoPlayers: [false, false],
    aiEngineConfig: {},
  },
  [GameModeType.ANALYSIS]: {
    modeType: GameModeType.ANALYSIS,
    allowTakebacks: true,
    enableTimer: false,
    enableLiveEval: true,
    strictRules: false,
    maxEvaluationDepth: 4,
    maxAssistMovesShown: 5,
    autoPlayers: [false, false],
    aiEngineConfig: {},
  },
};