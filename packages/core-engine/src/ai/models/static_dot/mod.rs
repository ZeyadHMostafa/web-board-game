use crate::rules::luts::EngineLUTs;
use crate::heuristics::evaluators::EvaluationEngine;

pub mod implementations;
pub(crate) mod utils;
pub use utils::load_weights_from_npy;

/// Global pre-computed weights matrix for quick initialization across files.
pub const DEFAULT_EVALUATOR_WEIGHTS: [[[[i32; 2]; 3]; 6]; 3] = StaticDotProductEvaluator::generate_weights(
    // Base action prospects based on TileType and SovereigntyState
    // Dimensions: [SovereigntyState: 6][TileType: 3]
    [
        //emty ally enmy
        [  03,  50, -15  ],// ally-unc
        [  01,  45,  25  ],// ally-dom
        [  00,  40,  40  ],// conf-non
        [  00,  35, -35  ],// conf-tie
        [ -01,  30, -45  ],// enmy-dom
        [ -03,  20, -50  ],// enmy-unc
    ],
    // Multipliers for each RegionType: Corner, Edge, Center
    [10, 11, 12],
);

pub struct StaticDotProductEvaluator {
    luts: &'static EngineLUTs,
    engine: EvaluationEngine,
    weights: [[[[i32; 2]; 3]; 6]; 3],
}

use std::sync::Arc;

/// Sibling evaluator designed to handle dynamic weights coming from Python ML loops
pub struct TrainableDotProductEvaluator {
    luts: &'static EngineLUTs,
    engine: EvaluationEngine,
    // Flattened weight array: 3 * 6 * 3 * 2 = 108 elements
    weights: Arc<Vec<i32>>, 
}