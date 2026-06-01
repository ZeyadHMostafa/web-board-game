use std::sync::Arc;
use crate::{ai::{models::static_dot::{DEFAULT_EVALUATOR_WEIGHTS, TrainableDotProductEvaluator}, search::algorithms::negamax::NegamaxSimulationAgent}, luts::EngineLUTs, rules::state::{GameState, Player}};


#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::rules::state::{GameState, Player};
    use crate::luts::EngineLUTs;
    use crate::ai::models::static_dot::{TrainableDotProductEvaluator, DEFAULT_EVALUATOR_WEIGHTS};
    use crate::ai::PositionEvaluator;
    use crate::ai::search::algorithms::negamax::NegamaxSimulationAgent;

    /// Helper function to flatten your constant matrix into the flat Arc<Vec<i32>> 
    /// that TrainableDotProductEvaluator expects, preserving the precise 
    /// loop indexing layout: flat_idx = p + 2 * (r + 3 * (s + 6 * t))
    fn get_flattened_default_weights() -> Arc<Vec<i32>> {
        let mut flat = Vec::with_capacity(108);
        for t in 0..3 {
            for s in 0..6 {
                for r in 0..3 {
                    for p in 0..2 {
                        flat.push(DEFAULT_EVALUATOR_WEIGHTS[t][s][r][p]);
                    }
                }
            }
        }
        Arc::new(flat)
    }

    #[test]
    fn run_comprehensive_evaluator_diagnostics() {
        let luts = EngineLUTs::get_engine_luts();
        let flat_weights = get_flattened_default_weights();
        
        let trainable_evaluator = TrainableDotProductEvaluator::new(luts, flat_weights);

        println!("\n==================================================");
        println!("STARTING MODEL PERSPECTIVE SYMMETRY DIAGNOSTIC");
        println!("==================================================");

        // --------------------------------------------------------------------
        // TEST CASE 1: Baseline Symmetry Verification
        // Create an asymmetric board: P1 is heavily winning.
        // P1 has multiple active pieces (0x7), P2 has only one piece (0x80).
        // --------------------------------------------------------------------
        let state_p1_turn = GameState::new(0x7, 0x80, Player::P1);
        
        // Exact mirror position: P2 has the material advantage, and it's P2's turn.
        let state_p2_turn = GameState::new(0x80, 0x7, Player::P2);

        println!("Phase 1: Evaluating Raw States via PositionEvaluator...");
        let eval_p1 = trainable_evaluator.evaluate(&state_p1_turn);
        let eval_p2 = trainable_evaluator.evaluate(&state_p2_turn);
        
        println!("  P1 Turn (Advantage P1) Raw Score: {:?}", eval_p1);
        println!("  P2 Turn (Advantage P2) Raw Score: {:?}", eval_p2);

        // --------------------------------------------------------------------
        // TEST CASE 2: Feature Matrix Permutation Checks
        // --------------------------------------------------------------------
        println!("\nPhase 2: Verifying Simulation Feature Maps...");
        let features_p1 = NegamaxSimulationAgent::extract_features(&state_p1_turn, luts);
        let features_p2 = NegamaxSimulationAgent::extract_features(&state_p2_turn, luts);

        // Verify length match
        assert_eq!(features_p1.len(), 108, "Features vector length must be 108");
        assert_eq!(features_p2.len(), 108, "Features vector length must be 108");

        // Scan if feature arrays are structurally completely identical
        let mut mismatch_count = 0;
        for i in 0..108 {
            if (features_p1[i] - features_p2[i]).abs() > 1e-5 {
                mismatch_count += 1;
            }
        }
        println!("  Feature elements mismatch between mirrored states: {}", mismatch_count);

        // --------------------------------------------------------------------
        // TEST CASE 3: Dot Product Re-Verification (Manual vs Evaluator)
        // --------------------------------------------------------------------
        println!("\nPhase 3: Calculating Flattened Weight Alignment...");
        
        let mut manual_dot_p1 = 0.0f32;
        let trainable_weights = trainable_evaluator.weights.as_ref();

        for i in 0..108 {
            manual_dot_p1 += features_p1[i] * (trainable_weights[i] as f32);
        }
        println!("  Manual Dot Product of P1 Features with Trainable Weights: {}", manual_dot_p1);

        // ====================================================================
        // PASS/FAIL VALIDATION ASSERTIONS
        // ====================================================================
        
        // Assertion A: The raw evaluations must yield identical scores 
        // because both views describe an identical advantage relative to the moving player.
        assert_eq!(
            eval_p1, eval_p2, 
            "CRITICAL SYMMETRY FAILURE: Mirrored positions yielded different values!"
        );

        // Assertion B: The extracted training features MUST match perfectly.
        assert_eq!(
            features_p1, features_p2,
            "CRITICAL FEATURE BREAK: Features do not match when swapping active players!"
        );
        
        println!("==================================================");
        println!("DIAGNOSTIC COMPLETE: Symmetries Are Intact.");
        println!("==================================================\n");
    }
}

#[test]
fn test_agent_target_score_sanity() {
    let luts = EngineLUTs::get_engine_luts();
    
    // Construct a flat weight array from DEFAULT_EVALUATOR_WEIGHTS
    let mut flat = Vec::with_capacity(108);
    for t in 0..3 {
        for s in 0..6 {
            for r in 0..3 {
                for p in 0..2 {
                    flat.push(DEFAULT_EVALUATOR_WEIGHTS[t][s][r][p]);
                }
            }
        }
    }
    let evaluator = Arc::new(TrainableDotProductEvaluator::new(luts, Arc::new(flat)));
    
    // Instantiate our simulation agent (epsilon = 0.0 to focus on competitive targets)
    let search_depth = 4;
    let agent = NegamaxSimulationAgent::new(luts, evaluator, search_depth, 0.0);

    println!("\n==================================================");
    println!("RUNNING SIMULATION AGENT TARGET SCORE SANITY");
    println!("==================================================");

    // --------------------------------------------------------------------
    // SITUATION 1: Active Player is completely winning (Forced Mate in 1)
    // --------------------------------------------------------------------
    // Let's assume P1 has a setup that forces an immediate tactical win.
    // Replace these bitboards with your actual test position values if needed
    let winning_state = GameState::new(0x7, 0x80, Player::P1);
    
    let (_move_w, _feat_w, target_winning) = agent.evaluate_and_select(&winning_state)
        .expect("Agent failed to return candidates for winning position");
    
    println!("  Winning State Target Score (Fed to Python): {}", target_winning);

    // --------------------------------------------------------------------
    // SITUATION 2: Active Player is completely losing (Forced Mated in 1)
    // --------------------------------------------------------------------
    // Now look from P2's chair where they are facing unavoidable defeat
    let losing_state = GameState::new(0x30400, 0x03, Player::P2);
    
    let (_move_l, _feat_l, target_losing) = agent.evaluate_and_select(&losing_state)
        .expect("Agent failed to return candidates for losing position");
    
    println!("  Losing State Target Score (Fed to Python):  {}", target_losing);

    println!("==================================================");

    // --------------------------------------------------------------------
    // THE MATHEMATICAL CONTRACT THAT MATTERS FOR PYTHON
    // --------------------------------------------------------------------
    assert!(target_winning > 0.0, "Winning positions must yield positive targets!");
    assert!(target_losing < 0.0, "Losing positions must yield negative targets!");
}