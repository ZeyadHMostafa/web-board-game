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
  private pendingRequests = new Map<number, (moves: Move[]) => void>();
  private requestCounter = 0;

  constructor(callbacks: EngineClientCallbacks) {
    this.callbacks = callbacks;
    this.initializeWorker();
  }

  private initializeWorker() {
    this.worker = new Worker(new URL('./aiWorker.ts', import.meta.url), {
      type: 'module'
    });

    this.worker.onmessage = (e: MessageEvent) => {
      const {type, move, moves, progress, error, id} = e.data;

      if (type === 'ALL_LEGAL_MOVES_READY') {
        const resolve = this.pendingRequests.get(id);
        if (resolve) {
          resolve(moves);
          this.pendingRequests.delete(id);
        }
        return;
      }

      if (error || type === 'ENGINE_ERROR') {
        this.callbacks.onError(error || 'Unknown infrastructure error');
        if (id !== undefined && this.pendingRequests.has(id)) {
          this.pendingRequests.delete(id); 
        }
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

  public requestAllLegalMoves(board: BoardMatrixState, player: PlayerIndex): Promise<Move[]> {
    return new Promise((resolve) => {
      if (!this.worker) {
        resolve([]);
        return;
      }

      const id = ++this.requestCounter;
      this.pendingRequests.set(id, resolve);

      this.worker.postMessage({
        type: 'COMPUTE_ALL_LEGAL_MOVES',
        board,
        currentPlayer: player,
        id
      });
    });
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
