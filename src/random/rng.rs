// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// A small, fast, seedable pseudo-random generator (SplitMix64): the same seed always gives the
/// same sequence, on every platform.
///
/// It is meant for reproducible work (tests, procedural content, shuffling with a recorded
/// seed) and passed explicitly to [`shuffle`](super::shuffle), [`sample`](super::sample) and
/// [`choose`](super::choose). It is **not cryptographically secure**: never derive a key, a
/// token or an identifier that must stay secret from it. Use the `random-entropy` feature for
/// that.
///
/// # Examples
///
/// ```
/// use helpers4::random::Rng;
///
/// let mut a = Rng::seeded(42);
/// let mut b = Rng::seeded(42);
/// assert_eq!(a.next_u64(), b.next_u64()); // same seed, same sequence
///
/// let roll = a.below(6).map(|n| n + 1); // 1..=6, without modulo bias
/// assert!(matches!(roll, Some(1..=6)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rng {
    state: u64,
}

impl Rng {
    /// Starts a generator from a seed. Any `u64` is valid, `0` included.
    ///
    /// # Arguments
    ///
    /// - `seed` - The starting state; equal seeds give equal sequences.
    ///
    /// # Returns
    ///
    /// The generator.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::random::Rng;
    ///
    /// assert_eq!(Rng::seeded(0).next_u64(), 0xE220_A839_7B1D_CDAF);
    /// ```
    #[must_use]
    pub const fn seeded(seed: u64) -> Self {
        Self { state: seed }
    }

    /// The next 64 random bits.
    ///
    /// # Returns
    ///
    /// A `u64` where every bit pattern is equally likely.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::random::Rng;
    ///
    /// let mut rng = Rng::seeded(1);
    /// assert_eq!(rng.next_u64(), 0x910A_2DEC_8902_5CC1);
    /// assert_eq!(rng.next_u64(), 0xBEEB_8DA1_658E_EC67);
    /// ```
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// A random number in `[0, 1)`, with 53 bits of precision (every value is a multiple of
    /// 2⁻⁵³).
    ///
    /// # Returns
    ///
    /// A value `>= 0.0` and `< 1.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::random::Rng;
    ///
    /// let x = Rng::seeded(7).next_f64();
    /// assert!((0.0..1.0).contains(&x));
    /// ```
    pub fn next_f64(&mut self) -> f64 {
        // The top 53 bits fit a f64 mantissa exactly, so the conversion loses nothing.
        #[allow(clippy::cast_precision_loss)]
        let mantissa = (self.next_u64() >> 11) as f64;
        mantissa * (1.0 / 9_007_199_254_740_992.0)
    }

    /// A random integer in `0..bound`, uniformly: values are rejected rather than folded with a
    /// modulo, which would favor small results when `bound` does not divide 2⁶⁴.
    ///
    /// # Arguments
    ///
    /// - `bound` - The exclusive upper bound.
    ///
    /// # Returns
    ///
    /// A value `< bound`, or `None` when `bound` is `0` (there is no such value).
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::random::Rng;
    ///
    /// let mut rng = Rng::seeded(3);
    /// assert!(rng.below(10).is_some_and(|n| n < 10));
    /// assert_eq!(rng.below(1), Some(0));
    /// assert_eq!(rng.below(0), None);
    /// ```
    pub fn below(&mut self, bound: u64) -> Option<u64> {
        if bound == 0 {
            return None;
        }
        // Lemire's method: the high 64 bits of x * bound are uniform in 0..bound once the
        // draws whose low bits fall in the biased zone are rejected.
        let mut product = u128::from(self.next_u64()) * u128::from(bound);
        #[allow(clippy::cast_possible_truncation)] // keeping the low 64 bits is the point
        let mut low = product as u64;
        if low < bound {
            let threshold = bound.wrapping_neg() % bound;
            while low < threshold {
                product = u128::from(self.next_u64()) * u128::from(bound);
                #[allow(clippy::cast_possible_truncation)]
                {
                    low = product as u64;
                }
            }
        }
        #[allow(clippy::cast_possible_truncation)] // the high half is below `bound`
        Some((product >> 64) as u64)
    }

    /// Fills `dest` with random bytes.
    ///
    /// # Arguments
    ///
    /// - `dest` - The buffer to fill.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::random::Rng;
    ///
    /// let mut buf = [0u8; 5];
    /// Rng::seeded(9).fill_bytes(&mut buf);
    /// assert_ne!(buf, [0u8; 5]);
    /// ```
    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        for chunk in dest.chunks_mut(8) {
            let bytes = self.next_u64().to_le_bytes();
            chunk.copy_from_slice(&bytes[..chunk.len()]);
        }
    }

    /// A uniform index in `0..len`, or `0` when `len` is `0` (callers check emptiness first).
    pub(crate) fn index(&mut self, len: usize) -> usize {
        u64::try_from(len)
            .ok()
            .and_then(|bound| self.below(bound))
            .and_then(|n| usize::try_from(n).ok())
            .unwrap_or(0)
    }
}

#[cfg(all(
    feature = "random-entropy",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
impl Rng {
    /// Starts a generator from a seed drawn from the operating system, for work that must differ
    /// on every run but needs no secrecy (a random order, a random pick).
    ///
    /// Needs the `random-entropy` feature. The generator itself is still not cryptographically
    /// secure: for keys and tokens, use [`random_bytes`](super::random_bytes) directly.
    ///
    /// # Returns
    ///
    /// A generator with an unpredictable seed.
    ///
    /// # Errors
    ///
    /// Returns an [`EntropyError`](super::EntropyError) when the operating system cannot provide
    /// random bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::random::{shuffle, Rng};
    ///
    /// let mut rng = Rng::from_entropy()?;
    /// let order = shuffle(&[1, 2, 3], &mut rng);
    /// assert_eq!(order.len(), 3);
    /// # Ok::<(), helpers4::random::EntropyError>(())
    /// ```
    pub fn from_entropy() -> Result<Self, super::EntropyError> {
        let mut seed = [0u8; 8];
        super::_fill_os::fill_os(&mut seed).map(|()| Self::seeded(u64::from_le_bytes(seed)))
    }
}

#[cfg(test)]
#[path = "rng.test.rs"]
mod tests;

#[cfg(test)]
#[path = "rng.spec.rs"]
mod spec;
