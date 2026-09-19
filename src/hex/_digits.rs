// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Digit tables and the shared encode/decode loops. Not re-exported.

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

#[cfg(test)]
#[path = "_digits.test.rs"]
mod tests;
