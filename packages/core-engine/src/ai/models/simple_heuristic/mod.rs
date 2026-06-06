

use crate::ai::{EvaluationScore, PositionEvaluator};
use crate::luts::LUTS;
use crate::rules::state::{Bitboard, GameState};

pub struct SimpleHeuristicPositionEvaluator{}

impl SimpleHeuristicPositionEvaluator {
    /// Creates a new static evaluator.
    pub const fn new() -> Self {
        Self {}
    }
}

/// Calculates piece connectiity
fn calc_adjacency_score(board: Bitboard)->u32{
    let luts = &LUTS;
    ( board & (board >> 1) & luts.not_h_file).count_ones() + 
    ( board & (board >> 7) & luts.not_a_file).count_ones() + 
    ( board & (board >> 8) ).count_ones() + 
    ( board & (board >> 9) & luts.not_h_file).count_ones() 
}

/// Calculates piece centrality
fn calc_centrality_sore(board: Bitboard)->u32{
    let luts = &LUTS;
    (board & Bitboard(luts.centrality_rings[0])).count_ones() * 4+
    (board & Bitboard(luts.centrality_rings[1])).count_ones() * 5+
    (board & Bitboard(luts.centrality_rings[2])).count_ones() * 6+
    (board & Bitboard(luts.centrality_rings[3])).count_ones() * 7
}

impl PositionEvaluator for SimpleHeuristicPositionEvaluator {
    fn evaluate(&self, state: &GameState) -> EvaluationScore {
        const PROGRESS_BASE_VALUE:i32 = 32;
        const PROGRESS_FACTOR:i32 = 20;
        const ADJACENCY_FACTOR:i32 = 10;

        let (allied_pieces, enemy_pieces) = state.get_player_pieces_relative();
        let allied_piece_count = allied_pieces.count_ones() as i32; 
        let enemy_piece_count = enemy_pieces.count_ones() as i32;

        let adjancency_diff = 
            calc_adjacency_score(allied_pieces) as i32 
            - calc_adjacency_score(enemy_pieces) as i32;

        let piece_diff =
            allied_piece_count - enemy_piece_count ;
        
        let progression_score =
            PROGRESS_BASE_VALUE - ( allied_piece_count + enemy_piece_count );
        
        let centrality_diff =
            calc_centrality_sore(allied_pieces) as i32
            - calc_centrality_sore(enemy_pieces) as i32;
            
        EvaluationScore::Value(
            piece_diff * progression_score * PROGRESS_FACTOR
            + adjancency_diff * ADJACENCY_FACTOR
            + centrality_diff
        )
    }
}