use rand::Rng;
use sha2::{Digest, Sha256};

pub fn random_bytes(size: usize) -> Vec<u8> {
    rand::thread_rng()
        .sample_iter(rand::distributions::Standard)
        .take(size)
        .collect()
}

pub fn random_string(size: usize) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

pub fn decode_hash256(hash: &str) -> Option<Vec<u8>> {
    match &hash.len() {
        // Sha256 is 64 chars long
        64 => match hex::decode(hash) {
            Ok(bytes) => Some(bytes),
            _ => None,
        },
        _ => None,
    }
}

pub fn decode_hash128(hash: &str) -> Option<Vec<u8>> {
    match &hash.len() {
        // 128 bits are 32 chars long
        32 => match hex::decode(hash) {
            Ok(bytes) => Some(bytes),
            _ => None,
        },
        _ => None,
    }
}

pub fn hash_sha256(meal: &[u8], pepper: &[u8]) -> Vec<u8> {
    let spiced_meal: Vec<u8> = meal.iter().cloned().chain(pepper.iter().cloned()).collect();
    let digested_meal = Sha256::digest(&spiced_meal);

    // println!("spiced meal: {:?}", spiced_meal);
    // println!("digested meal: {:?}", digested_meal);

    digested_meal.to_vec()
}

pub fn hash128_string(meal: &String) -> Vec<u8> {
    let digested_meal = Sha256::digest(meal.as_bytes());

    digested_meal[..=15].to_vec()
}
