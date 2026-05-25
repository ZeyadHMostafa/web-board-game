use crate::rules::bitboard::Bitboard;
use crate::rules::luts::EngineLUTs;

// ============================================================================
// HELPERS
// ============================================================================

/// Generates a raw absolute mask of all 4 cardinal squares adjacent to a piece index.
/// 
/// Left deliberately unintersected by any specific board state so outer callers 
/// can selectively mask it against allies, pivots, or custom evaluation shapes.
#[inline(always)]
pub fn get_orthogonal_mask(piece_idx: u8, luts: &EngineLUTs) -> Bitboard {
    let piece_mask = 1u64 << piece_idx;
    
    // Shift single piece bit globally in all 4 cardinal directions.
    // Invalid wrapping artifacts across files are cleanly vanished with file masks.
    let adjacent = ((piece_mask >> 8)                    ) | // North (-8 index offset)
                        ((piece_mask << 8)                    ) | // South (+8 index offset)
                        ((piece_mask << 1) & luts.not_a_file.0) | // East  (+1 index offset)
                        ((piece_mask >> 1) & luts.not_h_file.0);  // West  (-1 index offset)

    Bitboard::new(adjacent)
}

/// Fast un-clipped neighborhood sampler.
/// 
/// # Safety / Hazards
/// This function relies on modular wrapping math (`rem_euclid`) to prevent panic errors near borders.
/// As a consequence, index reads close to edges wrap onto the opposite side of the board and return 
/// structurally invalid data. **Must** be followed up immediately by an outer boundary wall override.
#[inline(always)]
fn sample_neighborhood_square(pivot_idx: u8, r_off: i8, f_off: i8, occupancy: Bitboard) -> u8 {
    let target_idx = (pivot_idx as i8)
        .wrapping_add(r_off.wrapping_mul(8))
        .wrapping_add(f_off);
        
    let safe_idx = target_idx.rem_euclid(64) as u32;
    ((occupancy.0 >> safe_idx) & 1) as u8
}

pub fn generate_7bit_key(piece_idx: u8, pivot_idx: u8, occupancy: Bitboard) -> (u8, u8) {
    // Corrected coordinate steps: North adds 8 rows (+1, 0), South subtracts 8 rows (-1, 0)
    let n  = sample_neighborhood_square(pivot_idx,  1,  0, occupancy);
    let tr = sample_neighborhood_square(pivot_idx,  1,  1, occupancy);
    let e  = sample_neighborhood_square(pivot_idx,  0,  1, occupancy);
    let br = sample_neighborhood_square(pivot_idx, -1,  1, occupancy);
    let s  = sample_neighborhood_square(pivot_idx, -1,  0, occupancy);
    let bl = sample_neighborhood_square(pivot_idx, -1, -1, occupancy);
    let w  = sample_neighborhood_square(pivot_idx,  0, -1, occupancy);
    let tl = sample_neighborhood_square(pivot_idx,  1, -1, occupancy);

    let mut packed_neighborhood = (tl << 7) | (w << 6) | (bl << 5) | (s << 4) | (br << 3) | (e << 2) | (tr << 1) | n;

    // --- GLOBAL BOUNDARY OVERRIDE MASKING ---
    let pivot_rank = pivot_idx / 8;
    let pivot_file = pivot_idx % 8;

    if pivot_rank == 7 { packed_neighborhood |= 0b10000011; } // Top Edge (Rank 7): Wall off [TL, TR, N]
    if pivot_rank == 0 { packed_neighborhood |= 0b00111000; } // Bottom Edge (Rank 0): Wall off [BL, BR, S]
    if pivot_file == 0 { packed_neighborhood |= 0b11100000; } // Left Edge (File 0): Wall off [TL, W, BL]
    if pivot_file == 7 { packed_neighborhood |= 0b00001110; } // Right Edge (File 7): Wall off [TR, E, BR]

    // --- THE BRANCHLESS OFFSET HASH ---
    let d_u8 = (piece_idx as i8).wrapping_sub(pivot_idx as i8) as u8;
    let initial_offset_type = (d_u8 & 1) | ((d_u8 >> 3) & 2);

    let shift_amount = initial_offset_type << 1;
    let aligned_neighborhood = packed_neighborhood.rotate_right(shift_amount as u32);

    let key = (aligned_neighborhood >> 1) & 0x7F;

    (key, initial_offset_type)
}

