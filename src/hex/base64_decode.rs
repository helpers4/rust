// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Base64DecodeError;

fn value(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

fn invalid_char(s: &str, index: usize) -> Base64DecodeError {
    // Every byte before `index` was a valid ASCII base64 character, so `index` is a char
    // boundary.
    Base64DecodeError::InvalidChar {
        index,
        found: s[index..]
            .chars()
            .next()
            .unwrap_or(char::REPLACEMENT_CHARACTER),
    }
}

/// Decodes `s` from base64 (RFC 4648 section 4).
///
/// # Arguments
///
/// - `s` - The base64 string to decode, with `=` padding to a multiple of 4 characters.
///
/// # Errors
///
/// Returns a [`Base64DecodeError`] when the length is not a multiple of 4, `=` appears anywhere
/// but the end, or a character is not in the base64 alphabet.
///
/// # Examples
///
/// ```
/// use helpers4::hex::base64_decode;
///
/// assert_eq!(base64_decode("TWFu").as_deref(), Ok(&b"Man"[..]));
/// assert!(base64_decode("TWFu!").is_err()); // not a multiple of 4
/// ```
pub fn base64_decode(s: &str) -> Result<Vec<u8>, Base64DecodeError> {
    let bytes = s.as_bytes();
    if bytes.len() % 4 != 0 {
        return Err(Base64DecodeError::InvalidLength {
            actual: bytes.len(),
        });
    }
    let pad = bytes
        .iter()
        .rev()
        .take_while(|&&b| b == b'=')
        .count()
        .min(2);
    let data = &bytes[..bytes.len() - pad];
    if data.contains(&b'=') {
        return Err(Base64DecodeError::InvalidPadding);
    }

    let mut sextets = Vec::with_capacity(data.len());
    for (index, &byte) in data.iter().enumerate() {
        sextets.push(value(byte).ok_or_else(|| invalid_char(s, index))?);
    }

    // `data.len()` is `bytes.len() - pad`, `bytes.len()` a multiple of 4 and `pad` at most 2, so
    // every group below has 2, 3 or 4 sextets — never 1 (6 bits alone can't start a byte).
    let mut out = Vec::with_capacity(sextets.len() * 3 / 4);
    for group in sextets.chunks(4) {
        let n = group
            .iter()
            .enumerate()
            .fold(0u32, |acc, (i, &v)| acc | (u32::from(v) << (18 - 6 * i)));
        out.push((n >> 16 & 0xFF) as u8);
        if group.len() >= 3 {
            out.push((n >> 8 & 0xFF) as u8);
        }
        if group.len() == 4 {
            out.push((n & 0xFF) as u8);
        }
    }
    Ok(out)
}

#[cfg(test)]
#[path = "base64_decode.test.rs"]
mod tests;

#[cfg(test)]
#[path = "base64_decode.spec.rs"]
mod spec;
