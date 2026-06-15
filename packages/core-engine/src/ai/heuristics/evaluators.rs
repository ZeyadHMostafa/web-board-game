use crate::ai::heuristics::FeatureMatrix;
use crate::luts::LUTS;
use crate::rules::state::Bitboard;
use crate::rules::moves::Move;
use crate::rules::moves::generate_piece_moves;
use crate::ai::heuristics::{
    RegionType, ParityType, IntermediateBatch,
    StructuralMoveMap, VerticalDensityMap
};

// ============================================================================
// EVALUATION PIPELINE
// ============================================================================

#[inline(always)]
pub fn evaluate_move(m:&Move, allied_pieces: Bitboard, enemy_pieces: Bitboard) -> i32 {
    let from_sq = m.from_square() as usize;
    let to_sq = m.to_square() as usize;
    
    let to_mask = crate::rules::state::Bitboard::from_square(m.to_square());
    let is_capture = !(to_mask & enemy_pieces).is_empty();

    let mut move_score: i32 = 0;

    let moore_neighborhood_lut = &LUTS.moore_neighborhood_lut;
    let from_neighborhood = moore_neighborhood_lut[from_sq];
    let to_neighborhood = moore_neighborhood_lut[to_sq];
    let from_allied_density = (from_neighborhood & allied_pieces).count_ones() as i32;
    let to_allied_density = (to_neighborhood & allied_pieces).count_ones() as i32;
    let to_enemy_density = (to_neighborhood & enemy_pieces).count_ones() as i32;
    move_score += ( to_allied_density - from_allied_density )* 50;
    
    if is_capture {
        // Bonus for any capture event
        move_score += 5000;
        move_score += to_enemy_density * 100;
    }

    // centrality bonus
    move_score += LUTS.centrality_lut[to_sq] as i32 -LUTS.centrality_lut[from_sq] as i32 ;

    // Invert the score so the highest values sort to the front of the list
    -move_score
}

/// Coordinates generation passes across both perspectives using engine lookups, 
/// then passes the batches directly into the vector-optimized reducer.
pub fn evaluate_position(
    allied_pieces: Bitboard,
    enemy_pieces: Bitboard,
) -> FeatureMatrix {
    let ally_batch = generate_intermediate_assets(allied_pieces, enemy_pieces);
    let enemy_batch = generate_intermediate_assets(enemy_pieces, allied_pieces);

    reduce_assets_to_matrix(&ally_batch, &enemy_batch, allied_pieces, enemy_pieces)
}

/// Iterates through the active side's pieces to construct intermediate maps.
/// Executes with IS_CONTROL_EVAL permanently true to track the entire control profile.
fn generate_intermediate_assets(
    mut movers: Bitboard,
    targets: Bitboard,
) -> IntermediateBatch {
    let mut union_map = StructuralMoveMap::default();
    let mut density = VerticalDensityMap::default();

    let base_movers = movers;
    while !movers.is_empty() {
        let piece_idx = movers.pop_lsb();

        let piece_moves = generate_piece_moves::<true>(piece_idx, base_movers, targets);

        // Accumulate Global Union using custom operator traits
        union_map.collective_moves |= piece_moves;

        // Accumulate Parallel Vertical Density (Full-board Half-Adder Circuit)
        let carry_1 = density.layer_0 & piece_moves;
        density.layer_0 ^= piece_moves;

        let carry_2 = density.layer_1 & carry_1;
        density.layer_1 ^= carry_1;

        // Saturation: Layer 2 acts as the overflow catch. If a square reaches 8
        // attackers, we clamp the representation at 7 by leaving its bits set here.
        density.layer_2 |= carry_2;
    }

    IntermediateBatch { union_map, density }
}

