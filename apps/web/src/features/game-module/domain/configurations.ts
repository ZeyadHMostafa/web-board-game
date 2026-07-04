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

export const AI_LEVELS = {
  TRAINEE: 'TRAINEE',
  COMPETITOR: 'COMPETITOR',
  MASTER: 'MASTER'
} as const;

export type AiLevel = typeof AI_LEVELS[keyof typeof AI_LEVELS];

export const AI_LEVEL_PRESETS: Record<AiLevel, Required<AiEngineConfiguration>> = {
  [AI_LEVELS.TRAINEE]: {
    minDepth: 1,
    maxDepth: 2,
    temp: 1.2,
    bThresh: 4
  },
  [AI_LEVELS.COMPETITOR]: {
    minDepth: 2,
    maxDepth: 4,
    temp: 0.8,
    bThresh: 2
  },
  [AI_LEVELS.MASTER]: {
    minDepth: 4,
    maxDepth: 7,
    temp: 0.2,
    bThresh: 1
  }
};

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
    maxAssistMovesShown: 0,
    autoPlayers: [false, false],
    aiEngineConfig: {},
  },
};