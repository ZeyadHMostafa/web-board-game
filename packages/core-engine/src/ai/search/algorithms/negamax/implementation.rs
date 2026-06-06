use std::sync::atomic::Ordering;
use crate::ai::search::algorithms::negamax::{SearchFrame, StepResult};
use crate::ai::search::{SearchContext, SearchTelemetry};
use crate::ai::search::utils::{HashEntryBounds, TranspositionTable, invert_score};
use crate::ai::heuristics::evaluators;
use crate::rules::moves::{Move, MoveList};
use crate::rules::state::GameState;
use crate::ai::evaluator::EvaluationScore;

// ============================================================================
// LOW-LEVEL INLINE SEARCH ASSISTANTS
// ============================================================================

/// Generates legal moves for the active player and sorts them in-place using 
/// material and positional weights to maximize alpha-beta pruning efficiency.
#[inline(always)]
pub(crate) fn generate_and_order_moves(state: &GameState, remaining_depth: usize) -> MoveList {
    let mut legal_moves = state.generate_legal_moves();
    if !legal_moves.is_empty() && remaining_depth > 1 {
        let allied_pieces = state.get_player_pieces(state.active_player);
        let enemy_pieces = state.get_player_pieces(state.active_player.opponent());
        legal_moves.sort_unstable_by_key(|m| {
            evaluators::evaluate_move(m, allied_pieces, enemy_pieces)
        });
    }
    legal_moves
}

/// Probes the transposition table cache to determine if an evaluation score 
/// for the current board state can immediately short-circuit the search node.
#[inline(always)]
pub(crate) fn evaluate_tt_cache(
    state: &GameState,
    tt: &TranspositionTable,
    remaining_depth: usize,
    alpha: EvaluationScore,
    beta: EvaluationScore,
) -> Option<EvaluationScore> {
    let entry = tt.lookup(state)?;
    if entry.depth < remaining_depth {
        return None;
    }

    match entry.bounds {
        HashEntryBounds::Exact => Some(entry.score),
        HashEntryBounds::Lower if entry.score >= beta => Some(entry.score),
        HashEntryBounds::Upper if entry.score <= alpha => Some(entry.score),
        _ => None,
    }
}

/// Checks if the node matches terminal rules (checkmate/stalemate or depth
/// limit hit). Returns the definitive evaluation score if terminal.
#[inline(always)]
pub(crate) fn evaluate_node_bounds(
    state: &GameState,
    ctx: &SearchContext,
    legal_moves: &MoveList,
    remaining_depth: usize,
) -> Option<EvaluationScore> {
    if legal_moves.is_empty() {
        return Some(EvaluationScore::Mated(0));
    }
    if remaining_depth == 0 {
        return Some(ctx.evaluator.evaluate(state));
    }
    None
}

/// Compiles a updated Principal Variation (PV) sequence by placing the 
/// leading move at the front of the downstream sequence path.
#[inline(always)]
pub(crate) fn compile_pv_line(leading_move: Move, child_pv: &[Move]) -> Vec<Move> {
    let mut new_pv = Vec::with_capacity(child_pv.len() + 1);
    new_pv.push(leading_move);
    new_pv.extend_from_slice(child_pv);
    new_pv
}

/// Computes structural bounds classification flags before preserving 
/// positional data evaluations into the transposition table.
#[inline(always)]
pub(crate) fn commit_node_to_tt(
    state: &GameState,
    tt: &mut TranspositionTable,
    max_score: EvaluationScore,
    original_alpha: EvaluationScore,
    beta: EvaluationScore,
    remaining_depth: usize,
) {
    let bounds = if max_score <= original_alpha {
        HashEntryBounds::Upper
    } else if max_score >= beta {
        HashEntryBounds::Lower
    } else {
        HashEntryBounds::Exact
    };
    tt.store(state, max_score, remaining_depth, bounds);
}

pub struct NegamaxStateMachine<'a, T: SearchTelemetry> {
    pub ctx: &'a SearchContext<'a>,
    pub tt: &'a mut TranspositionTable,
    pub telemetry: &'a T,
    pub state: GameState,
    pub stack: Vec<SearchFrame>,
    pub max_depth: usize,
}

impl<'a, T: SearchTelemetry> NegamaxStateMachine<'a, T> {
    /// Allocates and initializes a new search machine tracking stack frame states.
    pub fn new(
        ctx: &'a SearchContext<'a>,
        tt: &'a mut TranspositionTable,
        telemetry: &'a T,
        initial_state: GameState,
        target_depth: usize,
    ) -> Self {
        let mut machine = Self {
            ctx,
            tt,
            telemetry,
            state: initial_state,
            stack: Vec::with_capacity(target_depth + 1),
            max_depth: target_depth,
        };
        machine.push_root_frame();
        machine
    }

