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

    mod scores{
        pub const WIN:f32 = 1000.0;
        pub const CUTOFF:f32 = 900.0;
        pub const FACTOR:f32 = 1000.0;
        pub const TURN_COST:f32 = 10.0;
    }

    use crate::rules::state::GameState;

    /// Strong typing for evaluation bounds, handling standard scaling and explicit matings.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum EvaluationScore {
        Mated(u32),       // Distance to forced loss
        Value(i32),       // Continuous static/neural assessment metric
        Mating(u32),      // Distance to forced win
    }

    impl EvaluationScore {
        pub fn to_float(self) -> f32{
            match self {
                EvaluationScore::Value(val) => (val as f32 / scores::FACTOR).clamp(-scores::CUTOFF,scores::CUTOFF),
                EvaluationScore::Mating(n) => scores::WIN - scores::TURN_COST * (n as f32),
                EvaluationScore::Mated(n) => -(scores::WIN - scores::TURN_COST * (n as f32)),
            }
        }
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