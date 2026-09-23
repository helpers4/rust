// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// A token-bucket rate limiter with the clock passed in.
///
/// The bucket holds up to `capacity` tokens and gets `refill_per_second` new ones every second.
/// Each action takes tokens with [`try_acquire`](Self::try_acquire); when there are not enough
/// left the action is refused, so bursts up to `capacity` are allowed while the long-run rate
/// stays at `refill_per_second`. Nothing here reads a clock: pass the current time in
/// milliseconds (any monotonic count works). A time that goes backwards is treated as no time
/// having passed.
///
/// # Examples
///
/// ```
/// use helpers4::function::TokenBucket;
///
/// // A burst of 2, then one more every second.
/// let mut bucket = TokenBucket::new(2, 1, 0);
/// assert!(bucket.try_acquire(0));
/// assert!(bucket.try_acquire(0));
/// assert!(!bucket.try_acquire(500));  // half a token is not enough
/// assert!(bucket.try_acquire(1_000)); // one full token has come back
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenBucket {
    capacity_milli: u64,
    refill_per_second: u64,
    available_milli: u64,
    last_ms: u64,
}

impl TokenBucket {
    /// Creates a full bucket.
    ///
    /// # Arguments
    ///
    /// - `capacity` - The most tokens the bucket holds, which is also the largest burst.
    /// - `refill_per_second` - How many tokens are added every second.
    /// - `now_ms` - The current time in milliseconds.
    ///
    /// # Returns
    ///
    /// A bucket holding `capacity` tokens.
    #[must_use]
    pub fn new(capacity: u32, refill_per_second: u32, now_ms: u64) -> Self {
        let capacity_milli = u64::from(capacity) * 1000;
        Self {
            capacity_milli,
            refill_per_second: u64::from(refill_per_second),
            available_milli: capacity_milli,
            last_ms: now_ms,
        }
    }

    /// Takes one token if there is one.
    ///
    /// # Arguments
    ///
    /// - `now_ms` - The current time in milliseconds.
    ///
    /// # Returns
    ///
    /// `true` when a token was taken, `false` when the bucket is empty.
    pub fn try_acquire(&mut self, now_ms: u64) -> bool {
        self.try_acquire_n(now_ms, 1)
    }

    /// Takes `tokens` tokens at once if that many are available, and none otherwise.
    ///
    /// # Arguments
    ///
    /// - `now_ms` - The current time in milliseconds.
    /// - `tokens` - How many tokens the action costs.
    ///
    /// # Returns
    ///
    /// `true` when the tokens were taken, `false` when there are not enough (nothing is taken).
    pub fn try_acquire_n(&mut self, now_ms: u64, tokens: u32) -> bool {
        self.refill(now_ms);
        let cost = u64::from(tokens) * 1000;
        if self.available_milli < cost {
            return false;
        }
        self.available_milli -= cost;
        true
    }

    /// How many whole tokens are available right now.
    ///
    /// # Arguments
    ///
    /// - `now_ms` - The current time in milliseconds.
    ///
    /// # Returns
    ///
    /// The number of tokens that [`try_acquire_n`](Self::try_acquire_n) would grant at once.
    pub fn available(&mut self, now_ms: u64) -> u32 {
        self.refill(now_ms);
        u32::try_from(self.available_milli / 1000).unwrap_or(u32::MAX)
    }

    /// Adds the tokens earned since the last call, without exceeding the capacity.
    fn refill(&mut self, now_ms: u64) {
        let elapsed = now_ms.saturating_sub(self.last_ms);
        self.last_ms = self.last_ms.max(now_ms);
        // A token takes 1000 / refill_per_second ms, so each elapsed ms earns
        // `refill_per_second` thousandths of a token.
        let earned = elapsed.saturating_mul(self.refill_per_second);
        self.available_milli = self
            .available_milli
            .saturating_add(earned)
            .min(self.capacity_milli);
    }
}

#[cfg(test)]
#[path = "token_bucket.test.rs"]
mod tests;

#[cfg(test)]
#[path = "token_bucket.spec.rs"]
mod spec;
