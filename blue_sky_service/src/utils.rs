use sha2::{Sha256, Sha512, Digest};

pub fn get_hex(pwd: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(pwd.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}
