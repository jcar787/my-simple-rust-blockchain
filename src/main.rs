use std::time::{Instant};

use blockchain::utils::*;

fn main() {
    // println!("Program started now");
    // let now = Instant::now();

    let mut blockchain = Blockchain::new();

    println!("{}", blockchain);
    blockchain.add_block(
        Block::new(
            Operation::SELL, 5, "This is a note because I sold".to_string(), "00000000".to_string()
        )
    );
    println!("{}", blockchain);

    // for i in 1..=100000 {
    //     let data = String::from("THIS IS THE ROOT");
    //     let prev_hash = i.to_string();
    //     let genesis_block = Block::new(Operation::SELL, 5, data, prev_hash);
    //     println!("{}", genesis_block);
    // }
    // println!("Program finished: {}", now.elapsed().as_millis())

}
