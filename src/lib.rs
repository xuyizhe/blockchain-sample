pub mod block;
pub mod pow;
pub mod transaction;
pub mod utils;

pub use crate::block::Block;
pub use crate::pow::Pow;
pub use crate::transaction::Transaction;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let transactions = vec![Transaction(String::from("genesis"))];
        let genesis_block = Block::new([0u8; 32], 0, 0, transactions);
        println!(
            "Genesis block: {}",
            utils::from_be_bytes(&genesis_block.hash)
        );
        Self {
            blocks: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let prev_block = self.blocks.last().clone().unwrap();
        let mut block = Block::new(
            prev_block.hash,
            self.blocks.len() as u32,
            0x8u32 * 2,
            transactions,
        );
        println!("\nMining...");
        block.mine().unwrap();
        println!("{}", block);
        println!("\n{}s", utils::now_as_secs() - block.timestamp);
        self.blocks.push(block);
    }
}
