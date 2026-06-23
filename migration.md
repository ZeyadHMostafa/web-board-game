# Section 1: Global Core & The Tokenized Design System

This section establishes the architectural paradigm, the visual token boundaries, and the structural directory layout for the replatformed application. The developer must implement these foundational settings before introducing game mechanics or rendering logic.

---

## 1.1 The Explicit Command-Driven Loop Paradigm

The original codebase relies on an implicit reactive loop driven by cascading React `useEffect` hooks. In that model, side-effects (such as messaging the Web Worker) monitor changing state slices, causing unpredictable execution orders, redundant calculations, and tight coupling between the UI lifecycle and the infrastructure thread.

The new architecture pivots to an **Explicit, Command-Driven Pipeline**. React state is treated strictly as a passive data projection of your domain layer. Components do not trigger actions by changing states for hooks to observe; instead, user interactions invoke central handlers that explicitly execute business logic and dispatch infrastructure commands within the same execution frame.

```
[ User Interaction / Input Layer ]
               │
               ▼ (Invokes Explicit Callback)
[ Game Context Coordinator ]
               │
               ├─► [ Domain Rules Engine ] ──► (Validates & Updates State)
               │
               ▼ (Simultaneous Command Dispatch)
[ GameEngineClient Service ] ────────────────► (Posts Explicit Task to Worker)

```

### Architectural Mandates

* **No Side-Effect Monitors:** The use of `useEffect` to watch changes to properties like `board`, `currentPlayer`, or `showAssist` for the purpose of triggering worker messages or downstream calculations is strictly prohibited.
* **Single-Frame Dispatching:** State progression and parallel asynchronous thread notifications must occur together within an explicit command function.

---

## 1.2 Semantic Tailwind Design Tokens Setup

To eliminate utility class bloat and prevent hardcoded hex codes from scattering across components, you must establish a system of **Semantic Design Tokens**. Components must remain agnostic of specific color palettes, ensuring the UI can adapt fluidly when swapping layouts or themes between different game modes (e.g., a high-contrast theme for Strict Match mode vs. an information-rich theme for Analysis mode).

Configure the following tokens in your global entry stylesheet (`src/styles/index.css`) utilizing standard Tailwind CSS theme variables:

```css
@theme {
  /* Application Shell Framework */
  --color-app-bg: #020617;           /* slate-950 */
  --color-surface-card: #0f172a;     /* slate-900 */
  --color-border-muted: #1e293b;     /* slate-800 */
  
  /* Feature UI & HUD Tokens */
  --color-hud-bg: rgba(15, 23, 42, 0.6);      /* slate-900/60 with blur */
  --color-hud-border: rgba(30, 41, 59, 0.4);  /* slate-800/40 */
  --color-hud-card: rgba(15, 23, 42, 0.4);    /* slate-950/40 */
  
  /* Board Matrix Core Fields */
  --color-tile-light: #334155;       /* slate-700 */
  --color-tile-dark: #1e293b;        /* slate-850 */
  
  /* Tactical Action Overlays */
  --color-accent-primary: #2563eb;   /* blue-600 */
  --color-accent-glow: rgba(96, 165, 250, 0.85); /* blue-400 alpha */
  --color-indicator-legal: #10b981;  /* emerald-500 */
  --color-indicator-capture: #ef4444;/* red-500 */
  
  /* Typography Structure classes */
  --color-text-main: #f8fafc;        /* slate-50 */
  --color-text-muted: #94a3b8;       /* slate-400 */
}

```

### Layout Code Styling Rule

Components must use these semantic abstraction classes exclusively. Writing hardcoded color values directly inside the markup (such as `bg-slate-900` or `text-blue-400`) will fail code quality checks.

---

## 1.3 The Encapsulated Directory Layout Blueprint

The application is structured to isolate the core game engine from global application operations like matchmaking, leaderboards, and general navigation. Everything pertaining to the board game experience must live inside an encapsulated module boundary, exposing exactly **one** root component (`GameModuleRoot.tsx`) to the outside world.

Scaffold the following workspace structure inside your repository:

