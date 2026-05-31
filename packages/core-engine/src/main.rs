// mod testing;

use core_engine::testing;
use core_engine::rules::state::{GameState, Player};

const P1_START: u64 = 0x000000000000ffff;
const P2_START: u64 = 0xffff000000000000;

fn main() {
    // Construct a clean initial state configuration 
    let state = GameState::new(P1_START, P2_START, Player::P1);
    
    // Run the isolated diagnostic harness to an arbitrary depth threshold
    let target_diagnostic_ply = 8;
    let (search_result, _metrics) = testing::run_position_diagnostic(&state, target_diagnostic_ply);

    match search_result {
        Ok(chosen_move) => println!("Success: Best calculated candidate move is {:?}", chosen_move),
        Err(error_msg) => eprintln!("Execution Error encountered: {}", error_msg),
    }
}