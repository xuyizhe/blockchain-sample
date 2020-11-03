use crate::pow::Pow;
use crate::transaction::Transaction;
use crate::utils;

use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
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
    pub fn new(
        prev_block_hash: [u8; 32],
        height: u32,
        bits: u32,
        transactions: Vec<Transaction>,
    ) -> Self {
        Self {
            timestamp: utils::now_as_secs(),
            prev_block_hash,
            hash: [0u8; 32],
            nonce: 0,
            height,
            bits,
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
        println!("{}", pow);
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

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "prev: {}\nhash: {}\ntimestamp: {}\nbits: {}\nheight: {}\nnonce: {}",
            utils::from_be_bytes(&self.prev_block_hash),
            utils::from_be_bytes(&self.hash),
            self.timestamp,
            self.bits,
            self.height,
            self.nonce,
        )
    }
}
