#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate crypto_hash;
extern crate chrono;

mod hash;
mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let blocks = Blockchain::new();
}
