extern crate crypto_hash;
use crypto_hash::{hex_digest, Algorithm};

use crate::block::Block;
use crate::utils::hex_to_binary;
use crate::extensions::StringExt;

pub fn hash(block: &Block) -> String {
    hex_digest(Algorithm::SHA256, block.to_json().as_bytes())
}

pub fn hash_matches_difficulty(hash: String, difficulty: usize) -> bool {
    let binary = hex_to_binary(hash.clone());
    let required_prefix = "0".repeat(difficulty);

    return binary.starts_with(required_prefix, difficulty);
}
