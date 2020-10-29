#[derive(Debug)]
pub struct Transaction(pub String);

impl Transaction {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
