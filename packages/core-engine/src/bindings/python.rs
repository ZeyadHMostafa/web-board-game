#![cfg(feature = "python")]

use pyo3::prelude::*;
use numpy::{PyArray1, PyArray2, PyArrayMethods, PyReadonlyArray1, IntoPyArray};
use std::sync::Arc;
use crate::rules::luts;
use crate::ai::models::static_dot::TrainableDotProductEvaluator;
use crate::simulation::parallel::run_self_play_batch;
use crate::rules::state::{GameState, Player};

#[pymodule]
fn core_engine(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTrainingEnvironment>()?;
    Ok(())
}

#[pyclass]
pub struct PyTrainingEnvironment {
    luts: &'static luts::EngineLUTs,
    current_evaluator: Option<Arc<TrainableDotProductEvaluator>>,
}

#[pymethods]
impl PyTrainingEnvironment {
    #[new]
    fn new() -> Self {
        let luts_ptr = luts::EngineLUTs::get_engine_luts(); 
        Self {
            luts: luts_ptr,
            current_evaluator: None,
        }
    }

    fn update_weights(&mut self, weights: PyReadonlyArray1<f32>) -> PyResult<()> {
        let weights_view = weights.as_slice()?;
        if weights_view.len() != 108 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "The weight vector must contain exactly 108 elements.",
            ));
        }

        let scaling_factor = 1000.0;
        let mut int_weights = Vec::with_capacity(108);
        
        for &w in weights_view {
            int_weights.push((w * scaling_factor) as i32);
        }

        let evaluator = Arc::new(TrainableDotProductEvaluator::new(
            self.luts,
            Arc::new(int_weights),
        ));

        self.current_evaluator = Some(evaluator);
        Ok(())
    }

    fn run_simulation<'py>(
        &self,
        py: Python<'py>,
        num_games: usize,
        search_depth: usize,
        p1_start: u64,
        p2_start: u64,
    ) -> PyResult<(Bound<'py, PyArray2<f32>>, Bound<'py, PyArray1<f32>>)> {
        let evaluator = self.current_evaluator.as_ref().ok_or_else(|| {
            pyo3::exceptions::PyRuntimeError::new_err("Weights must be initialized before running simulations.")
        })?;

        let initial_state = GameState::new(p1_start, p2_start, Player::P1);

        // Explicitly use the type-safe multi-threaded runner to release the GIL
        let batch = py.detach(move || {
            run_self_play_batch(self.luts, evaluator.clone(), num_games, search_depth, initial_state)
        });

        let total_samples = batch.samples.len();
        let mut flat_features = Vec::with_capacity(total_samples * 108);
        let mut flat_targets = Vec::with_capacity(total_samples);

        for sample in batch.samples {
            flat_features.extend(sample.features);
            flat_targets.push(sample.target_score);
        }

        // Convert allocations directly to Bound PyArrays avoiding un-typed object conversions
        let features_shape = [total_samples, 108];
        let py_features = flat_features
            .into_pyarray(py)
            .reshape(features_shape)?;
            
        let py_targets = flat_targets.into_pyarray(py);

        Ok((py_features, py_targets))
    }
}