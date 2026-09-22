// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_digits::decode_into;
use super::error::DecodeError;

/// Decodes a hexadecimal string into `out`, which must be exactly half as long as the string.
///
/// Nothing is allocated; on error `out` may be partially written.
///
/// # Arguments
///
/// - `hex` - The hexadecimal string to decode.
/// - `out` - The buffer to decode into; must be exactly half of `hex`'s length.
///
/// # Errors
///
/// [`DecodeError::OddLength`], [`DecodeError::InvalidLength`] when the string does not match
/// `out.len() * 2`, or [`DecodeError::InvalidChar`].
///
/// # Examples
///
/// ```
/// use helpers4::hex::decode_to_slice;
///
/// let mut buf = [0u8; 2];
/// decode_to_slice("beef", &mut buf)?;
/// assert_eq!(buf, [0xbe, 0xef]);
/// # Ok::<(), helpers4::hex::DecodeError>(())
/// ```
pub fn decode_to_slice(hex: &str, out: &mut [u8]) -> Result<(), DecodeError> {
    if hex.len() % 2 != 0 {
        return Err(DecodeError::OddLength);
    }
    if hex.len() != out.len() * 2 {
        return Err(DecodeError::InvalidLength {
            expected: out.len() * 2,
            actual: hex.len(),
        });
    }
    decode_into(hex, out)
}

#[cfg(test)]
#[path = "decode_to_slice.test.rs"]
mod tests;

#[cfg(test)]
#[path = "decode_to_slice.spec.rs"]
mod spec;
