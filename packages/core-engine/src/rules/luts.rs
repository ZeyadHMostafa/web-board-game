use crate::rules::bitboard::Bitboard;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DiagonalRays {
    pub ne: Bitboard,
    pub nw: Bitboard,
    pub se: Bitboard,
    pub sw: Bitboard,
}

// ============================================================================
// ENGINE LOOK-UP TABLES (LUTs) IMPLEMENTATION
// ============================================================================

pub struct EngineLUTs {
    pub neighborhood_rotation_lut: [u8; 128],
    pub cardinal_offset_lut: [Bitboard; 16],
    pub(crate) diagonal_ray_lut: [DiagonalRays; 64],
    pub not_a_file: Bitboard,
    pub not_h_file: Bitboard,
}

impl EngineLUTs {
    /// Instantiates the static precalculated lookup architecture.
    /// Can be executed within a `const` context or once at startup.
    pub const fn new() -> Self {
        Self {
            neighborhood_rotation_lut: Self::generate_rotation_evaluator(),
            cardinal_offset_lut: Self::generate_relative_move_masks(),
            diagonal_ray_lut: Self::generate_diagonal_rays(),
            not_a_file: Bitboard(0xFEFEFEFEFEFEFEFE),
            not_h_file: Bitboard(0x7F7F7F7F7F7F7F7F),
        }
    }

    const fn generate_rotation_evaluator() -> [u8; 128] {
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

    const fn generate_relative_move_masks() -> [Bitboard; 16] {
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
    const fn generate_diagonal_rays() -> [DiagonalRays; 64] {
        let mut lut = [DiagonalRays { 
            ne: Bitboard::EMPTY, 
            nw: Bitboard::EMPTY, 
            se: Bitboard::EMPTY, 
            sw: Bitboard::EMPTY 
        }; 64];
        let mut sq = 0;
        
        while sq < 64 {
            lut[sq] = DiagonalRays {
                ne: Self::generate_ray(sq, 1, 1),
                nw: Self::generate_ray(sq, -1, 1),
                se: Self::generate_ray(sq, 1, -1),
                sw: Self::generate_ray(sq, -1, -1),
            };
            sq += 1;
        }
        
        lut
    }

    /// Helper utility to project a raw structural ray across board boundaries.
    const fn generate_ray(square: usize, file_step: i32, rank_step: i32) -> Bitboard {
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
}