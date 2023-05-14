use crypto_hash::{digest, Algorithm};

// use sha256 to hash strings
pub fn hash_strings(data: &[u8]) -> Vec<u8> {
    digest(Algorithm::SHA256, data)
}
