use std::sync::Arc;
use std::sync::RwLock;
use std::sync::atomic::AtomicBool;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

use crate::rules::state::{GameState, Player};
use crate::rules::moves::{Move, MoveList};
use crate::ai::search::SearchContext;
use crate::ai::search::controllers::IterativeDeepeningController;
use crate::ai::search::selector::{ActionSelector, SelectorMode, Difficulty};
use crate::ai::PositionEvaluator;
use crate::ai::EvaluationScore;

// ============================================================================
// SERDE DATA TRANSFER STRUCTURES FOR JAVASCRIPT EXCHANGES
// ============================================================================

#[derive(Serialize, Deserialize)]
pub struct WasmGameState {
    pub p1_pieces: u64,
    pub p2_pieces: u64,
    pub active_player: String,
}

#[derive(Serialize, Deserialize)]
pub struct WasmMove {
    pub from_square: u8,
    pub to_square: u8,
    pub is_capture: bool,
}

#[derive(Serialize, Deserialize)]
pub struct WasmScoredMove {
    pub current_move: WasmMove,
    pub score_value: i32,
    pub score_label: String,
}

#[derive(Serialize, Deserialize)]
pub struct WasmSearchProgress {
    pub candidates: Vec<WasmScoredMove>,
    pub depth_reached: usize,
    pub nodes_explored: usize,
    pub branching_factor: f64,
    pub pv: Vec<WasmMove>,
}

// ============================================================================
// SYSTEM ARCHITECTURE WASM EXPORTS
// ============================================================================

#[wasm_bindgen]
pub struct WasmEngine;

