// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_digits::decode_into;
use super::error::DecodeError;

/// Decodes a hexadecimal string (either case) into bytes.
///
/// The string must be made of digit pairs only: surrounding whitespace, a `0x` prefix or
/// separators are errors, so trim or strip them first.
///
/// # Errors
///
/// [`DecodeError::OddLength`] for an odd number of characters, [`DecodeError::InvalidChar`] for
/// a character that is not a hex digit.
///
/// # Examples
///
/// ```
/// use helpers4::hex::decode;
///
/// assert_eq!(decode("DeadBeef")?, vec![0xde, 0xad, 0xbe, 0xef]);
/// assert!(decode("abc").is_err());
/// # Ok::<(), helpers4::hex::DecodeError>(())
/// ```
pub fn decode(s: &str) -> Result<Vec<u8>, DecodeError> {
    if s.len() % 2 != 0 {
        return Err(DecodeError::OddLength);
    }
    let mut out = vec![0u8; s.len() / 2];
    decode_into(s, &mut out)?;
    Ok(out)
}

#[cfg(test)]
#[path = "decode.test.rs"]
mod tests;

#[cfg(test)]
#[path = "decode.spec.rs"]
mod spec;
