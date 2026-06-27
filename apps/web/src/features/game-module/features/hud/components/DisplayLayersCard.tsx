import {useGameStore} from '../../../store/useGameStore';

export const DisplayLayersCard: React.FC = () => {
  const showAssist = useGameStore((state) => state.showAssist);
  const toggleAssist = useGameStore((state) => state.toggleAssist);

  return (
    <div className="flex flex-col gap-2">
      <button
        onClick={() => toggleAssist()}
        className={`w-full flex items-center justify-between px-4 py-3 rounded-lg border text-xs font-semibold tracking-wider uppercase transition-all duration-150 cursor-pointer group ${
          showAssist
            ? 'bg-accent-primary/10 border-accent-primary/40 text-accent-glow shadow-md'
            : 'bg-hud-card/30 border-border-muted text-text-muted hover:bg-hud-card/60 hover:text-text-main'
        }`}
      >
        <span className="flex items-center gap-2">
          <span className="material-icons text-sm">
            {showAssist ? 'visibility' : 'visibility_off'}
          </span>
          <span>AI Assist Vectors</span>
        </span>
        
        <kbd className="px-1.5 py-0.5 text-[10px] font-mono rounded bg-surface-card border border-border-muted text-text-muted group-hover:text-text-main">
          F2
        </kbd>
      </button>
    </div>
  );
};

export default DisplayLayersCard;