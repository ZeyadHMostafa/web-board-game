use crossterm::event::{KeyCode, KeyEvent};
use crate::app::{App, GameMode, ActivePanelTab, ControllerAgent, SelectionState};
use core_engine::simulation::Agent;
use core_engine::simulation::GameClock;
use core_engine::rules::luts::EngineLUTs;

pub mod strict;
pub mod freeform;

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) {
    match key_event.code {
        // Core System Triggers
        KeyCode::Char('q') | KeyCode::Char('Q') => app.running = false,
        KeyCode::Char('r') | KeyCode::Char('R') => app.reset_to_starting_position(),
        
        // Mode Shifting Block
        KeyCode::Tab => {
            app.selection = SelectionState::None;
            app.mode = match app.mode {
                GameMode::Strict => {
                    app.log("Mode shifted: Freeform Sandbox Active.");
                    GameMode::Freeform
                }
                GameMode::Freeform => {
                    app.log("Mode shifted: Strict Rules Enforcement Active.");
                    GameMode::Strict
                }
            };
        }

        // Tab Navigation Registry
        KeyCode::Char(']') => {
            let next_idx = (app.active_tab as usize + 1) % 4;
            app.active_tab = ActivePanelTab::from_index(next_idx);
        }
        KeyCode::Char('[') => {
            let prev_idx = if app.active_tab as usize == 0 { 3 } else { app.active_tab as usize - 1 };
            app.active_tab = ActivePanelTab::from_index(prev_idx);
        }

        // Agent Controller Allocations (P1 = '4', P2 = '5')
        KeyCode::Char('4') => {
            app.p1_agent = match app.p1_agent {
                ControllerAgent::Human => { app.log("Player 1 assigned to AI."); ControllerAgent::AI },
                ControllerAgent::AI => { app.log("Player 1 assigned to Human."); ControllerAgent::Human },
            };
        }
        KeyCode::Char('5') => {
            app.p2_agent = match app.p2_agent {
                ControllerAgent::Human => { app.log("Player 2 assigned to AI."); ControllerAgent::AI },
                ControllerAgent::AI => { app.log("Player 2 assigned to Human."); ControllerAgent::Human },
            };
        }

        // Selection Cancellations
        KeyCode::Esc => {
            app.selection = SelectionState::None;
            app.log("Active coordinates selection flushed.");
        }

        // 2D Grid Movement Matrix
        KeyCode::Up    => if app.cursor_y < 7 { app.cursor_y += 1; },
        KeyCode::Down  => if app.cursor_y > 0 { app.cursor_y -= 1; },
        KeyCode::Left  => if app.cursor_x > 0 { app.cursor_x -= 1; },
        KeyCode::Right => if app.cursor_x < 7 { app.cursor_x += 1; },

        // Contextual Interaction Executions
        KeyCode::Char(' ') | KeyCode::Enter => {
            let current_idx = app.cursor_index();
            match app.mode {
                GameMode::Strict => strict::handle_strict_click(current_idx, app),
                GameMode::Freeform => freeform::handle_freeform_click(current_idx, app),
            }
        }

        // Fall through to sandbox-specific paint tools if key wasn't explicitly trapped above
        other => {
            if app.mode == GameMode::Freeform {
                freeform::handle_sandbox_paint_tools(other, app);
            }
        }
    }

    // Trigger AI Execution immediately if it's an AI's turn and the game isn't over
    if app.mode == GameMode::Strict && app.is_active_player_ai() && !app.game_state.is_lost(EngineLUTs::get_engine_luts()) {
        trigger_ai_move(app);
    }
}

fn trigger_ai_move(app: &mut App) {
    app.log("AI is processing position parameters...");
    
    // Create a static 10-second buffer with no increment for testing
    let mock_clock = GameClock {
        active_player_time: std::time::Duration::from_secs(5),
        opponent_time: std::time::Duration::from_secs(5),
        increment: std::time::Duration::from_secs(0),
    };

    match futures::executor::block_on(app.search_engine.select_move(&app.game_state, Some(mock_clock))) {
        Ok(best_move) => {
            app.game_state.make_move(best_move);
            app.log(&format!(
                "AI (Negamax) executed transition: {} -> {}", 
                best_move.from_square(), 
                best_move.to_square()
            ));
        }
        Err(e) => app.log(&format!("AI Error encountered: {}", e)),
    }
}