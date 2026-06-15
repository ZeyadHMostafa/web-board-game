export default function RenderedPiece({ type, tileSize }) {
  const isWhite = type === 'W';
  
  const ringColor = isWhite ? 'bg-slate-800' : 'bg-neutral-300';
  const innerColor = isWhite ? 'bg-neutral-300' : 'bg-slate-700';

	function make_even(num) {
		return num % 2 === 0 ? num : num + 1;
	}

  const snappedTile = make_even(Math.floor(tileSize));
  const outerDiameter = make_even(Math.floor(snappedTile * 0.75));
  const innerDiameter = make_even(Math.floor(snappedTile * 0.625));

  // Exact integer pixel center offsets to bypass sub-pixel distribution errors
  const outerOffset = Math.floor((snappedTile - outerDiameter) / 2);
  const innerOffset = Math.floor((outerDiameter - innerDiameter) / 2);

  return (
    <div 
      className={`absolute rounded-full shadow-md ${ringColor}`}
      style={{ 
        width: `${outerDiameter}px`, 
        height: `${outerDiameter}px`,
        transform: `translate3d(${outerOffset}px, ${outerOffset}px, 0px)`
      }}
    >
      <div 
        className={`absolute rounded-full shadow-inner ${innerColor}`} 
        style={{ 
          width: `${innerDiameter}px`, 
          height: `${innerDiameter}px`,
          transform: `translate3d(${innerOffset}px, ${innerOffset}px, 0px)`
        }}
      />
    </div>
  );
}