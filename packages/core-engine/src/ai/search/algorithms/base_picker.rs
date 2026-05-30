use std::sync::Arc;
use crate::luts;
use crate::rules::state::GameState;
use crate::rules::moves::Move;
use crate::simulation::{Agent, GameClock};
use crate::ai::PositionEvaluator;
use crate::ai::evaluator::EvaluationScore;

/// A rudimentary 1-ply picker that tests structural pipelines 
/// by choosing the immediately highest scoring local position.
pub struct BasePickerSearch {
    pub evaluator: Arc<dyn PositionEvaluator>,
    pub luts: &'static luts::EngineLUTs
}

impl BasePickerSearch {
    pub fn new(luts: &'static luts::EngineLUTs, evaluator: Arc<dyn PositionEvaluator>) -> Self {
        Self { evaluator: evaluator , luts}
    }
}

impl Agent for BasePickerSearch {
    async fn select_move(
        &self, 
        state: &GameState, 
        time: Option<GameClock>
    ) -> Result<Move, String> {
        // Generate valid transitions for the active player.
        // Assumes GameState provides an iterator or vector of legal active moves.
        let legal_moves = state.generate_legal_moves(&self.luts);
        
        if legal_moves.is_empty() {
            return Err("No legal moves available for current board state.".to_string());
        }

        let mut best_move = legal_moves[0];
        let mut max_score = EvaluationScore::Value(i32::MIN);

        for current_move in legal_moves {
            // Tentatively clone and apply transition to inspect perspective shifts
            let mut next_state = state.clone();
            next_state.make_move(current_move);

            // Our evaluator evaluates positions from the perspective of the side whose turn it currently is.
            // Since make_move switches the active side to our opponent, we invert the raw value evaluation
            // to find the score relative to ourselves.
            let raw_score = self.evaluator.evaluate(&next_state);
            
            let relative_score = match raw_score {
                EvaluationScore::Value(v) => EvaluationScore::Value(-v),
                EvaluationScore::Mating(d) => EvaluationScore::Mated(d + 1),
                EvaluationScore::Mated(d) => EvaluationScore::Mating(d + 1),
            };

            if relative_score > max_score {
                max_score = relative_score;
                best_move = current_move;
            }
        }

        Ok(best_move)
    }
}