// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::time::Duration;

/// The delay before retry number `attempt`, doubling each time up to `max`: exponential backoff.
///
/// Attempt `1` waits `base`, attempt `2` waits `2 × base`, attempt `3` `4 × base`, and so on,
/// never more than `max`. Attempt `0` is treated like attempt `1`. The arithmetic saturates, so
/// a huge attempt number gives `max` instead of overflowing. Nothing sleeps: the caller decides
/// what to do with the duration (see [`retry`](super::retry)).
///
/// # Arguments
///
/// - `attempt` - The number of the attempt that just failed, starting at `1`.
/// - `base` - The delay after the first failure.
/// - `max` - The longest delay to ever return.
///
/// # Returns
///
/// The duration to wait, between `base` and `max` (or `max` alone if `base` is larger).
///
/// # Examples
///
/// ```
/// use helpers4::function::backoff;
/// use std::time::Duration;
///
/// let base = Duration::from_millis(100);
/// let max = Duration::from_secs(1);
/// assert_eq!(backoff(1, base, max), Duration::from_millis(100));
/// assert_eq!(backoff(3, base, max), Duration::from_millis(400));
/// assert_eq!(backoff(10, base, max), max);
/// ```
#[must_use]
pub fn backoff(attempt: u32, base: Duration, max: Duration) -> Duration {
    let factor = 1u32
        .checked_shl(attempt.saturating_sub(1))
        .unwrap_or(u32::MAX);
    base.saturating_mul(factor).min(max)
}

#[cfg(test)]
#[path = "backoff.test.rs"]
mod tests;

#[cfg(test)]
#[path = "backoff.spec.rs"]
mod spec;
