use std::time::{SystemTime, UNIX_EPOCH};

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
