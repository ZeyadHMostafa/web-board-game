use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::{ai::{PositionEvaluator, ScoredMove}, luts::EngineLUTs};

pub mod controllers;
pub mod algorithms;
pub(crate) mod selector;
pub(super) mod utils;

pub struct SearchContext<'a> {
    pub evaluator: &'a dyn PositionEvaluator,
    pub luts: &'static EngineLUTs,
    pub cancelled: &'a AtomicBool,
}

#[derive(Debug, Clone)]
pub struct SearchProgress {
    pub candidates: Vec<ScoredMove>,
    pub depth_reached: usize,
    pub nodes_explored: usize,
    pub branching_factor: f64,
}

pub trait SearchTelemetry: Send + Sync {
    fn record_node_explored(&self);
}

pub struct ActiveTelemetry {
    pub nodes_explored: AtomicUsize,
}

impl SearchTelemetry for ActiveTelemetry {
    #[inline(always)]
    fn record_node_explored(&self) {
        self.nodes_explored.fetch_add(1, Ordering::Relaxed);
    }
}

pub struct NoOpTelemetry;

impl SearchTelemetry for NoOpTelemetry {
    #[inline(always)]
    fn record_node_explored(&self) {}
}