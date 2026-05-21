import TurnIndicator from './TurnIndicator';
import ControlToggle from './ControlToggle';
import MoveScorer from './MoveScorer';

export default function HUD({
  currentPlayer,
  gameEnded,
  showAssist,
  showControl,
  autoPlayers,
  assistMoves,
  historyLength,
  currentTimelineIndex,
  onToggleAssist,
  onToggleControl,
  onToggleAuto,
  onStepBackward,
  onStepForward,
}) {
  // Determine timeline boundary locks
  const isAtStart = currentTimelineIndex === 0;
  const isAtEnd = currentTimelineIndex === historyLength - 1;

  return (
    <div 
      className="w-full lg:h-full flex flex-col gap-5 p-6 bg-slate-900/60 backdrop-blur-md border border-slate-800 rounded-xl shadow-xl lg:overflow-y-auto scrollbar-thin scrollbar-thumb-slate-800"
    >
      {/* Header Info Block */}
      <div className="shrink-0">
        <h2 className="text-xl font-black tracking-tight text-white">Simulation Hub</h2>
        <p className="text-xs text-slate-400 mt-0.5">Control state parameters & monitoring</p>
      </div>

      <hr className="border-slate-800 shrink-0" />

      {/* Turn Tracker Sub-Module */}
      <TurnIndicator currentPlayer={currentPlayer} gameEnded={gameEnded} />

      {/* Dynamic AI Score Data Module Box */}
      <MoveScorer moves={assistMoves} showAssist={showAssist} />

      {/* Timeline Time-Travel Interface Dashboard Block */}
      <div className="flex flex-col gap-2 bg-slate-950/40 border border-slate-800/60 p-3 rounded-lg shrink-0">
        <div className="flex items-center justify-between text-[11px] font-bold uppercase tracking-wider text-slate-500 px-1 mb-1">
          <span>Simulation History</span>
          <span className="text-slate-400 font-mono text-xs normal-case font-medium">
            Frame {currentTimelineIndex} / {historyLength - 1}
          </span>
        </div>
        
        <div className="grid grid-cols-2 gap-2">
          <button
            onClick={onStepBackward}
            disabled={isAtStart}
            className="flex items-center justify-center gap-1.5 py-2 px-3 text-xs font-semibold rounded-md transition-all border border-slate-700/60 bg-slate-800/50 text-slate-200 hover:bg-slate-700/70 active:scale-[0.98] disabled:opacity-40 disabled:pointer-events-none disabled:transform-none"
            title="Step back one move (Left Arrow)"
          >
            Back
          </button>
          <button
            onClick={onStepForward}
            disabled={isAtEnd}
            className="flex items-center justify-center gap-1.5 py-2 px-3 text-xs font-semibold rounded-md transition-all border border-slate-700/60 bg-slate-800/50 text-slate-200 hover:bg-slate-700/70 active:scale-[0.98] disabled:opacity-40 disabled:pointer-events-none disabled:transform-none"
            title="Step forward one move (Right Arrow)"
          >
            Forward
          </button>
        </div>
      </div>

      {/* Control Actions Section Block */}
      <div className="flex flex-col gap-2">
        <span className="text-[11px] font-bold uppercase tracking-wider text-slate-500 px-1">Display Layers</span>
        
        <ControlToggle
          label="AI Assist Paths"
          isActive={showAssist}
          onClick={onToggleAssist}
          hotkeyHint="F2"
        />
        <ControlToggle
          label="Control Evaluation Map"
          isActive={showControl}
          onClick={onToggleControl}
          hotkeyHint="F3"
        />
      </div>

      {/* Automation Parameters Block */}
      <div className="flex flex-col gap-2">
        <span className="text-[11px] font-bold uppercase tracking-wider text-slate-500 px-1">Automation Configuration</span>
        
        <ControlToggle
          label="Auto-Play Player 1 (White)"
          isActive={autoPlayers[0]}
          onClick={() => onToggleAuto(0)}
          hotkeyHint="Q"
        />
        <ControlToggle
          label="Auto-Play Player 2 (Black)"
          isActive={autoPlayers[1]}
          onClick={() => onToggleAuto(1)}
          hotkeyHint="A"
        />
      </div>

    </div>
  );
}