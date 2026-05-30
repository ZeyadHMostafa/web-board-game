use crate::rules::state::GameState;
use crate::ai::evaluator::EvaluationScore;

pub trait PositionEvaluator: Send + Sync {
    /// Evaluates the position relative to the side whose turn it currently is.
    fn evaluate(&self, state: &GameState) -> EvaluationScore;
}

// Re-export concrete implementations
pub mod static_dot;
pub use static_dot::StaticDotProductEvaluator;