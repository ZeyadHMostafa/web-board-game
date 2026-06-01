use crate::ai::search::algorithms::negamax::NegamaxSimulationAgent;
use crate::luts::EngineLUTs;
use crate::rules::state::{GameState, Player, Bitboard};
use crate::simulation::TrainingSample;

pub struct SimulationEnvironment {
    luts: &'static EngineLUTs,
    max_moves: usize,
}

impl SimulationEnvironment {
    pub fn new(luts: &'static EngineLUTs, max_moves: usize) -> Self {
        Self { luts, max_moves }
    }

    /// Simulates a single complete game transaction using the provided configuration.
    pub fn run_game(
        &self,
        initial_state: GameState,
        agent: &NegamaxSimulationAgent,
    ) -> Vec<TrainingSample> {
        let mut current_state = initial_state;
        let mut game_samples = Vec::new();
        let mut move_counter = 0;

        // Tracks history allocations per player via rolling index stacks
        let mut p1_history = [Bitboard::EMPTY; 4];
        let mut p2_history = [Bitboard::EMPTY; 4];
        let mut game_is_drawn = false;

        while !current_state.is_lost(self.luts) && move_counter < self.max_moves {
            let current_pieces = current_state.get_player_pieces(current_state.active_player);
            let history_ref = match current_state.active_player {
                Player::P1 => &p1_history,
                Player::P2 => &p2_history,
            };

            if history_ref.iter().any(|&past| past == current_pieces && !past.is_empty()) {
                game_is_drawn = true;
                break;
            }

            let history_index = (move_counter / 2) % 4;
            match current_state.active_player {
                Player::P1 => p1_history[history_index] = current_pieces,
                Player::P2 => p2_history[history_index] = current_pieces,
            }

            // Expose state and collect evaluation features along with the chosen path
            let (chosen_move, features, target_score) = match agent.evaluate_and_select(&current_state) {
                Some(res) => res,
                None => break,
            };

            game_samples.push(TrainingSample {
                features,
                target_score,
            });

            current_state.make_move(chosen_move);
            move_counter += 1;
        }

        self.append_terminal_conditions(&current_state, &mut game_samples, game_is_drawn, move_counter);
        game_samples
    }

    /// Appends baseline records for final termination configurations.
    fn append_terminal_conditions(
        &self,
        state: &GameState,
        samples: &mut Vec<TrainingSample>,
        is_drawn: bool,
        move_count: usize,
    ) {
        if is_drawn || move_count >= self.max_moves {
            // drop move, not worth adding
            // there are no true draws in the game
            // this is just used to stop games where
            // it converged to a stand-off
        } else if state.is_lost(self.luts) {
            println!("player lost");
            samples.push(TrainingSample {
                features: NegamaxSimulationAgent::extract_features(state, self.luts),
                target_score: -1000.0,
            });
        }
    }
}