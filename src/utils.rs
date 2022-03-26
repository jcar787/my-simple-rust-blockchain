use std::fmt;
use std::fmt::{ Formatter, Display };

use chrono::{ Utc };
use sha256::digest;

pub enum Operation { START, BUY, SELL }

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let operation = match self {
            Operation::START => "START",
            Operation::BUY => "BUY",
            Operation::SELL => "SELL"
        };
        write!(f, "{}", operation)
    }
}

pub struct Block {
    operation: Operation,
    quantity: i32,
    data: String,
    hash: String,
    prev_hash: String,
    timestamp: i64,
}

impl Block {
    pub fn new(operation: Operation, quantity: i32, data: String, prev_hash: String) -> Block {
        let timestamp = Utc::now().timestamp_millis();
        let hash = Self::calculate_hash_first_time(&operation.to_string(), &quantity.to_string(), &data, &prev_hash, &timestamp.to_string());

        Block {
            operation,
            quantity,
            data,
            hash,
            prev_hash,
            timestamp
        }
    }

    pub fn reset_hash(&mut self) {
        self.hash = self.recalculate_hash();
    }

    fn recalculate_hash(&self) -> String {
        digest(format!("{}{}{}{}{}", self.operation, self.quantity, self.data, self.prev_hash, self.timestamp))
    }

    fn set_previous_hash(&mut self, previous_hash: &str) {
        self.prev_hash = String::from(previous_hash);
    }


    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    pub fn is_block_valid(&self) -> bool {
        self.hash == self.recalculate_hash()
    }

    fn calculate_hash_first_time(operation: &str, quantity: &str, data: &str, prev_hash: &str, timestamp: &str) -> String {
        digest(format!("{}{}{}{}{}", operation, quantity, data, prev_hash, timestamp))
    }
}
impl Display for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "operation: {} \nquantity: {} \ndata: {} \ntimestamp: {} \nhash: {} \nprev_hash: {}",
               self.operation, self.quantity, self.data, self.timestamp, self.hash, self.prev_hash)
    }
}

pub struct Blockchain {
    blocks: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Block::new(Operation::START, 0, "This is the beginning".to_string(), "0000000000".to_string());
        Blockchain {
            blocks: vec![genesis_block]
        }
    }

    pub fn add_block(&mut self, block: Block) {
        let result = self.blocks.last();
        match result {
            Some(last_block) => {
                let new_block = Block {
                    prev_hash: last_block.get_hash().to_string(),
                    ..block
                };
                self.blocks.push(new_block);
            },
            None => {
                eprintln!("Blockchain wasn't initialized properly");
            }
        }
    }
}

impl Display for Blockchain {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut new_line_separated_value = String::new();

        for (i, block) in self.blocks.iter().enumerate() {
            new_line_separated_value.push_str(&format!("{}. {}", i+1, block));
            if i < self.blocks.len() - 1 {
                new_line_separated_value.push_str("\n----------\n");
            }
        }
        write!(f, "{}", new_line_separated_value)
    }
}