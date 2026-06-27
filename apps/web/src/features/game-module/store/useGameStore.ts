import { create } from 'zustand';
import { createTimelineSlice, type TimelineSlice } from './timelineSlice';
import { createGameStateSlice, type GameStateSlice } from './gameStateSlice';
import { createSelectionSlice, type SelectionSlice } from './selectionSlice';
import { createEngineSlice, type EngineSlice } from './engineSlice';
import { createEvalSlice, type EvalSlice } from './evalSlice';
import { createConfigSlice, type ConfigSlice } from './configSlice';

export type GameStoreState = TimelineSlice & 
  GameStateSlice & 
  SelectionSlice & 
  EngineSlice & 
  EvalSlice & 
  ConfigSlice;

export const useGameStore = create<GameStoreState>()((set, get, store) => ({
  ...createTimelineSlice(set, get, store),
  ...createGameStateSlice(set, get, store),
  ...createSelectionSlice(set, get, store),
  ...createEngineSlice(set, get, store),
  ...createEvalSlice(set, get, store),
  ...createConfigSlice(set, get, store),
}));

useGameStore.getState().bootstrapGame('ANALYSIS');