#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate crypto_hash;
extern crate chrono;

mod server;
mod hash;
mod utils;
mod extensions;
mod block;
mod blockchain;

use blockchain::Blockchain;
use extensions::StringExt;

fn main() {
    let mut blocks = Blockchain::new();

    let blocks_2 = Blockchain::new();

    println!("{}", blocks.to_json());

    println!("{}", blocks.get_latest_block().to_json());

    println!("{}", blocks.generate_next_block(String::from("hasan")).to_json());

    println!("{}", blocks.is_valid_chain());

    println!("{}", blocks.replace_chain(blocks_2));

    println!("{}", blocks.to_json());

    println!("{}",blocks.get_latest_block().current);

    println!("{}", utils::hex_to_binary(blocks.get_latest_block().current));

    println!("{}", String::from("hasan").starts_with(String::from("hasa"), 4));

    server::start();
}