#[wasm_bindgen]
impl WasmEngine {
    /// Evaluates the list of valid legal move choices available for the active state layout.
    pub fn generate_legal_moves(js_state: JsValue) -> Result<JsValue, JsValue> {
        let state: GameState = self::parse_game_state(js_state)?;
        let legal_moves: MoveList = state.generate_legal_moves();

        let response: Vec<WasmMove> = legal_moves
            .into_iter()
            .map(|m| WasmMove {
                from_square: m.from_square(),
                to_square: m.to_square(),
                is_capture: m.is_capture(),
            })
            .collect();

        serde_wasm_bindgen::to_value(&response)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Mutates the state layout by playing a specified move coordinates choice.
    pub fn make_move(js_state: JsValue, js_move: JsValue) -> Result<JsValue, JsValue> {
        let mut state: GameState = self::parse_game_state(js_state)?;
        let m: Move = self::parse_move_primitive(js_move)?;

        state.make_move(m);

        let response = WasmGameState {
            p1_pieces: *state.p1_pieces,
            p2_pieces: *state.p2_pieces,
            active_player: match state.active_player {
                Player::P1 => "P1".to_string(),
                Player::P2 => "P2".to_string(),
            },
        };

        serde_wasm_bindgen::to_value(&response)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Executes iterative deep search loops on a separate worker background scope sequence thread.
    pub fn compute_ai_move(
        js_state: JsValue,
        min_depth: usize,
        max_depth: usize,
        temp: f32,
        b_thresh: i32,
    ) -> Result<JsValue, JsValue> {
        let state: GameState = self::parse_game_state(js_state)?;
        
        let cancelled = AtomicBool::new(false);
        let evaluator = DefaultPositionEvaluator::new();
        let ctx = SearchContext {
            evaluator: &evaluator,
            cancelled: &cancelled,
        };

        let progress = Arc::new(RwLock::new(crate::ai::search::SearchProgress {
            candidates: Vec::new(),
            depth_reached: 0,
            nodes_explored: 0,
            branching_factor: 0.0,
            pv: Vec::new(),
        }));

        let controller = IterativeDeepeningController::new(
            &ctx,
            min_depth,
            max_depth,
            progress.clone(),
        );

        controller.search(&state);

        let final_progress = progress.read().unwrap();
        let difficulty = Difficulty { temp, b_thresh };
        
        let chosen_move = ActionSelector::select_move(
            &final_progress,
            SelectorMode::AdaptiveDifficulty(difficulty),
        ).ok_or_else(|| JsValue::from_str("Engine failed to resolve valid move selection targets"))?;

        let response = WasmMove {
            from_square: chosen_move.from_square(),
            to_square: chosen_move.to_square(),
            is_capture: chosen_move.is_capture(),
        };

        serde_wasm_bindgen::to_value(&response)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Evaluates the target position layer, returning scores and metrics for UI components.
    pub fn compute_evaluation_progress(
        js_state: JsValue,
        min_depth: usize,
        max_depth: usize,
    ) -> Result<JsValue, JsValue> {
        let state: GameState = self::parse_game_state(js_state)?;
        
        let cancelled = AtomicBool::new(false);
        let evaluator = DefaultPositionEvaluator::new();
        let ctx = SearchContext {
            evaluator: &evaluator,
            cancelled: &cancelled,
        };

        let progress = Arc::new(RwLock::new(crate::ai::search::SearchProgress {
            candidates: Vec::new(),
            depth_reached: 0,
            nodes_explored: 0,
            branching_factor: 0.0,
            pv: Vec::new(),
        }));

        let controller = IterativeDeepeningController::new(
            &ctx,
            min_depth,
            max_depth,
            progress.clone(),
        );

        controller.search(&state);

        let final_progress = progress.read().unwrap();
        
        let candidates: Vec<WasmScoredMove> = final_progress.candidates
            .iter()
            .map(|c| WasmScoredMove {
                current_move: WasmMove {
                    from_square: c.current_move.from_square(),
                    to_square: c.current_move.to_square(),
                    is_capture: c.current_move.is_capture(),
                },
                score_value: self::extract_score_scalar(c.score),
                score_label: format!("{:?}", c.score),
            })
            .collect();

        let pv: Vec<WasmMove> = final_progress.pv
            .iter()
            .map(|m| WasmMove {
                from_square: m.from_square(),
                to_square: m.to_square(),
                is_capture: m.is_capture(),
            })
            .collect();

        let response = WasmSearchProgress {
            candidates,
            depth_reached: final_progress.depth_reached,
            nodes_explored: final_progress.nodes_explored,
            branching_factor: final_progress.branching_factor,
            pv,
        };

        serde_wasm_bindgen::to_value(&response)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

// ============================================================================
// LOW-LEVEL INLINE INTERNAL ASSISTANTS
// ============================================================================

/// Converts a JavaScript-allocated generic object into native Rust game state entities.
#[inline(always)]
fn parse_game_state(raw: JsValue) -> Result<GameState, JsValue> {
    let raw_state: WasmGameState = serde_wasm_bindgen::from_value(raw)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let active_player = match raw_state.active_player.as_str() {
        "P1" => Player::P1,
        "P2" => Player::P2,
        _ => return Err(JsValue::from_str("Invalid player configuration identifier")),
    };

    Ok(GameState::new(
        raw_state.p1_pieces,
        raw_state.p2_pieces,
        active_player,
    ))
}

/// Converts incoming JS coordinates parameters into localized bitboard Move wrappers.
#[inline(always)]
fn parse_move_primitive(raw: JsValue) -> Result<Move, JsValue> {
    let raw_move: WasmMove = serde_wasm_bindgen::from_value(raw)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Structural packing matching engine definitions
    Ok(Move::new(
        raw_move.from_square,
        raw_move.to_square,
        if raw_move.is_capture {1} else {0},
    ))
}

/// Normalizes specialized match tracking enum variants into signed scalar dimensions.
#[inline(always)]
fn extract_score_scalar(score: EvaluationScore) -> i32 {
    match score {
        EvaluationScore::Value(v) => v,
        EvaluationScore::Mating(ply) => i32::MAX - (ply as i32),
        EvaluationScore::Mated(ply) => i32::MIN + (ply as i32),
    }
}

/// Internal stub type implementation satisfying interface dynamic trait requirements.
struct DefaultPositionEvaluator;

impl DefaultPositionEvaluator {
    #[inline(always)]
    const fn new() -> Self {
        Self
    }
}

impl PositionEvaluator for DefaultPositionEvaluator {
    #[inline(always)]
    fn evaluate(&self, state: &GameState) -> EvaluationScore {
        let p1_count = state.p1_pieces.count_ones() as i32;
        let p2_count = state.p2_pieces.count_ones() as i32;
        
        let score_diff = match state.active_player {
            Player::P1 => p1_count - p2_count,
            Player::P2 => p2_count - p1_count,
        };
        
        EvaluationScore::Value(score_diff * 100)
    }
}