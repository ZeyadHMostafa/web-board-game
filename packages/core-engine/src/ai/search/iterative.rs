use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use crate::ai::search::negamax::{SearchContext, invert_score, negamax};
use crate::ai::EvaluationScore;
use crate::ai::search::transposition_table::TranspositionTable;
use crate::rules::move_structs::Move;
use crate::rules::state::GameState;

pub struct IterativeDeepeningController<'a> {
    ctx: SearchContext<'a>,
    min_depth: usize,
    max_depth: usize,
}

#[derive(Debug, Clone)]
pub struct ScoredMove {
    pub current_move: Move,
    pub score: EvaluationScore,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub candidates: Vec<ScoredMove>,
    pub depth_reached: usize,
    pub nodes_explored: usize,
    pub branching_factor: f64,
}

impl<'a> IterativeDeepeningController<'a> {
    pub fn new(ctx: SearchContext<'a>, min_depth: usize, max_depth: usize) -> Self {
        Self { ctx, max_depth, min_depth }
    }

    /// Iteratively deepens the search tree up to the configured ply limit.
    /// Returns evaluations for all available root moves found during the deepest complete pass.
    pub fn search(&self, true_state: & GameState, time_limit: Option<Duration>) -> SearchResult {
        let start_time = Instant::now();
        
        let mut state = true_state.clone();
        let mut final_result = SearchResult {
            candidates: Vec::new(),
            depth_reached: 0,
            nodes_explored: 0,
            branching_factor: 0.0
        };

        let mut legal_moves = state.generate_legal_moves(self.ctx.luts);
        if legal_moves.is_empty() {
            return final_result;
        }

        let tt = &mut TranspositionTable::with_capacity(10);

        for current_depth in self.min_depth..=self.max_depth {
            if let Some(limit) = time_limit {
                if start_time.elapsed() >= limit {
                    break;
                }
            }

            let mut current_ply_candidates = Vec::with_capacity(legal_moves.len());
            let mut alpha = EvaluationScore::Value(i32::MIN);
            let beta = EvaluationScore::Value(i32::MAX);

            // Sort root options based on the best performer of the prior depth
            if let Some(prev_best) = final_result.candidates.iter().max_by_key(|c| match c.score {
                EvaluationScore::Value(v) => v,
                EvaluationScore::Mating(_) => i32::MAX,
                EvaluationScore::Mated(_) => i32::MIN
            }) {
                if let Some(pos) = legal_moves.iter().position(|&m| m == prev_best.current_move) {
                    legal_moves.move_to_front(pos);
                }
            }

            for &current_move in legal_moves.as_slice() {
                if self.ctx.cancelled.load(Ordering::Relaxed) {
                    break;
                }

                if let Some(limit) = time_limit {
                    if start_time.elapsed() >= limit {
                        self.ctx.cancelled.store(true, Ordering::Relaxed);
                        break;
                    }
                }

                state.make_move(current_move);

                let score = invert_score(negamax(
                    &self.ctx,
                    tt,
                    &state,
                    current_depth - 1,
                    invert_score(beta),
                    invert_score(alpha),
                ));

                state.unmake_move(current_move);

                if score > alpha {
                    alpha = score;
                }

                current_ply_candidates.push(ScoredMove {
                    current_move,
                    score,
                });
            }

            // Commit the complete ply evaluation records only if execution was uninterrupted
            if !self.ctx.cancelled.load(Ordering::Relaxed) {
                let total_nodes = self.ctx.nodes_explored.load(Ordering::Relaxed);
                
                // Calculate Effective Branching Factor: d-th root of total nodes
                let ebf = if current_depth > 0 && total_nodes > 0 {
                    (total_nodes as f64).powf(1.0 / current_depth as f64)
                } else {
                    0.0
                };
                final_result = SearchResult {
                    candidates: current_ply_candidates,
                    depth_reached: current_depth,
                    nodes_explored: total_nodes,
                    branching_factor: ebf,
                };
            } else {
                break;
            }
        }

        // Fallback safety layer to populate baseline options in case of immediate timeout
        if final_result.candidates.is_empty() {
            final_result.candidates = legal_moves
                .into_iter()
                .map(|m| ScoredMove {
                    current_move: m,
                    score: EvaluationScore::Value(i32::MIN),
                })
                .collect();
        }

        final_result
    }
}