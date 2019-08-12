#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    index: i32,
    data: String,
    previous: String,
    current: String,
    timestamp: String
}

impl Block {
    pub fn new(index: i32, previous: String, current: String, timestamp: String, data: String) -> Block {
        Block {
            index,
            data,
            previous,
            current,
            timestamp
        }
    }
}
