use std::ops::Deref;

#[derive(Debug)]
pub struct Transaction<T = String>(pub T);

impl<T> Transaction<T> {
    pub fn new(x: T) -> Transaction<T> {
        Transaction(x)
    }
}

impl<T> Deref for Transaction<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
