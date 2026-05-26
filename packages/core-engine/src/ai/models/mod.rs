use crate::rules::state::GameState;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EvaluationScore {
    Mated(u32),
    Value(i32),
    Mating(u32),
}

impl Default for EvaluationScore {
    fn default() -> Self {
        EvaluationScore::Value(0)
    }
}

pub trait PositionEvaluator: Send + Sync {
    /// Evaluates the position relative to the side whose turn it currently is.
    fn evaluate(&self, state: &GameState) -> EvaluationScore;
}

// Re-export concrete implementations
pub mod static_dot;
pub use static_dot::StaticDotProductEvaluator;