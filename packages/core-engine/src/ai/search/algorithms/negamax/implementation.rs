use std::sync::atomic::Ordering;
use crate::ai::search::algorithms::negamax::{SearchFrame, StepResult};
use crate::ai::search::{SearchContext, SearchTelemetry};
use crate::ai::search::utils::{HashEntryBounds, TranspositionTable, invert_score};
use crate::ai::heuristics::evaluators;
use crate::rules::state::GameState;

use crate::ai::evaluator::EvaluationScore;

pub struct NegamaxStateMachine<'a, T: SearchTelemetry> {
    pub ctx: &'a SearchContext<'a>,
    pub tt: &'a mut TranspositionTable,
    pub telemetry: &'a T,
    pub state: GameState,
    pub stack: Vec<SearchFrame>,
    pub max_depth: usize,
}

impl<'a, T: SearchTelemetry> NegamaxStateMachine<'a, T> {
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

    /// Initializes the initial search frame at the root of the tree.
    fn push_root_frame(&mut self) {
        let alpha = EvaluationScore::Value(i32::MIN);
        let beta = EvaluationScore::Value(i32::MAX);
        
        let mut legal_moves = self.state.generate_legal_moves(self.ctx.luts);
        if !legal_moves.is_empty() && self.max_depth > 1 {
            let allied_pieces = self.state.get_player_pieces(self.state.active_player);
            let enemy_pieces = self.state.get_player_pieces(self.state.active_player.opponent());
            legal_moves.sort_unstable_by_key(|m| {
                evaluators::evaluate_move(m, allied_pieces, enemy_pieces)
            });
        }

        self.stack.push(SearchFrame {
            move_idx: 0,
            legal_moves,
            alpha,
            beta,
            max_score: EvaluationScore::Value(i32::MIN),
            original_alpha: alpha,
        });
    }

    /// Advances the state machine by exactly one operation.
    pub fn step(&mut self) -> StepResult {
        if self.ctx.cancelled.load(Ordering::Relaxed) {
            return StepResult::Done { best_score: EvaluationScore::Value(0) };
        }

        let current_depth = self.stack.len() - 1;
        let remaining_depth = self.max_depth - current_depth;

        self.telemetry.record_node_explored();

        if let Some(frame) = self.stack.last_mut() {
            if frame.move_idx == 0 {
                if let Some(entry) = self.tt.lookup(&self.state) {
                    if entry.depth >= remaining_depth {
                        match entry.bounds {
                            HashEntryBounds::Exact => {
                                // Corrected: This value is already oriented to our perspective.
                                // We update max_score directly and call finalize to avoid double inversion!
                                frame.max_score = entry.score;
                                return self.finalize_and_backtrack();
                            }
                            HashEntryBounds::AlphaLower => {
                                // Fail-low entry: The true value is guaranteed <= entry.score
                                if entry.score <= frame.alpha {
                                    frame.max_score = entry.score;
                                    return self.finalize_and_backtrack();
                                }
                            }
                            HashEntryBounds::BetaUpper => {
                                // Fail-high entry: The true value is guaranteed >= entry.score
                                if entry.score >= frame.beta {
                                    frame.max_score = entry.score;
                                    return self.finalize_and_backtrack();
                                }
                            }
                        }
                    }
                }

                if remaining_depth == 0 {
                    let score = self.ctx.evaluator.evaluate(&self.state);
                    return StepResult::Backtrack { score };
                }

                if frame.legal_moves.is_empty() {
                    return StepResult::Backtrack { score: EvaluationScore::Mated(0) };
                }
            }

            if frame.move_idx >= frame.legal_moves.len() {
                return self.finalize_and_backtrack();
            }

            let chosen_move = frame.legal_moves[frame.move_idx];
            frame.move_idx += 1;

            self.state.make_move(chosen_move);

            // Compute alpha-beta bounds for the child node path
            let next_alpha = invert_score(frame.beta);
            let next_beta = invert_score(frame.alpha);

            let mut next_legal_moves = self.state.generate_legal_moves(self.ctx.luts);
            let next_remaining_depth = remaining_depth - 1;
            if !next_legal_moves.is_empty() && next_remaining_depth > 1 {
                let allied_pieces = self.state.get_player_pieces(self.state.active_player);
                let enemy_pieces = self.state.get_player_pieces(self.state.active_player.opponent());
                next_legal_moves.sort_unstable_by_key(|m| {
                    evaluators::evaluate_move(m, allied_pieces, enemy_pieces)
                });
            }

            self.stack.push(SearchFrame {
                move_idx: 0,
                legal_moves: next_legal_moves,
                alpha: next_alpha,
                beta: next_beta,
                max_score: EvaluationScore::Value(i32::MIN),
                original_alpha: next_alpha,
            });

            StepResult::Deepen
        } else {
            StepResult::Done { best_score: EvaluationScore::Value(0) }
        }
    }

    /// Processes a completed node value returned from a child branch execution.
    pub fn handle_backtrack(&mut self, child_score: EvaluationScore) -> StepResult {
        if let Some(last_frame) = self.stack.last_mut() {
            // Only unmake a move if we actually shifted the board state to evaluate a child
            if last_frame.move_idx > 0 {
                let last_executed_move = last_frame.legal_moves[last_frame.move_idx - 1];
                self.state.unmake_move(last_executed_move);
            }
        }

        let relative_score = invert_score(child_score);
        
        if let Some(frame) = self.stack.last_mut() {
            // If this frame was evaluated early (move_idx == 0), it acts as an immediate leaf value
            if frame.move_idx == 0 {
                frame.max_score = relative_score;
                return self.finalize_and_backtrack();
            }

            if relative_score > frame.max_score {
                frame.max_score = relative_score;
            }

            if frame.max_score > frame.alpha {
                frame.alpha = frame.max_score;
            }

            // Trigger a cut-off immediately if alpha meets or exceeds beta boundaries
            if frame.alpha >= frame.beta {
                return self.finalize_and_backtrack();
            }

            if frame.move_idx >= frame.legal_moves.len() {
                return self.finalize_and_backtrack();
            }

            StepResult::Deepen
        } else {
            StepResult::Done { best_score: relative_score }
        }
    }

    /// Evaluates structural bounds classifications and caches entries within the transposition table.
    fn finalize_and_backtrack(&mut self) -> StepResult {
        if let Some(frame) = self.stack.pop() {
            let remaining_depth = self.max_depth - self.stack.len();
            
            let bounds = if frame.max_score <= frame.original_alpha {
                HashEntryBounds::AlphaLower
            } else if frame.max_score >= frame.beta {
                HashEntryBounds::BetaUpper
            } else {
                HashEntryBounds::Exact
            };

            self.tt.store(&self.state, frame.max_score, remaining_depth, bounds);

            if self.stack.is_empty() {
                StepResult::Done { best_score: frame.max_score }
            } else {
                StepResult::Backtrack { score: frame.max_score }
            }
        } else {
            StepResult::Done { best_score: EvaluationScore::Value(0) }
        }
    }
}