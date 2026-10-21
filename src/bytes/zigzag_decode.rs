// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Reverses [`zigzag_encode`](super::zigzag_encode): `0, 1, 2, 3, 4, ...` become
/// `0, -1, 1, -2, 2, ...`.
///
/// # Arguments
///
/// - `value` - The zigzag-encoded value.
///
/// # Returns
///
/// The signed integer. A value read from a 32-bit varint always fits in an `i32`.
///
/// # Examples
///
/// ```
/// use helpers4::bytes::zigzag_decode;
///
/// assert_eq!(zigzag_decode(0), 0);
/// assert_eq!(zigzag_decode(1), -1);
/// assert_eq!(zigzag_decode(2), 1);
/// assert_eq!(zigzag_decode(u64::MAX), i64::MIN);
/// ```
#[must_use]
pub fn zigzag_decode(value: u64) -> i64 {
    let half = i64::from_ne_bytes((value >> 1).to_ne_bytes());
    let sign = -i64::from_ne_bytes((value & 1).to_ne_bytes());
    half ^ sign
}

#[cfg(test)]
#[path = "zigzag_decode.test.rs"]
mod tests;

#[cfg(test)]
#[path = "zigzag_decode.spec.rs"]
mod spec;
