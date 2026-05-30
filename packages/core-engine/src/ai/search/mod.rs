use std::sync::atomic::{AtomicBool, AtomicUsize};

use crate::{ai::{PositionEvaluator, ScoredMove}, luts::EngineLUTs};

pub mod controllers;
pub mod algorithms;
pub(crate) mod selector;
pub(super) mod utils;

pub struct SearchContext<'a> {
    pub evaluator: &'a dyn PositionEvaluator,
    pub luts: &'static EngineLUTs,
    pub cancelled: &'a AtomicBool,
    pub nodes_explored: &'a AtomicUsize,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub candidates: Vec<ScoredMove>,
    pub depth_reached: usize,
    pub nodes_explored: usize,
    pub branching_factor: f64,
}