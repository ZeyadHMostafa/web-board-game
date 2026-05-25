use core_engine::rules::luts::EngineLUTs;
use core_engine::rules::state::{GameState, Player};
use core_engine::rules::bitboard::Bitboard;

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

pub struct App {
    // Core Game Engines
    pub luts: EngineLUTs,
    pub game_state: GameState,
    
    // TUI Environment States
    pub mode: GameMode,
    pub cursor_x: u8, // 0..7
    pub cursor_y: u8, // 0..7
    pub selection: SelectionState,
    pub message_log: String,
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        // Your specific coordinate cluster presets
        const P1_START: u64 = 0x0000_0000_0000_ffff;
        const P2_START: u64 = 0xffff_0000_0000_0000;

        let x = EngineLUTs::new();
        print!("{:?}", x.neighborhood_rotation_lut);
        Self {
            luts: x,
            game_state: GameState::new(P1_START, P2_START, Player::P1),
            mode: GameMode::Strict,
            cursor_x: 3, // Start near center
            cursor_y: 3,
            selection: SelectionState::None,
            message_log: String::from("Engine initialized in Strict Mode. P1 Turn."),
            running: true,
        }
    }

    /// Converts the internal 2D cursor positions into a 1D bitboard index (0..63)
    #[inline(always)]
    pub fn cursor_index(&self) -> u8 {
        self.cursor_y * 8 + self.cursor_x
    }

    /// Safely updates message logs
    pub fn log(&mut self, msg: &str) {
        self.message_log = msg.to_string();
    }
}