// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashSet;
use std::hash::Hash;

/// Removes elements whose `key` was already seen, keeping the first of each key in order.
///
/// # Examples
///
/// ```
/// use helpers4::array::unique_by;
///
/// let words = ["apple", "avocado", "banana", "blueberry", "cherry"];
/// assert_eq!(
///     unique_by(&words, |w| w.chars().next()),
///     vec!["apple", "banana", "cherry"]
/// );
/// ```
pub fn unique_by<T: Clone, K: Eq + Hash>(items: &[T], mut key: impl FnMut(&T) -> K) -> Vec<T> {
    let mut seen = HashSet::new();
    items
        .iter()
        .filter(|item| seen.insert(key(item)))
        .cloned()
        .collect()
}

#[cfg(test)]
#[path = "unique_by.test.rs"]
mod tests;

#[cfg(test)]
#[path = "unique_by.spec.rs"]
mod spec;
