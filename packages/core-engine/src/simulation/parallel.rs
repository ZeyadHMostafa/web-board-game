use rayon::prelude::*;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use crate::rules::state::{GameState, Player};
use crate::rules::luts::EngineLUTs;
use crate::ai::search::negamax::SearchContext;
use crate::ai::search::iterative::IterativeDeepeningController;
use crate::ai::models::static_dot::TrainableDotProductEvaluator;
use crate::ai::EvaluationScore;
use crate::simulation::{SimulationBatch, TrainingSample};
use crate::ai::search::utils::{ActionSelector, SelectorMode};

pub fn run_self_play_batch(
    luts: &'static EngineLUTs,
    evaluator: Arc<TrainableDotProductEvaluator>,
    num_games: usize,
    search_depth: usize,
    initial_state: GameState,
) -> SimulationBatch {
    let cancelled = std::sync::atomic::AtomicBool::new(false);
    const MAX_MOVES: usize = 100; // Hard cap to prevent infinite games

    let samples: Vec<TrainingSample> = (0..num_games)
        .into_par_iter()
        .flat_map(|_| {
            let mut current_state = initial_state.clone();
            let mut game_samples = Vec::new();

            // Tracking the last 4 unique configurations per player via sliding index stacks
            let mut p1_history = [crate::rules::bitboard::Bitboard::EMPTY; 4];
            let mut p2_history = [crate::rules::bitboard::Bitboard::EMPTY; 4];
            let mut move_counter = 0;
            let nodes_explored = AtomicUsize::new(0);

            let context = SearchContext {
                evaluator: evaluator.as_ref(),
                luts,
                cancelled: &cancelled,
                nodes_explored: &nodes_explored
            };
            
            let controller = IterativeDeepeningController::new(context, search_depth, search_depth);
           
            let extract_score_value = |score: EvaluationScore| match score {
                EvaluationScore::Value(v) => v,
                EvaluationScore::Mating(_) => i32::MAX,
                EvaluationScore::Mated(_) => i32::MIN,
            };

            let extract_features = |state: &GameState| {
                let engine = crate::heuristics::evaluators::EvaluationEngine::new();
                let matrix = engine.evaluate_position(
                    if state.active_player == Player::P1 { state.p1_pieces } else { state.p2_pieces },
                    if state.active_player == Player::P2 { state.p1_pieces } else { state.p2_pieces },
                    luts
                );

                let mut features = Vec::with_capacity(108);
                for t in 0..3 {
                    for s in 0..6 {
                        for r in 0..3 {
                            for p in 0..2 {
                                features.push(matrix.values[t][s][r][p] as f32);
                            }
                        }
                    }
                }
                features
            };

            let mut game_is_drawn = false;

            while !current_state.is_lost(luts) && move_counter < MAX_MOVES {
                // --- REPETITION CHECK ---
                let current_pieces = current_state.get_player_pieces(current_state.active_player);
                let history_ref = match current_state.active_player {
                    Player::P1 => &p1_history,
                    Player::P2 => &p2_history,
                };

                // If this arrangement has been seen in the player's last 4 moves, trigger a draw
                if history_ref.iter().any(|&past| past == current_pieces && !past.is_empty()) {
                    game_is_drawn = true;
                    break;
                }

                // Update the history ring buffer before making the move
                let history_index = (move_counter / 2) % 4;
                match current_state.active_player {
                    Player::P1 => p1_history[history_index] = current_pieces,
                    Player::P2 => p2_history[history_index] = current_pieces,
                }

                let search_result = controller.search(&current_state, None);
                if search_result.candidates.is_empty() {
                    break;
                }

                let best_candidate = search_result.candidates
                    .iter()
                    .max_by_key(|c| extract_score_value(c.score))
                    .unwrap();

                let mode = SelectorMode::TrainingExploration { epsilon: 0.15 };
                let chosen_move = match ActionSelector::select_move(&search_result, mode) {
                    Some(m) => m,
                    None => break,
                };

                let features = extract_features(&current_state);
                let target_score = match best_candidate.score {
                    EvaluationScore::Value(val) => (val as f32 / 1000.0).clamp(-900.0, 900.0),
                    EvaluationScore::Mating(n) =>  1000.0 -10.0*(n as f32) ,
                    EvaluationScore::Mated(n) => -(1000.0 -10.0*(n as f32)),
                };

                game_samples.push(TrainingSample {
                    features,
                    target_score,
                });

                current_state.make_move(chosen_move);
                move_counter += 1;
            }

            // Record terminal evaluation conditions
            if game_is_drawn || move_counter >= MAX_MOVES {
                // A draw penalizes passive looping: set target score to 0.0
                let terminal_features = extract_features(&current_state);
                game_samples.push(TrainingSample {
                    features: terminal_features,
                    target_score: 0.0,
                });
            } else if current_state.is_lost(luts) {
                let terminal_features = extract_features(&current_state);
                game_samples.push(TrainingSample {
                    features: terminal_features,
                    target_score: -1000.0,
                });
            }

            game_samples
        })
        .collect();

    SimulationBatch { samples }
}