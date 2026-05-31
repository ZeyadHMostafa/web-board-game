use std::sync::Arc;
use std::sync::RwLock;
use crate::ai::search::controllers::IterativeDeepeningController;
use crate::ai::search::{SearchContext, SearchProgress};
use crate::ai::search::selector::{ActionSelector, SelectorMode};
use crate::rules::state::GameState;
use crate::rules::moves::Move;
use crate::simulation::Agent;

pub struct NegamaxPlayAgent {
    min_depth: usize,
    max_depth: usize,
}

impl NegamaxPlayAgent {
    pub fn new(
        min_depth: usize,
        max_depth: usize,
    ) -> Self {
        Self {
            min_depth,
            max_depth,
        }
    }
}

impl Agent for NegamaxPlayAgent {

    /// Synchronously executes the iterative deepening sequence up to the maximum configuration limits.
    /// Progress evaluations are written directly to the shared tracking reference mid-execution.
    fn search_position(
        &self,
        state: &GameState,
        ctx: &SearchContext,
        shared_progress: Arc<RwLock<SearchProgress>>,
    ) {
        let controller = IterativeDeepeningController::new(
            ctx,
            self.min_depth,
            self.max_depth,
            shared_progress,
        );
        controller.search(state);
    }

    /// Selects the absolute best structural move from compiled progress.
    fn select_move(&self, progress: &SearchProgress) -> Result<Move, String> {
        match ActionSelector::select_move(progress, SelectorMode::Competitive) {
            Some(m) => Ok(m),
            None => Err("Search failed to converge on a valid move choice.".to_string()),
        }
    }
}