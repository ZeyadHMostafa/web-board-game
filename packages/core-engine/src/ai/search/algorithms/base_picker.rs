use std::sync::{Arc, RwLock};
use crate::luts;
use crate::rules::state::GameState;
use crate::rules::moves::Move;
use crate::simulation::Agent;
use crate::ai::PositionEvaluator;
use crate::ai::evaluator::EvaluationScore;
use crate::ai::search::{SearchContext, SearchProgress, ScoredMove};
use crate::ai::search::selector::{ActionSelector, SelectorMode};

/// A rudimentary 1-ply picker that tests structural pipelines 
/// by choosing the immediately highest scoring local position.
pub struct BasePickerSearch {
    pub evaluator: Arc<dyn PositionEvaluator>,
    pub luts: &'static luts::EngineLUTs
}

impl BasePickerSearch {
    pub fn new(luts: &'static luts::EngineLUTs, evaluator: Arc<dyn PositionEvaluator>) -> Self {
        Self { evaluator, luts }
    }
}

impl Agent for BasePickerSearch {
    fn search_position(
        &self,
        state: &GameState,
        _ctx: &SearchContext,
        shared_progress: Arc<RwLock<SearchProgress>>,
    ) {
        let legal_moves = state.generate_legal_moves();
        let mut layer_candidates = Vec::with_capacity(legal_moves.len());

        for current_move in legal_moves {
            let mut next_state = *state;
            next_state.make_move(current_move);

            let raw_score = self.evaluator.evaluate(&next_state);
            
            let relative_score = match raw_score {
                EvaluationScore::Value(v) => EvaluationScore::Value(-v),
                EvaluationScore::Mating(d) => EvaluationScore::Mated(d + 1),
                EvaluationScore::Mated(d) => EvaluationScore::Mating(d + 1),
            };

            layer_candidates.push(ScoredMove {
                current_move,
                score: relative_score,
            });
        }

        let mut progress = shared_progress.write().unwrap();
        progress.candidates = layer_candidates;
        progress.depth_reached = 1;
        progress.nodes_explored = progress.candidates.len();
        progress.branching_factor = 1.0;
    }

    fn select_move(&self, progress: &SearchProgress) -> Result<Move, String> {
        match ActionSelector::select_move(progress, SelectorMode::Competitive) {
            Some(m) => Ok(m),
            None => Err("BasePickerSearch failed to isolate a valid move selection.".to_string()),
        }
    }
}