use std::{f32::consts::E, sync::{Arc, atomic::{AtomicBool, AtomicUsize}}};

use core_engine::{
    ai::{EvaluationScore::{self, Mating, Value}, models::static_dot, search::{SearchContext, algorithms::negamax::NegamaxAgent}},
    rules::state::{GameState, Player},
    luts::EngineLUTs,
    simulation::Agent
};

const P1_START:u64 = 0x0000000000000ffff;
const P2_START:u64 = 0x0000000000000ffff;
fn main() {
    let state = GameState::new(P1_START, P2_START, Player::P1);
    let cancelled = Arc::new(AtomicBool::new(false));
    let nodes_explored = AtomicUsize::new(0);
    let evaluator = Arc::new(
            static_dot::StaticDotProductEvaluator::new(
                EngineLUTs::get_engine_luts(),
                static_dot::DEFAULT_EVALUATOR_WEIGHTS)
            );
    let ctx = SearchContext{
        cancelled: cancelled.as_ref(),
        evaluator: evaluator.as_ref(),
        luts: EngineLUTs::get_engine_luts(),
        nodes_explored: &nodes_explored
    };

    let agent = NegamaxAgent::new(
        EngineLUTs::get_engine_luts(),
        evaluator.clone(),8
    );
        
        
    let alpha = EvaluationScore::Value(i32::MIN);
    let beta = EvaluationScore::Value(i32::MAX);
    
    
    println!("Starting heavy profiling run...");
    // Force a deep search without a strict time limit to stress the CPU
    let result = agent.select_move(&state, None);
    let x = futures::executor::block_on(result);
    match x {
        Ok(m) => print!("success"),
        Err(e) => print!("error")
    }

    // let evaluation_score = negamax(&ctx, &state, 8, alpha, beta);
    // match evaluation_score {
    //     EvaluationScore::Value(v)=> print!("positional value: {}",v),
    //     EvaluationScore::Mating(n) => print!("winning in {} moves", n),
    //     EvaluationScore::Mated(n) => print!("losing in {} moves", n),
    // }
}