```
apps/web/src/
├── components/                       # Global shell components (Navigation, Layout)
│   └── Navigation.tsx
├── pages/                            # Top-level view routing layers
│   ├── MatchmakingPage.tsx           # Future shell module
│   ├── ScoreboardPage.tsx            # Future shell module
│   └── GamePage.tsx                  # Imports and renders <GameModuleRoot /> exclusively
│
└── features/
    └── game-module/                  # THE COMPLETE SYSTEM DECOUPLING GATEWAY
        ├── GameModuleRoot.tsx        # Singular layout element exported to outer shell
        │
        ├── context/
        │   └── GameContext.tsx       # State coordinator & unified command dispatcher
        │
        ├── domain/                   # PURE TYPESCRIPT (Zero React/DOM imports allowed)
        │   ├── types.ts              # Global engine interfaces, types, and enums
        │   ├── configurations.ts     # Feature configurations for strict/casual/analysis
        │   └── rules.ts              # Pure board state mutations & validations
        │
        ├── services/                 # INFRASTRUCTURE COUPLING ISOLATION LAYER
        │   └── engine/
        │       ├── aiWorker.ts       # Direct background worker WASM thread execution script
        │       ├── workerClient.ts   # Class facade managing thread promises and callbacks
        │       └── engineAdapterMock.ts # Native offline sandbox tool interface
        │
        ├── utils/                    # MATH & GEOMETRY UTILITIES
        │   └── gridGeometry.ts       # Pure matrix-to-pixel coordinate equations
        │
        └── features/                 # SUB-FEATURE GRAPHIC WORKSPACES
            ├── board/                # Visual board engine domain
            │   ├── BoardContainer.tsx# Wrapper assembling the structural layout layers
            │   ├── hooks/            # Local high-frequency input & drag hooks
            │   ├── layers/           # Base layers (UnifiedCanvas, SvgVector, PieceHtml)
            │   └── library/          # Graphic brush assets & SVG template assets
            │
            ├── hud/                  # Info dashboard domain
            │   ├── HUD.tsx           # Structural framework grid layout panel
            │   └── components/       # Micro-modules (EvalBar, GameTimer, HistoryList)
            │
            └── modes/                # VIEW Blueprints
                ├── PlayView.tsx      # Assembles elements for timed, strict games
                └── AnalysisView.tsx  # Assembles elements for live-evaluation layouts

```

---

## 1.4 Phase 1 Implementation Steps

1. Verify that your development framework has TypeScript initialized with strict typing checks turned on (`strict: true` in `tsconfig.json`).
2. Add the semantic theme variables directly to your global tailwind styles layout file.
3. Build out the complete directory folder configuration inside `src/features/game-module/` exactly as detailed above.
4. Place an export block placeholder inside `GameModuleRoot.tsx` to serve as a clean visual target for your outer application pages framework before migrating your domain files.

---

# Section 2: The Domain & Feature Flag Registry

This section isolates the structural patterns, definitions, and pure validation systems of the game from the user interface. The developer must ensure that nothing inside this directory imports from React or touches the DOM. It is designed as a standalone, testable, type-safe TypeScript library.

---

When you have initialized this folder structure and configured the design tokens, let's proceed to Section 2: The Domain & Feature Flag Registry.

---

## 2.1 Strict Typing Specifications (`domain/types.ts`)

To eliminate untyped data objects, we formalize every data shape required by both the UI and the underlying WebAssembly binary.

```typescript
export type PlayerColor = 'W' | 'B';

export enum PlayerIndex {
    WHITE = 0,
    BLACK = 1
}

export enum GameModeType {
    STRICT = 'STRICT',
    CASUAL = 'CASUAL',
    ANALYSIS = 'ANALYSIS'
}

export interface Coordinate {
    row: number; // Validated grid bounds: 0 to 7
    col: number; // Validated grid bounds: 0 to 7
}

export interface Move {
    from: Coordinate;
    to: Coordinate;
    isCapture: boolean;
}

export interface EngineCandidateMove extends Move {
    scoreValue: number;
    scoreLabel: string;
}

export interface EvaluationProgress {
    candidates: EngineCandidateMove[];
    depthReached: number;
    nodesExplored: number;
    branchingFactor: number;
    pv: Move[];
}

export type BoardMatrixState = (PlayerColor | null)[][];

export interface GameSnapshot {
    board: BoardMatrixState;
    currentPlayer: PlayerIndex;
    lastMove: Move | null;
}

```

