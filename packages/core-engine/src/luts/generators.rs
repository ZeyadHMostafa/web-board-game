use crate::{luts::DiagonalRays, rules::state::Bitboard};


pub(super) const fn generate_rotation_evaluator() -> [u8; 128] {
    let mut table = [0u8; 128];
    let mut key = 0;

    while key < 128 {
        let is_tr_occupied = (key & (1 << 0)) != 0;
        let is_e_occupied  = (key & (1 << 1)) != 0;
        let is_br_occupied = (key & (1 << 2)) != 0;
        let is_s_occupied  = (key & (1 << 3)) != 0;
        let is_bl_occupied = (key & (1 << 4)) != 0;
        let is_w_occupied  = (key & (1 << 5)) != 0;
        let is_tl_occupied = (key & (1 << 6)) != 0;

        let mut arrival_mask = 0u8;

        // Clockwise Path (North -> East -> South -> West)
        if !is_tr_occupied {
            arrival_mask |= 1 << 1; // East is reachable (Bit 1)
            if !is_e_occupied && !is_br_occupied {
                arrival_mask |= 1 << 2; // South is reachable (Bit 2)
                
                if !is_s_occupied && !is_bl_occupied {
                    arrival_mask |= 1 << 3; // West is reachable (Bit 3)
                }
            }
        }

        // Counter-Clockwise Path (North -> West -> South -> East)
        if !is_tl_occupied {
            arrival_mask |= 1 << 3; // West is reachable (Bit 3)
            
            if !is_w_occupied && !is_bl_occupied {
                arrival_mask |= 1 << 2; // South is reachable (Bit 2)
                
                if !is_s_occupied && !is_br_occupied {
                    arrival_mask |= 1 << 1; // East is reachable (Bit 1)
                }
            }
        }

        table[key] = arrival_mask;
        key += 1;
    }

    table
}

pub(super) const fn generate_relative_move_masks() -> [Bitboard; 16] {
    const LOCAL_ORIGIN_IDX: u8 = 28; 
    let mut table = [Bitboard::EMPTY; 16];
    let mut mask = 0;

    while mask < 16 {
        let mut bitboard_val = 0u64;

        let target_north = (mask & (1 << 0)) != 0; // Bit 0 = North
        let target_east  = (mask & (1 << 1)) != 0; // Bit 1 = East
        let target_south = (mask & (1 << 2)) != 0; // Bit 2 = South
        let target_west  = (mask & (1 << 3)) != 0; // Bit 3 = West

        if target_north { bitboard_val |= 1 << (LOCAL_ORIGIN_IDX + 8); }
        if target_south { bitboard_val |= 1 << (LOCAL_ORIGIN_IDX - 8); }
        if target_east  { bitboard_val |= 1 << (LOCAL_ORIGIN_IDX + 1); }
        if target_west  { bitboard_val |= 1 << (LOCAL_ORIGIN_IDX - 1); }

        table[mask] = Bitboard::new(bitboard_val);
        mask += 1;
    }

    table
}

/// Generates precalculated diagonal ray masks for all 64 spaces on the board.
pub(super) const fn generate_diagonal_rays() -> [DiagonalRays; 64] {
    let mut lut = [DiagonalRays { 
        ne: Bitboard::EMPTY, 
        nw: Bitboard::EMPTY, 
        se: Bitboard::EMPTY, 
        sw: Bitboard::EMPTY 
    }; 64];
    let mut sq = 0;
    
    while sq < 64 {
        lut[sq] = DiagonalRays {
            ne: generate_ray(sq, 1, 1),
            nw: generate_ray(sq, -1, 1),
            se: generate_ray(sq, 1, -1),
            sw: generate_ray(sq, -1, -1),
        };
        sq += 1;
    }
    
    lut
}

