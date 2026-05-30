use crate::{luts::{ParityMasks, RegionalMasks}, rules::state::Bitboard};

// ============================================================================
// CORE ENUMS WITH EXPLICIT DISCRIMINANTS
// ============================================================================

/// The physical occupancy of a tile at the start of the evaluation pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum TileType {
    Empty             = 0, // Open terrain with no pieces currently present
    AlliedPiece       = 1, // Physical presence of an Allied piece
    EnemyPiece        = 2, // Physical presence of an Enemy piece
}

/// The absolute qualitative control profile of a tile based on step depth.
/// This acts as an exhaustive, perspective-invariant classification of spatial power.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum SovereigntyState {
    AllyUncontested    = 0, // Allied depth > 0, Enemy depth == 0 (Pure Allied territory)
    AllyDominates      = 1, // Allied depth > Enemy depth > 0 (Ally wins tactical trades)
    NoConflict         = 2, // Allied depth == 0, Enemy depth == 0 (No-man's land)
    TiedConflict       = 3, // Allied depth == Enemy depth > 0 (Dead even standoff)
    EnemyDominates     = 4, // Enemy depth > Allied depth > 0 (Enemy wins tactical trades)
    EnemyUncontested   = 5, // Enemy depth > 0, Allied depth == 0 (Pure Enemy territory)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum RegionType {
    Corner2x2 = 0,
    Edge4x2   = 1,
    Center4x4 = 2,
}

impl RegionalMasks {
    #[inline(always)]
    pub const fn get(&self, region: RegionType) -> Bitboard {
        match region {
            RegionType::Corner2x2 => self.corners,
            RegionType::Edge4x2   => self.edges,
            RegionType::Center4x4 => self.center,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum ParityType {
    Even = 0,
    Odd  = 1,
}

impl ParityMasks {
    #[inline(always)]
    pub const fn get(&self, parity: ParityType) -> Bitboard {
        match parity {
            ParityType::Even => self.even,
            ParityType::Odd  => self.odd,
        }
    }
}

// ============================================================================
// PIPELINE DATA STRUCTURES
// ============================================================================

/// Accumulates a side's overall board control profile simultaneously in parallel.
/// Uses a base-2 vertical counter layout tracking up to 7+ tracking layers.
#[derive(Debug, Clone, Copy, Default)]
pub struct VerticalDensityMap {
    pub layer_0: Bitboard, // Ones-place bits (1, 3, 5, 7)
    pub layer_1: Bitboard, // Twos-place bits (2, 3, 6, 7)
    pub layer_2: Bitboard, // Fours-place bits (4, 5, 6, 7)
}

/// Stores the unified geometric footprints for a player's pieces across the board.
#[derive(Debug, Clone, Copy, Default)]
pub struct StructuralMoveMap {
    pub collective_moves: Bitboard,
}

/// A clean container capturing all raw pipeline assets extracted for a single color 
/// during the single-pass move generation phase.
pub struct IntermediateBatch {
    pub union_map: StructuralMoveMap,
    pub density:   VerticalDensityMap,
}

// ============================================================================
// PERSPECTIVE-INVARIANT COUPLING MATRIX
// ============================================================================

/// Aggregated spatial metric matrix mapped out dynamically across dimensions:
/// [TileType (3)][SovereigntyState (6)][RegionType (3)][ParityType (2)]
/// This unified tensor representation contains 108 orthogonal feature fields.
#[derive(Debug, Clone, Copy)]
pub struct HeuristicMatrix {
    pub values: [[[[i16; 2]; 3]; 6]; 3],
}

impl HeuristicMatrix {
    pub const fn new() -> Self {
        Self {
            values: [[[[0; 2]; 3]; 6]; 3],
        }
    }
}