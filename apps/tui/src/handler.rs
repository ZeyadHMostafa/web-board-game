use crate::app::{App, GameMode, SelectionState, RightPanelMode};
use core_engine::rules::bitboard::Bitboard;
use core_engine::rules::moves::generate_piece_moves;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        // Force quit
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.running = false;
        }

        // Toggle Viewport Selection Layout
        KeyCode::Char('m') | KeyCode::Char('M') => {
            match app.panel_mode {
                RightPanelMode::ControlPanel => {
                    app.panel_mode = RightPanelMode::HeuristicMatrix;
                    app.log("View changed to Spatial Sovereignty Tensor.");
                }
                RightPanelMode::HeuristicMatrix => {
                    app.panel_mode = RightPanelMode::ControlPanel;
                    app.log("View changed to standard Control Panel.");
                }
            }
        }

        // Toggle Engine Modes
        KeyCode::Tab => {
            app.selection = SelectionState::None;
            match app.mode {
                GameMode::Strict => {
                    app.mode = GameMode::Freeform;
                    app.log("Switched to Freeform Sandbox Mode.");
                }
                GameMode::Freeform => {
                    app.mode = GameMode::Strict;
                    app.log("Switched to Strict Game Mode.");
                }
            }
        }

        // Drop active selections
        KeyCode::Esc => {
            app.selection = SelectionState::None;
            app.log("Selection cleared.");
        }

        // Grid Navigation Matrix
        KeyCode::Up => {
            if app.cursor_y < 7 {
                app.cursor_y += 1;
            }
        }
        KeyCode::Down => {
            if app.cursor_y > 0 {
                app.cursor_y -= 1;
            }
        }
        KeyCode::Left => {
            if app.cursor_x > 0 {
                app.cursor_x -= 1;
            }
        }
        KeyCode::Right => {
            if app.cursor_x < 7 {
                app.cursor_x += 1;
            }
        }

        // Action Trigger (Select or Execute Movement Commands)
        KeyCode::Char(' ') | KeyCode::Enter => {
            let current_idx = app.cursor_index();
            
            match app.mode {
                GameMode::Strict => handle_strict_click(current_idx, app),
                GameMode::Freeform => handle_freeform_click(current_idx, app),
            }
        }

        // Freeform Sandbox Contextual Injection Tools
        KeyCode::Char('1') => {
            if app.mode == GameMode::Freeform {
                let mask = 1u64 << app.cursor_index();
                app.game_state.p1_pieces |= Bitboard::new(mask);
                app.game_state.p2_pieces &= !Bitboard::new(mask);
                app.log("Spawned Player 1 piece.");
            }
        }
        KeyCode::Char('2') => {
            if app.mode == GameMode::Freeform {
                let mask = 1u64 << app.cursor_index();
                app.game_state.p2_pieces |= Bitboard::new(mask);
                app.game_state.p1_pieces &= !Bitboard::new(mask);
                app.log("Spawned Player 2 piece.");
            }
        }
        KeyCode::Char('3') => {
            if app.mode == GameMode::Freeform {
                let mask = 1u64 << app.cursor_index();
                app.game_state.p1_pieces &= !Bitboard::new(mask);
                app.game_state.p2_pieces &= !Bitboard::new(mask);
                app.log("Cleared contents of cell.");
            }
        }
        KeyCode::Char('t') | KeyCode::Char('T') => {
            if app.mode == GameMode::Freeform {
                app.game_state.switch_turn();
                app.log("Manually swapped active player turn indicator.");
            }
        }

        _ => {}
    }
}

fn handle_strict_click(idx: u8, app: &mut App) {
    let allied_pieces = app.game_state.get_player_pieces(app.game_state.active_player);
    let enemy_pieces = app.game_state.get_player_pieces(app.game_state.active_player.opponent());

    match app.selection {
        SelectionState::None => {
            if allied_pieces.has_bit(idx) {
                let valid_moves = generate_piece_moves::<false>(idx, allied_pieces, enemy_pieces, &app.luts);
                if valid_moves.is_empty() {
                    app.log("Selected piece has no valid legal moves available.");
                } else {
                    app.selection = SelectionState::PieceSelected { index: idx, valid_moves };
                    app.log("Piece highlighted. Select destination square.");
                }
            } else {
                app.log("Select a piece belonging to the active player.");
            }
        }
        SelectionState::PieceSelected { index: src_idx, valid_moves } => {
            if src_idx == idx {
                app.selection = SelectionState::None;
                app.log("Selection canceled.");
                return;
            }

            if allied_pieces.has_bit(idx) {
                let new_moves = generate_piece_moves::<false>(idx, allied_pieces, enemy_pieces, &app.luts);
                app.selection = SelectionState::PieceSelected { index: idx, valid_moves: new_moves };
                app.log("Switched active piece selection.");
                return;
            }

            if valid_moves.has_bit(idx) {
                execute_board_movement(src_idx, idx, app);
                app.selection = SelectionState::None;
                
                app.game_state.switch_turn();
                if app.game_state.is_lost(&app.luts) {
                    app.log("Match concluded. Game Over encountered.");
                } else {
                    app.log("Move verified. Turn updated.");
                }
            } else {
                app.log("Invalid destination target square.");
            }
        }
    }
}

fn handle_freeform_click(idx: u8, app: &mut App) {
    match app.selection {
        SelectionState::None => {
            let has_p1 = app.game_state.p1_pieces.has_bit(idx);
            let has_p2 = app.game_state.p2_pieces.has_bit(idx);
            
            if has_p1 || has_p2 {
                app.selection = SelectionState::PieceSelected { index: idx, valid_moves: Bitboard::ALL };
                app.log("Sandbox selection hoisted. Place anywhere on the map.");
            } else {
                app.log("No piece present on selected square to move.");
            }
        }
        SelectionState::PieceSelected { index: src_idx, .. } => {
            if src_idx == idx {
                app.selection = SelectionState::None;
                app.log("Sandbox selection dropped.");
                return;
            }
            execute_board_movement(src_idx, idx, app);
            app.selection = SelectionState::None;
            app.log("Sandbox item repositioned.");
        }
    }
}

fn execute_board_movement(src: u8, dst: u8, app: &mut App) {
    let src_mask = Bitboard::new(1u64 << src);
    let dst_mask = Bitboard::new(1u64 << dst);

    if (app.game_state.p1_pieces & src_mask) != Bitboard::EMPTY {
        app.game_state.p1_pieces &= !src_mask;
        app.game_state.p1_pieces |= dst_mask;
        app.game_state.p2_pieces &= !dst_mask;
    } else if (app.game_state.p2_pieces & src_mask) != Bitboard::EMPTY {
        app.game_state.p2_pieces &= !src_mask;
        app.game_state.p2_pieces |= dst_mask;
        app.game_state.p1_pieces &= !dst_mask;
    }
}