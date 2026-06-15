use crate::rules::state::Bitboard;
mod generators;
use generators::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DiagonalRays {
    pub ne: Bitboard,
    pub nw: Bitboard,
    pub se: Bitboard,
    pub sw: Bitboard,
}

// ============================================================================
// SPATIAL MASKING CONTEXT
// ============================================================================

pub(crate) struct RegionalMasks {
    pub corners: Bitboard,
    pub edges:   Bitboard,
    pub center:  Bitboard,
}

impl RegionalMasks {
    /// Generates pre-computed masks representing fixed geometric partitions of the board.
    /// The layout differentiates corners, outer edge lines, and inner core zones.
    pub const fn new() -> Self {
        Self {
            corners: Bitboard(0xC3C300000000C3C3),
            edges:   Bitboard(0x3C3CC3C3C3C33C3C), 
            center:  Bitboard(0x00003C3C3C3C0000), 
        }
    }
}

pub(crate) struct ParityMasks {
    pub even: Bitboard,
    pub odd:  Bitboard,
}

impl ParityMasks {
    /// Generates checkerboard parity patterns. These masks are used to isolate
    /// alternating diagonal squares for tracking field colors and board parity.
    pub const fn new() -> Self {
        Self {
            even: Bitboard(0x55AA55AA55AA55AA),
            odd:  Bitboard(0xAA55AA55AA55AA55),
        }
    }
}

// ============================================================================
// ENGINE LOOK-UP TABLES (LUTs) IMPLEMENTATION
// ============================================================================

pub struct EngineLUTs {
    pub neighborhood_rotation_lut: [u8; 128],
    pub cardinal_offset_lut: [Bitboard; 16],
    pub(crate) diagonal_ray_lut: [DiagonalRays; 64],
    pub moore_neighborhood_lut: [Bitboard; 64],
    
    pub not_a_file: Bitboard,
    pub not_h_file: Bitboard,

    pub(crate) regions: RegionalMasks,
    pub(crate) parities: ParityMasks,

    pub centrality_lut: [u8; 64],
    pub centrality_rings: [u64; 4],

    pub topology_idx_lut: [u8; 64],
    pub topology_wall_masks: [u8; 9],
}

impl EngineLUTs {
    /// Instantiates the static precalculated lookup architecture.
    const fn new() -> Self {
        Self {
            neighborhood_rotation_lut: generate_rotation_evaluator(),
            cardinal_offset_lut: generate_relative_move_masks(),
            diagonal_ray_lut: generate_diagonal_rays(),
            not_a_file: Bitboard(0xFEFEFEFEFEFEFEFE),
            not_h_file: Bitboard(0x7F7F7F7F7F7F7F7F),
            moore_neighborhood_lut: generate_moore_neighborhood_lut(),

            regions: RegionalMasks::new(),
            parities: ParityMasks::new(),

            centrality_lut: generate_centrality_lut(),
            centrality_rings: CENTRALITY_RINGS,

            topology_idx_lut: generate_topology_maps(),

            // Bit indices correspond to direction vectors:
            // TL  L BL  B BR  R TR  T
            //  7  6  5  4  3  2  1  0
            topology_wall_masks: [
                0b00000000, // 0: Center (No walls)
                0b00111000, // 1: Bottom Edge (S, SE, SW are walls)
                0b11000001, // 2: Top Edge (N, NE, NW are walls)
                0b11100000, // 3: Left Edge (W, NW, SW are walls)
                0b00001110, // 4: Right Edge (E, NE, SE are walls)
                0b11111000, // 5: Bottom-Left Corner
                0b00111110, // 6: Bottom-Right Corner
                0b11100011, // 7: Top-Left Corner
                0b10001111, // 8: Top-Right Corner
            ],
        }
    }
}

/// Global compile-time computed lookup tables.
/// Provides a zero-cost abstraction for accessing precalculated spatial assets.
pub static LUTS: EngineLUTs = EngineLUTs::new();