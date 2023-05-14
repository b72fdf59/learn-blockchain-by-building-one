use crypto_hash::{digest, Algorithm};
use std::fs::File;
use std::io::Read;

// use sha256 to hash file from filepath
pub fn hash_files(filepath: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(filepath)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(digest(Algorithm::SHA256, data.as_slice()))
}
