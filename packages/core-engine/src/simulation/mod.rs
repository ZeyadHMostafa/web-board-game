pub mod environment;

#[cfg(feature = "python")]
pub mod parallel;

// Re-export the core interfaces
pub use agent::{Agent, GameClock};

pub struct TrainingSample {
    pub features: Vec<f32>,
    pub target_score: f32,
}

pub struct SimulationBatch {
    pub samples: Vec<TrainingSample>,
}

mod agent {
    use std::sync::{Arc, RwLock};
use std::time::Duration;
    use crate::ai::search::{SearchContext, SearchProgress};
use crate::rules::state::GameState;
    use crate::rules::moves::Move;

    /// Keeps track of the remaining match time allocations.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct GameClock {
        pub active_player_time: Duration,
        pub opponent_time: Duration,
        pub increment: Duration,
    }

    /// The unified synchronous engine interface.
    /// Execution environments (OS threads, Web Workers, or simulations) are responsible 
    /// for driving this interface and managing blocking or time thresholds via SearchContext.
    pub trait Agent: Send + Sync {
        /// Drives the internal controller to search a position, updating the shared progress reference in place.
        fn search_position(
            &self,
            state: &GameState,
            ctx: &SearchContext,
            shared_progress: Arc<RwLock<SearchProgress>>,
        );

        /// Evaluates the final compiled search progress block to isolate the chosen move.
        fn select_move(&self, progress: &SearchProgress) -> Result<Move, String>;
    }
}