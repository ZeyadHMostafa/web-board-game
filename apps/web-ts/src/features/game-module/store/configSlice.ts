import type { StateCreator } from 'zustand';
import type { GameStoreState } from './useGameStore';
import { MODE_REGISTRY, AI_LEVELS, type FeatureConfiguration} from '../domain/configurations';
import type { GameModeType } from '../domain/types';

export interface ConfigState {
  mode: GameModeType | null;
  config: FeatureConfiguration | null;
}

export interface ConfigActions {
  bootstrapGame: (mode: GameModeType) => void;
  cleanupGame: () => void;
}

export type ConfigSlice = ConfigState & ConfigActions;

export const createConfigSlice: StateCreator<
  GameStoreState,
  [],
  [],
  ConfigSlice
> = (set, get) => ({
  mode: null,
  config: null,

  bootstrapGame: (mode) => {
    const config = MODE_REGISTRY[mode];
    
    set({
      mode,
      config,
    });

    get().setupInitialEngines(
      config.autoPlayers[0],
      config.autoPlayers[1],
      AI_LEVELS.COMPETITOR
    );

    set({ showAssist: config.enableLiveEval });

    get().initEngine();
  },

  cleanupGame: () => {
    get().terminateEngine();
    set({
      mode: null,
      config: null,
    });
  }
});