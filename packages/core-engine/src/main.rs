// mod testing;

use core_engine::testing;
use core_engine::rules::state::{GameState, Player};

// const P1_START: u64 =   (0b0000_0000_0000_1000         ) | 
//                         (0b0000_0000_0000_0100 <<  8   ) | 
//                         (0b0000_0000_0000_0001 << (8*2)) | 
//                         (0b0000_0000_0000_0000 << (8*3)) | 
//                         (0b0000_0000_0000_0000 << (8*4)) | 
//                         (0b0000_0000_0000_0000 << (8*5)) | 
//                         (0b0000_0000_0000_0000 << (8*6)) | 
//                         (0b0000_0000_0000_0000 << (8*7)) ;

// const P2_START: u64 =   (0b0000_0000_0000_0011         ) | 
//                         (0b0000_0000_0000_0000 <<  8   ) | 
//                         (0b0000_0000_0000_0000 << (8*2)) | 
//                         (0b0000_0000_0000_0000 << (8*3)) | 
//                         (0b0000_0000_0000_0000 << (8*4)) | 
//                         (0b0000_0000_0000_0000 << (8*5)) | 
//                         (0b0000_0000_0000_0000 << (8*6)) | 
//                         (0b0000_0000_0000_0000 << (8*7)) ;

const P1_START: u64 = 0xffff_0000_0000_0000;
const P2_START: u64 = 0x0000_0000_0000_ffff; 

fn main() {
    // Construct a clean initial state configuration 
    let state = GameState::new(P1_START, P2_START, Player::P1);
    
    // Run the isolated diagnostic harness to an arbitrary depth threshold
    let target_diagnostic_ply = 7;
    let (search_result, _metrics) = testing::run_position_diagnostic(&state, target_diagnostic_ply);

    match search_result {
        Ok(chosen_move) => println!("Success: Best calculated candidate move is {:?}", chosen_move),
        Err(error_msg) => eprintln!("Execution Error encountered: {}", error_msg),
    }
}