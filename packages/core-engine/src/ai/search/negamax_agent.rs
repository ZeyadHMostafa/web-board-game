use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;
use crate::ai::search::iterative::IterativeDeepeningController;
use crate::ai::search::negamax::SearchContext;
use crate::ai::search::utils::{ActionSelector, SelectorMode};
use crate::rules::luts;
use crate::rules::state::GameState;
use crate::rules::moves::Move;
use crate::simulation::{Agent, GameClock};
use crate::ai::models::PositionEvaluator;

pub struct NegamaxAgent {
    pub evaluator: Arc<dyn PositionEvaluator>,
    pub luts: &'static luts::EngineLUTs,
    pub max_depth: usize,
}

impl NegamaxAgent {
    pub fn new(luts: &'static luts::EngineLUTs, evaluator: Arc<dyn PositionEvaluator>, max_depth: usize) -> Self {
        Self {
            evaluator,
            luts,
            max_depth,
        }
    }

    /// Smart allocation of available thinking time
    fn calculate_time_budget(&self, clock: GameClock) -> Duration {
        let base_alloc = clock.active_player_time / 20;
        base_alloc + clock.increment
    }
}

impl Agent for NegamaxAgent {
    fn select_move(
        &self, 
        state: &GameState, 
        clock: Option<GameClock>
    ) -> impl std::future::Future<Output = Result<Move, String>> + Send {
        
        let state = state.clone();
        let evaluator = self.evaluator.clone();
        let luts = self.luts;
        let max_depth = self.max_depth;

        async move {
            let time_budget = clock.map(|c| self.calculate_time_budget(c));
            let cancelled = Arc::new(AtomicBool::new(false));
            let nodes_explored = AtomicUsize::new(0);
            
            let ctx = SearchContext {
                evaluator: evaluator.as_ref(),
                luts,
                cancelled: &cancelled,
                nodes_explored: &nodes_explored,
            };

            let controller = IterativeDeepeningController::new(ctx,2, max_depth);
            
            let timer_cancelled = cancelled.clone();
            if let Some(budget) = time_budget {
                std::thread::spawn(move || {
                    std::thread::sleep(budget);
                    timer_cancelled.store(true, Ordering::Relaxed);
                });
            }

            let search_result = controller.search(&state, time_budget);
            
            // Production deployment utilizes strict competitive maximization choices
            match ActionSelector::select_move(&search_result, SelectorMode::Competitive) {
                Some(m) => Ok(m),
                None => Err("Search failed to converge on a valid move choice.".to_string()),
            }
        }
    }
}