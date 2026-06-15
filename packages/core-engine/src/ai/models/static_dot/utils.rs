use crate::{
    ai::{EvaluationScore, heuristics::{FeatureMatrix, SovereigntyState, evaluators}, models::static_dot::StaticDotProductEvaluatorWeights}, 
    rules::state::GameState
};

/// Global pre-computed weights matrix for quick initialization across files.
pub const DEFAULT_EVALUATOR_WEIGHTS: StaticDotProductEvaluatorWeights
    = StaticDotProductEvaluatorWeights(generate_weights(
    // Base action prospects based on TileType and SovereigntyState
    // Dimensions: [SovereigntyState: 6][TileType: 3]
    [
        //emty ally enmy
        [  03,  50, -15  ], // ally-unc
        [  01,  45,  25  ], // ally-dom
        [  00,  40,  40  ], // conf-non
        [  00,  35, -35  ], // conf-tie
        [ -01,  30, -45  ], // enmy-dom
        [ -03,  20, -50  ], // enmy-unc
    ],
    // Multipliers for each RegionType: Corner, Edge, Center
    [10, 11, 12],
));

/// Generates the multi-dimensional weights matrix using procedural combination rules.
/// Parity values (Even and Odd columns) remain identical as parity yields no effect here.   
pub(crate) const fn generate_weights(
    prospects: [[i32; 3]; 6],
    region_multipliers: [i32; 3],
) -> [[[[i32; 2]; 3]; 6]; 3] {
    let mut matrix = [[[[0; 2]; 3]; 6]; 3];

    let mut t = 0;
    while t < 3 {
        let mut s = 0;
        while s < 6 {
            let mut r = 0;
            while r < 3 {
                let final_score = prospects[s][t] * region_multipliers[r];

                matrix[t][s][r][0] = final_score;
                matrix[t][s][r][1] = final_score;

                r += 1;
            }
            s += 1;
        }
        t += 1;
    }

    matrix
}

/// Extracts structural, spatial, and territorial metrics out of the current game state.
/// Maps bitboard allocations directly onto flat multidimensional feature partitions.
#[inline(always)]
pub(crate) fn evaluate_state_features(state: &GameState) -> FeatureMatrix {
    let (allied_pieces, enemy_pieces) = state.get_player_pieces_relative();
    let raw_matrix = evaluators::evaluate_position(
        allied_pieces,
        enemy_pieces
    );
    
    FeatureMatrix { values: raw_matrix.values }
}

/// Executes a vectorized dot product calculation over a standardized 108-element feature tensor.
/// Relies entirely on external injection for state feature maps and weight indices.
#[inline(always)]
pub(crate) fn compute_dot_product<W>(
    features: &FeatureMatrix,
    weight_lookup: W,
) -> EvaluationScore
where
    W: Fn(usize, usize, usize, usize) -> i32,
{
    let mut total_score: i32 = 0;
    let mut player_move_count: u8 = 0;

    for t in 0..3 {
        for s in 0..6 {
            for r in 0..3 {
                for p in 0..2 {
                    let count = features.values[t][s][r][p] as i32;
                    let weight = weight_lookup(t, s, r, p);
                    total_score += count * weight;
                    
                    let is_active_zone = !(
                        s == SovereigntyState::NoConflict as usize ||
                        s == SovereigntyState::EnemyUncontested as usize
                    );
                    player_move_count += (is_active_zone as u8) * (count > 0) as u8;
                }
            }
        }
    }

    if player_move_count == 0 {
        EvaluationScore::Mated(0)
    } else {
        EvaluationScore::Value(total_score)
    }
}