// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Rng;

/// Returns the elements in a random order (Fisher–Yates: every permutation is equally likely),
/// leaving `items` as it is.
///
/// The order depends only on the generator, so the same seed always gives the same result.
///
/// # Arguments
///
/// - `items` - The elements to shuffle.
/// - `rng` - The generator that decides the order.
///
/// # Returns
///
/// A new `Vec` with the same elements in a random order.
///
/// # Examples
///
/// ```
/// use helpers4::random::{shuffle, Rng};
///
/// let deck = [1, 2, 3, 4, 5];
/// let first = shuffle(&deck, &mut Rng::seeded(7));
/// let again = shuffle(&deck, &mut Rng::seeded(7));
/// assert_eq!(first, again); // reproducible
///
/// let mut sorted = first.clone();
/// sorted.sort_unstable();
/// assert_eq!(sorted, deck); // nothing lost, nothing added
/// ```
#[must_use]
pub fn shuffle<T: Clone>(items: &[T], rng: &mut Rng) -> Vec<T> {
    let mut out = items.to_vec();
    for i in (1..out.len()).rev() {
        let j = rng.index(i + 1);
        out.swap(i, j);
    }
    out
}

#[cfg(test)]
#[path = "shuffle.test.rs"]
mod tests;

#[cfg(test)]
#[path = "shuffle.spec.rs"]
mod spec;
