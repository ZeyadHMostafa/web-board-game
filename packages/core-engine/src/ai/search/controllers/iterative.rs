use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;
use crate::ai::search::algorithms::negamax::{NegamaxStateMachine, StepResult};
use crate::ai::search::{SearchContext, SearchProgress, ScoredMove};
use crate::ai::search::utils::TranspositionTable;
use crate::ai::evaluator::EvaluationScore;
use crate::rules::state::GameState;
use crate::ai::search::ActiveTelemetry;

pub struct IterativeDeepeningController<'a> {
    ctx: &'a SearchContext<'a>,
    min_depth: usize,
    max_depth: usize,
    shared_progress: Arc<RwLock<SearchProgress>>,
}

impl<'a> IterativeDeepeningController<'a> {
    pub fn new(
        ctx: &'a SearchContext<'a>, 
        min_depth: usize, 
        max_depth: usize,
        shared_progress: Arc<RwLock<SearchProgress>>,
    ) -> Self {
        Self { ctx, min_depth, max_depth, shared_progress }
    }

    /// Drives the state-machine search through escalating plys.
    pub fn search(&self, true_state: &GameState) {
        let mut tt = TranspositionTable::with_capacity(20);
        let telemetry = ActiveTelemetry {
            nodes_explored: AtomicUsize::new(0),
        };

        // Pre-populate root canvas with fallback defaults
        let initial_moves = true_state.generate_legal_moves(self.ctx.luts);
        {
            let mut progress = self.shared_progress.write().unwrap();
            progress.candidates = initial_moves.into_iter().map(|m| ScoredMove {
                current_move: m,
                score: EvaluationScore::Value(i32::MIN),
            }).collect();
            progress.depth_reached = 0;
            progress.nodes_explored = 0;
            progress.branching_factor = 0.0;
        }

        for current_depth in self.min_depth..=self.max_depth {
            if self.ctx.cancelled.load(Ordering::Relaxed) {
                break;
            }

            let mut machine = NegamaxStateMachine::new(
                self.ctx,
                &mut tt,
                &telemetry,
                true_state.clone(),
                current_depth,
            );

            // Fetch the root frame to manage candidates at this depth layer
            let root_moves_count = machine.stack[0].legal_moves.len();
            if root_moves_count == 0 {
                break;
            }

            let mut layer_candidates = Vec::with_capacity(root_moves_count);
            let mut status = StepResult::Deepen;

            while !self.ctx.cancelled.load(Ordering::Relaxed) {
                match status {
                    StepResult::Deepen => {
                        status = machine.step();
                    }
                    StepResult::Backtrack { score } => {
                        // Captures evaluations returning directly back to root elements
                        if machine.stack.len() == 1 {
                            let explored_idx = machine.stack[0].move_idx - 1;
                            let target_move = machine.stack[0].legal_moves[explored_idx];
                            layer_candidates.push(ScoredMove {
                                current_move: target_move,
                                score,
                            });
                        }
                        status = machine.handle_backtrack(score);
                    }
                    StepResult::Done { .. } => {
                        break;
                    }
                }
            }

            // Commit results only if the full ply layer finished cleanly without cancellation interruptions
            if !self.ctx.cancelled.load(Ordering::Relaxed) {
                let total_nodes = telemetry.nodes_explored.load(Ordering::Relaxed);
                let ebf = if current_depth > 0 && total_nodes > 0 {
                    (total_nodes as f64).powf(1.0 / current_depth as f64)
                } else {
                    0.0
                };

                let mut progress = self.shared_progress.write().unwrap();
                progress.candidates = layer_candidates;
                progress.depth_reached = current_depth;
                progress.nodes_explored = total_nodes;
                progress.branching_factor = ebf;
            } else {
                break;
            }
        }
    }
}