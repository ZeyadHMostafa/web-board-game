pub mod models;
pub mod search;
pub mod heuristics;

pub use evaluator::{PositionEvaluator, EvaluationScore};
use crate::rules::moves::Move;

#[derive(Debug, Clone)]
pub struct ScoredMove {
    pub current_move: Move,
    pub score: EvaluationScore,
}

mod evaluator {
    use crate::rules::state::GameState;

    /// Strong typing for evaluation bounds, handling standard scaling and explicit matings.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum EvaluationScore {
        Mated(u32),       // Distance to forced loss
        Value(i32),       // Continuous static/neural assessment metric
        Mating(u32),      // Distance to forced win
    }

    impl Default for EvaluationScore {
        fn default() -> Self {
            EvaluationScore::Value(0)
        }
    }

    /// The hot-swappable brain layer.
    /// Operates completely synchronously over data snapshots passed into leaf nodes.
    pub trait PositionEvaluator: Send + Sync {
        /// Evaluates the position strictly favoring the player whose turn it currently is.
        fn evaluate(&self, state: &GameState) -> EvaluationScore;
    }
}