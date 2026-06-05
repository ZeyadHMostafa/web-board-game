use crossterm::event::{KeyCode, KeyEvent};
use crate::app::{App, GameMode, ActivePanelTab, ControllerAgent, SelectionState};
use core_engine::simulation::Agent;
use core_engine::simulation::GameClock;
use core_engine::luts::EngineLUTs;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use core_engine::ai::search::{SearchContext, SearchProgress};

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
    // If the engine is already computing a move from a previous frame trigger, early-exit
    if app.is_ai_searching {
        return;
    }
    
    app.log("AI calculation thread initializing...");
    app.is_ai_searching = true;

    // Configure structural parameters and context tokens
    let luts = core_engine::luts::EngineLUTs::get_engine_luts();
    let cancelled = Arc::new(AtomicBool::new(false));
    let nodes_explored = std::sync::atomic::AtomicUsize::new(0);

    let shared_progress = Arc::new(RwLock::new(SearchProgress {
        candidates: Vec::new(),
        depth_reached: 0,
        nodes_explored: 0,
        branching_factor: 0.0,
        pv: Vec::new(),
    }));

    // Clone inputs to safely transfer ownership to the background worker thread
    let state_clone = app.game_state.clone();
    let agent_clone = app.search_engine.clone(); // Assumes your engine pointer is wrapped in an Arc
    let worker_cancelled = cancelled.clone();
    let worker_progress = shared_progress.clone();
    let evaluator = app.evaluator.clone(); // Accesses the underlying positional evaluator
        
    // Spawn an OS thread to run the search without locking up the user interface rendering loop
    std::thread::spawn(move || {
        let ctx = SearchContext {
            cancelled: &worker_cancelled,
            evaluator: evaluator.as_ref(),
            luts,
        };

        // Execute the synchronous iterative deepen operation inside the worker space
        agent_clone.search_position(&state_clone, &ctx, worker_progress);
    });

    // Spawn a companion supervisor timer thread to handle time budgeting boundaries
    let timer_cancelled = cancelled.clone();
    let time_budget = Duration::from_millis(500);
    
    std::thread::spawn(move || {
        std::thread::sleep(time_budget);
        // Intercept execution pathways gracefully if the computation cycle crosses our threshold
        timer_cancelled.store(true, Ordering::Relaxed);
    });

    // Store references inside your App state map so the TUI frame updater can monitor progress
    app.ai_search_progress = Some(shared_progress);
    app.ai_cancellation_token = Some(cancelled);
}