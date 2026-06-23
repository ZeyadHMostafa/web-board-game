import React from 'react';
import {useGame} from '../../../context/GameContext';

export const MoveScorer: React.FC = () => {
  const { liveEval } = useGame();

  if (!liveEval) {
    return (
      <div className="p-4 rounded border border-border-muted bg-hud-card text-center text-xs text-text-muted italic">
        Awaiting engine evaluation data...
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-2 bg-hud-card border border-border-muted rounded-lg p-3">
      <div className="flex items-center justify-between border-b border-border-muted pb-1.5 mb-1 text-[10px] font-bold uppercase tracking-wider text-text-muted">
        <span className="flex items-center gap-1">
          <span className="material-icons text-[12px]">memory</span>
          Engine Candidates (d:{liveEval.depthReached})
        </span>
        <span className="font-mono text-accent-glow">Score</span>
      </div>

      {liveEval.candidates.length === 0 ? (
        <p className="text-xs text-text-muted italic py-2 text-center">No calculations found</p>
      ) : (
        <div className="flex flex-col gap-1 max-h-36 overflow-y-auto pr-0.5">
          {liveEval.candidates.slice(0, 4).map((candidate, idx) => (
            <div 
              key={idx}
              className="flex items-center justify-between text-xs py-1 px-2 rounded bg-surface-card/40 border border-transparent hover:border-border-muted transition-all"
            >
              <div className="flex items-center gap-1.5 font-mono">
                <span className="text-text-muted opacity-50 w-3 text-[10px]">{idx + 1}.</span>
                <span className="text-text-main font-semibold bg-surface-card px-1 rounded border border-border-muted">
                  {/* Safely translating rows/cols labels if coordinates aren't algebraic yet */}
                  {`[${candidate.from.row},${candidate.from.col}]`}
                </span>
                <span className="text-text-muted text-[10px]">→</span>
                <span className="text-text-main font-semibold bg-surface-card px-1 rounded border border-border-muted">
                  {`[${candidate.to.row},${candidate.to.col}]`}
                </span>
                {candidate.isCapture && (
                  <span className="material-icons text-indicator-capture text-xs ml-1">gavel</span>
                )}
              </div>
              <span className="font-mono font-bold text-accent-glow text-[11px]">
                {candidate.scoreLabel}
              </span>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default MoveScorer;