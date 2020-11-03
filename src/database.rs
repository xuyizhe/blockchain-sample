use rocksdb::{Options, DB};
use std::path::Path;

#[derive(Debug)]
pub struct Database {
    pub value: DB,
    pub last_block_key: &'static [u8],
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let db = DB::open_default(path).unwrap();
        Self {
            value: db,
            last_block_key: b"l",
        }
    }

    pub fn put<K, V>(&self, k: K, v: V)
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>,
    {
        self.value.put(k, v).unwrap()
    }

    pub fn get<K: AsRef<[u8]>>(&self, k: K) -> Option<Vec<u8>> {
        self.value.get(k).unwrap()
    }

    pub fn destroy<P: AsRef<Path>>(path: P) {
        let _ = DB::destroy(&Options::default(), path);
    }
}
