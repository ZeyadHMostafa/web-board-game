use crate::{ai::EvaluationScore, rules::state::{GameState, Player}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashEntryBounds {
    Exact,       // We searched this node fully; the score is precise.
    AlphaLower,  // The search failed-low (it's worse than alpha); this is an upper bound.
    BetaUpper,   // The search failed-high (caused a beta cutoff); this is a lower bound.
}

#[derive(Clone, Copy, Debug)]
pub struct TranspositionEntry {
    pub state_key: u128,             // Combined p1 and p2 bitboards to verify identity
    pub score: EvaluationScore,      // The score we calculated previously
    pub depth: usize,                // The depth of the search that found this score
    pub bounds: HashEntryBounds,     // How reliable this score is
}

pub struct TranspositionTable {
    table: Vec<Option<TranspositionEntry>>,
    mask: usize,
}

impl TranspositionTable {
    pub fn with_capacity(power_of_two: usize) -> Self {
        let size = 1 << power_of_two; // e.g., 1 << 20 = 1,048,576 slots
        Self {
            table: vec![None; size],
            mask: size - 1,
        }
    }

    #[inline(always)]
    fn make_key(&self, state: &GameState) -> u128 {
        // Pack both 64-bit bitboards into a unified 128-bit identity signature
        let mut key = ((state.p1_pieces.0 as u128) << 64) | (state.p2_pieces.0 as u128);
        // Safely tag the active player bit at the absolute end
        if state.active_player == Player::P2 {
            key |= 1 << 127;
        }
        key
    }

    /// Looks up a state. Returns the entry only if it matches our exact board state.
    #[inline(always)]
    pub fn lookup(&self, state: &GameState) -> Option<TranspositionEntry> {
        let key = self.make_key(state);
        // Fast modulo using bitwise AND (works because size is a power of two)
        let index = (key as usize) & self.mask; 
        
        if let Some(entry) = self.table[index] {
            if entry.state_key == key {
                return Some(entry);
            }
        }
        None
    }

    /// Stores a newly calculated score into the table, overwriting old shallow data.
    #[inline(always)]
    pub fn store(&mut self, state: &GameState, score: EvaluationScore, depth: usize, bounds: HashEntryBounds) {
        let key = self.make_key(state);
        let index = (key as usize) & self.mask;

        // Replacement Strategy: Overwrite if slot is empty, or if new search is deeper
        if let Some(existing) = self.table[index] {
            if existing.depth > depth && existing.state_key == key {
                return; // Keep the deeper, more valuable data
            }
        }

        self.table[index] = Some(TranspositionEntry {
            state_key: key,
            score,
            depth,
            bounds,
        });
    }
}