#[inline(always)]
pub(crate) fn shift_and_clip_mask(local_mask: u64, pivot_idx: u8, luts: &EngineLUTs) -> Bitboard {
    const LOCAL_ORIGIN_IDX: i32 = 28;
    
    // --- 1. BRANCHLESS SPATIAL TRANSLATION ---
    // Calculate the signed distance from our template origin
    let shift_delta = pivot_idx as i32 - LOCAL_ORIGIN_IDX;
    
    // To avoid branching between left-shift and right-shift, we use a 128-bit space.
    // We position our 64-bit mask directly in the middle of a u128, apply a single 
    // unified shift (adding 64 to our delta ensures it's always a positive left-shift),
    // and then harvest the upper/lower bits.
    let absolute_128 = (local_mask as u128) << (64 + shift_delta);
    let mut abs_mask = (absolute_128 >> 64) as u64;

    // --- 2. BRANCHLESS EDGE-WRAP CLIPPING ---
    let pivot_file = (pivot_idx & 7) as i32;

    // Create a bitmask that evaluates to a solid block of 1s if pivot_file == 0, else 0s.
    // (pivot_file == 0) turns into 1 or 0, negating it flips it to -1 (0xFFFFFFFF) or 0.
    let is_a_file_mask = -((pivot_file == 0) as i64) as u64;
    // Do the exact same for the H-file (pivot_file == 7)
    let is_h_file_mask = -((pivot_file == 7) as i64) as u64;

    // Apply the correction masks using bitwise selection:
    // If on the A-file, clear out the non-A-file bits. Otherwise, leave abs_mask completely untouched.
    abs_mask &= !(is_a_file_mask & luts.not_a_file.0);
    abs_mask &= !(is_h_file_mask & luts.not_h_file.0);

    Bitboard::new(abs_mask)
}

/// Shifts a local relative 4-bit arrival mask back into absolute directional coordinates.
#[inline(always)]
pub fn align_relative_mask(relative_mask: u8, initial_offset_type: u8) -> u8 {
    // Isolate the lower 4 cardinal bits
    let mask = relative_mask & 0x0F;
    
    // Perform a 4-bit barrel ring rotation using the approach offset.
    // This shifts bits cleanly across boundaries without any branch overhead.
    let shifted = (mask << initial_offset_type) | (mask >> (4 - initial_offset_type));
    
    // Clean up any overflow bits outside our 4-bit cardinal structure
    shifted & 0x0F
}

/// Core raycasting engine for diagonal frog-hopping tracks.
/// 
/// Evaluates an arbitrary chain of contiguous allied obstacles along a precomputed 
/// vector mask using parallel bit-smearing logic.
#[inline(always)]
pub(crate) fn compute_ray_moves<const IS_CONTROL_EVAL: bool, F>(
    ray: Bitboard,
    allied_blockers: Bitboard,
    piece_idx: u8,
    neighbor_shift: i32,
    _find_closest_bit: F, // No longer needed as we use parallel sequence evaluation
) -> Bitboard 
where
    F: Fn(Bitboard) -> u32,
{
    let piece_bit = 1u64 << piece_idx;
    let ray_allies = ray.0 & allied_blockers.0;

    // Calculate the square immediately neighboring our starting piece along this ray
    let immediate_neighbor = if neighbor_shift > 0 {
        piece_bit.checked_shl(neighbor_shift as u32).unwrap_or(0)
    } else {
        piece_bit.checked_shr((-neighbor_shift) as u32).unwrap_or(0)
    };

    // Rule: There MUST be an ally immediately next to us to initiate a valid frog-hop sequence
    if (immediate_neighbor & ray_allies) == 0 {
        return Bitboard::EMPTY;
    }

    let mut landing_square = 0u64;

    if neighbor_shift > 0 {
        // --- FORWARD SCANNING CHIPS (NE / NW) ---
        // Isolate the consecutive run of allies starting directly from our neighbor.
        // We use the classic carry-ripple trick: (allies | ~ray) fills everything outside the ray with 1s.
        // Adding the immediate neighbor bit will ripple a carry through the unbroken chain of 1s,
        // bursting out at the first open square past the allied sequence.
        let state_mask = ray_allies | !ray.0;
        let carry_ripple = state_mask.wrapping_add(immediate_neighbor);
        
        // The landing square is the bit that caught the carry explosion
        landing_square = carry_ripple & & !state_mask;
    } else {
        // --- BACKWARD SCANNING CHIPS (SE / SW) ---
        // For right-to-left shifts, a carry ripple moves the wrong way. Instead, we use bit-smearing.
        // We look for the first 0 (empty/enemy) on the ray by inverting the allies.
        let ray_holes = !ray_allies & ray.0;
        
        if ray_holes != 0 {
            // Find the highest active bit below our piece index (the closest hole)
            let closest_hole_idx = 63 - ray_holes.leading_zeros();
            landing_square = 1u64 << closest_hole_idx;
        }
    }

    // Edge check: Verify the discovered landing spot resides within the active geometric ray line
    if (landing_square & ray.0) == 0 {
        return Bitboard::EMPTY;
    }

    let landing_mask = Bitboard::new(landing_square);

    // Apply conditional filters based on evaluation flags
    if IS_CONTROL_EVAL {
        landing_mask
    } else {
        landing_mask & !allied_blockers
    }
}