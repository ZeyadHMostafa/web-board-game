use crate::luts::EngineLUTs;

pub mod implementations;
pub(crate) mod utils;
pub use utils::load_weights_from_npy;
pub use utils::DEFAULT_EVALUATOR_WEIGHTS;

pub struct StaticDotProductEvaluator {
    luts: &'static EngineLUTs,
    weights: [[[[i32; 2]; 3]; 6]; 3],
}

use std::sync::Arc;

/// Sibling evaluator designed to handle dynamic weights coming from Python ML loops
pub struct TrainableDotProductEvaluator {
    luts: &'static EngineLUTs,
    // Flattened weight array: 3 * 6 * 3 * 2 = 108 elements
    pub weights: Arc<Vec<i32>>, 
}