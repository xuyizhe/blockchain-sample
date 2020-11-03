use lib::{Blockchain, Database, Transaction};

fn main() {
    {
        let mut blockchain = Blockchain::new();
        for _ in 0..3 {
            blockchain.add_block(vec![
                Transaction(String::from("TxHash")),
                Transaction(String::from("TxHash")),
            ]);
        }
    }
    // Database::destroy("blockchain-sample-db");
}
