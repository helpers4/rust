// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Alphabet;
use crate::random::Rng;

/// A short id of `len` characters drawn from an [`Alphabet`] with a seedable generator, every
/// character equally likely.
///
/// The same generator state always gives the same id, which makes it right for tests and
/// reproducible fixtures, and **wrong for anything secret** (a 64-bit seed can be guessed): use
/// `new_short_id` (feature `random-entropy`) for tokens.
///
/// # Arguments
///
/// - `alphabet` - The characters to draw from.
/// - `len` - The number of characters.
/// - `rng` - The generator that decides them.
///
/// # Returns
///
/// A string of `len` characters (`len` characters, not bytes, for a non-ASCII alphabet).
///
/// # Examples
///
/// ```
/// use helpers4::id::{short_id, Alphabet};
/// use helpers4::random::Rng;
///
/// let id = short_id(&Alphabet::safe(), 8, &mut Rng::seeded(1));
/// assert_eq!(id.chars().count(), 8);
/// assert_eq!(id, short_id(&Alphabet::safe(), 8, &mut Rng::seeded(1))); // reproducible
/// ```
#[must_use]
pub fn short_id(alphabet: &Alphabet, len: usize, rng: &mut Rng) -> String {
    (0..len)
        .map(|_| alphabet.pick(rng.index(alphabet.size())))
        .collect()
}

#[cfg(test)]
#[path = "short_id.test.rs"]
mod tests;

#[cfg(test)]
#[path = "short_id.spec.rs"]
mod spec;
