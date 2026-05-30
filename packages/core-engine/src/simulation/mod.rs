pub mod environment;
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
    use std::time::Duration;
    use crate::rules::state::GameState;
    use crate::rules::moves::Move;

    /// Keeps track of the remaining match time allocations.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct GameClock {
        pub active_player_time: Duration,
        pub opponent_time: Duration,
        pub increment: Duration,
    }

    /// The unified asynchronous engine blackbox interface.
    /// This abstract agent can be wrapped by self-play loops, web servers, 
    /// or WebAssembly UI components.
    pub trait Agent: Send + Sync {
        /// Computes the absolute best move given the current state and tracking context.
        fn select_move(
            &self, 
            state: &GameState, 
            clock: Option<GameClock>
        ) -> impl std::future::Future<Output = Result<Move, String>> + Send;
    }
}