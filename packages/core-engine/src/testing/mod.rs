use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use crate::ai::EvaluationScore;
use crate::ai::search::algorithms::negamax::NegamaxPlayAgent;
use crate::ai::search::{SearchContext, SearchProgress};
use crate::ai::models::static_dot::{StaticDotProductEvaluator, DEFAULT_EVALUATOR_WEIGHTS};
use crate::rules::moves::Move;
use crate::rules::state::GameState;
use crate::luts::EngineLUTs;
use crate::simulation::Agent;
mod sim_test;

pub struct DiagnosticMetrics {
    pub duration_ms: u128,
    pub total_nodes: usize,
    pub final_depth: usize,
    pub branching_factor: f64,
    pub nodes_per_second: f64,
}

/// Executes a synchronous diagnostic profiling pass on a targeted game state.
pub fn run_position_diagnostic(
    initial_state: &GameState,
    target_depth: usize,
) -> (Result<Move, String>, DiagnosticMetrics) {
    let luts = EngineLUTs::get_engine_luts();
    let evaluator = Arc::new(StaticDotProductEvaluator::new(
        luts,
        DEFAULT_EVALUATOR_WEIGHTS,
    ));

    // Initialize the synchronized communication primitives
    let cancelled = Arc::new(AtomicBool::new(false));
    let shared_progress = Arc::new(RwLock::new(SearchProgress {
        candidates: Vec::new(),
        depth_reached: 0,
        nodes_explored: 0,
        branching_factor: 0.0,
    }));

    let ctx = SearchContext {
        cancelled: cancelled.as_ref(),
        evaluator: evaluator.as_ref(),
        luts,
    };

    let agent = NegamaxPlayAgent::new( target_depth, target_depth);

    println!("--------------------------------------------------");
    println!("Executing engine diagnostic profiling...");
    println!("Target Depth Limit: {}", target_depth);
    println!("Active Player: {:?}", initial_state.active_player);
    println!("--------------------------------------------------");

    let start_time = Instant::now();
    
    // Execute the search synchronously on the current thread
    agent.search_position(initial_state, &ctx, shared_progress.clone());
    
    let duration = start_time.elapsed();
    let duration_ms = duration.as_millis();

    // Read the compiled metrics out from the shared progress canvas
    let final_progress = shared_progress.read().unwrap().clone();
    let chosen_move_result = agent.select_move(&final_progress);

    let total_nodes = final_progress.nodes_explored;
    let seconds = duration.as_secs_f64();
    let nodes_per_second = if seconds > 0.0 { total_nodes as f64 / seconds } else { 0.0 };

    let metrics = DiagnosticMetrics {
        duration_ms,
        total_nodes,
        final_depth: final_progress.depth_reached,
        branching_factor: final_progress.branching_factor,
        nodes_per_second,
    };

    print_diagnostic_report(&metrics, &final_progress);

    (chosen_move_result, metrics)
}

/// Formats and outputs the profiling summary to the standard output buffer.
fn print_diagnostic_report(metrics: &DiagnosticMetrics, progress: &SearchProgress) {
    println!("\n=== DIAGNOSTIC SEARCH COMPLETE ===");
    println!("Time Elapsed:         {} ms", metrics.duration_ms);
    println!("Max Depth Reached:    {} plies", metrics.final_depth);
    println!("Total Nodes Explored: {}", metrics.total_nodes);
    println!("Performance Rate:     {:.2} nodes/sec", metrics.nodes_per_second);
    println!("Effective Branching:  {:.4}", metrics.branching_factor);
    println!("--------------------------------------------------");
    println!("Root Layer Move Evaluations:");
    
    let mut sorted_candidates = progress.candidates.clone();
    sorted_candidates.sort_by_key(|c| match c.score {
        EvaluationScore::Value(v) => -v,
        EvaluationScore::Mating(_) => i32::MIN,
        EvaluationScore::Mated(_) => i32::MAX,
    });

    for candidate in sorted_candidates {
        println!(
            "  Move: {:?} | Evaluation Score: {:?}",
            candidate.current_move, candidate.score
        );
    }
    println!("==================================================\n");
}

