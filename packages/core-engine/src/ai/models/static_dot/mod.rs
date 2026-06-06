use std::sync::Arc;
use std::ops::{Deref, DerefMut};

pub(crate) mod utils;
pub(crate) mod store;
pub mod implementations;

pub type RawWeightTensor = [[[[i32; 2]; 3]; 6]; 3];
pub use utils::DEFAULT_EVALUATOR_WEIGHTS;
pub use store::load_weights_from_npy;

/// High-performance container wrapping static array-allocated evaluator weights.
/// Implements transparent dereferencing directly into structural tensor definitions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StaticDotProductEvaluatorWeights(pub RawWeightTensor);

impl Deref for StaticDotProductEvaluatorWeights {
    type Target = RawWeightTensor;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StaticDotProductEvaluatorWeights {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct StaticDotProductEvaluator {
    pub(crate) weights: StaticDotProductEvaluatorWeights,
}

pub struct TrainableDotProductEvaluator {
    pub(crate) weights: Arc<Vec<i32>>,
}