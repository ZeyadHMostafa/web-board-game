# Modern Orbital Chess (React Port)

A web-based, interactive implementation of the classic abstract strategy board game. This project ports a legacy architecture originally featuring a C++ backend and Python GUI into a high-performance, fully client-side React and HTML5 Canvas application. It includes real-time rule validation, dynamic control-map overlays, timeline history navigation, and heuristic AI automation.

## Project Background

The original iteration of this game paired a C++ simulation engine (`board_cpp.dll`) with a Python GUI wrapper to handle gameplay and execute depth-first search (DFS) with pruning algorithms. 

This React architecture completely modularizes those core concepts for the modern web:
* **Decoupled State Hook:** Manages non-destructive timeline indexing, history tracking, and automation loops.
* **Layered HTML5 Canvases:** Offloads intensive background grid, custom vector assist paths, control weights, and selection markers to specialized independent rendering layers.
* **Deterministic Rule Engine:** Re-implements the unique diagonal jumping and pivotal sliding physics cleanly in modern JavaScript.

---

## Gameplay Mechanics

### Win Condition
A player wins when the opposing player has zero legal moves available on their turn.

### Movement Variants
Each turn, a player can move a friendly piece to an empty tile or onto an enemy-occupied tile (capturing it and removing it from play). Moves fall into two distinct geometric categories:

#### 1. Diagonal Moves
A piece may jump over a continuous chain of one or more friendly pieces along a strict diagonal vector. 
* Single-step diagonal slides are invalid.
* Intermediary squares must contain friendly pieces.

#### 2. Circular (Pivotal) Moves
A piece orthogonally adjacent to another friendly piece (the pivot) can swing 90, 180, or 270 degrees into a new orthogonally adjacent position around that same pivot.
* The sweep path must be physically clear. 
* The inner diagonal corner being crossed during the arc swing cannot be blocked by any piece.

### Auxiliary Constraints
* There are no draw conditions; the game must resolve decisively.
* Moving back and forth to repeat identical board states is structurally prohibited by the rules engine.

---

## Technical Architecture

The frontend is built using standard React patterns alongside performance-optimized HTML5 Canvases to prevent unnecessary DOM reflows:

* **GameController:** The structural shell coordinating user inputs, hotkeys, dragging visual layers, and sizing calculations.
* **useGameState:** Custom React hook managing the underlying state machines, historical undo/redo buffers, and procedural timers.
* **EngineAdapterMock:** Evaluates spatial vectors, computes matrix safety invariants, and scores procedural AI recommendations.
* **CanvasDrawers:** Pure utility file containing context blueprints for painting vector paths, text positioning, and alpha-blended grid cells.

---

## Interaction and Controls

The interface simultaneously supports seamless desktop Drag-and-Drop functionality and accurate Mobile Click-to-Select mechanics.

### Mouse Interactions
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

Ensure you have Node.js installed on your machine.
Here is the corrected, cleanly formatted installation and setup section:

---

## Installation and Setup

Ensure you have Node.js installed on your machine.

1. Clone the repository and navigate to the project directory:
```shell
cd board-game-web
```

2. Install the necessary development dependencies:

```shell
npm install
```

3. Spin up the local development compilation pipeline:
```shell
npm run dev
```

4. Build the optimized production distribution package:
```shell
npm run build
```