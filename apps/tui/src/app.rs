use core_engine::rules::luts::EngineLUTs;
use core_engine::rules::state::{GameState, Player};
use core_engine::rules::bitboard::Bitboard;
use core_engine::heuristics::evaluators::EvaluationEngine;
use core_engine::heuristics::HeuristicMatrix;

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
pub enum RightPanelMode {
    ControlPanel,
    HeuristicMatrix,
}

pub struct App {
    pub luts: EngineLUTs,
    pub game_state: GameState,
    pub evaluator: EvaluationEngine,
    pub mode: GameMode,
    pub cursor_x: u8,
    pub cursor_y: u8,
    pub selection: SelectionState,
    pub message_log: String,
    pub panel_mode: RightPanelMode,
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        const P1_START: u64 = 0x0000_0000_0000_ffff;
        const P2_START: u64 = 0xffff_0000_0000_0000;

        let x = EngineLUTs::new();
        Self {
            luts: x,
            game_state: GameState::new(P1_START, P2_START, Player::P1),
            evaluator: EvaluationEngine::new(),
            mode: GameMode::Strict,
            cursor_x: 3,
            cursor_y: 3,
            selection: SelectionState::None,
            message_log: String::from("Engine initialized in Strict Mode. P1 Turn."),
            panel_mode: RightPanelMode::ControlPanel,
            running: true,
        }
    }

    /// Evaluates the position from the perspective of the active player.
    pub fn get_current_heuristics(&self) -> HeuristicMatrix {
        let allied = self.game_state.get_player_pieces(self.game_state.active_player);
        let enemy = self.game_state.get_player_pieces(self.game_state.active_player.opponent());
        self.evaluator.evaluate_position(allied, enemy, &self.luts)
    }

    #[inline(always)]
    pub fn cursor_index(&self) -> u8 {
        self.cursor_y * 8 + self.cursor_x
    }

    pub fn log(&mut self, msg: &str) {
        self.message_log = msg.to_string();
    }
}