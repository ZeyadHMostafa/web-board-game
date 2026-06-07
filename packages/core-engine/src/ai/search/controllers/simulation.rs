use crate::ai::ScoredMove;
use crate::ai::search::{NoOpTelemetry, SearchContext, SearchProgress};
use crate::ai::search::algorithms::negamax::{NegamaxStateMachine, StepResult};
use crate::ai::search::utils::{TranspositionTable, invert_score};
use crate::ai::evaluator::EvaluationScore;
use crate::rules::state::GameState;

pub struct SimulationController<'a> {
    ctx: &'a SearchContext<'a>,
    target_depth: usize,
}

impl<'a> SimulationController<'a> {
    pub fn new(ctx: &'a SearchContext<'a>, target_depth: usize) -> Self {
        Self { ctx, target_depth }
    }

    /// Searches exactly to the target depth with minimal performance tracking.
    pub fn search(&self, true_state: &GameState, tt: &mut TranspositionTable) -> EvaluationScore {
        let telemetry = NoOpTelemetry;
        let mut machine = NegamaxStateMachine::new(
            self.ctx,
            tt,
            &telemetry,
            *true_state,
            self.target_depth,
        );

        let mut status = StepResult::Deepen;
        let total_score ;

        loop {
            match status {
                StepResult::Deepen => {
                    status = machine.step();
                }
                StepResult::Backtrack { score, pv } => {
                    status = machine.handle_backtrack(score, pv);
                }
                StepResult::Done { best_score , pv: _pv} => {
                    total_score = best_score;
                    break;
                }
            }
        }

        total_score
    }

    pub fn search_candidates(&self, true_state: &GameState, tt: &mut TranspositionTable) -> SearchProgress {
        let telemetry = NoOpTelemetry;
        let mut machine = NegamaxStateMachine::new(
            self.ctx,
            tt,
            &telemetry,
            *true_state,
            self.target_depth,
        );

        let root_moves_count = machine.stack[0].legal_moves.len();
        let mut candidates = Vec::with_capacity(root_moves_count);
        let mut status = StepResult::Deepen;

        loop {
            match status {
                StepResult::Deepen => {
                    status = machine.step();
                }
                StepResult::Backtrack { score, pv } => {
                    // When backing into the root frame, intercept the score and assign it to the corresponding move
                    if machine.stack.len() == 1 {
                        let explored_idx = machine.stack[0].move_idx - 1;
                        let target_move = machine.stack[0].legal_moves[explored_idx];
                        candidates.push(ScoredMove {
                            current_move: target_move,
                            score: invert_score(score), //final invert to set move from out prespective
                        });
                    }
                    status = machine.handle_backtrack(score, pv);
                }
                StepResult::Done { .. } => {
                    break;
                }
            }
        }

        SearchProgress {
            candidates,
            depth_reached: self.target_depth,
            nodes_explored: 0, // Swapped to 0 instantly due to NoOpTelemetry
            branching_factor: 0.0,
            pv: Vec::new()
        }
    }
}