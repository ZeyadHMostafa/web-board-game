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

// if false && let Some(entry) = self.tt.lookup(&self.state) {
//                     if entry.depth >= remaining_depth {
//                         match entry.bounds {
//                             HashEntryBounds::Exact => {
//                                 frame.max_score = entry.score;
//                                 return self.finalize_and_backtrack();
//                             }
//                             HashEntryBounds::AlphaLower => {
//                                 // Lower bound: True value is >= entry.score
//                                 if entry.score >= frame.beta {
//                                     frame.max_score = entry.score;
//                                     return self.finalize_and_backtrack();
//                                 }
//                             }
//                             HashEntryBounds::BetaUpper => {
//                                 // Upper bound: True value is <= entry.score
//                                 if entry.score <= frame.alpha {
//                                     frame.max_score = entry.score;
//                                     return self.finalize_and_backtrack();
//                                 }
//                             }
//                         }
//                     }
//                 }

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
        let alpha = EvaluationScore::Mated(0);
        let beta = EvaluationScore::Mating(0);
        
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
            max_score: EvaluationScore::Mated(0),
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

        // 1. Snag a mutable reference to the active layout frame
        if let Some(frame) = self.stack.last_mut() {
            
            // First-time Node Initialization Logic
            if frame.move_idx == 0 {
                if let Some(entry) = self.tt.lookup(&self.state) {
                    if entry.depth >= remaining_depth {
                        let mut hit = false;
                        match entry.bounds {
                            HashEntryBounds::Exact => {
                                hit = true;
                            }
                            HashEntryBounds::Lower => { 
                                // Lower bound means the true value is >= entry.score.
                                // If it is already >= our beta cutoff, we can prune!
                                if entry.score >= frame.beta {
                                    hit = true;
                                }
                            }
                            HashEntryBounds::Upper => { 
                                // Upper bound means the true value is <= entry.score.
                                // If it is already <= our alpha, it's useless to search further.
                                if entry.score <= frame.alpha {
                                    hit = true;
                                }
                            }
                        }

                        if hit {
                            self.stack.pop(); 

                            if self.stack.is_empty() {
                                return StepResult::Done { best_score: entry.score };
                            } else {
                                return StepResult::Backtrack { score: entry.score };
                            }
                        }
                    }
                }
                // Check physical game termination (Checkmate / Stalemate)
                if frame.legal_moves.is_empty() {
                    frame.max_score = EvaluationScore::Mated(0);
                    // println!("mate detected here, calling finalize and back track!");
                    return self.finalize_and_backtrack();
                }
                
                // Check depth limit exhaustion
                if remaining_depth == 0 {
                    let score = self.ctx.evaluator.evaluate(&self.state);
                    // FIX: Mutate frame score directly and pop cleanly via structural logic
                    frame.max_score = score;
                    return self.finalize_and_backtrack();
                }
            }

            // If we have exhausted all legal options at this node
            if frame.move_idx >= frame.legal_moves.len() {
                return self.finalize_and_backtrack();
            }

            // Pull the next candidate move from the collection
            let chosen_move = frame.legal_moves[frame.move_idx];
            frame.move_idx += 1;

            self.state.make_move(chosen_move);

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
                max_score: EvaluationScore::Mated(0),
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
            if last_frame.move_idx > 0 {
                let last_executed_move = last_frame.legal_moves[last_frame.move_idx - 1];
                self.state.unmake_move(last_executed_move);
            }
        }

        let relative_score = invert_score(child_score);
        
        if let Some(frame) = self.stack.last_mut() {
            // FIX: Removed the early-return "if frame.move_idx == 0" check.
            // Terminal leaf logic is now gracefully managed inside step()!


            // println!("\n\n");
            // println!("BACKTRACK trace:");
            // println!("before:");
            // println!("relative score: {:?}",relative_score);
            // println!("max score: {:?}",frame.max_score);
            // println!("alpha: {:?}",frame.alpha);
            // println!("beta: {:?}",frame.beta);

            if relative_score > frame.max_score {
                frame.max_score = relative_score;
            }

            if frame.max_score > frame.alpha {
                frame.alpha = frame.max_score;
            }
            // println!("after:");
            // println!("relative score: {:?}",relative_score);
            // println!("max score: {:?}",frame.max_score);
            // println!("alpha: {:?}",frame.alpha);
            // println!("beta: {:?}",frame.beta);

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
                HashEntryBounds::Upper  // Fail-low: True value is <= max_score
            } else if frame.max_score >= frame.beta {
                HashEntryBounds::Lower  // Fail-high: True value is >= max_score
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