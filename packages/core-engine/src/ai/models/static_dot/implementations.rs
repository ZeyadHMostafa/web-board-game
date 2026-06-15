use std::sync::Arc;

use crate::ai::{EvaluationScore, PositionEvaluator};
use crate::ai::models::static_dot::{
    StaticDotProductEvaluator,
    StaticDotProductEvaluatorWeights,
    TrainableDotProductEvaluator,
    utils::{compute_dot_product, evaluate_state_features, generate_weights},
};
use crate::rules::state::GameState;

impl StaticDotProductEvaluator {
    /// Creates a new static evaluator using an externally provided weights matrix.
    pub const fn new(weights: StaticDotProductEvaluatorWeights) -> Self {
        Self {
            weights,
        }
    }

    /// Generates the multi-dimensional weights matrix using procedural combination rules.
    /// Parity values (Even and Odd columns) remain identical as parity yields no effect here.
    pub const fn generate_weights(
        prospects: [[i32; 3]; 6],
        region_multipliers: [i32; 3],
    ) -> [[[[i32; 2]; 3]; 6]; 3] {
        generate_weights(prospects, region_multipliers)
    }
}

impl PositionEvaluator for StaticDotProductEvaluator {
    #[inline(always)]
    fn evaluate(&self, state: &GameState) -> EvaluationScore {
        let features = evaluate_state_features(state);
        
        compute_dot_product(&features, |t, s, r, p| {
            self.weights[t][s][r][p]
        })
    }
}

impl TrainableDotProductEvaluator {
    pub fn new(weights: Arc<Vec<i32>>) -> Self {
        assert_eq!(weights.len(), 108, "Weight vector must contain exactly 108 elements.");
        Self {
            weights,
        }
    }
}

impl PositionEvaluator for TrainableDotProductEvaluator {
    #[inline(always)]
    fn evaluate(&self, state: &GameState) -> EvaluationScore {
        let features = evaluate_state_features(state);
        
        compute_dot_product(&features, |t, s, r, p| {
            let flat_idx = p + 2 * (r + 3 * (s + 6 * t));
            self.weights[flat_idx]
        })
    }
}