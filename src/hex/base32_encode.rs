// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

/// Encodes `data` as base32 (RFC 4648 section 6), with `=` padding.
///
/// # Arguments
///
/// - `data` - The bytes to encode.
///
/// # Returns
///
/// The base32 string; its length is always a multiple of 8.
///
/// # Examples
///
/// ```
/// use helpers4::hex::base32_encode;
///
/// assert_eq!(base32_encode(b"foobar"), "MZXW6YTBOI======");
/// assert_eq!(base32_encode(b""), "");
/// ```
#[must_use]
pub fn base32_encode(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(5) * 8);
    for chunk in data.chunks(5) {
        let mut buf = [0u8; 5];
        buf[..chunk.len()].copy_from_slice(chunk);
        let n = buf.iter().fold(0u64, |acc, &b| (acc << 8) | u64::from(b));
        let chars_needed = (chunk.len() * 8).div_ceil(5);
        for (i, shift) in [35, 30, 25, 20, 15, 10, 5, 0].into_iter().enumerate() {
            out.push(if i < chars_needed {
                char::from(ALPHABET[((n >> shift) & 0x1F) as usize])
            } else {
                '='
            });
        }
    }
    out
}

#[cfg(test)]
#[path = "base32_encode.test.rs"]
mod tests;

#[cfg(test)]
#[path = "base32_encode.spec.rs"]
mod spec;
