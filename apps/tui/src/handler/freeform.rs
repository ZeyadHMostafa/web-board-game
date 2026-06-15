use crate::app::{App, SelectionState};
use core_engine::rules::state::Bitboard;
use crossterm::event::KeyCode;

pub fn handle_freeform_click(idx: u8, app: &mut App) {
    match app.selection {
        SelectionState::None => {
            if app.game_state.p1_pieces.has_bit(idx) || app.game_state.p2_pieces.has_bit(idx) {
                app.selection = SelectionState::PieceSelected { index: idx, valid_moves: Bitboard::ALL };
                app.log("Sandbox anchor hoisted. Place anywhere on the grid layout.");
            } else {
                app.log("No valid structural entity exists at the specified grid coordinate.");
            }
        }
        SelectionState::PieceSelected { index: src_idx, .. } => {
            if src_idx == idx {
                app.selection = SelectionState::None;
                app.log("Sandbox target dropped.");
                return;
            }

            // Safe bit transfers across board spaces
            let src_mask = !Bitboard::from_square(src_idx);
            let dst_mask = Bitboard::from_square(idx);

            if app.game_state.p1_pieces.has_bit(src_idx) {
                app.game_state.p1_pieces &= src_mask;
                app.game_state.p1_pieces |= dst_mask;
                app.game_state.p2_pieces &= !dst_mask;
            } else {
                app.game_state.p2_pieces &= src_mask;
                app.game_state.p2_pieces |= dst_mask;
                app.game_state.p1_pieces &= !dst_mask;
            }

            app.selection = SelectionState::None;
            app.log("Sandbox entity repositioned.");
        }
    }
}

pub fn handle_sandbox_paint_tools(key: KeyCode, app: &mut App) {
    let mask = Bitboard::from_square(app.cursor_index());
    match key {
        KeyCode::Char('1') => {
            app.game_state.p1_pieces |= mask;
            app.game_state.p2_pieces &= !mask;
            app.log("Injected Player 1 piece.");
        }
        KeyCode::Char('2') => {
            app.game_state.p2_pieces |= mask;
            app.game_state.p1_pieces &= !mask;
            app.log("Injected Player 2 piece.");
        }
        KeyCode::Char('3') => {
            app.game_state.p1_pieces &= !mask;
            app.game_state.p2_pieces &= !mask;
            app.log("Cleared contents of cell.");
        }
        KeyCode::Char('t') | KeyCode::Char('T') => {
            app.game_state.switch_turn();
            app.log("Manually toggled active player ownership.");
        }
        _ => {}
    }
}