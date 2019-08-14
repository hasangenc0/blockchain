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
}
