export default function ControlToggle({ label, isActive, onClick, hotkeyHint }) {
  return (
    <button
      onClick={onClick}
      className={`w-full flex items-center justify-between px-4 py-3 rounded-lg border text-sm font-medium transition-all duration-150 cursor-pointer group ${
        isActive
          ? 'bg-blue-600/20 border-blue-500/40 text-blue-300 shadow-md'
          : 'bg-slate-800/30 border-slate-700/40 text-slate-400 hover:bg-slate-800/60 hover:text-slate-300'
      }`}
    >
      <span className="flex items-center gap-2">
        {/* Simple visual toggle indicator bulb */}
        <span className={`h-2 w-2 rounded-full transition-colors duration-150 ${isActive ? 'bg-blue-400' : 'bg-slate-600 group-hover:bg-slate-500'}`} />
        {label}
      </span>
      
      {hotkeyHint && (
        <kbd className="px-1.5 py-0.5 text-[10px] font-mono rounded bg-slate-800/80 border border-slate-600/30 text-slate-500 group-hover:text-slate-400">
          {hotkeyHint}
        </kbd>
      )}
    </button>
  );
}