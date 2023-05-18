use crypto_hash::{digest, Algorithm};

pub fn hash_email(input_data: &str, secret_phrase: &str) -> Vec<u8> {
    let binding = [input_data].join(secret_phrase);
    let combined_data = binding.as_bytes();
    digest(Algorithm::SHA256, combined_data)
}
