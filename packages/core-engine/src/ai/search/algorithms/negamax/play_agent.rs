use std::sync::Arc;
use std::sync::RwLock;

use crate::ai::search::controllers::IterativeDeepeningController;
use crate::ai::search::{SearchContext, SearchProgress};
use crate::ai::search::selector::{ActionSelector, SelectorMode, Difficulty};
use crate::rules::state::GameState;
use crate::rules::moves::Move;
use crate::simulation::Agent;

pub struct NegamaxPlayAgent {
    min_depth: usize,
    max_depth: usize,
    difficulty: Difficulty,
}

impl NegamaxPlayAgent {
    /// Creates a new Negamax agent configured with explicit search parameters and difficulty scaling rules.
    pub fn new(
        min_depth: usize,
        max_depth: usize,
        difficulty: Difficulty,
    ) -> Self {
        Self {
            min_depth,
            max_depth,
            difficulty,
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

    /// Selects the best move choice modulated by the designated structural difficulty configuration.
    fn select_move(&self, progress: &SearchProgress) -> Result<Move, String> {
        let mode = SelectorMode::AdaptiveDifficulty(self.difficulty);
        match ActionSelector::select_move(progress, mode) {
            Some(m) => Ok(m),
            None => Err("Search failed to converge on a valid move choice.".to_string()),
        }
    }
}