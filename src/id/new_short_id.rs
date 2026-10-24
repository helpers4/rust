// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Alphabet;
use crate::random::_fill_os::fill_os;
use crate::random::EntropyError;

/// A short id of `len` characters drawn from an [`Alphabet`] with the operating system's random
/// source, every character equally likely.
///
/// Needs the `random-entropy` feature. Bytes that would favor some characters (256 is rarely a
/// multiple of the alphabet size) are discarded, not folded with a modulo. The result carries
/// `len × log2(size)` bits of randomness: 21 characters of [`url_safe`](Alphabet::url_safe) give
/// 126 bits, as much as a UUID.
///
/// # Arguments
///
/// - `alphabet` - The characters to draw from.
/// - `len` - The number of characters.
///
/// # Returns
///
/// A string of `len` characters.
///
/// # Errors
///
/// Returns an [`EntropyError`] when the operating system cannot provide random bytes.
///
/// # Examples
///
/// ```
/// use helpers4::id::{new_short_id, Alphabet};
///
/// let id = new_short_id(&Alphabet::url_safe(), 21)?;
/// assert_eq!(id.chars().count(), 21);
/// # Ok::<(), helpers4::random::EntropyError>(())
/// ```
pub fn new_short_id(alphabet: &Alphabet, len: usize) -> Result<String, EntropyError> {
    short_id_with(alphabet, len, &mut fill_os)
}

/// The same, with the source of bytes passed in so that the rejection and the error paths can be
/// tested without a failing operating system.
fn short_id_with(
    alphabet: &Alphabet,
    len: usize,
    fill: &mut dyn FnMut(&mut [u8]) -> Result<(), EntropyError>,
) -> Result<String, EntropyError> {
    let size = alphabet.size();
    // Only bytes below the largest multiple of `size` map to a character without bias.
    let limit = 256 - 256 % size;
    let mut out = String::with_capacity(len);
    let mut buffer = vec![0u8; len + len / 4 + 8];
    while out.chars().count() < len {
        fill(&mut buffer)?;
        for &byte in &buffer {
            if usize::from(byte) < limit && out.chars().count() < len {
                out.push(alphabet.pick(usize::from(byte) % size));
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
#[path = "new_short_id.test.rs"]
mod tests;
