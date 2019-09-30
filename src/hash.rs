extern crate crypto_hash;
use crypto_hash::{hex_digest, Algorithm};

use crate::block::Block;

pub fn hash(block: &Block) -> String {
    hex_digest(Algorithm::SHA256, block.to_json().as_bytes())
}

pub fn hashMatchesDifficulty(hash: String, difficulty: i32) -> bool {

    return true
}