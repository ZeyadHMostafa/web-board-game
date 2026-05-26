use std::sync::Arc;
use std::time::Duration;
use core_engine::rules::luts::EngineLUTs;
use core_engine::rules::state::{GameState, Player};
use core_engine::heuristics::evaluators::EvaluationEngine;
use core_engine::ai::models::{PositionEvaluator, StaticDotProductEvaluator};
use core_engine::ai::search::BasePickerSearch;
use core_engine::simulation::SearchContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Strict,
    Freeform,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionState {
    None,
    PieceSelected { index: u8, valid_moves: core_engine::rules::bitboard::Bitboard },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllerAgent {
    Human,
    AI,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePanelTab {
    ControlMap = 0,
    GameState = 1,
    HeuristicsTable = 2,
    AIMoveAnalysis = 3,
}

impl ActivePanelTab {
    pub fn from_index(index: usize) -> Self {
        match index {
            1 => ActivePanelTab::GameState,
            2 => ActivePanelTab::HeuristicsTable,
            3 => ActivePanelTab::AIMoveAnalysis,
            _ => ActivePanelTab::ControlMap,
        }
    }
}

pub struct App {
    // Core Engine Contexts (Owned once at application root)
    pub luts: EngineLUTs,
    pub evaluator: Arc<dyn PositionEvaluator>,
    pub search_engine: BasePickerSearch,
    pub heuristic_engine: EvaluationEngine,

    // Game Core State
    pub game_state: GameState,
    pub mode: GameMode,
    pub p1_agent: ControllerAgent,
    pub p2_agent: ControllerAgent,

    // TUI View Navigation Controls
    pub cursor_x: u8,
    pub cursor_y: u8,
    pub selection: SelectionState,
    pub active_tab: ActivePanelTab,
    pub message_log: String,
    pub running: bool,
}


const LUTS:EngineLUTs = EngineLUTs::new();

impl App {
    pub fn new() -> Self {
        // Instantiate our new data-oriented static evaluation neural matrices
        let evaluator = Arc::new(StaticDotProductEvaluator::new(&LUTS));
        // Setup our 1-ply base picker search engine
        let search_engine = BasePickerSearch::new(&LUTS, evaluator.clone());
        
        let mut app = Self {
            luts: LUTS,
            evaluator,
            search_engine,
            heuristic_engine: EvaluationEngine::new(),
            game_state: GameState::new(0, 0, Player::P1), // Initialized empty, reset below
            mode: GameMode::Strict,
            p1_agent: ControllerAgent::Human,
            p2_agent: ControllerAgent::Human,
            cursor_x: 3,
            cursor_y: 3,
            selection: SelectionState::None,
            active_tab: ActivePanelTab::ControlMap,
            message_log: String::from("System Boot Complete. Core modules linked."),
            running: true,
        };
        app.reset_to_starting_position();
        app
    }

    /// Resets the bitboards back to default operational deployment grids.
    pub fn reset_to_starting_position(&mut self) {
        const P1_START: u64 = 0x0000_0000_0000_ffff;
        const P2_START: u64 = 0xffff_0000_0000_0000;
        
        self.game_state = GameState::new(P1_START, P2_START, Player::P1);
        self.selection = SelectionState::None;
        self.log("Game state reset to initial competitive configuration.");
    }

    /// Pulls the calculated structural matrix directly out of our baseline evaluator engine.
    pub fn get_current_heuristics(&self) -> core_engine::heuristics::HeuristicMatrix {
        let (allied, enemy) = match self.game_state.active_player {
            Player::P1 => (self.game_state.p1_pieces, self.game_state.p2_pieces),
            Player::P2 => (self.game_state.p2_pieces, self.game_state.p1_pieces),
        };
        self.heuristic_engine.evaluate_position(allied, enemy, &self.luts)
    }

    /// Evaluates the positional score from the perspective of the active moving player.
    pub fn get_position_score(&self) -> i32 {
        match self.evaluator.evaluate(&self.game_state) {
            core_engine::ai::models::EvaluationScore::Value(v) => v,
            core_engine::ai::models::EvaluationScore::Mating(_) => i32::MAX,
            core_engine::ai::models::EvaluationScore::Mated(_) => i32::MIN,
        }
    }

    /// Returns whether the active player is configured as an AI controller.
    pub fn is_active_player_ai(&self) -> bool {
        match self.game_state.active_player {
            Player::P1 => self.p1_agent == ControllerAgent::AI,
            Player::P2 => self.p2_agent == ControllerAgent::AI,
        }
    }

    #[inline(always)]
    pub fn cursor_index(&self) -> u8 {
        self.cursor_y * 8 + self.cursor_x
    }

    pub fn log(&mut self, msg: &str) {
        self.message_log = msg.to_string();
    }
}