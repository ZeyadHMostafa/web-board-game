import React, { useState } from 'react';
import GameStatusBar from './features/hud/GameStatusBar';
import GameSideBar from './features/hud/GameSideBar';
import GameProvider from './context/GameProvider';
import MainBoard from './features/board/GameBoard';

export const GameModuleRoot: React.FC = () => {
	const [isDrawerExpanded, setIsDrawerExpanded] = useState<boolean>(false);

	const toggleDrawer = () => {
		setIsDrawerExpanded((prev) => !prev);
	};

	return (
		<div className="w-full h-full flex flex-col landscape:flex-row min-h-0 min-w-0 overflow-hidden relative">
			<GameProvider mode="ANALYSIS">
				{/* Simulation Viewport Canvas Workspace */}
				<div className="flex-1 min-w-0 min-h-0 bg-app-bg flex items-center justify-center p-4">
					<MainBoard />
				</div>

				{/* Unified Secondary Control Surface System */}
				<div 
					className={`
						shrink-0 flex flex-col
						w-full landscape:w-[320px] landscape:h-full landscape:border-l landscape:border-border-muted
						portrait:relative portrait:w-full portrait:z-20
					`}
				>
					{/* Persistent High-Level Operational State Indicator */}
					<div className="w-full portrait:order-first">
						<GameStatusBar onDrawerToggle={toggleDrawer} isDrawerExpanded={isDrawerExpanded} />
					</div>

					{/* Deep-Dive Parameter Sub-Systems (Inline Panel on Desktop / Vertical Drawer on Mobile) */}
					<div 
						className={`
							flex-1 min-h-0 w-full bg-surface-card
							transition-transform duration-300 ease-in-out
							landscape:transform-none landscape:static
							portrait:fixed portrait:top-full portrait:left-0 portrait:right-0 portrait:h-[60vh] portrait:border-t portrait:border-border-muted
							${isDrawerExpanded ? 'portrait:-translate-y-full' : 'portrait:translate-y-0'}
						`}
					>
						{/* Visual Drag Handle Anchor Indicator for Portrait Interaction */}
						<div 
							onClick={toggleDrawer}
							className="w-full h-6 flex items-center justify-center cursor-pointer bg-hud-bg border-b border-border-muted landscape:hidden"
						>
							<div className="w-12 h-1 rounded-full bg-text-muted/40" />
						</div>

						<div className="w-full h-full portrait:h-[calc(60vh-24px)]">
							<GameSideBar />
						</div>
					</div>
				</div>
			</GameProvider>
		</div>
	);
};

export default GameModuleRoot;