use crate::block::Block;
use crate::hash::hash;
use chrono::{DateTime, Utc};
use std::sync::{
    Arc,
    Mutex,
};

pub struct Blockchain {
    pub chain: Arc<Mutex<Vec<Block>>>
}

impl Blockchain {
    const GENESIS_DATA: &'static str = "genesis";
    const GENESIS_INDEX: i32 = 0;

    pub fn new() -> Blockchain {
        let block = Self::genesis();
        let blocks = vec![block];
        let chain = Arc::new(Mutex::new(blocks));

        Blockchain {
            chain
        }
    }

    fn genesis() -> Block {
        let now: DateTime<Utc> = Utc::now();

        let mut genesis = Block {
            index: Blockchain::GENESIS_INDEX,
            data: String::from(Blockchain::GENESIS_DATA),
            previous: String::from(""),
            current: String::from(""),
            timestamp: now.timestamp(),
        };

        genesis.current = hash(&genesis);

        return genesis;
    }

    pub fn to_json(&self) -> String {
        let mut chain = self.chain.lock().unwrap();
        let mut blocks: Vec<String> = Vec::new();

        for block in chain.iter() {
            let block_json = block.to_json();
            blocks.push(block_json);
        }

        return serde_json::to_string(&blocks).unwrap();
    }

    pub fn generate_next_block(block_data: String) {

    }

    pub fn get_latest_block(&self) -> Block {
        let mut chain = self.chain.lock().unwrap();

        return chain.last().unwrap().clone();
    }
}
