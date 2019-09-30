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

    pub fn to_json(&self) -> String {
        let chain = self.chain.lock().unwrap();
        let mut blocks: Vec<String> = Vec::new();

        for block in chain.iter() {
            let block_json = block.to_json();
            blocks.push(block_json);
        }

        return serde_json::to_string(&blocks).unwrap();
    }

    pub fn generate_next_block(&self, block_data: String) -> Block {
        let latest_block = self.get_latest_block();
        let now: DateTime<Utc> = Utc::now();

        let mut new_block = Block {
            index: latest_block.index + 1,
            data: block_data,
            previous: latest_block.current,
            current: String::from(""),
            timestamp: now.timestamp()
        };

        new_block.current = hash(&new_block);

        return new_block;

    }

    pub fn get_latest_block(&self) -> Block {
        let chain = self.chain.lock().unwrap();

        return chain.last().unwrap().clone();
    }

    pub fn is_valid_block(current: Block, previous: Block) -> bool {
        if previous.index + 1 != current.index {
            return false
        } else if previous.current != current.previous {
            return false
        }

        return true
    }

    pub fn is_valid_block_structure(block: Block) -> bool {
        true
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

    pub fn is_valid_chain(&self) -> bool {
        let chain = self.chain.lock().unwrap();
        let blocks = chain.clone();

        for (position, _) in blocks.iter().enumerate() {
            if position == 0 {

            } else if !Self::is_valid_block(blocks[position].clone(), blocks[position - 1].clone()) {
                return false;
            }
        }

        return true;
    }

    pub fn replace_chain(&mut self, new_blocks: Blockchain) -> bool {
        let chain = self.chain.lock().unwrap().clone();
        let new_chain = new_blocks.chain.lock().unwrap().clone();

        if new_blocks.is_valid_chain() && new_chain.clone().len() > chain.len() {
            self.chain = new_blocks.chain.clone();
            return true;
        }

        return false;
    }
}
