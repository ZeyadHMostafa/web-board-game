use crate::app::{App, SelectionState};
use core_engine::rules::state::Bitboard;
use core_engine::rules::moves::generate_piece_moves;
use core_engine::rules::moves::Move;
use core_engine::rules::state::Player;
use core_engine::rules::luts::EngineLUTs;

pub fn handle_strict_click(idx: u8, app: &mut App) {
    let (allied_pieces, enemy_pieces) = match app.game_state.active_player {
        Player::P1 => (app.game_state.p1_pieces, app.game_state.p2_pieces),
        Player::P2 => (app.game_state.p2_pieces, app.game_state.p1_pieces),
    };

    match app.selection {
        SelectionState::None => {
            if allied_pieces.has_bit(idx) {
                let valid_moves = generate_piece_moves::<false>(idx, allied_pieces, enemy_pieces, EngineLUTs::get_engine_luts());
                if valid_moves.is_empty() {
                    app.log("Selected target possesses zero outbound mobility options.");
                } else {
                    app.selection = SelectionState::PieceSelected { index: idx, valid_moves };
                    app.log("Piece locked. Confirm destination index.");
                }
            } else {
                app.log("Invalid item target. Must select an allied component.");
            }
        }
        SelectionState::PieceSelected { index: src_idx, valid_moves } => {
            if src_idx == idx {
                app.selection = SelectionState::None;
                app.log("Piece selection dropped.");
                return;
            }

            // Allow quick target reassignment if another allied piece is clicked instead
            if allied_pieces.has_bit(idx) {
                let new_moves = generate_piece_moves::<false>(idx, allied_pieces, enemy_pieces, EngineLUTs::get_engine_luts());
                app.selection = SelectionState::PieceSelected { index: idx, valid_moves: new_moves };
                app.log("Active piece selection reassigned.");
                return;
            }

            if valid_moves.has_bit(idx) {
                let verified_move = Move::new(src_idx, idx, 0);
                app.game_state.make_move(verified_move);
                app.selection = SelectionState::None;

                if app.game_state.is_lost(EngineLUTs::get_engine_luts()) {
                    app.log("Terminal node state detected. Game Over.");
                } else {
                    app.log("Movement validated. Perspective turn swapped.");
                }
            } else {
                app.log("Target cell breaks legal geometric constraints.");
            }
        }
    }
}