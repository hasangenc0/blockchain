#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: i32,
    pub data: String,
    pub previous: String,
    pub current: String,
    pub timestamp: i64
}

impl Block {
    fn new(index: i32, previous: String, current: String, timestamp: i64, data: String) -> Block {
        Block {
            index,
            data,
            previous,
            current,
            timestamp
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
