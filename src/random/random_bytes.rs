// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_fill_os::fill_os;
use super::EntropyError;

/// Random bytes from the operating system, suitable for keys, tokens and salts.
///
/// Needs the `random-entropy` feature. It returns an error rather than weaker randomness when
/// the system cannot provide bytes.
///
/// # Arguments
///
/// - `len` - The number of bytes.
///
/// # Returns
///
/// `len` random bytes, or an [`EntropyError`].
///
/// # Errors
///
/// Returns an [`EntropyError`] when the operating system cannot provide random bytes.
///
/// # Examples
///
/// ```
/// use helpers4::random::random_bytes;
///
/// let key = random_bytes(32)?;
/// assert_eq!(key.len(), 32);
/// # Ok::<(), helpers4::random::EntropyError>(())
/// ```
pub fn random_bytes(len: usize) -> Result<Vec<u8>, EntropyError> {
    let mut bytes = vec![0u8; len];
    fill_os(&mut bytes).map(|()| bytes)
}

#[cfg(test)]
#[path = "random_bytes.test.rs"]
mod tests;
