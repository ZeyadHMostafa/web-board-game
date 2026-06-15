
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