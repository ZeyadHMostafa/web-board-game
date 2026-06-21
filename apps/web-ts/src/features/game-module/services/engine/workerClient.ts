import type { FeatureConfiguration, AiEngineConfiguration } from '../../domain/configurations';
import type {
  BoardMatrixState,
  PlayerIndex,
  Move,
  EvaluationProgress
} from '../../domain/types';

interface EngineClientCallbacks {
  onMoveReady: (move: Move) => void;
  onEvaluationUpdate: (progress: EvaluationProgress) => void;
  onError: (error: string) => void;
}

export class GameEngineClient {
  private worker: Worker | null = null;
  private callbacks: EngineClientCallbacks;

  constructor(callbacks: EngineClientCallbacks) {
    this.callbacks = callbacks;
    this.initializeWorker();
  }

  private initializeWorker() {
    this.worker = new Worker(new URL('./aiWorker.ts', import.meta.url), {
      type: 'module'
    });

    this.worker.onmessage = (e: MessageEvent) => {
      const {type, move, progress, error} = e.data;

      if (error || type === 'ENGINE_ERROR') {
        this.callbacks.onError(error || 'Unknown infrastructure error');
        return;
      }

      switch (type) {
        case 'AI_MOVE_READY':
          this.callbacks.onMoveReady(move);
          break;
        case 'EVAL_PROGRESS_UPDATE':
          this.callbacks.onEvaluationUpdate(progress);
          break;
      }
    };
  }

  public requestAIMove(
    board: BoardMatrixState,
    player: PlayerIndex,
    config: AiEngineConfiguration
  ): void {
    this.worker?.postMessage({
      type: 'COMPUTE_AI_MOVE',
      board,
      currentPlayer: player,
      config
    });
  }

  public requestLiveEvaluation(
    board: BoardMatrixState,
    player: PlayerIndex,
    config: FeatureConfiguration
  ): void {
    this.worker?.postMessage({
      type: 'COMPUTE_LIVE_EVAL',
      board,
      currentPlayer: player,
      config: {
        minDepth: 1,
        maxDepth: config.maxEvaluationDepth
      }
    });
  }

  public terminate(): void {
    this.worker?.terminate();
    this.worker = null;
  }
}
