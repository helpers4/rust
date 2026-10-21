// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Maps a signed integer to an unsigned one so that small magnitudes stay small: `0, -1, 1, -2,
/// 2, ...` become `0, 1, 2, 3, 4, ...`. Varint formats (Protocol Buffers `sint`, Kiwi) encode
/// signed values this way before LEB128, so that `-1` takes one byte instead of ten.
///
/// The result does not depend on the integer width: an `i32` widened to `i64` encodes to the
/// same value as with 32-bit zigzag.
///
/// # Arguments
///
/// - `value` - The signed integer.
///
/// # Returns
///
/// The zigzag-encoded value; [`zigzag_decode`](super::zigzag_decode) reverses it.
///
/// # Examples
///
/// ```
/// use helpers4::bytes::zigzag_encode;
///
/// assert_eq!(zigzag_encode(0), 0);
/// assert_eq!(zigzag_encode(-1), 1);
/// assert_eq!(zigzag_encode(1), 2);
/// assert_eq!(zigzag_encode(i64::MIN), u64::MAX);
/// ```
#[must_use]
pub fn zigzag_encode(value: i64) -> u64 {
    // Shift the sign out to the right and spread it with an arithmetic shift: bit-casts, no loss.
    u64::from_ne_bytes(((value << 1) ^ (value >> 63)).to_ne_bytes())
}

#[cfg(test)]
#[path = "zigzag_encode.test.rs"]
mod tests;

#[cfg(test)]
#[path = "zigzag_encode.spec.rs"]
mod spec;
