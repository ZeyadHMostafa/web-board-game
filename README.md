# Modern Orbital Abstract BoardGame

A high-performance, polyglot monorepo implementation of an abstract strategy board game. This architecture features a core deterministic rules engine, AI decision logic, and simulation layers written in Rust, compiled down to WebAssembly for a fluid, fully client-side React frontend, and exposed via C-extensions to a Python machine learning pipeline.

---

## Technical Architecture & Monorepo Layout

The project is structured as a unified monorepo coordinating Rust, Node.js/Vite, and Python environments.

```text
.
├── apps/
│   ├── web/               # React / Vite frontend application
│   └── tui/               # Terminal User Interface for native play/debugging
├── packages/
│   ├── core-engine/       # Rust core (Rules, AI, Search, Wasm/Python Bindings)
│   └── python-ml/         # Python training pipeline (PyTorch/ML models)
└── docker/                # Isolated multi-stage build environments

```

### High-Level Domain Isolation

* **Pure Game Rules (`src/rules/`):** A zero-dependency, deterministic representation of the game board using high-performance integer bitmasks (Bitboards) for ultra-fast, constant-time state manipulation.
* **The Decision Engine (`src/ai/`):** Combines static board evaluation matrices (including dot-product vector models) with a depth-first horizon **Negamax** tree search and a prunable **Transposition Table** cache.
* **The Sandbox (`src/simulation/`):** Runs automated, end-to-end match cycles multi-threaded across CPU cores via `rayon` to harvest training datasets.
* **The Polyglot API Gateway (`src/bindings/`):** Maps internal Rust types directly to memory layouts consumable as native JavaScript objects via `wasm-bindgen` and Python tensors via `pyo3`/`numpy`.

---

## Gameplay Mechanics

### Win Condition

A player wins when the opposing player has zero legal moves available on their turn.

### Movement Variants

Each turn, a player can move a friendly piece to an empty tile or onto an enemy-occupied tile (capturing it). Moves fall into two distinct geometric categories:

#### 1. Diagonal Moves

A piece may jump over a continuous chain of one or more friendly pieces along a strict diagonal vector.

* Single-step diagonal slides are invalid.
* Intermediary squares must contain friendly pieces.

#### 2. Circular (Pivotal) Moves

A piece orthogonally adjacent to another friendly piece (the pivot) can swing 90, 180, or 270 degrees in either direction into a new orthogonally adjacent position around that same pivot.

* The sweep path must be physically clear.
* The inner diagonal corner being crossed during the arc swing cannot be blocked by any piece.

### Auxiliary Constraints

* There are no draw conditions; the game must resolve decisively.
* Moving back and forth to repeat identical board states results in th first player repeating a board position to lose the game. and is thus considered forbidden
* this however has not been implemented yet

---

## Frontend Integration & Interaction

The `apps/web/` workspace builds an optimized user interface using standard React patterns alongside performance-optimized HTML5 Canvases to prevent unnecessary DOM reflows:

* **GameController:** The structural shell coordinating user inputs, hotkeys, dragging visual layers, and sizing calculations.
* **useGameState & useWorkerOrchestrator:** Handles local UI state hooks while offloading deep AI search tasks to a background browser **Web Worker** (`aiWorker.js`), ensuring the main thread never drops frames.
* **Layered HTML5 Canvases:** Offloads intensive background grid rendering, custom vector assist paths, control weights, and selection markers to specialized independent rendering layers.

### Mouse & Touch Interactions

* **Click to Select:** Click a friendly piece to select it and highlight its entire legal matrix options. Click an authorized dot indicator to complete the movement.
* **Drag and Drop:** Press and hold a piece to automatically reveal its options, dragging it directly onto a target tile to execute.

### Hotkeys

* **F2** : Toggle AI Assist Overlay (Visualizes top recommended paths and scoring metrics)
* **F3** : Toggle Control Overlay (Displays active grid balance weights; White = positive, Black = negative)
* **Q** : Toggle Automation Engine for Player 1 (White)
* **A** : Toggle Automation Engine for Player 2 (Black)
* **R** : Hard reset the entire board history buffer
* **Left Arrow** : Step backward one turn in the timeline history
* **Right Arrow** : Step forward one turn in the timeline history

---

## Installation and Setup

### Prerequisites

* [Node.js](https://nodejs.org/) (v18+ recommended)
* [Rust Toolchain](https://rustup.rs/) (Stable Only if rebuilding webassembly)
* [wasm-pack](https://www.google.com/search?q=https://rustwasm.github.io/wasm-pack/installer/) (`cargo install wasm-pack`)

### 1. Compile the WebAssembly Core

From the monorepo root, compile the Rust engine into a native ES module tailored for the web workspace:

```shell
npm run wasm:build

```

*Note: The pre-compiled WebAssembly artifact may already be available.*

### 2. Local Web Development

Navigate to the web workspace, install frontend dependencies, and launch the Vite compilation pipeline:

```shell
cd apps/web
npm install
npm run web:dev

```

### 3. Production UI Build

Compile the highly optimized production distribution package:

```shell
npm run web:build

```

---

## Project Legacy & Port Background

The original iteration of this strategy game paired a legacy C++ simulation engine backend (`board_cpp.dll`) with a heavy Python desktop GUI wrapper to handle gameplay loops and execute depth-first search (DFS) with alpha-beta pruning.

This modern architecture completely supersedes that model by porting the entire system to WebAssembly and React. It updates the core concepts for the modern web ecosystem:

* **C++ to Safe Rust:** The unsafe, platform-dependent C++ `.dll` has been rewritten into reliable, thread-safe, cross-platform Rust with targetable feature gates.
* **Desktop Python to Web Worker Orchestration:** Instead of a blocking desktop GUI application, calculations run asynchronously inside isolated web threads, bringing full-speed algorithmic search straight to standard browser runtimes.
* **Native Bitwise Performance:** Leverages hardware-level bitwise operations natively inside the browser via Wasm, achieving calculation parity with bare-metal compiled code.