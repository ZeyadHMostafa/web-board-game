export default function TurnIndicator({ currentPlayer, gameEnded }) {
  if (gameEnded) {
    return (
      <div className="bg-emerald-950/40 border border-emerald-500/30 rounded-lg p-4 text-center">
        <h3 className="text-xl font-bold text-emerald-400 animate-pulse">Game Concluded</h3>
        <p className="text-xs text-slate-400 mt-1">Press 'R' to reset the board</p>
      </div>
    );
  }

  const isWhiteTurn = currentPlayer === 0;

  return (
    <div className="bg-slate-800/40 border border-slate-700/50 rounded-lg p-4 flex items-center justify-between">
      <div>
        <span className="text-xs font-semibold uppercase tracking-wider text-slate-400">Current Turn</span>
        <h3 className="text-lg font-bold text-slate-200 mt-0.5">
          {isWhiteTurn ? 'White Engine' : 'Black Engine'}
        </h3>
      </div>
      
      {/* Visual Avatar Ring reflecting active piece */}
      <div className="relative flex h-8 w-8 items-center justify-center">
        <span className={`animate-ping absolute inline-flex h-full w-full rounded-full opacity-25 ${isWhiteTurn ? 'bg-neutral-300' : 'bg-slate-500'}`} />
        <div className={`h-6 w-6 rounded-full border shadow-sm ${isWhiteTurn ? 'bg-neutral-200 border-neutral-400' : 'bg-slate-700 border-slate-600'}`} />
      </div>
    </div>
  );
}