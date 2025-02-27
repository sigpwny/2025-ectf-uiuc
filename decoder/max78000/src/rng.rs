use crate::tmr::Tmr2;
use core::{
    cell::{OnceCell, RefCell},
    slice,
};
use critical_section::Mutex;
use hal::trng::Trng;
use rand::{CryptoRng, Rng, RngCore, SeedableRng};
use rand_chacha::{
    rand_core::impls::{fill_bytes_via_next, next_u64_via_u32},
    ChaCha20Rng,
};
use sha3::{Digest, Sha3_256};

const RESEED_COUNTER: u32 = 64;

static GLOBAL_RNG: Mutex<OnceCell<RefCell<CustomRng>>> = Mutex::new(OnceCell::new());

pub fn init_global_rng(seed: &[u8], trng: Trng, tmr2: Tmr2) {
    let rng = CustomRng::new::<0x100>(seed, trng, tmr2);
    critical_section::with(|cs| {
        if GLOBAL_RNG.borrow(cs).set(RefCell::new(rng)).is_err() {
            panic!("init_global_rng called more than once!");
        }
    });
}

#[no_mangle]
extern "C" fn randombytes(bytes: *mut cty::c_uchar, len: cty::c_ulonglong) {
    let byte_slice = unsafe { slice::from_raw_parts_mut(bytes, len as usize) };

    critical_section::with(|cs| {
        GLOBAL_RNG
            .borrow(cs)
            .get()
            .unwrap()
            .borrow_mut()
            .fill(byte_slice);
    });
}

/// Rng which uses a reseeding process to gather fresh entropy from Trng occasionally
pub struct CustomRng {
    rng: ChaCha20Rng,
    reseed_counter: u32,
    trng: Trng,
    tmr2: Tmr2,
}

pub fn seed_rng<const N: usize>(seed: &[u8], trng: &Trng, tmr2: &Tmr2) -> ChaCha20Rng {
    let mut hasher = Sha3_256::new();
    hasher.update(seed);
    for _ in 0..N {
        hasher.update(trng.gen_u32().to_ne_bytes());
        hasher.update(tmr2.get_tick_count().to_ne_bytes());
    }
    ChaCha20Rng::from_seed(hasher.finalize().into())
}

impl CustomRng {
    /// Create a new CustomRng with a custom seeding process to collect entropy
    pub fn new<const N: usize>(seed: &[u8], trng: Trng, tmr2: Tmr2) -> CustomRng {
        let rng = seed_rng::<N>(seed, &trng, &tmr2);
        CustomRng {
            rng,
            reseed_counter: 0,
            trng,
            tmr2,
        }
    }

    fn reseed(&mut self) {
        let prev_seed = &self.rng.get_seed();
        self.rng = seed_rng::<8>(prev_seed, &self.trng, &self.tmr2);
    }

    fn check_reseed(&mut self) {
        if self.reseed_counter >= RESEED_COUNTER {
            self.reseed_counter = 0;
            self.reseed();
        }
        self.reseed_counter += 1;
    }
}

impl RngCore for CustomRng {
    fn next_u32(&mut self) -> u32 {
        self.check_reseed();
        self.rng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        next_u64_via_u32(self)
    }

    fn fill_bytes(&mut self, dst: &mut [u8]) {
        fill_bytes_via_next(self, dst);
    }
}

impl CryptoRng for CustomRng {}
