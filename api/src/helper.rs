use rand::Rng;
use std::fmt::Write;

/// Generates a random l2_blockid in the format: 0x<64-char-hex>.
/// WARN: only to be used for testing
pub fn generate_random_l2_blockid() -> String {
    let mut rng = rand::thread_rng();
    let mut hex_string = String::with_capacity(66); // "0x" + 64 hex characters

    // Append the "0x" prefix
    hex_string.push_str("0x");

    // Generate 64 random hexadecimal characters (32 bytes)
    for _ in 0..32 {
        let byte: u8 = rng.gen();
        write!(&mut hex_string, "{:02x}", byte).unwrap();
    }
    hex_string
}
