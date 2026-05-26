pub mod environment;
pub mod parallel;

// Re-export the core interfaces
pub use agent::{Agent, GameClock, SearchContext};

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

    /// Bundles context needed to safely restrict a search operation.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct SearchContext {
        pub clock: Option<GameClock>,
        pub max_depth: Option<usize>,
    }

    /// The unified asynchronous engine blackbox interface.
    /// This abstract agent can be wrapped by self-play loops, web servers, 
    /// or WebAssembly UI components.
    pub trait Agent: Send + Sync {
        /// Computes the absolute best move given the current state and tracking context.
        fn select_move(
            &self, 
            state: &GameState, 
            context: &SearchContext
        ) -> impl std::future::Future<Output = Result<Move, String>> + Send;
    }
}