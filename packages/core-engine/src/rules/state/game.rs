use crate::rules::state::Bitboard;
use crate::luts::EngineLUTs;
use crate::rules::moves::{Move, MoveList, generate_piece_moves};
use std::fmt::Formatter;
use std::fmt::Debug;

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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameState {
    pub p1_pieces: Bitboard,
    pub p2_pieces: Bitboard,
    pub active_player: Player,
}

impl Debug for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "GameState {{")?;
        writeln!(f, "  Active Player: {:?}", self.active_player)?;
        writeln!(f, "  Board Topology:")?;
        
        // Print the board from Rank 8 down to Rank 1
        for rank in (0..8).rev() {
            write!(f, "    {} |", rank + 1)?;
            for file in 0..8 {
                let idx = rank * 8 + file;
                
                if self.p1_pieces.has_bit(idx) {
                    write!(f, " 1")?;
                } else if self.p2_pieces.has_bit(idx) {
                    write!(f, " 2")?;
                } else {
                    write!(f, " .")?;
                }
            }
            writeln!(f)?;
        }
        
        writeln!(f, "      -----------------")?;
        writeln!(f, "        A B C D E F G H")?;
        writeln!(f, "  P1 Raw: 0x{:016X}", self.p1_pieces.0)?;
        writeln!(f, "  P2 Raw: 0x{:016X}", self.p2_pieces.0)?;
        write!(f, "}}")
    }
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

    /// Helper to get the pieces belonging to a both players in relative order.
    #[inline(always)]
    pub fn get_player_pieces_relative(&self) -> (Bitboard,Bitboard) {
        match self.active_player {
            Player::P1 => (self.p1_pieces, self.p2_pieces),
            Player::P2 => (self.p2_pieces, self.p1_pieces)
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
            let legal_moves = self.generate_legal_moves(luts);

            // If we find even a single valid destination square across any piece,
            // the active player is still safely in the game.
            if !legal_moves.is_empty() {
                return false;
            }
        }

        // Checked every piece and found 0 available moves. Game over.
        true
    }

    /// Step 1: Implements GameState.generate_legal_moves()
    /// Sweeps the active player's pieces and converts absolute mobility targets into concrete Move variants.
    pub fn generate_legal_moves(&self, luts: &EngineLUTs) -> MoveList {
        let mut move_list = MoveList::new();

        let (allied_pieces, enemy_pieces) = match self.active_player {
            Player::P1 => (self.p1_pieces, self.p2_pieces),
            Player::P2 => (self.p2_pieces, self.p1_pieces),
        };

        let mut moving_pieces = allied_pieces;
        while !moving_pieces.is_empty() {
            let piece_idx = moving_pieces.pop_lsb();

            let move_mask = generate_piece_moves::<false>(
                piece_idx,
                allied_pieces,
                enemy_pieces,
                luts,
            );

            // Pass reference to the stack-allocated list for zero-allocation collection
            Move::extract_moves_from_mask(piece_idx, move_mask, enemy_pieces, 0, &mut move_list);
        }

        move_list
    }

    /// Step 2: Implements GameState.make_move(current_move)
    /// Performs in-place mutation to update piece placements and swap turn ownership.
    pub fn make_move(&mut self, current_move: Move) {
        let from_mask = Bitboard::from_square(current_move.from_square());
        let to_mask = Bitboard::from_square(current_move.to_square());

        match self.active_player {
            Player::P1 => {
                // Relocate Allied Piece
                self.p1_pieces &= !from_mask;
                self.p1_pieces |= to_mask;

                // Capture validation: Evict enemy presence from target coordinate
                self.p2_pieces &= !to_mask;

                // Hand over play control
                self.active_player = Player::P2;
            }
            Player::P2 => {
                // Relocate Allied Piece
                self.p2_pieces &= !from_mask;
                self.p2_pieces |= to_mask;

                // Capture validation: Evict enemy presence from target coordinate
                self.p1_pieces &= !to_mask;

                // Hand over play control
                self.active_player = Player::P1;
            }
        }
    }
    /// Performs an in-place mutation to reverse a move execution,
    /// restoring any captured pieces and resetting play control.
    pub fn unmake_move(&mut self, historical_move: Move) {
        // Step 1: Hand back play control to the person who made the move
        self.active_player = self.active_player.opponent();

        let from_mask = Bitboard::from_square(historical_move.from_square());
        let to_mask = Bitboard::from_square(historical_move.to_square());

        match self.active_player {
            Player::P1 => {
                // Return Allied piece back to its starting square
                self.p1_pieces |= from_mask;
                self.p1_pieces &= !to_mask;

                // If it was a capture, resurrect the enemy piece at the destination
                if historical_move.is_capture() {
                    self.p2_pieces |= to_mask;
                }
            }
            Player::P2 => {
                // Return Allied piece back to its starting square
                self.p2_pieces |= from_mask;
                self.p2_pieces &= !to_mask;

                // If it was a capture, resurrect the enemy piece at the destination
                if historical_move.is_capture() {
                    self.p1_pieces |= to_mask;
                }
            }
        }
    }

    /// Toggles the active turn to the opposing player.
    #[inline(always)]
    pub fn switch_turn(&mut self) {
        self.active_player = self.active_player.opponent();
    }
}