// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::gcd;

/// Least common multiple of `a` and `b`, or `None` when it does not fit in a `u64`.
///
/// `lcm(0, n)` is `0`.
///
/// # Examples
///
/// ```
/// use helpers4::number::lcm;
///
/// assert_eq!(lcm(4, 6), Some(12));
/// assert_eq!(lcm(0, 5), Some(0));
/// assert_eq!(lcm(u64::MAX, u64::MAX - 1), None);
/// ```
#[must_use]
pub fn lcm(a: u64, b: u64) -> Option<u64> {
    let divisor = gcd(a, b);
    if divisor == 0 {
        return Some(0);
    }
    (a / divisor).checked_mul(b)
}

#[cfg(test)]
#[path = "lcm.test.rs"]
mod tests;

#[cfg(test)]
#[path = "lcm.spec.rs"]
mod spec;
