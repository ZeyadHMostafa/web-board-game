use crate::{
    ai::{EvaluationScore, heuristics::{SovereigntyState, evaluators}}, luts::EngineLUTs, rules::state::{GameState, Player}
};

/// Global pre-computed weights matrix for quick initialization across files.
pub const DEFAULT_EVALUATOR_WEIGHTS: [[[[i32; 2]; 3]; 6]; 3] = generate_weights(
    // Base action prospects based on TileType and SovereigntyState
    // Dimensions: [SovereigntyState: 6][TileType: 3]
    [
        //emty ally enmy
        [  03,  50, -15  ],// ally-unc
        [  01,  45,  25  ],// ally-dom
        [  00,  40,  40  ],// conf-non
        [  00,  35, -35  ],// conf-tie
        [ -01,  30, -45  ],// enmy-dom
        [ -03,  20, -50  ],// enmy-unc
    ],
    // Multipliers for each RegionType: Corner, Edge, Center
    [10, 11, 12],
);

/// Generates the multi-dimensional weights matrix using procedural combination rules.
/// Parity values (Even and Odd columns) remain identical as parity yields no effect here.    
pub(crate) const fn generate_weights(
        prospects: [[i32; 3]; 6],
        region_multipliers: [i32; 3],
    ) -> [[[[i32; 2]; 3]; 6]; 3] {
        let mut matrix = [[[[0; 2]; 3]; 6]; 3];

        let mut t = 0;
        while t < 3 {
            let mut s = 0;
            while s < 6 {
                let mut r = 0;
                while r < 3 {
                    // Combine the base prospect with the tile value modifier and apply the region multiplier
                    let final_score = prospects[s][t] * region_multipliers[r];

                    // Assign the computed score to both Even and Odd parity columns
                    matrix[t][s][r][0] = final_score;
                    matrix[t][s][r][1] = final_score;

                    r += 1;
                }
                s += 1;
                
            }
            t += 1;
        }

        matrix
    }

/// Shared execution kernel. 
/// Both static and trainable models pass their weight lookup logic here.
#[inline(always)]
pub(crate) fn compute_dot_product<F>(
    state: &GameState,
    luts: &EngineLUTs,
    weight_lookup: F,
) -> EvaluationScore
where
    F: Fn(usize, usize, usize, usize) -> i32,
{
    // Run the identical high-performance pipeline from your existing engine
    let matrix = evaluators::evaluate_position(
        if state.active_player == Player::P1 { state.p1_pieces } else { state.p2_pieces }, 
        if state.active_player == Player::P2 { state.p1_pieces } else { state.p2_pieces }, 
        luts
    );

    let mut total_score: i32 = 0;
    let mut player_move_count :u8 = 0;

    // Vectorized dot product execution over the 108 structural features
    for t in 0..3 {
        for s in 0..6 {
            for r in 0..3 {
                for p in 0..2 {
                    let count = matrix.values[t][s][r][p] as i32;
                    let weight = weight_lookup(t, s, r, p);
                    total_score += count * weight;
                    
                    // count number of moves player has
                    player_move_count += ( ! (
                        s==SovereigntyState::NoConflict as usize ||
                        s==SovereigntyState::EnemyUncontested as usize
                    ) )as u8; 
                }
            }
        }
    }

    if player_move_count==0{
        EvaluationScore::Mated(0)
    } else {
        EvaluationScore::Value(total_score)
    }
}

use std::fs::File;
use std::io::{Read};
use std::path::Path;

/// Reads a NumPy .npy file containing 108 flat float32 weights.
/// If the file is missing or malformed, it gracefully returns an error.
pub fn load_weights_from_npy<P: AsRef<Path>>(path: P) -> Result<[[[[i32; 2]; 3]; 6]; 3], String> {
    let mut file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| format!("Failed to read file: {}", e))?;
    
    if buffer.len() < 10 {
        return Err("File too small to be a valid .npy file".to_string());
    }

    // Verify magic header prefix (\x93NUMPY)
    if &buffer[0..6] != b"\x93NUMPY" {
        return Err("Invalid file format: Missing NumPy header magic bytes".to_string());
    }

    // Read the header length (stored as a 2-byte little-endian unsigned short at offset 8)
    let header_len = u16::from_le_bytes([buffer[8], buffer[9]]) as usize;
    let data_start_offset = 10 + header_len;

    let expected_data_bytes = 108 * 4; // 108 flat float32 elements
    if buffer.len() < data_start_offset + expected_data_bytes {
        return Err("File does not contain a full 108-element float32 vector".to_string());
    }

    let mut weights = [0.0f32; 108];
    let data_slice = &buffer[data_start_offset..data_start_offset + expected_data_bytes];

    for i in 0..108 {
        let byte_offset = i * 4;
        let bytes = [
            data_slice[byte_offset],
            data_slice[byte_offset + 1],
            data_slice[byte_offset + 2],
            data_slice[byte_offset + 3],
        ];
        weights[i] = f32::from_le_bytes(bytes);
    }
    let mut matrix = [[[[0i32; 2]; 3]; 6]; 3];
    let mut index = 0;

    // Replicate the exact dimensional tracking sequence used during data pooling
    for t in 0..3 {            // TileType
        for s in 0..6 {        // SovereigntyState
            for r in 0..3 {    // RegionType
                for p in 0..2 { // ParityType
                    // Scale parameter back up into integer space and round cleanly
                    matrix[t][s][r][p] = (weights[index] * 1000.0).round() as i32;
                    index += 1;
                }
            }
        }
    }


    Ok(matrix)
}