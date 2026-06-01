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
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum EvaluationScore {
        Mated(u32),       // Distance to forced loss (fewer plies = worse)
        Value(i32),       // Continuous static/neural assessment metric
        Mating(u32),      // Distance to forced win (fewer plies = better)
    }

    impl Ord for EvaluationScore {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match (self, other) {
                // 1. Same Variants: Special distance ordering logic
                (EvaluationScore::Mating(p1), EvaluationScore::Mating(p2)) => {
                    // Fewer plies to mate is BETTER (e.g., Mating(1) > Mating(3))
                    p2.cmp(p1) 
                }
                (EvaluationScore::Mated(p1), EvaluationScore::Mated(p2)) => {
                    // Fewer plies to getting mated is WORSE (e.g., Mated(1) < Mated(3))
                    p1.cmp(p2)
                }
                (EvaluationScore::Value(v1), EvaluationScore::Value(v2)) => v1.cmp(v2),

                // 2. Cross-Variant Comparisons: Variants dictate ultimate priority
                (EvaluationScore::Mating(_), _) => std::cmp::Ordering::Greater,
                (_, EvaluationScore::Mating(_)) => std::cmp::Ordering::Less,
                
                (EvaluationScore::Mated(_), _) => std::cmp::Ordering::Less,
                (_, EvaluationScore::Mated(_)) => std::cmp::Ordering::Greater,
            }
        }
}

impl PartialOrd for EvaluationScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
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