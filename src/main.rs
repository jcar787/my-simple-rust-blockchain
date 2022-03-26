use std::fmt;
use std::fmt::{ Formatter, Display };


use chrono::{ Utc };
use sha256::digest;

#[derive(Debug)]
struct Block {
    data: String,
    hash: String,
    prev_hash: String,
    timestamp: i64,
}

impl Block {
    fn new(data: String, prev_hash: String) -> Block {
        let timestamp = Utc::now().timestamp();
        let hash = Self::calculate_hash_first_time(&data, &prev_hash, &timestamp.to_string());
        

        Block {
            data,
            hash,
            prev_hash,
            timestamp
        }
    }

    fn calculate_hash_first_time(data: &String, prev_hash: &String, timestamp: &String) -> String {
        let mut new_string = String::from("");
        new_string.push_str(data);
        new_string.push_str(prev_hash);
        new_string.push_str(timestamp);

        digest(new_string)
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "data: {} \ntimestamp: {} \nhash: {} \nprev_hash: {}",
               self.data, self.timestamp, self.hash, self.prev_hash)
    }
}

fn main() {
    let timestamp = Utc::now().timestamp();

    // let genesis_block = Block {
    //     data: "THIS IS THE ROOT".to_string(),
    //     hash: "".to_string(),
    //     prev_hash: "000000000".to_string(),
    //     timestamp
    // };

    let data = String::from("THIS IS THE ROOT");
    let prev_hash = String::from("0000000000");

    let genesis_block = Block::new(data, prev_hash);

    println!("{}", genesis_block);
}
