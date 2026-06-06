use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicBool;
use core_engine::ai::heuristics::FeatureMatrix;
use core_engine::ai::models::static_dot::StaticDotProductEvaluatorWeights;
use core_engine::luts::LUTS;
use core_engine::{
    ai::{
        EvaluationScore, PositionEvaluator,
        heuristics::{HeuristicMatrix, evaluators},
        models::static_dot::{
            DEFAULT_EVALUATOR_WEIGHTS,
            StaticDotProductEvaluator,
            load_weights_from_npy
        },
        search::{SearchProgress, algorithms::negamax::NegamaxPlayAgent}
    },
    luts::EngineLUTs,
    rules::state::{GameState, Player, Bitboard}
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Strict,
    Freeform,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionState {
    None,
    PieceSelected { index: u8, valid_moves: Bitboard },
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
    pub evaluator: Arc<dyn PositionEvaluator>,
    pub search_engine: Arc<NegamaxPlayAgent>,

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

    // Background Thread Orchestration States
    pub is_ai_searching: bool,
    pub ai_search_progress: Option<Arc<RwLock<SearchProgress>>>,
    pub ai_cancellation_token: Option<Arc<AtomicBool>>,
}

impl App {
    pub fn new(weights_path: Option<&str>) -> Self {
        // Resolve target weight vector matrix
        let final_weights = match weights_path {
            Some(path) => match load_weights_from_npy(path) {
                Ok(loaded_matrix) => StaticDotProductEvaluatorWeights(loaded_matrix),
                Err(_err) => DEFAULT_EVALUATOR_WEIGHTS,
            }
            None => DEFAULT_EVALUATOR_WEIGHTS,
        };

        // Instantiate structural components using the resolved weights
        let luts = &LUTS;
        let evaluator = Arc::new(StaticDotProductEvaluator::new(final_weights));
        
        // Pass the structural lookups and static evaluator layers to match the updated parameters block
        let search_engine = Arc::new(NegamaxPlayAgent::new( 2, 12));

        let mut app = Self {
            evaluator,
            search_engine,
            game_state: GameState::new(0, 0, Player::P1),
            mode: GameMode::Strict,
            p1_agent: ControllerAgent::Human,
            p2_agent: ControllerAgent::Human,
            cursor_x: 3,
            cursor_y: 3,
            selection: SelectionState::None,
            active_tab: ActivePanelTab::ControlMap,
            message_log: if weights_path.is_some() && final_weights != DEFAULT_EVALUATOR_WEIGHTS {
                String::from("System Boot Complete. Custom trained neural weight layers deployed.")
            } else {
                String::from("System Boot Complete. Core modules linked using baseline static matrix.")
            },
            running: true,
            is_ai_searching: false,
            ai_search_progress: None,
            ai_cancellation_token: None,
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
        
        // Ensure background calculations are dropped if the state resets abruptly mid-search
        if let Some(ref cancel_token) = self.ai_cancellation_token {
            cancel_token.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        self.is_ai_searching = false;
        self.ai_search_progress = None;
        self.ai_cancellation_token = None;

        self.log("Game state reset to initial competitive configuration.");
    }

    /// Pulls the calculated structural matrix directly out of our baseline evaluator engine.
    pub fn get_current_heuristics(&self) -> FeatureMatrix {
        let (allied, enemy) = self.game_state.get_player_pieces_relative();
        evaluators::evaluate_position(allied, enemy)
    }

    /// Evaluates the positional score from the perspective of the active moving player.
    pub fn get_position_score(&self) -> i32 {
        match self.evaluator.evaluate(&self.game_state) {
            EvaluationScore::Value(v) => v,
            EvaluationScore::Mating(_) => i32::MAX,
            EvaluationScore::Mated(_) => i32::MIN,
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