// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Rng;

/// Picks `count` different elements at random (without replacement), in random order.
///
/// Elements are different by position: two equal values in `items` can both be picked. When
/// `count` is at least `items.len()`, every element is returned, shuffled.
///
/// # Arguments
///
/// - `items` - The elements to pick from.
/// - `count` - How many to pick.
/// - `rng` - The generator that decides the picks.
///
/// # Returns
///
/// A new `Vec` of `min(count, items.len())` elements.
///
/// # Examples
///
/// ```
/// use helpers4::random::{sample, Rng};
///
/// let winners = sample(&["ana", "bo", "cy", "di", "ed"], 2, &mut Rng::seeded(1));
/// assert_eq!(winners.len(), 2);
/// assert_ne!(winners[0], winners[1]);
///
/// assert_eq!(sample(&[1, 2], 10, &mut Rng::seeded(1)).len(), 2);
/// ```
#[must_use]
pub fn sample<T: Clone>(items: &[T], count: usize, rng: &mut Rng) -> Vec<T> {
    let mut pool = items.to_vec();
    let take = count.min(pool.len());
    // Partial Fisher–Yates: the first `take` slots end up as a uniform random selection.
    for i in 0..take {
        let j = i + rng.index(pool.len() - i);
        pool.swap(i, j);
    }
    pool.truncate(take);
    pool
}

#[cfg(test)]
#[path = "sample.test.rs"]
mod tests;

#[cfg(test)]
#[path = "sample.spec.rs"]
mod spec;
