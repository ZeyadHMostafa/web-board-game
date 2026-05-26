use crate::rules::luts::EngineLUTs;
use crate::rules::state::{GameState, Player};
use crate::heuristics::evaluators::EvaluationEngine;
use crate::ai::models::{PositionEvaluator, EvaluationScore};

pub struct StaticDotProductEvaluator {
    luts: &'static EngineLUTs,
    engine: EvaluationEngine,
    weights: [ [ [ [i32; 2]; 3]; 6]; 3],
}

impl StaticDotProductEvaluator {
    /// Creates a new static evaluator with pre-configured weights.
    pub const fn new(luts: &'static EngineLUTs) -> Self {
        Self {
            engine: EvaluationEngine::new(),
            // Dimensions: [TileType: 3][SovereigntyState: 6][RegionType: 3][ParityType: 2]
            // Parity Columns:  Even, Odd
            // Region Rows:    Corner, Edge, Center
            #[rustfmt::skip]
            weights: [
                // ====================================================================
                // TILE TYPE: Empty (0)
                // ====================================================================
                [
                    /* AllyDominates */    [[ 10,  10], [ 10,  10], [ 10,  10]],
                    /* EnemyDominates */   [[-10, -10], [-10, -10], [-10, -10]],
                    /* AllyUncontested */  [[ 08,  08], [ 08,  08], [ 08,  08]],
                    /* EnemyUncontested */ [[-08, -08], [-08, -08], [-08, -08]],
                    /* TiedConflict */     [[ 00,  00], [ 00,  00], [ 00,  00]],
                    /* NoConflict */       [[ 00,  00], [ 00,  00], [ 00,  00]],
                ],
                // ====================================================================
                // TILE TYPE: AlliedPiece (1)
                // ====================================================================
                [
                    /* AllyDominates */    [[ 15,  15], [ 15,  15], [ 15,  15]],
                    /* EnemyDominates */   [[-30, -30], [-30, -30], [-30, -30]],
                    /* AllyUncontested */  [[ 20,  20], [ 20,  20], [ 20,  20]],
                    /* EnemyUncontested */ [[-45, -45], [-45, -45], [-45, -45]],
                    /* TiedConflict */     [[-05, -05], [-05, -05], [-05, -05]],
                    /* NoConflict */       [[ 05,  05], [ 05,  05], [ 05,  05]],
                ],
                // ====================================================================
                // TILE TYPE: EnemyPiece (2)
                // ====================================================================
                [
                    /* AllyDominates */    [[ 30,  30], [ 30,  30], [ 30,  30]],
                    /* EnemyDominates */   [[-05, -05], [-05, -05], [-05, -05]],
                    /* AllyUncontested */  [[ 45,  45], [ 45,  45], [ 45,  45]],
                    /* EnemyUncontested */ [[-05, -05], [-05, -05], [-05, -05]],
                    /* TiedConflict */     [[ 05,  05], [ 05,  05], [ 05,  05]],
                    /* NoConflict */       [[ 00,  00], [ 00,  00], [ 00,  00]],
                ],
            ],
            luts: luts
        }
    }
}

impl PositionEvaluator for StaticDotProductEvaluator {
    fn evaluate(&self, state: &GameState) -> EvaluationScore {
        // Run the high-performance digital comparator pipeline
        // Assumes state.luts is accessible via your game context or globally
        let matrix = self.engine.evaluate_position(
            if state.active_player == Player::P1 { state.p1_pieces} else { state.p2_pieces}, 
            if state.active_player == Player::P2 { state.p1_pieces} else { state.p2_pieces}, 
            &self.luts
        );

        let mut total_score: i32 = 0;

        // Vectorized dot product execution over the 108 structural features
        for t in 0..3 {
            for s in 0..6 {
                for r in 0..3 {
                    for p in 0..2 {
                        let count = matrix.values[t][s][r][p] as i32;
                        let weight = self.weights[t][s][r][p];
                        total_score += count * weight;
                    }
                }
            }
        }

        EvaluationScore::Value(total_score)
    }
}