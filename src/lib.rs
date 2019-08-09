//! ## Weyl, a fast PRNG based on the Middle Square Weyl Sequence.
//! Warning: Not cryptographically secure.

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

/// Middle Square Weyl Sequence PRNG
/// In other words it generates random numbers.
pub struct Rng {
    x: u64,
    w: u64,
    s: u64,
    seed: u64, // original seed
}

impl Rng {
    pub fn new() -> Self {
        Self::new_seed(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
        )
    }
    pub fn new_seed(seed: u64) -> Self {
        Self {
            x: 0,
            w: 0,
            s: (seed << 1).wrapping_add(0xb5ad4eceda1ce2a9),
            seed: seed,
        }
    }

    /// reseed the generator
    pub fn seed(&mut self, seed: u64) {
        *self = Self::new_seed(seed);
    }

    /// generates a random u64
    pub fn u64(&mut self) -> u64 {
        self.w = self.w.wrapping_add(self.s);
        self.x = (self.x.wrapping_mul(self.x).wrapping_add(self.w) >> 32)
            | (self.x.wrapping_mul(self.x).wrapping_add(self.w) << 32);
        self.x
    }
    /// generates a random f64
    pub fn f64(&mut self) -> f64 {
        self.u64() as f64 / ((0xFFFFFFFFFFFFFFFFu64 as f64) + 1.0)
    }
    /// fill bytes slice with random data
    pub fn fill(&mut self, bytes: &mut [u8]) {
        let mut rval = self.u64();
        let mut i = 0;
        for b in bytes {
            *b = (rval & 0xff) as u8;
            rval >>= 8;
            i += 1;
            if i & 7 == 0 {
                rval = self.u64();
            }
        }
    }
}

lazy_static! {
    static ref RAND: Mutex<Rng> = { Mutex::new(Rng::new()) };
}

pub fn last_seed() -> u64 {
    RAND.lock().unwrap().seed
}
/// reseed the generator
pub fn seed(seed: u64) {
    RAND.lock().unwrap().seed(seed)
}
/// returns a random u64
pub fn u64() -> u64 {
    RAND.lock().unwrap().u64()
}
/// returns a random f64
pub fn f64() -> f64 {
    RAND.lock().unwrap().f64()
}

/// fill bytes slice with random data
pub fn fill(bytes: &mut [u8]) {
    RAND.lock().unwrap().fill(bytes)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn various() {
        assert!(last_seed() != 0);
        assert!(u64() != 0);
        let mut bytes = vec![0; 64];
        fill(&mut bytes);
        let mut ok = false;
        for b in &bytes {
            if *b != 0 {
                ok = true;
                break;
            }
        }
        assert!(ok);
    }
}
