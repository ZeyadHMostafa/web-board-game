use crate::{ai::EvaluationScore, rules::state::{GameState, Player}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashEntryBounds {
    Exact,      // Value is precise (Alpha < score < Beta)
    Upper,      // Fail-Low: The true value is <= score (Upper Bound)
    Lower,      // Fail-High: The true value is >= score (Lower Bound)
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
    fn get_table_index(&self, state: &GameState) -> (u128, usize) {
        let key = self.make_key(state);
        
        // Fold the 128-bit board identity layer down to a 64-bit integer
        let mut hash = ((key >> 64) ^ key) as u64;
        
        // MurmurHash3 finalizer mixer constants to guarantee bit avalanching
        hash ^= hash >> 33;
        hash = hash.wrapping_mul(0xff51afd7ed558ccd);
        hash ^= hash >> 33;
        hash = hash.wrapping_mul(0xc4ceb9fe1a85ec53);
        hash ^= hash >> 33;

        let index = (hash as usize) & self.mask;
        (key, index)
    }

    #[inline(always)]
    fn make_key(&self, state: &GameState) -> u128 {
        // Dynamically assign local variables based on who is currently making the decision
        let (active_pieces, opponent_pieces) = match state.active_player {
            Player::P1 => (state.p1_pieces.0, state.p2_pieces.0),
            Player::P2 => (state.p2_pieces.0, state.p1_pieces.0), // Mirror perspective!
        };

        // Pack them into a unified 128-bit identity signature.
        // No extra bit tracking needed—the layout itself inherently dictates whose perspective this score belongs to.
        ((active_pieces as u128) << 64) | (opponent_pieces as u128)
    }

    /// Looks up a state. Returns the entry only if it matches our exact board state.
    #[inline(always)]
    pub fn lookup(& self, state: &GameState) -> Option<TranspositionEntry> {
        let (key, index) = self.get_table_index(state);
        
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
        let (key, index) = self.get_table_index(state);

        if let Some(existing) = self.table[index] {
            if existing.depth > depth && existing.state_key == key {
                return;
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