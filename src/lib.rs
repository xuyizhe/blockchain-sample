use blake3;
use std::convert::TryInto;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{u32, u64};

#[derive(Debug)]
pub struct Transaction(pub String);

impl Transaction {
    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[derive(Debug)]
pub struct Block {
    pub timestamp: u64,
    pub prev_block_hash: [u8; 32],
    pub hash: [u8; 32],
    pub nonce: u32,
    pub height: u32,
    pub bits: u32,
    pub transactions: Vec<Transaction>,
    pub merkle_hash: [u8; 32],
}

impl Block {
    pub fn new(prev_block_hash: [u8; 32], transactions: Vec<Transaction>, height: u32) -> Self {
        Self {
            timestamp: utils::now_as_secs(),
            prev_block_hash,
            hash: [0u8; 32],
            nonce: 0,
            height,
            bits: 0x8 * 2,
            transactions,
            merkle_hash: [0u8; 32],
        }
    }

    pub fn mine(&mut self) -> Result<(), &'static str> {
        let timestamp_bytes = self.timestamp.to_be_bytes();
        let transactions_bytes = self
            .transactions
            .iter()
            .map(|t| t.as_bytes())
            .collect::<Vec<&[u8]>>();
        let pow = Pow::new(self.bits);
        let mut pow_input: Vec<&[u8]> = Vec::new();
        pow_input.push(&timestamp_bytes);
        pow_input.push(&self.prev_block_hash);
        pow_input.extend(&transactions_bytes);
        if let Some((nonce, hash)) = pow.calc(&pow_input) {
            self.nonce = nonce;
            self.hash = hash.as_bytes()[..].try_into().unwrap();
            return Ok(());
        };
        Err("Mining failed.")
    }
}

#[derive(Debug)]
pub struct Pow {
    pub target: [u8; 32],
}

impl Pow {
    pub fn new(bits: u32) -> Self {
        let mut target = [0u8; 32];
        target[match bits {
            0..=0x8 => 0,
            x => x as usize / 0x8,
        }] = 0x1;
        Self { target }
    }

    pub fn calc(&self, input: &Vec<&[u8]>) -> Option<(u32, blake3::Hash)> {
        for nonce in 0..u32::MAX {
            let nonce_bytes = nonce.to_be_bytes();
            let mut hash_input: Vec<&[u8]> = vec![&nonce_bytes];
            hash_input.extend(input);
            let hash = Self::hash(&hash_input);
            if self.target > *hash.as_bytes() {
                return Some((nonce, hash));
            }
        }
        None
    }

    pub fn hash(input: &Vec<&[u8]>) -> blake3::Hash {
        let mut hasher = blake3::Hasher::new();
        for bytes in input.iter() {
            hasher.update(bytes);
        }
        hasher.finalize()
    }
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block::new(
                [0u8; 32],
                vec![Transaction(String::from("genesis"))],
                0,
            )],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let prev_block = self.blocks.last().clone().unwrap();
        let mut block = Block::new(prev_block.hash, transactions, self.blocks.len() as u32);
        println!("\nMining...");
        block.mine().unwrap();
        println!("hash: {}", utils::from_be_bytes(&block.hash));
        println!("prev: {}", utils::from_be_bytes(&block.prev_block_hash));
        println!("timestamp: {}", block.timestamp);
        println!("bits: {}", block.bits);
        println!("height: {}", block.height);
        self.blocks.push(block);
    }
}

mod utils {
    use super::*;

    pub fn now_as_secs() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    pub fn from_be_bytes(bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|x| format!("{:02x?}", x))
            .collect::<Vec<String>>()
            .join("")
    }
}
