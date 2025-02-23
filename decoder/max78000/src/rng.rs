use crate::tmr::Tmr2;
use common::constants::LEN_RNG_SEED;
use hal::trng::Trng;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use sha3::{Digest, Sha3_256};

/// Create a new ChaCha20Rng with a custom seeding process to collect entropy
pub fn new_custom_rng(seed: &[u8; LEN_RNG_SEED], trng: &Trng, tmr2: &Tmr2) -> ChaCha20Rng {
    let mut hasher = Sha3_256::new();
    hasher.update(seed);
    for _ in 0..0x100 {
        hasher.update(trng.gen_u32().to_ne_bytes());
        hasher.update(tmr2.get_tick_count().to_ne_bytes());
    }
    ChaCha20Rng::from_seed(hasher.finalize().into())
}