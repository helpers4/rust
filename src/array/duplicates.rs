// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Returns the elements that appear more than once in `items`, each one once, in the order they
/// first appear.
///
/// The counterpart of [`unique`](super::unique): what `unique` keeps, this reports as repeated.
///
/// # Arguments
///
/// - `items` - The elements to scan.
///
/// # Examples
///
/// ```
/// use helpers4::array::duplicates;
///
/// assert_eq!(duplicates(&[1, 2, 3, 2, 1, 2]), vec![1, 2]);
/// assert_eq!(duplicates(&["a", "b", "c"]), Vec::<&str>::new());
/// ```
#[must_use]
pub fn duplicates<T: Clone + Eq + Hash>(items: &[T]) -> Vec<T> {
    let mut counts: HashMap<&T, usize> = HashMap::new();
    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }
    let mut reported = HashSet::new();
    items
        .iter()
        .filter(|item| counts[*item] > 1 && reported.insert(*item))
        .cloned()
        .collect()
}

#[cfg(test)]
#[path = "duplicates.test.rs"]
mod tests;

#[cfg(test)]
#[path = "duplicates.spec.rs"]
mod spec;