    /// Initializes the structural search parameters for the primary root frame node.
    fn push_root_frame(&mut self) {
        let alpha = EvaluationScore::Mated(0);
        let beta = EvaluationScore::Mating(0);
        let legal_moves = generate_and_order_moves(&self.state, self.max_depth);

        self.stack.push(SearchFrame {
            move_idx: 0,
            legal_moves,
            alpha,
            beta,
            max_score: EvaluationScore::Mated(0),
            original_alpha: alpha,
            pv_line: Vec::with_capacity(self.max_depth),
        });
    }

    /// Advances the state machine exploration tracking index parameters exactly one step.
    pub fn step(&mut self) -> StepResult {

        // Check Cancellation
        if self.ctx.cancelled.load(Ordering::Relaxed) {
            return StepResult::Done { best_score: EvaluationScore::Value(0), pv: Vec::new() };
        }

        // Quantify depth
        let current_depth = self.stack.len() - 1;
        let remaining_depth = self.max_depth - current_depth;

        // Record telemetry
        self.telemetry.record_node_explored();

        // Get current frame
        let frame = match self.stack.last_mut() {
            Some(f) => f,
            None => return StepResult::Done { best_score: EvaluationScore::Value(0), pv: Vec::new() },
        };

        // Initial position assesment
        if frame.move_idx == 0 {

            // Check cached position evaluations
            if let Some(cached_score) = evaluate_tt_cache(&self.state, self.tt, remaining_depth, frame.alpha, frame.beta) {
                self.stack.pop();
                if self.stack.is_empty() {
                    return StepResult::Done { best_score: cached_score, pv: Vec::new() };
                } else {
                    return StepResult::Backtrack { score: cached_score, pv: Vec::new() };
                }
            }

            // Evaluate Terminal Nodes
            if let Some(terminal_score) = evaluate_node_bounds(&self.state, self.ctx, &frame.legal_moves, remaining_depth) {
                frame.max_score = terminal_score;
                return self.finalize_and_backtrack();
            }
        }
        // Return if no more moves are left
        else if frame.move_idx >= frame.legal_moves.len() {
            return self.finalize_and_backtrack();
        }

        // Make next legal move
        let chosen_move = frame.legal_moves[frame.move_idx];
        frame.move_idx += 1;

        self.state.make_move(chosen_move);

        // Prepare next frame
        let next_alpha = invert_score(frame.beta);
        let next_beta = invert_score(frame.alpha);
        let next_remaining_depth = remaining_depth - 1;
        let next_legal_moves = generate_and_order_moves(&self.state, next_remaining_depth);

        self.stack.push(SearchFrame {
            move_idx: 0,
            legal_moves: next_legal_moves,
            alpha: next_alpha,
            beta: next_beta,
            max_score: EvaluationScore::Mated(0),
            original_alpha: next_alpha,
            pv_line: Vec::new(),
        });

        StepResult::Deepen
    }

    /// Receives downstream evaluation feedback from a completed child sub-node exploration.
    pub fn handle_backtrack(&mut self, child_score: EvaluationScore, child_pv: Vec<Move>) -> StepResult {
        
        // Clean up last move
        let mut last_executed_move = None;
        if let Some(last_frame) = self.stack.last_mut() {
            if last_frame.move_idx > 0 {
                let m = last_frame.legal_moves[last_frame.move_idx - 1];
                last_executed_move = Some(m);
                self.state.unmake_move(m);
            }
        }

        let relative_score = invert_score(child_score);
        let is_root = self.stack.len() == 1;
        
        // Get last frame or return if None
        let frame = match self.stack.last_mut() {
            Some(f) => f,
            None => return StepResult::Done { best_score: relative_score, pv: child_pv },
        };

        // If move is best sibling switch lines
        if relative_score > frame.max_score {
            frame.max_score = relative_score;
            if let Some(m) = last_executed_move {
                frame.pv_line = compile_pv_line(m, &child_pv);
            }
        }
        
        // Alpha Beta Prune on non-initial moves
        if !is_root {
            if frame.max_score > frame.alpha {
                frame.alpha = frame.max_score;
            }

            if frame.alpha >= frame.beta {
                return self.finalize_and_backtrack();
            }
        }

        // Return if no moves are left
        if frame.move_idx >= frame.legal_moves.len() {
            return self.finalize_and_backtrack();
        }

        StepResult::Deepen
    }

    /// Evaluates final alpha-beta constraints and preserves information in the cache before popping the frame.
    fn finalize_and_backtrack(&mut self) -> StepResult {
        let frame = match self.stack.pop() {
            Some(f) => f,
            None => return StepResult::Done { best_score: EvaluationScore::Value(0), pv: Vec::new() },
        };

        let remaining_depth = self.max_depth - self.stack.len();
        
        commit_node_to_tt(
            &self.state,
            self.tt,
            frame.max_score,
            frame.original_alpha,
            frame.beta,
            remaining_depth,
        );

        if self.stack.is_empty() {
            StepResult::Done { 
                best_score: frame.max_score, 
                pv: frame.pv_line,
            }
        } else {
            StepResult::Backtrack { 
                score: frame.max_score, 
                pv: frame.pv_line,
            }
        }
    }
}