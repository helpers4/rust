// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::EntropyError;
use super::random_bytes::random_bytes;

/// Random bytes from the operating system, written as lowercase hexadecimal: the usual form of
/// a master key or a session token (`random_hex(32)` is a 256-bit key in 64 characters).
///
/// Needs the `random-entropy` feature.
///
/// # Arguments
///
/// - `byte_len` - The number of random bytes; the result has twice as many characters.
///
/// # Returns
///
/// `2 * byte_len` characters in `0-9a-f`, or an [`EntropyError`].
///
/// # Errors
///
/// Returns an [`EntropyError`] when the operating system cannot provide random bytes.
///
/// # Examples
///
/// ```
/// use helpers4::random::random_hex;
///
/// let key = random_hex(32)?;
/// assert_eq!(key.len(), 64);
/// assert!(key.chars().all(|c| c.is_ascii_hexdigit()));
/// # Ok::<(), helpers4::random::EntropyError>(())
/// ```
pub fn random_hex(byte_len: usize) -> Result<String, EntropyError> {
    random_bytes(byte_len).map(|bytes| to_hex(&bytes))
}

fn to_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(char::from(DIGITS[usize::from(byte >> 4)]));
        out.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    out
}

#[cfg(test)]
#[path = "random_hex.test.rs"]
mod tests;
