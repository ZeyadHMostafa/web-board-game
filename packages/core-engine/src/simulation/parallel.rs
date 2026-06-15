use rayon::prelude::*;
use std::sync::Arc;
use crate::ai::search::algorithms::negamax::NegamaxSimulationAgent;
use crate::luts::EngineLUTs;
use crate::rules::state::GameState;
use crate::ai::models::static_dot::TrainableDotProductEvaluator;
use crate::simulation::environment::SimulationEnvironment;
use crate::simulation::{SimulationBatch, TrainingSample};

pub fn run_self_play_batch(
    luts: &'static EngineLUTs,
    evaluator: Arc<TrainableDotProductEvaluator>,
    num_games: usize,
    search_depth: usize,
    initial_state: GameState,
) -> SimulationBatch {
    const MAX_MOVES: usize = 100;
    const EXPLORATION_EPSILON: f32 = 0.15;

    let environment = SimulationEnvironment::new(luts, MAX_MOVES);
    let agent = NegamaxSimulationAgent::new(evaluator, search_depth, EXPLORATION_EPSILON);

    let samples: Vec<TrainingSample> = (0..num_games)
        .into_par_iter()
        .flat_map(|_| {
            environment.run_game(initial_state.clone(), &agent)
        })
        .collect();

    SimulationBatch { samples }
}