/// Helper utility to project a raw structural ray across board boundaries.
pub(super) const fn generate_ray(square: usize, file_step: i32, rank_step: i32) -> Bitboard {
    let mut ray_val: u64 = 0;
    
    let start_file = (square % 8) as i32;
    let start_rank = (square / 8) as i32;
    
    let mut f = start_file + file_step;
    let mut r = start_rank + rank_step;
    
    while f >= 0 && f < 8 && r >= 0 && r < 8 {
        let target_square = (r * 8 + f) as usize;
        ray_val |= 1u64 << target_square;
        
        f += file_step;
        r += rank_step;
    }
    
    Bitboard::new(ray_val)
}

pub(super) const fn generate_moore_neighborhood_lut() -> [Bitboard; 64] {
    let mut lut = [Bitboard::EMPTY; 64];
    let mut i = 0;
    
    // File masks to prevent left/right edge wrapping during shifting
    let not_a_file = 0xFEFEFEFEFEFEFEFE;
    let not_h_file = 0x7F7F7F7F7F7F7F7F;

    while i < 64 {
        let bit = 1u64 << i;
        let mut neighbors = 0u64;

        // North and South
        neighbors |= bit << 8;
        neighbors |= bit >> 8;

        // West and its diagonals (Clear H-File wrap)
        let west_bits = bit >> 1;
        if (west_bits & not_h_file) != 0 {
            neighbors |= west_bits;
            neighbors |= west_bits << 8;
            neighbors |= west_bits >> 8;
        }

        // East and its diagonals (Clear A-File wrap)
        let east_bits = bit << 1;
        if (east_bits & not_a_file) != 0 {
            neighbors |= east_bits;
            neighbors |= east_bits << 8;
            neighbors |= east_bits >> 8;
        }

        lut[i] = Bitboard::new(neighbors);
        i += 1;
    }
    
    lut
}

pub(super) const fn generate_topology_maps() -> [u8; 64] {
    let mut idx_table = [0u8; 64];
    let mut sq = 0;

    while sq < 64 {
        let rank = sq / 8;
        let file = sq % 8;

        let is_bottom = rank == 0;
        let is_top = rank == 7;
        let is_left = file == 0;
        let is_right = file == 7;
        let mut topo_idx = 0u8;

        if !is_bottom && !is_top && !is_left && !is_right {
            topo_idx = 0; // Center
        } else if is_bottom && !is_left && !is_right {
            topo_idx = 1; // Bottom Edge
        } else if is_top && !is_left && !is_right {
            topo_idx = 2; // Top Edge
        } else if is_left && !is_bottom && !is_top {
            topo_idx = 3; // Left Edge
        } else if is_right && !is_bottom && !is_top {
            topo_idx = 4; // Right Edge
        } else if is_bottom && is_left {
            topo_idx = 5; // BL Corner
        } else if is_bottom && is_right {
            topo_idx = 6; // BR Corner
        } else if is_top && is_left {
            topo_idx = 7; // TL Corner
        } else if is_top && is_right {
            topo_idx = 8; // TR Corner
        }

        idx_table[sq] = topo_idx;
        sq += 1;
    }

    idx_table
}

pub(super) const fn generate_centrality_lut() -> [u8; 64] {
    let mut lut = [0u8; 64];
    let mut i = 0;

    while i < 64 {
        let rank = i / 8;
        let file = i % 8;

        // Calculate distance from the center (3 or 4)
        let rank_dist = if rank < 4 { 3 - rank } else { rank - 4 };
        let file_dist = if file < 4 { 3 - file } else { file - 4 };

        // Manual max implementation for const context
        let dist = if rank_dist > file_dist { rank_dist } else { file_dist };

        lut[i] = 3 - dist as u8;
        i += 1;
    }
    
    lut
}

pub(super) const CENTRALITY_RINGS: [u64; 4] = {
    let mut rings = [0u64; 4];
    let mut i = 0;

    while i < 64 {
        let rank = i / 8;
        let file = i % 8;

        let rank_dist = if rank < 4 { 3 - rank } else { rank - 4 };
        let file_dist = if file < 4 { 3 - file } else { file - 4 };
        
        let dist = if rank_dist > file_dist { rank_dist } else { file_dist };
        let ring_idx = 3 - dist;

        rings[ring_idx] |= 1 << i;
        i += 1;
    }
    
    rings
};