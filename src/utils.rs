use rand::distr::Alphanumeric;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn generate_hash() -> u64 {
    let rng = ThreadRng::default();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let mut hasher = DefaultHasher::new();
    random_string.hash(&mut hasher);
    hasher.finish()
}