/// Employs parallel digital magnitude comparators over both batch layers 
/// to divide the 64-bit board space into 6 mutually exclusive sovereignty masks.
fn reduce_assets_to_matrix(
    ally_batch: &IntermediateBatch,
    enemy_batch: &IntermediateBatch,
    allied_pieces: Bitboard,
    enemy_pieces: Bitboard,
) -> FeatureMatrix {
    let mut matrix = FeatureMatrix::new();

    // 1. Extract raw bitwise layers for both players
    let a0 = ally_batch.density.layer_0;
    let a1 = ally_batch.density.layer_1;
    let a2 = ally_batch.density.layer_2;

    let b0 = enemy_batch.density.layer_0;
    let b1 = enemy_batch.density.layer_1;
    let b2 = enemy_batch.density.layer_2;

    // 2. Perform parallel magnitude comparisons across all 64 squares simultaneously
    let eq2 = !(a2 ^ b2);
    let eq1 = !(a1 ^ b1);
    let eq0 = !(a0 ^ b0);
    
    // 1. Identify where control is tied or non-existent
    let absolute_equal_depth = eq2 & eq1 & eq0;
    let any_ally_control = a0 | a1 | a2;
    let any_enemy_control = b0 | b1 | b2;

    // 2. Extract raw Directional Dominance (Strictly Greater Control Depth)
    let ally_greater_mask = (a2 & !b2) | (eq2 & (a1 & !b1)) | (eq2 & eq1 & (a0 & !b0));
    let enemy_greater_mask = (b2 & !a2) | (eq2 & (b1 & !a1)) | (eq2 & eq1 & (b0 & !a0));

    // 3. Synthesize clean Sovereignty States based purely on control capability
    let ally_uncontested = ally_greater_mask & !any_enemy_control; // Only Ally can reach it
    let ally_dominates = ally_greater_mask & any_enemy_control;  // Overlapping, but Ally has more power
    
    let enemy_uncontested = enemy_greater_mask & !any_ally_control; // Only Enemy can reach it
    let enemy_dominates = enemy_greater_mask & any_ally_control; // Overlapping, but Enemy has more power

    let tied_conflict = absolute_equal_depth & any_ally_control;  // Overlapping and perfectly even depth
    let no_conflict = !(any_ally_control | any_enemy_control);    // Completely out of reach for both

    // Map synthesized masks directly to the SovereigntyState enum indexes
    let sovereignty_masks = [
        ally_uncontested,    // SovereigntyState::AllyUncontested
        ally_dominates,      // SovereigntyState::AllyDominates
        no_conflict,         // SovereigntyState::NoConflict
        tied_conflict,       // SovereigntyState::TiedConflict
        enemy_dominates,     // SovereigntyState::EnemyDominates
        enemy_uncontested,   // SovereigntyState::EnemyUncontested
    ];

    // 4. Synthesize the 3 Tile Type Placement Bitmasks
    let tile_masks = [
        !(allied_pieces | enemy_pieces), // TileType::Empty
        allied_pieces,                   // TileType::AlliedPiece
        enemy_pieces,                    // TileType::EnemyPiece
    ];

    let regions = [RegionType::Corner2x2, RegionType::Edge4x2, RegionType::Center4x4];
    let parities = [ParityType::Even, ParityType::Odd];

    // 5. Unrolled Parallel Masking Pass over Geographic and Structural Dimensions
    for &region in &regions {
        let r_mask = LUTS.regions.get(region);
        let r_idx = region as usize;

        for &parity in &parities {
            let p_mask = LUTS.parities.get(parity);
            let p_idx = parity as usize;

            let spatial_filter = r_mask & p_mask;

            for t_idx in 0..3 {
                let tile_filter = spatial_filter & tile_masks[t_idx];

                for s_idx in 0..6 {
                    let combined_filter = tile_filter & sovereignty_masks[s_idx];
                    
                    matrix.values[t_idx][s_idx][r_idx][p_idx] = 
                        combined_filter.count_ones() as u8;
                }
            }
        }
    }

    matrix
}
