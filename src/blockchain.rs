use sha2::{Digest, Sha256};
use std::fmt::Write;
use serde_derive::{Serialize};

use chrono::prelude::*;



#[derive(Serialize, Debug)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    pre_block_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Serialize, Debug)]
pub struct Block {
    header: Blockheader,
    transaction_count: u32,
    transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender_address: String,
    receiver_address: String,
    amount: f32,
}

pub struct Chain {
    chain: Vec<Block>,
    curr_trans: Vec<Transaction>,
    reward: f32,
    difficulty: u32,
    miner_address: String,

}

impl Chain {
    pub fn new(miner_address: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            reward: 123.12,
            difficulty,
            miner_address,

        };

        chain.create_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender_address: String, receiver_address: String, amount: f32) -> bool {
        self.curr_trans.push(Transaction {
            sender_address,
            receiver_address,
            amount,
        });

        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    pub fn create_new_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: Utc::now().timestamp_millis(),
            nonce: 0,
            pre_block_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };

        let reward_trans = Transaction {
            sender_address: String::from("chain emmisions"),
            receiver_address: self.miner_address.clone(),
            amount: self.reward,
        };

        let mut block = Block {
            header,
            transaction_count: 0,
            transactions: vec![],
        };

        block.transactions.push(reward_trans);
        block.transactions.append(&mut self.curr_trans);
        block.transaction_count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);

        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }

    fn get_merkle(curr_trans: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        for t in &curr_trans {
            let hash = Chain::hash(t); // creates hash from transaction
            merkle.push(hash); // merkle  array of hashes
        }

        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2);
            let nh = Chain::hash(&h1);
            merkle.push(nh);
        }
        merkle.pop().unwrap()
    }

    pub fn proof_of_work(header: &mut Blockheader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];
            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("Block hash: {}", hash);
                        break;
                    }
                }
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            };
        }
    }

    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let res = hasher.finalize();
        let vec_res = res.to_vec();

        Chain::hex_to_string(vec_res.as_slice())
    }

    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::new();
        for b in vec_res {
            write!(&mut s, "{:x}", b).expect("error writing");
        }
        s
    }
}
