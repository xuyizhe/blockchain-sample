use crate::block::Block;
use crate::database::Database;
use crate::transaction::Transaction;
use crate::utils;

#[derive(Debug)]
pub struct Blockchain {
    pub last_block_hash: Vec<u8>,
    pub db: Database,
}

impl Blockchain {
    pub fn new() -> Self {
        let db = Database::new("blockchain-sample-db");
        match db.get(&db.last_block_key) {
            Some(last_block_hash) => Self {
                last_block_hash,
                db,
            },
            None => {
                let transactions = vec![Transaction(String::from("genesis"))];
                let genesis_block = Block::new([0u8; 32], 0, 0, transactions);
                println!(
                    "Genesis block: {}",
                    utils::from_be_bytes(&genesis_block.hash)
                );
                db.put(
                    genesis_block.hash,
                    bincode::serialize(&genesis_block).unwrap(),
                );
                db.put(db.last_block_key, genesis_block.hash);
                Self {
                    last_block_hash: genesis_block.hash.to_vec(),
                    db,
                }
            }
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let last_block_encoded = self.db.get(&self.last_block_hash).unwrap();
        let last_block_decoded: Block = bincode::deserialize(&last_block_encoded[..]).unwrap();
        let mut block = Block::new(
            last_block_decoded.hash,
            last_block_decoded.height + 1,
            0x8u32 * 2,
            transactions,
        );
        println!("\nMining...");
        block.mine().unwrap();
        println!("{}", block);
        println!("\n{}s", utils::now_as_secs() - block.timestamp);
        let block_encoded: Vec<u8> = bincode::serialize(&block).unwrap();
        self.db.put(block.hash, block_encoded);
        self.db.put(self.db.last_block_key, block.hash);
        self.last_block_hash = block.hash.to_vec();
    }
}