---

## 2.2 The Game-Agnostic Feature Configuration Model (`domain/configurations.ts`)

Instead of cluttering components with inline conditional checks for mode variables, the application evaluates a unified, centralized feature flag configuration registry.

```typescript
export interface FeatureConfiguration {
    modeType: GameModeType;
    allowTakebacks: boolean;
    enableTimer: boolean;
    enableLiveEval: boolean;
    strictRules: boolean;
    maxEvaluationDepth: number;
    maxAssistMovesShown: number;
}

export const MODE_REGISTRY: Record<GameModeType, FeatureConfiguration> = {
    [GameModeType.STRICT]: {
        modeType: GameModeType.STRICT,
        allowTakebacks: false,
        enableTimer: true,
        enableLiveEval: false,
        strictRules: true,
        maxEvaluationDepth: 0,
        maxAssistMovesShown: 0,
    },
    [GameModeType.CASUAL]: {
        modeType: GameModeType.CASUAL,
        allowTakebacks: true,
        enableTimer: true,
        enableLiveEval: false,
        strictRules: true,
        maxEvaluationDepth: 0,
        maxAssistMovesShown: 0,
    },
    [GameModeType.ANALYSIS]: {
        modeType: GameModeType.ANALYSIS,
        allowTakebacks: true,
        enableTimer: false,
        enableLiveEval: true,
        strictRules: false, // Bypasses engine rules for free-form sandbox editing
        maxEvaluationDepth: 4,
        maxAssistMovesShown: 5,
    },
};

```

---

## 2.3 Stateless Game Rules & Matrix Translations (`domain/rules.ts`)

Port the core array manipulation utilities from your original codebase into this pure module. Ensure it focuses exclusively on immutable mutations and grid bounds enforcement.

### Key Implementation Guidelines

* **`createInitialPosition()`**: Must return a fixed $8 \times 8$ nested matrix layout. The initialization logic handles filling player index slots without reading external state.
* **`setPiece(board, row, col, piece)`**: To support React's shallow change detection accurately, this function must not mutate the incoming board matrix. It must execute a clean copy pass:
```typescript
export const setPiece = (board: BoardMatrixState, row: number, col: number, piece: PlayerColor | null): BoardMatrixState => {
    const nextBoard = board.map(innerRow => [...innerRow]);
    if (row >= 0 && row < 8 && col >= 0 && col < 8) {
        nextBoard[row][col] = piece;
    }
    return nextBoard;
};

```



---

## 2.4 The Immutable History Timeline Engine

The original layout relied on tracking loose index counters directly inside visual event hooks. The replatformed approach handles the match history as an **immutable append-only sequence of snapshots**.

* **Timeline Index Shifts:** Navigating backward or forward via timeline travelers updates a simple pointer integer (`currentIndex`) within your state context.
* **Branch Clearing Rule:** If the timeline pointer is positioned at an earlier frame (`currentIndex < history.length - 1`) and the user performs a new physical piece movement on the board, the system slices the history array down to that index point, clears out all obsolete alternative futures (`history.slice(0, currentIndex + 1)`), and appends the new move snapshot.

---

## 2.5 Phase 2 Implementation Steps

1. Create `src/features/game-module/domain/types.ts` and add the strong typing interfaces.
2. Build the `MODE_REGISTRY` configuration map. Verify that adding a new variant flag configuration does not break compatibility with existing feature contracts.
3. Validate that no files inside the `domain/` directory contain imports pointing to React libraries or layout elements.

---

When your domain models are locked down, let's advance to Section 3: Infrastructure & Thread Management Services.

---

# Section 3: Infrastructure & Thread Management Services

This section details how to isolate WebAssembly compilation, worker lifecycles, and cross-thread communications behind an asynchronous class facade. The main thread and UI layers must never handle raw web worker events or post directly to background threads; they interface exclusively with a stateless engine client.

---

## 3.1 The Stateless Class-Based Web Worker Client (`services/engine/workerClient.ts`)

