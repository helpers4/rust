// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_digits::encode_with;

/// Encodes `bytes` as lowercase hexadecimal.
///
/// # Examples
///
/// ```
/// use helpers4::hex::encode;
///
/// assert_eq!(encode(&[0xde, 0xad, 0xbe, 0xef]), "deadbeef");
/// assert_eq!(encode(&[]), "");
/// ```
pub fn encode(bytes: &[u8]) -> String {
    encode_with(bytes, false)
}

#[cfg(test)]
#[path = "encode.test.rs"]
mod tests;

#[cfg(test)]
#[path = "encode.spec.rs"]
mod spec;
