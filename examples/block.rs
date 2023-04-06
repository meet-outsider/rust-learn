extern crate crypto;

use digest::{Digest, FixedOutput};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
struct Block {
    timestamp: u64,
    data: String,
    prev_hash: String,
    hash: String,
}

impl Block {
    fn new(data: String, prev_hash: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut hasher = Sha256::new();
        let hash_data = format!("{}{}{}", timestamp, data, prev_hash);
        // hasher.input_str(&hash_data);
        // let hash = hasher.result_str();
        let hash=String::from("ss");

        Block {
            timestamp,
            data,
            prev_hash,
            hash,
        }
    }
}

#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(String::from("Genesis Block"), String::from(""));
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: String) {
        let prev_block = self.blocks.last().unwrap().clone();
        let new_block = Block::new(data, prev_block.hash);
        self.blocks.push(new_block);
    }

    fn is_valid(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if i == 0 {
                if block.hash != self.blocks[0].hash {
                    return false;
                }
            } else {
                let prev_block = &self.blocks[i - 1];
                if block.prev_hash != prev_block.hash || block.hash != Block::new(block.data.clone(), block.prev_hash.clone()).hash {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block(String::from("Block 1"));
    blockchain.add_block(String::from("Block 2"));
    blockchain.add_block(String::from("Block 3"));

    println!("Blockchain: {:?}", blockchain);

    println!("Is the blockchain valid? {}", blockchain.is_valid());
}