Instead of trapping thread interaction inside a reactive `useEffect` block, communication with the worker thread is encapsulated within a pure TypeScript class. This class converts the messaging layer into a structured, callback-driven command target.

```typescript
import { BoardMatrixState, PlayerIndex, FeatureConfiguration, Move, EvaluationProgress } from '../../domain/types';

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
        // Initializes the worker utilizing Vite's native explicit module routing syntax
        this.worker = new Worker(
            new URL('./aiWorker.ts', import.meta.url),
            { type: 'module' }
        );

        this.worker.onmessage = (e: MessageEvent) => {
            const { type, move, progress, error } = e.data;

            if (error) {
                this.callbacks.onError(error);
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

    public requestAIMove(board: BoardMatrixState, player: PlayerIndex, config: any): void {
        this.worker?.postMessage({
            type: 'COMPUTE_AI_MOVE',
            board,
            currentPlayer: player,
            config
        });
    }

    public requestLiveEvaluation(board: BoardMatrixState, player: PlayerIndex, config: FeatureConfiguration): void {
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

    /**
     * Explicitly dismantles the background worker instance to ensure zero orphaned thread memory leaks
     */
    public terminate(): void {
        this.worker?.terminate();
        this.worker = null;
    }
}

```

---

## 3.2 Boundary-Level Data Cleansing & Normalization (`services/engine/aiWorker.ts`)

The background web worker script runs intensive computational steps, keeping operations entirely separated from the primary rendering layout thread.

The developer must migrate data-formatting, payload sorting, and binary validation rules **into the worker execution context itself**. This protects the main application thread from processing raw matrices or performing calculations during message event passes.

### Structural Requirements for `aiWorker.ts`

