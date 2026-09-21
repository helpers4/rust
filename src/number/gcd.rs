// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Greatest common divisor of `a` and `b`; `gcd(0, 0)` is `0`.
///
/// # Examples
///
/// ```
/// use helpers4::number::gcd;
///
/// assert_eq!(gcd(12, 18), 6);
/// assert_eq!(gcd(7, 13), 1);
/// assert_eq!(gcd(0, 5), 5);
/// ```
#[must_use]
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

#[cfg(test)]
#[path = "gcd.test.rs"]
mod tests;

#[cfg(test)]
#[path = "gcd.spec.rs"]
mod spec;
