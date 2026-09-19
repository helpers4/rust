// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Digit tables and the shared encode/decode loops. Not re-exported.

use super::error::DecodeError;

const LOWER: &[u8; 16] = b"0123456789abcdef";
const UPPER: &[u8; 16] = b"0123456789ABCDEF";

pub(crate) fn encode_with(bytes: &[u8], upper: bool) -> String {
    let table = if upper { UPPER } else { LOWER };
    let mut out = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        out.push(char::from(table[usize::from(byte >> 4)]));
        out.push(char::from(table[usize::from(byte & 0x0f)]));
    }
    out
}

fn nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn invalid_char(s: &str, index: usize) -> DecodeError {
    // Every byte before `index` was a valid ASCII digit, so `index` is a char boundary.
    DecodeError::InvalidChar {
        index,
        found: s[index..]
            .chars()
            .next()
            .unwrap_or(char::REPLACEMENT_CHARACTER),
    }
}

/// Decodes `s` into `out`. The caller guarantees `s.len() == 2 * out.len()`.
pub(crate) fn decode_into(s: &str, out: &mut [u8]) -> Result<(), DecodeError> {
    for (i, (pair, slot)) in s.as_bytes().chunks_exact(2).zip(out.iter_mut()).enumerate() {
        let high = nibble(pair[0]).ok_or_else(|| invalid_char(s, 2 * i))?;
        let low = nibble(pair[1]).ok_or_else(|| invalid_char(s, 2 * i + 1))?;
        *slot = (high << 4) | low;
    }
    Ok(())
}

#[cfg(test)]
#[path = "_digits.test.rs"]
mod tests;
