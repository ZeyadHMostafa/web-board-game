# Core Engine Architecture Overview

The `core-engine` architecture follows a decoupled, idiomatically separated layout. It cleanly isolates pure game rules, AI evaluation/search states, simulation runtime engines, and target environment bindings.

---

## High-Level Architectural Layout

```text
src/
├── rules/       # The Source of Truth (Pure, stateless game logic)
├── ai/          # The Decision Engine (Heuristics, search trees, and agents)
├── simulation/  # The Sandbox (Running games in parallel for training/testing)
├── bindings/    # The Polyglot Layer (Exposing the engine to WebAssembly and Python)
└── luts/        # The Optimization Layer (Look-Up Tables for constant-time operations)

```

---

## Core Modules & Philosophies

### 1. The Rules Domain (`src/rules/`)

The foundational layer of the engine, containing zero dependencies on AIs, WebAssembly, or Python. It serves as a deterministic, pure representation of the game board.

* **State Management (`state/`):** Utilizes high-performance bitboard representations (`bitboard.rs`), packing piece placements, occupancies, and movement masks into raw integer bitmasks processed via bitwise operations. This is bundled with active turns, move history, and clock metadata (`game.rs`).
* **Move Generation (`moves/`):** Encapsulates the legal move matrix. It defines strict move primitives (`structs.rs`) and provides the logic to aggregate, calculate, and filter valid moves for any given board state (`aggregation.rs`).

### 2. The AI Domain (`src/ai/`)

This layer consumes the `rules` domain to analyze board states and execute decision tree operations. It separates static evaluation from horizon searching.

* **Heuristics (`heuristics/`):** Evaluates static board states without lookahead depth, scoring structural advantages such as material balance, positional weightings, and safety matrices.
* **Models (`models/`):** Implements specialized evaluation structures like dot-product vector weights (`static_dot/`). This allows the machine learning pipeline to train weights and inject them directly into the Rust evaluation path.
* **Search Execution (`search/`):** Drives tactical depth logic. It features a recursive **Negamax** tree search implementation, a highly optimized **Transposition Table** to cache and prune duplicate game state subtrees, and an **Iterative Deepening Controller** to handle time-boxed search management.

### 3. The Simulation Domain (`src/simulation/`)

Responsible for running end-to-end games independent of external interfaces.

* **Environment Execution (`environment.rs`):** Implements the localized arena where discrete agents run automated match cycles.
* **Parallel Execution (`parallel.rs`):** Leverages multi-threaded processing via `rayon` to execute thousands of simultaneous simulations, acting as the primary engine for dataset harvesting and automated validation.

### 4. The Bindings Domain (`src/bindings/`)

Acts as a polyglot API gateway. To maintain a pristine core logic layer, this module isolates ecosystem-specific type-casting and serialization away from the core types.

* **WebAssembly (`wasm.rs`):** Uses `wasm-bindgen` and `serde-wasm-bindgen` to safely serialize engine outputs into fast JavaScript objects, serving the frontend workspace.
* **Python (`python.rs`):** Leverages `pyo3` and `numpy` to map native memory matrices directly into Python machine learning pipelines.

### 5. Look-Up Tables (`src/luts/`)

An optimization layer focused on compute reduction. It handles the precomputation of heavy mathematical structures, sliding piece attack paths, and lookup masks during initialization or build cycles. This substitutes complex runtime algorithms with fast, constant-time array access operations.