## 1. Component Signatures

### `src/components/GameController.jsx`

* **`GameController()`**
* *Inputs:* None (Top-level application entry view orchestrator).



### `src/components/BoardContainer/BoardContainer.jsx`

* **`BoardContainer({ boardState, showAssist, assistMoves, onMoveAttempt })`**
* *`boardState`:* 2D Matrix Array ($8 \times 8$).
* *`showAssist`:* Boolean toggle for overlay visibility.
* *`assistMoves`:* Array of calculated vector move objects.
* *`onMoveAttempt`:* Callback function `(fromCoords, toCoords) => void`.



### `src/components/BoardContainer/BackgroundCanvas.jsx`

* **`BackgroundCanvas()`**
* *Inputs:* None (Renders static checkerboard grid onto Layer 1).



### `src/components/BoardContainer/AssistCanvas.jsx`

* **`AssistCanvas({ moves, showAssist })`**
* *`moves`:* Array of active AI suggestion route objects.
* *`showAssist`:* Boolean overlay toggle indicator.



### `src/components/BoardContainer/PieceLayer.jsx`

* **`PieceLayer({ boardState })`**
* *`boardState`:* The current $8 \times 8$ raw array representing cell data.



### `src/components/BoardContainer/Piece.jsx`

* **`Piece({ type, row, col })`**
* *`type`:* String identifier character (`'W'` or `'B'`).
* *`row`:* Integer matrix row target index (`0-7`).
* *`col`:* Integer matrix column target index (`0-7`).



### `src/components/DragOverlay.jsx`

* **`DragOverlay({ isDragging, type, mousePos })`**
* *`isDragging`:* Boolean flag indicating an active mouse drag sequence.
* *`type`:* String identifier character of the floating asset (`'W'` or `'B'`).
* *`mousePos`:* Vector object tracking active screen offsets `{ x, y }`.



### `src/components/HUD/HUD.jsx`

* **`HUD({ currentPlayer, gameEnded, showAssist, showControl, autoPlayers, assistMoves, onToggleAssist, onToggleControl, onToggleAuto })`**
* *`currentPlayer`:* Integer identifying active player turn status (`0` or `1`).
* *`gameEnded`:* Boolean game loop flag.
* *`showAssist` / `showControl`:* Boolean layer rendering switches.
* *`autoPlayers`:* Array tracking player automation configurations `[bool, bool]`.
* *`assistMoves`:* Pure reference tracking AI evaluation profiles.
* *`onToggleAssist` / `onToggleControl`:* UI event click signals `() => void`.
* *`onToggleAuto`:* Positional modifier callback execution `(playerIndex) => void`.



### `src/components/HUD/TurnIndicator.jsx`

* **`TurnIndicator({ currentPlayer, gameEnded })`**
* *`currentPlayer`:* Integer player tracker code (`0` or `1`).
* *`gameEnded`:* Game status conditional boolean.



### `src/components/HUD/ControlToggle.jsx`

* **`ControlToggle({ label, isActive, onClick, hotkeyHint })`**
* *`label`:* Text string displaying operational toggle target.
* *`isActive`:* Current operational setting boolean state indicator.
* *`onClick`:* Formulated UI click callback `() => void`.
* *`hotkeyHint`:* Text label identifying associated keyboard shortcut modifier.



### `src/components/HUD/MoveScorer.jsx`

* **`MoveScorer({ moves, showAssist })`**
* *`moves`:* Evaluated AI coordinate array data models.
* *`showAssist`:* View filter configuration visibility toggle.



---

## 2. Custom Hooks Architecture

### `src/hooks/useHotkeys.js`

* **`useHotkeys(keyMap)`**
* *`keyMap`:* Dynamic dictionary mapping keyboard parameters straight to operational callbacks `{ [keyString]: (event) => void }`.



### `src/hooks/useGameState.js`

* **`useGameState()`**
* *Inputs:* None.
* *Returns:* An ecosystem management object containing:
* `board`: 2D Matrix Array.
* `currentPlayer`: Integer (`0` or `1`).
* `gameEnded`: Boolean.
* `showAssist` / `showControl`: Booleans.
* `autoPlayers`: Boolean Array `[p0, p1]`.
* `assistMoves`: Array of mock scoring profiles.
* `setShowAssist` / `setShowControl`: React dispatcher functions.
* `executeMove`: Execution method `(from, to) => boolean`.
* `resetGame`: Restoration method `() => void`.
* `togglePlayerAuto`: Setting modifier method `(playerIndex) => void`.





---

## 3. Utilities & Data Modules

### `src/utils/boardMatrix.js` (`BoardMatrix` API namespace)

* **`createEmpty()`** $\rightarrow$ Returns empty $8 \times 8$ matrix grid block.
* **`createInitialPosition()`** $\rightarrow$ Returns configured initial simulation array layout.
* **`getPiece(grid, row, col)`** $\rightarrow$ Inputs: Array, Int, Int. Returns piece string/null.
* **`setPiece(grid, row, col, piece)`** $\rightarrow$ Inputs: Array, Int, Int, String. Returns a new immutable copy of the grid.
* **`clone(grid)`** $\rightarrow$ Inputs: Array. Returns a shallow matrix copy.

### `src/utils/boardGeometry.js` (`BoardGeometry` API namespace)

* **`matrixToPixels(row, col)`** $\rightarrow$ Inputs: Int, Int. Returns center tile pixel values `{ x, y }`.
* **`matrixToTileTopLeft(row, col)`** $\rightarrow$ Inputs: Int, Int. Returns top-left tile boundary limits `{ x, y }`.
* **`pixelsToMatrix(x, y)`** $\rightarrow$ Inputs: Float, Float. Returns snapped row and column matrix indexes `{ row, col }`.
* **`matrixToAlgebraic(row, col)`** $\rightarrow$ Inputs: Int, Int. Returns string representation (e.g., `"E4"`).

### `src/utils/canvasDrawers.js` (`CanvasDrawers` API namespace)

* **`clear(ctx, size)`** $\rightarrow$ Inputs: `CanvasRenderingContext2D`, Int.
* **`drawVectorLine(ctx, fromPixels, toPixels, color)`** $\rightarrow$ Inputs: Context, Object `{x,y}`, Object `{x,y}`, String.
* **`drawAnchorNode(ctx, pixels, color)`** $\rightarrow$ Inputs: Context, Object `{x,y}`, String.
* **`renderAssistOverlay(ctx, moves, displayLimit)`** $\rightarrow$ Inputs: Context, Array, Int.

### `src/utils/engineAdapterMock.js` (`EngineAdapterMock` API namespace)

* **`getInitialBoard()`** $\rightarrow$ Returns initial initialization layout arrangement array.
* **`isValidMove(board, from, to)`** $\rightarrow$ Inputs: Array, Object `{row,col}`, Object `{row,col}`. Returns Boolean.
* **`getMockAIAssistMoves(currentPlayer)`** $\rightarrow$ Inputs: Int. Returns static candidate score arrays.