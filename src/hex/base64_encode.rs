// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Encodes `data` as base64 (RFC 4648 section 4), with `=` padding.
///
/// # Arguments
///
/// - `data` - The bytes to encode.
///
/// # Returns
///
/// The base64 string; its length is always a multiple of 4.
///
/// # Examples
///
/// ```
/// use helpers4::hex::base64_encode;
///
/// assert_eq!(base64_encode(b"any carnal pleas"), "YW55IGNhcm5hbCBwbGVhcw==");
/// assert_eq!(base64_encode(b""), "");
/// ```
#[must_use]
pub fn base64_encode(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let mut buf = [0u8; 3];
        buf[..chunk.len()].copy_from_slice(chunk);
        let n = buf.iter().fold(0u32, |acc, &b| (acc << 8) | u32::from(b));
        let chars_needed = (chunk.len() * 8).div_ceil(6);
        for (i, shift) in [18, 12, 6, 0].into_iter().enumerate() {
            out.push(if i < chars_needed {
                char::from(ALPHABET[((n >> shift) & 0x3F) as usize])
            } else {
                '='
            });
        }
    }
    out
}

#[cfg(test)]
#[path = "base64_encode.test.rs"]
mod tests;

#[cfg(test)]
#[path = "base64_encode.spec.rs"]
mod spec;
