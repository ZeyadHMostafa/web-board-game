
use std::sync::Arc;

use crate::ai::{EvaluationScore, PositionEvaluator};
use crate::ai::heuristics::evaluators;
use crate::ai::models::static_dot::{
    TrainableDotProductEvaluator,
    utils::{compute_dot_product, generate_weights},
    StaticDotProductEvaluator,
};
use crate::luts::EngineLUTs;
use crate::rules::state::{Bitboard, GameState};

impl StaticDotProductEvaluator {
    /// Creates a new static evaluator using an externally provided weights matrix.
    pub const fn new(luts: &'static EngineLUTs, weights: [[[[i32; 2]; 3]; 6]; 3]) -> Self {
        Self {
            weights,
            luts,
        }
    }

    /// Generates the multi-dimensional weights matrix using procedural combination rules.
    /// Parity values (Even and Odd columns) remain identical as parity yields no effect here.
    pub const fn generate_weights(
        prospects: [[i32; 3]; 6],
        region_multipliers: [i32; 3],
    ) -> [[[[i32; 2]; 3]; 6]; 3]{
        generate_weights(prospects, region_multipliers)
    }
}

///Temp
fn calc_adjacency_score(board: Bitboard, luts: &EngineLUTs)->u32{
    ( board & (board >> 1) & luts.not_h_file).count_ones() + 
    ( board & (board >> 7) & luts.not_a_file).count_ones() + 
    ( board & (board >> 8) ).count_ones() + 
    ( board & (board >> 9) & luts.not_h_file).count_ones() 
}

fn calc_centrality_sore(board: Bitboard, luts: &EngineLUTs)->u32{
    (board & Bitboard(luts.centrality_rings[0])).count_ones() * 4+
    (board & Bitboard(luts.centrality_rings[1])).count_ones() * 5+
    (board & Bitboard(luts.centrality_rings[2])).count_ones() * 6+
    (board & Bitboard(luts.centrality_rings[3])).count_ones() * 7
}

fn simple_eval(luts: &EngineLUTs, state: &GameState) -> EvaluationScore {
    let (allied_pieces, enemy_pieces) = state.get_player_pieces_relative();
    let allied_piece_count = allied_pieces.count_ones() as i32; 
    let enemy_piece_count = enemy_pieces.count_ones() as i32;

    let adjancency_diff = 
        calc_adjacency_score(allied_pieces, luts) as i32 - 
        calc_adjacency_score(enemy_pieces, luts) as i32;

    let piece_diff =
        allied_piece_count - enemy_piece_count ;
    
    let progression_score =
        32 - ( allied_piece_count + enemy_piece_count );
    
    let centrality_diff =
        calc_centrality_sore(allied_pieces, luts) as i32 -
        calc_centrality_sore(enemy_pieces, luts) as i32;
        
    EvaluationScore::Value(piece_diff*progression_score*20+adjancency_diff*10+centrality_diff)
}

impl PositionEvaluator for StaticDotProductEvaluator {
    fn evaluate(&self, state: &GameState) -> EvaluationScore {
        // simple heuristic
        simple_eval(self.luts, state)

        // main function
        // compute_dot_product(state, self.luts, |t, s, r, p| {
        //     self.weights[t][s][r][p]
        // })
    }
}


impl TrainableDotProductEvaluator {
    pub fn new(luts: &'static EngineLUTs, weights: Arc<Vec<i32>>) -> Self {
        assert_eq!(weights.len(), 108, "Weight vector must contain exactly 108 elements.");
        Self {
            luts,
            weights,
        }
    }
}

impl PositionEvaluator for TrainableDotProductEvaluator {
    fn evaluate(&self, state: &GameState) -> EvaluationScore {
        // Pass a closure that calculates the flat index on the fly
        compute_dot_product(state, self.luts, |t, s, r, p| {
            let flat_idx = p + 2 * (r + 3 * (s + 6 * t));
            self.weights[flat_idx]
        })
    }
}