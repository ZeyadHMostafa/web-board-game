import { BoardGeometry } from '../../utils/boardGeometry';

export default function MoveScorer({ moves = [], showAssist }) {
  if (!showAssist) return null;

  return (
    <div className="flex flex-col gap-2 bg-slate-800/20 border border-slate-700/30 rounded-lg p-4">
      <div className="flex items-center justify-between border-b border-slate-800 pb-2 mb-1">
        <span className="text-[11px] font-bold uppercase tracking-wider text-slate-400">Top Engine Candidates</span>
        <span className="text-[10px] bg-blue-950 text-blue-400 px-1.5 py-0.5 rounded border border-blue-900/40 font-mono">Rating</span>
      </div>

      {moves.length === 0 ? (
        <p className="text-xs text-slate-500 italic py-1 text-center">No calculated routes available</p>
      ) : (
        <div className="flex flex-col gap-1.5 max-h-48 overflow-y-auto pr-1">
          {moves.slice(0, 5).map((move, index) => {
            // Translate raw matrix keys to readable algebraic labels ("A8", "E4")
            const fromLabel = BoardGeometry.matrixToAlgebraic(move.from.row, move.from.col);
            const toLabel = BoardGeometry.matrixToAlgebraic(move.to.row, move.to.col);

            return (
              <div 
                key={index}
                className="flex items-center justify-between text-xs py-1 px-2 rounded bg-slate-800/40 hover:bg-slate-800/80 border border-transparent hover:border-slate-700/30 transition-all duration-150"
              >
                <div className="flex items-center gap-2 text-slate-300">
                  <span className="text-slate-500 font-mono text-[10px] w-3">{index + 1}.</span>
                  <span className="font-semibold text-slate-200 bg-slate-800 px-1.5 py-0.5 rounded border border-slate-700/40">{fromLabel}</span>
                  <span className="text-slate-500">→</span>
                  <span className="font-semibold text-slate-200 bg-slate-800 px-1.5 py-0.5 rounded border border-slate-700/40">{toLabel}</span>
                </div>
                
                {/* Precise decimal scores mapped straight from the AI wrapper data */}
                <span className="font-mono font-bold text-blue-400">
                  {move.rating.toFixed(1)}
                </span>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}