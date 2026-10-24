// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_fill_os::fill_os;
use super::EntropyError;

const ALPHABET: &[u8; 62] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

/// A random string of letters and digits (`A-Za-z0-9`) from the operating system's random
/// source, with every character equally likely.
///
/// Needs the `random-entropy` feature. Bytes that would favor some characters are discarded
/// rather than folded with a modulo (256 is not a multiple of 62), so a 20-character result
/// carries `20 × log2(62)` ≈ 119 bits of randomness.
///
/// # Arguments
///
/// - `len` - The number of characters.
///
/// # Returns
///
/// A string of `len` characters, or an [`EntropyError`].
///
/// # Errors
///
/// Returns an [`EntropyError`] when the operating system cannot provide random bytes.
///
/// # Examples
///
/// ```
/// use helpers4::random::random_alphanumeric;
///
/// let token = random_alphanumeric(24)?;
/// assert_eq!(token.len(), 24);
/// assert!(token.chars().all(|c| c.is_ascii_alphanumeric()));
/// # Ok::<(), helpers4::random::EntropyError>(())
/// ```
pub fn random_alphanumeric(len: usize) -> Result<String, EntropyError> {
    alphanumeric_with(len, &mut fill_os)
}

/// The same, with the source of bytes passed in so that the rejection and the error paths can be
/// tested without a failing operating system.
fn alphanumeric_with(
    len: usize,
    fill: &mut dyn FnMut(&mut [u8]) -> Result<(), EntropyError>,
) -> Result<String, EntropyError> {
    let mut out = String::with_capacity(len);
    // Only bytes below 248 (= 4 × 62) map to a character without bias, about 97% of them.
    let mut buffer = vec![0u8; len + len / 16 + 8];
    while out.len() < len {
        fill(&mut buffer)?;
        for &byte in &buffer {
            if byte < 248 && out.len() < len {
                out.push(char::from(ALPHABET[usize::from(byte % 62)]));
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
#[path = "random_alphanumeric.test.rs"]
mod tests;
