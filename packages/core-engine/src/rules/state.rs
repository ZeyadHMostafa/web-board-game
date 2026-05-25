use crate::rules::bitboard::Bitboard;
use crate::rules::luts::EngineLUTs;
use crate::rules::moves::generate_piece_moves;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    P1,
    P2,
}

impl Player {
    /// Returns the opposing player.
    #[inline(always)]
    pub fn opponent(self) -> Self {
        match self {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameState {
    pub p1_pieces: Bitboard,
    pub p2_pieces: Bitboard,
    pub active_player: Player,
}

impl GameState {
    /// Creates a fresh board state with specified initial pieces.
    pub const fn new(p1_start: u64, p2_start: u64, starting_player: Player) -> Self {
        Self {
            p1_pieces: Bitboard::new(p1_start),
            p2_pieces: Bitboard::new(p2_start),
            active_player: starting_player,
        }
    }

    /// Helper to get the pieces belonging to a specific player.
    #[inline(always)]
    pub fn get_player_pieces(&self, player: Player) -> Bitboard {
        match player {
            Player::P1 => self.p1_pieces,
            Player::P2 => self.p2_pieces,
        }
    }

    /// Checks if the active player has completely run out of legal moves.
    /// Returns true if they have lost the game.
    pub fn is_lost(&self, luts: &EngineLUTs) -> bool {
        let (allied_pieces, enemy_pieces) = match self.active_player {
            Player::P1 => (self.p1_pieces, self.p2_pieces),
            Player::P2 => (self.p2_pieces, self.p1_pieces),
        };

        // If the player has no physical pieces remaining, they have automatically lost.
        if allied_pieces.is_empty() {
            return true;
        }

        // Clone the bitmask of active pieces so we can destructively scrape them
        let mut pieces_to_scan = allied_pieces;

        while !pieces_to_scan.is_empty() {
            let piece_idx = pieces_to_scan.pop_lsb();

            // Compute standard legal moves (IS_CONTROL_EVAL = false)
            let legal_moves = generate_piece_moves::<false>(
                piece_idx,
                allied_pieces,
                enemy_pieces,
                luts,
            );

            // If we find even a single valid destination square across any piece,
            // the active player is still safely in the game.
            if !legal_moves.is_empty() {
                return false;
            }
        }

        // Checked every piece and found 0 available moves. Game over.
        true
    }

    /// Toggles the active turn to the opposing player.
    #[inline(always)]
    pub fn switch_turn(&mut self) {
        self.active_player = self.active_player.opponent();
    }
}