* **Isolate WASM State Loading:** The script imports and invokes the generated WebAssembly initialization handles (`init()`) locally within its execution block.
* **On-Boundary Sorting:** When responding to a `COMPUTE_LIVE_EVAL` request, the worker thread must explicitly pre-sort and slice the structural arrays (e.g., sorting candidates based on engine score values up to the configuration's maximum limits) before sending the payload up via `postMessage`.

---

## 3.3 Environment Interface Fallback Protocols

To ensure front-end design, component composition, and sandbox workflows can be fully debugged without relying on compiled WebAssembly environments, the system preserves an alternate fallback client interface layer.

* **The Engine Adapter Fallback (`services/engine/engineAdapterMock.ts`):** Retain your original mock implementation architecture, wrapping its data utilities to fit the exact interfaces defined in Section 2.
* **Safe Sandbox Testing:** If the WASM environment initialization fails or if a sandbox configurations toggle bypasses backend calls, the orchestration layer falls back to using the local mock tool without crashing the user interface.

---

## 3.4 Phase 3 Implementation Steps

1. Implement the `GameEngineClient` class under `src/features/game-module/services/engine/`.
2. Port your worker thread message event loop directly into `aiWorker.ts`. Ensure that candidate sorting actions take place entirely within the worker thread background context.
3. Verify that the class exposes an explicit `.terminate()` destructor, ensuring resource leaks are prevented when the module is unmounted or reset.

---

When your infrastructure worker client is up and running, let's step into Section 4: The Shared Graphics Library & Interaction Math.

---

# Section 4: The Shared Graphics Library & Interaction Math

This section provides the mathematical layout functions and stateless graphic primitives required to draw components on an $8 \times 8$ grid. By tying dimensions and strokes to a standard scale variable, the entire visual system becomes scale-invariant—guaranteeing perfect proportions on mobile viewports, responsive dashboard grids, or desktop monitors.

---

## 4.1 Scale-Invariant Grid Coordinate Equations (`utils/gridGeometry.ts`)

This utility file isolates the pure mathematical transformations that bridge the gap between 2D array coordinates and absolute screen pixels. No file inside this script may read from the global window context or reference active component trees.

```typescript
import { Coordinate } from '../domain/types';

export const GridGeometry = {
    /**
     * Translates grid indexes to absolute pixel coordinates mapping the true center of a square.
     * Inverts the Y-axis calculation so that row 0 represents the bottom visual row.
     */
    matrixToPixels(row: number, col: number, boardSize: number): { x: number; y: number } {
        const tileSize = boardSize / 8;
        return {
            x: col * tileSize + tileSize / 2,
            y: (7 - row) * tileSize + tileSize / 2
        };
    },

    /**
     * Translates grid indexes to the exact top-left pixel corner of a cell.
     * Ideal for positioning HTML piece wrappers or computing bounding boxes.
     */
    matrixToTileTopLeft(row: number, col: number, boardSize: number): { x: number; y: number } {
        const tileSize = boardSize / 8;
        return {
            x: col * tileSize,
            y: (7 - row) * tileSize
        };
    },

    /**
     * Converts raw canvas bounding pixel hits back into a row/column grid intersection.
     * Crucial for intercepting where a pointer tap down or drag drop occurs.
     */
    pixelsToMatrix(x: number, y: number, boardSize: number): Coordinate {
        const tileSize = boardSize / 8;
        const col = Math.floor(x / tileSize);
        const row = 7 - Math.floor(y / tileSize);
        
        return {
            row: Math.max(0, Math.min(7, row)),
            col: Math.max(0, Math.min(7, col))
        };
    },

    /**
     * Translates coordinates to standard chess notation strings.
     */
    matrixToAlgebraic(row: number, col: number): string {
        const files = "ABCDEFGH";
        return `${files[col]}${row}`;
    }
};

```

---

## 4.2 The Presets-Only Canvas Brush Subsystem (`features/board/library/canvasDrawers.ts`)

Instead of allowing layout components to manage render clearances, stroke colors, or execution states, low-level canvas drawings are extracted into stateless utility operations. They accept a 2D rendering context, layout coordinates, and scaling properties to paint shapes directly.

```typescript
import { GridGeometry } from '../../../utils/gridGeometry';

export const CanvasDrawers = {
    clear(ctx: CanvasRenderingContext2D, size: number): void {
        ctx.clearRect(0, 0, size, size);
    },

    drawTileHighlight(ctx: CanvasRenderingContext2D, row: number, col: number, color: string, boardSize: number): void {
        const { x, y } = GridGeometry.matrixToTileTopLeft(row, col, boardSize);
        const tileSize = boardSize / 8;
        ctx.fillStyle = color;
        ctx.fillRect(x, y, tileSize, tileSize);
    },

    drawSelectionRing(ctx: CanvasRenderingContext2D, row: number, col: number, boardSize: number): void {
        const { x, y } = GridGeometry.matrixToTileTopLeft(row, col, boardSize);
        const tileSize = boardSize / 8;
        const padding = tileSize * 0.08; // Proportional 8% inner padding

        ctx.strokeStyle = 'rgba(96, 165, 250, 0.85)'; // Maps to design tokens
        ctx.lineWidth = Math.max(2, boardSize / 256);
        ctx.lineJoin = 'round';
        
        ctx.strokeRect(x + padding, y + padding, tileSize - padding * 2, tileSize - padding * 2);
        ctx.fillStyle = 'rgba(96, 165, 250, 0.08)';
        ctx.fillRect(x + padding, y + padding, tileSize - padding * 2, tileSize - padding * 2);
    },

    drawValidMoveIndicator(ctx: CanvasRenderingContext2D, row: number, col: number, isCapture: boolean, boardSize: number): void {
        const { x, y } = GridGeometry.matrixToPixels(row, col, boardSize);
        const tileSize = boardSize / 8;
        const scaleFactor = boardSize / 512;

        if (isCapture) {
            // Renders a target ring overlay when a tile is occupied by an enemy piece
            ctx.strokeStyle = 'rgba(248, 113, 113, 0.6)';
            ctx.lineWidth = 3 * scaleFactor;
            ctx.beginPath();
            ctx.arc(x, y, (tileSize / 2) - (6 * scaleFactor), 0, Math.PI * 2);
            ctx.stroke();
        } else {
            // Renders a simple centered dot on empty destinations
            ctx.fillStyle = 'rgba(16, 185, 129, 0.5)';
            ctx.beginPath();
            ctx.arc(x, y, 6 * scaleFactor, 0, Math.PI * 2);
            ctx.fill();
        }
    }
};

```

---

## 4.3 Declarative SVG Layout Vector Templates (`features/board/library/svgComponents.tsx`)

Cross-tile graphics—such as engine calculation arrows or complex paths—suffer from pixelation and high math complexity when rendered on canvas contexts across varying display densities. We replace them with sharp, responsive, vector-based TSX templates.

```tsx
import React from 'react';
import { Coordinate } from '../../../domain/types';
import { GridGeometry } from '../../../utils/gridGeometry';

interface SvgArrowProps {
    from: Coordinate;
    to: Coordinate;
    color: string;
    boardSize: number;
    index: number;
    total: number;
}

export const SvgArrow: React.FC<SvgArrowProps> = ({ from, to, color, boardSize, index, total }) => {
    const start = GridGeometry.matrixToPixels(from.row, from.col, boardSize);
    const end = GridGeometry.matrixToPixels(to.row, to.col, boardSize);
    
    const scale = boardSize / 512;
    const intensity = (1 - index / total) * 0.8 + 0.2; // Diminishes stroke thickness for lower-ranked choices
    const strokeWidth = 5 * scale * intensity;

    return (
        <g opacity={intensity}>
            {/* White backing path to provide contrast across dark and light board squares */}
            <line 
                x1={start.x} y1={start.y} x2={end.x} y2={end.y}
                stroke="white" strokeWidth={strokeWidth * 1.8} strokeLinecap="round"
            />
            {/* Core colored vector line */}
            <line 
                x1={start.x} y1={start.y} x2={end.x} y2={end.y}
                stroke={color} strokeWidth={strokeWidth} strokeLinecap="round"
            />
            {/* Joint anchor points */}
            <circle cx={start.x} cy={start.y} r={4 * scale} fill="white" />
            <circle cx={end.x} cy={end.y} r={4 * scale} fill={color} />
        </g>
    );
};

```

---

## 4.4 Phase 4 Implementation Steps

1. Create `src/features/game-module/utils/gridGeometry.ts` to implement the translation functions.
2. Build the `CanvasDrawers` file. Confirm that every operation scales dynamically via `boardSize` or explicit scale-factor variables.
3. Establish `svgComponents.tsx` to handle vector overlays. Ensure all stroke properties are scale-invariant.

---

When your graphics brushes and transformation math are verified, let's step into Section 5: Feature-Driven Rendering & Multi-Layer View Architecture.

---

# Section 5: Feature-Driven Rendering & Multi-Layer View Architecture

This section handles the structural migration of the visual layouts. We break down the monolithic interface containers into single-responsibility elements, splitting presentation concerns into a **Stacked Hybrid Board Assembly** and a **Modular HUD Feature Component Library**.

---

## 5.1 The Stacked Hybrid Board Composition Engine

To minimize memory overhead and context switching, we eliminate loose canvas layouts and merge them into a single, structured $8 \times 8$ presentation field. We stack three distinct technology layers inside `features/game-module/features/board/layers/`, utilizing each strictly for its architectural strengths.

### 1. The Unified Canvas Layer (`layers/UnifiedCanvasLayer.tsx`)

This layer handles low-level, non-interactive visual highlights (e.g., selection markers, legal move dots, capture rings). It processes all tracking arrays in a single rendering pass.

```typescript
import React, { useEffect, useRef } from 'react';
import { CanvasDrawers } from '../library/canvasDrawers';

export enum MarkerType {
    HIGHLIGHT = 'HIGHLIGHT',
    SELECTION = 'SELECTION',
    VALID_MOVE = 'VALID_MOVE',
    CAPTURE = 'CAPTURE'
}

export interface BoardMarker {
    type: MarkerType;
    row: number;
    col: number;
    color?: string;
}

interface UnifiedCanvasLayerProps {
    markers: BoardMarker[];
    boardSize: number;
}

export const UnifiedCanvasLayer: React.FC<UnifiedCanvasLayerProps> = ({ markers, boardSize }) => {
    const canvasRef = useRef<HTMLCanvasElement>(null);

    useEffect(() => {
        const canvas = canvasRef.current;
        if (!canvas) return;

        const ctx = canvas.getContext('2d');
        if (!ctx) return;

        CanvasDrawers.clear(ctx, boardSize);

        markers.forEach(marker => {
            switch (marker.type) {
                case MarkerType.HIGHLIGHT:
                    CanvasDrawers.drawTileHighlight(ctx, marker.row, marker.col, marker.color || 'rgba(251, 191, 36, 0.2)', boardSize);
                    break;
                case MarkerType.SELECTION:
                    CanvasDrawers.drawSelectionRing(ctx, marker.row, marker.col, boardSize);
                    break;
                case MarkerType.VALID_MOVE:
                    CanvasDrawers.drawValidMoveIndicator(ctx, marker.row, marker.col, false, boardSize);
                    break;
                case MarkerType.CAPTURE:
                    CanvasDrawers.drawValidMoveIndicator(ctx, marker.row, marker.col, true, boardSize);
                    break;
            }
        });
    }, [markers, boardSize]);

    return (
        <canvas
            ref={canvasRef}
            width={boardSize}
            height={boardSize}
            className="absolute top-0 left-0 pointer-events-none z-10"
        />
    );
};

```

### 2. The SVG Vector Layer (`layers/SvgVectorLayer.tsx`)

This layer stretches across the board container to render crisp, cross-tile strategic paths (like top AI choice arrows). It leverages the declarative vector layout components defined in Section 4.

### 3. The Piece HTML Layer (`layers/PieceHtmlLayer.tsx`)

Renders pieces within standard grid-aligned HTML containers. This keeps game assets inside DOM nodes, allowing the app to retain full support for hardware-accelerated transitions, custom CSS dragging filters, and web touch APIs.

---

## 5.2 The Pointer-Event Cross-Platform Interaction Hook (`board/hooks/useBoardInteractions.ts`)

To support mouse drags on desktop and fluid touch interactions on mobile devices without fracturing your code, we implement a unified input stream.

This hook uses standard W3C **Pointer Events**, which map mouse clicks and finger touches onto a single API. It encapsulates high-frequency layout changes (like tracking screen positions during active drags), isolating them so they don't cause unrelated HUD dashboard elements to re-render.

```typescript
import { useState, useCallback } from 'react';
import { Coordinate } from '../../../domain/types';

interface InteractionProps {
    onMoveAttempt: (from: Coordinate, to: Coordinate) => void;
    onSquareClick: (coords: Coordinate) => void;
    autoPlayers: boolean[];
    currentPlayer: number;
}

export const useBoardInteractions = ({ onMoveAttempt, onSquareClick, autoPlayers, currentPlayer }: InteractionProps) => {
    const [draggedPiece, setDraggedPiece] = useState<string | null>(null);
    const [dragOrigin, setDragOrigin] = useState<Coordinate | null>(null);
    const [pointerPosition, setPointerPosition] = useState({ x: 0, y: 0 });

    const handlePointerDown = useCallback((coords: Coordinate, piece: string, clientX: number, clientY: number) => {
        if (autoPlayers[currentPlayer]) return;
        
        setDraggedPiece(piece);
        setDragOrigin(coords);
        setPointerPosition({ x: clientX, y: clientY });
        onSquareClick(coords);
    }, [autoPlayers, currentPlayer, onSquareClick]);

    const handlePointerMove = useCallback((clientX: number, clientY: number) => {
        if (!draggedPiece) return;
        setPointerPosition({ x: clientX, y: clientY });
    }, [draggedPiece]);

    const handlePointerUp = useCallback((targetCoords: Coordinate | null) => {
        if (draggedPiece && dragOrigin && targetCoords) {
            onMoveAttempt(dragOrigin, targetCoords);
        }
        setDraggedPiece(null);
        setDragOrigin(null);
    }, [draggedPiece, dragOrigin, onMoveAttempt]);

    return {
        draggedPiece,
        pointerPosition,
        handlePointerDown,
        handlePointerMove,
        handlePointerUp
    };
};

```

---

## 5.3 The Modular HUD Grid Library

The original `HUD.jsx` layout is broken down into isolated, single-responsibility components inside `features/game-module/features/hud/components/`.

* **`EvalBar.tsx` (NEW):** Displays a real-time ratio panel comparing team strengths. It receives evaluation numbers directly from the background service client context and sets layout proportions smoothly using a CSS custom property.
* **`GameTimer.tsx` (NEW):** Contains its own internal interval loop that ticks down the current player's clock budget. This keeps frequent clock state updates completely contained within this individual sub-module.
* **`MoveHistoryList.tsx` (NEW):** Processes the historical snapshots array to generate a standard, interactive move ledger. Clicking any move link calls an explicit frame travel index modification directly in the state manager.
* **`GameConclusionPanel.tsx` (NEW):** A clean layout container that displays overlay modals when a match ends (e.g., checkmate confirmations, draw flags, timer forfeitures), blocking all physical piece actions on the grid.

---

When your interface modules and pointer hooks are verified, let's step into the final chapter, **Section 6: View Orchestration & Component Assembly**.

---

# Section 6: View Orchestration & Component Assembly

This final section details the compilation of the entire system. We establish the root **Context Provider** to act as our explicit command engine, structure the **Mode View Mount Layouts** to swap interfaces cleanly based on active feature flags, and implement resource destruction lifecycles to guarantee zero memory leaks.

---

## 6.1 The Unified `GameContext` State Machine (`context/GameContext.tsx`)

This centralized React Context acts as the primary coordinator for the application. It manages history timelines, monitors active feature configuration parameters, and triggers the infrastructure worker client via the explicit, imperative commands defined in Section 3.

---

## 6.2 Feature View Mount Layouts (`features/modes/`)

Rather than maintaining a single workspace layout file filled with complicated layout logic, views are separated into individual, clean assembly wrappers that construct layouts depending entirely on the configuration flags.

### The Structured Assembly Component (`features/modes/AnalysisView.tsx`)

This layout is mounted exclusively when the system operates under the `ANALYSIS` rule parameters. It integrates the live evaluation ratios and move ledger bars while skipping game elements like rigid match timers.

```tsx
import React from 'react';
import { useGame } from '../../context/GameContext';
import { UnifiedCanvasLayer } from '../board/layers/UnifiedCanvasLayer';
import { SvgVectorLayer } from '../board/layers/SvgVectorLayer';
import { EvalBar } from '../hud/components/EvalBar';
import { MoveHistoryList } from '../hud/components/MoveHistoryList';

export const AnalysisView: React.FC<{ boardSize: number }> = ({ boardSize }) => {
    const { liveEval } = useGame();

    return (
        <div className="w-full flex flex-col lg:flex-row gap-6 items-center justify-center p-4">
            {/* Mounted exclusively because enableLiveEval configuration flag parameters are active */}
            <EvalBar progress={liveEval} height={boardSize} />

            <div className="relative shadow-2xl rounded-lg overflow-hidden bg-surface-card border border-border-muted" style={{ width: boardSize, height: boardSize }}>
                <UnifiedCanvasLayer markers={[]} boardSize={boardSize} />
                <SvgVectorLayer moves={liveEval?.candidates || []} showAssist={true} boardSize={boardSize} />
            </div>

            <div className="w-full max-w-sm lg:w-80 shrink-0 bg-hud-bg backdrop-blur-md border border-hud-border p-5 rounded-xl">
                <MoveHistoryList />
            </div>
        </div>
    );
};

```

---

## 6.3 Module Root Initialization & Lifespan Guardrails

The `GameModuleRoot.tsx` serves as the entry gate exposed to your upper global shell (`pages/GamePage.tsx`). It wraps the subsystem inside our context provider, monitors layout resizing behaviors, and routes to the correct view manager.

### Key Implementation Guidelines

* **Responsive Snapping Loop:** Maintain the `ResizeObserver` script exactly as outlined in Section 5. Ensure width sizes snap explicitly to clean multiples of 8 (`Math.floor(width / 8) * 8`) before updating state to guarantee pixel-perfect rendering across layers.
* **Component-Level Unmounts:** When switching modes (e.g., exiting an AnalysisView and entering a strict PlayView), verify that the underlying context completely recycles itself. This fires the `.terminate()` destructor class command, preventing memory leaks from hidden web worker threads.

---

### Replatforming Complete

You have successfully generated the architecture roadmap for your soft-rewrite. The complete subsystem—including type contracts, immutable timeline engines, thread client bridges, scale-invariant vector brushes, and decoupled features—is organized to ensure a clean transition from your old code repository into a scalable layout system.