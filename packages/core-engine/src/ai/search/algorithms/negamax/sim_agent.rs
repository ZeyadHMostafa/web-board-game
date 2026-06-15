use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use crate::ai::search::controllers::SimulationController;
use crate::ai::search::{SearchContext, utils::TranspositionTable};
use crate::ai::models::static_dot::TrainableDotProductEvaluator;
use crate::ai::heuristics::evaluators;
use crate::ai::search::selector::{ActionSelector, SelectorMode};
use crate::rules::state::{GameState};
use crate::rules::moves::Move;

pub struct NegamaxSimulationAgent {
    evaluator: Arc<TrainableDotProductEvaluator>,
    search_depth: usize,
    epsilon: f32,
}

impl NegamaxSimulationAgent {
    pub fn new(
        evaluator: Arc<TrainableDotProductEvaluator>,
        search_depth: usize,
        epsilon: f32,
    ) -> Self {
        Self { evaluator, search_depth, epsilon }
    }

    /// Investigates the position to a fixed depth and provides selection metrics along with features arrays.
    pub fn evaluate_and_select(&self, state: &GameState) -> Option<(Move, Vec<f32>, f32)> {
        let cancelled = AtomicBool::new(false);
        let ctx = SearchContext {
            evaluator: self.evaluator.as_ref(),
            cancelled: &cancelled,
        };

        let mut local_tt = TranspositionTable::with_capacity(20);
        let controller = SimulationController::new(&ctx, self.search_depth);
        
        let search_result = controller.search_candidates(state, &mut local_tt);
        if search_result.candidates.is_empty() {
            return None;
        }

        // Get the ideal competitive move to serve as a value baseline target
        let best_move = ActionSelector::select_move(&search_result, SelectorMode::Competitive)?;
        let best_candidate = search_result.candidates.iter().find(|c| c.current_move == best_move)?;

        let mode = SelectorMode::TrainingExploration { epsilon: self.epsilon };
        let chosen_move = ActionSelector::select_move(&search_result, mode)?;

        let features = Self::extract_features(state);
        let target_score = best_candidate.score.to_float();

        Some((chosen_move, features, target_score))
    }

    /// Transforms bitboard structures into structural vector features.
    pub fn extract_features(state: &GameState) -> Vec<f32> {
        let (allied_pieces, enemy_pieces) = state.get_player_pieces_relative();
        let matrix = evaluators::evaluate_position(
            allied_pieces,
            enemy_pieces
        );

        let mut features = Vec::with_capacity(108);
        for t in 0..3 {
            for s in 0..6 {
                for r in 0..3 {
                    for p in 0..2 {
                        features.push(matrix.values[t][s][r][p] as f32);
                    }
                }
            }
        }
        features
    }
}