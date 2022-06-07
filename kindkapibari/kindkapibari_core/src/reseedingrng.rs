use rand::RngCore;
#[cfg(feature = "server")]
use rand_chacha::ChaCha20Rng;

pub struct AutoReseedingRng<const MAX_BYTES: usize> {
    rng_core: ChaCha20Rng,
    bytes_generated: usize,
}

impl<const MAX_BYTES: usize> AutoReseedingRng<MAX_BYTES> {
    pub fn new() -> AutoReseedingRng<MAX_BYTES> {
        AutoReseedingRng {
            rng_core: ChaCha20Rng::from_entropy(),
            bytes_generated: 0,
        }
    }

    fn check_rng_bytes_remaining(&mut self, written: usize) {
        if self.bytes_generated.saturating_add(written) >= MAX_BYTES {
            self.rng_core = ChaCha20Rng::from_entropy();
        } else {
            self.bytes_generated += written;
        }
    }

    pub fn generate_bytes<const N: usize>(&mut self) -> [u8; N] {
        let mut arr: [u8; N] = [0_u8; N];
        self.rng_core.fill_bytes(&mut arr);
        self.check_rng_bytes_remaining(N);
        arr
    }
}
