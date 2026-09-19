// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::decode_to_slice::decode_to_slice;
use super::error::DecodeError;

/// Decodes a hexadecimal string into a fixed-size array, e.g. a 32-byte key from 64 hex digits.
///
/// # Errors
///
/// Same as [`decode_to_slice`](super::decode_to_slice): the string must have exactly `2 * N`
/// hex digits.
///
/// # Examples
///
/// ```
/// use helpers4::hex::decode_array;
///
/// let key: [u8; 4] = decode_array("deadbeef")?;
/// assert_eq!(key, [0xde, 0xad, 0xbe, 0xef]);
/// assert!(decode_array::<4>("dead").is_err());
/// # Ok::<(), helpers4::hex::DecodeError>(())
/// ```
pub fn decode_array<const N: usize>(s: &str) -> Result<[u8; N], DecodeError> {
    let mut out = [0u8; N];
    decode_to_slice(s, &mut out)?;
    Ok(out)
}

#[cfg(test)]
#[path = "decode_array.test.rs"]
mod tests;

#[cfg(test)]
#[path = "decode_array.spec.rs"]
mod spec;
