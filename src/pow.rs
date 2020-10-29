use crate::utils;
use blake3;
use std::fmt;

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

impl fmt::Display for Pow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "target: {}", utils::from_be_bytes(&self.target))
    }
}
