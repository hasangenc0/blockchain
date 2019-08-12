use sha2::{Sha256, Digest};
use crate::block::Block;
use std::sync::{
    Arc,
    Mutex,
};

pub struct Blockchain {
    chain: Arc<Mutex<Vec<Block>>>
}

impl Blockchain {
    pub fn new(genesis_block: Block) -> Blockchain {
        let chain = chain: Arc::new(Mutex::new(Vec::new()));
        chain.push(genesis_block);

        Blockchain {
            chain
        }
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.input(data);
        let hash = hasher.result();
        return String::from("asdasd");
    }

    pub fn genesis_block() -> Block {
        Block {
            index: 0,
            data: String::from("genesis block"),
            previous: String::from(""),
            current: String::from("816534932c2b7154836da6afc367695e6337db8a921823784c14378abed4f7d7"),
            timestamp: "1564825004".parse().unwrap(),
        }
    }

    pub fn generateNextBlock() -> Block {

    }
}
