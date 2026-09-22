// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_digits::encode_with;

/// Encodes `bytes` as uppercase hexadecimal.
///
/// # Arguments
///
/// - `bytes` - The bytes to encode.
///
/// # Examples
///
/// ```
/// use helpers4::hex::encode_upper;
///
/// assert_eq!(encode_upper(&[0xde, 0xad, 0xbe, 0xef]), "DEADBEEF");
/// ```
#[must_use]
pub fn encode_upper(bytes: &[u8]) -> String {
    encode_with(bytes, true)
}

#[cfg(test)]
#[path = "encode_upper.test.rs"]
mod tests;

#[cfg(test)]
#[path = "encode_upper.spec.rs"]
mod spec;
