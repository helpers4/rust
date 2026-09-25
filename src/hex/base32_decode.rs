// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Base32DecodeError;

fn value(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'2'..=b'7' => Some(byte - b'2' + 26),
        _ => None,
    }
}

fn invalid_char(s: &str, index: usize) -> Base32DecodeError {
    // Every byte before `index` was a valid ASCII base32 character, so `index` is a char
    // boundary.
    Base32DecodeError::InvalidChar {
        index,
        found: s[index..]
            .chars()
            .next()
            .unwrap_or(char::REPLACEMENT_CHARACTER),
    }
}

/// Decodes `s` from base32 (RFC 4648 section 6).
///
/// # Arguments
///
/// - `s` - The base32 string to decode, with `=` padding to a multiple of 8 characters.
///
/// # Errors
///
/// Returns a [`Base32DecodeError`] when the length is not a multiple of 8, `=` appears anywhere
/// but the end, or a character is not in the base32 alphabet.
///
/// # Examples
///
/// ```
/// use helpers4::hex::base32_decode;
///
/// assert_eq!(base32_decode("MZXW6YTBOI======").as_deref(), Ok(&b"foobar"[..]));
/// assert!(base32_decode("MZXW6YTBOI").is_err()); // not a multiple of 8
/// ```
pub fn base32_decode(s: &str) -> Result<Vec<u8>, Base32DecodeError> {
    let bytes = s.as_bytes();
    if bytes.len() % 8 != 0 {
        return Err(Base32DecodeError::InvalidLength {
            actual: bytes.len(),
        });
    }
    let pad = bytes
        .iter()
        .rev()
        .take_while(|&&b| b == b'=')
        .count()
        .min(6);
    let data = &bytes[..bytes.len() - pad];
    if data.contains(&b'=') {
        return Err(Base32DecodeError::InvalidPadding);
    }

    let mut out = Vec::with_capacity(data.len() * 5 / 8);
    for (chunk_index, group) in data.chunks(8).enumerate() {
        let mut n: u64 = 0;
        for (i, &byte) in group.iter().enumerate() {
            let v = value(byte).ok_or_else(|| invalid_char(s, chunk_index * 8 + i))?;
            n |= u64::from(v) << (35 - 5 * i);
        }
        let real_bytes = group.len() * 5 / 8;
        for i in 0..real_bytes {
            out.push((n >> (32 - 8 * i) & 0xFF) as u8);
        }
    }
    Ok(out)
}

#[cfg(test)]
#[path = "base32_decode.test.rs"]
mod tests;

#[cfg(test)]
#[path = "base32_decode.spec.rs"]
mod spec;
