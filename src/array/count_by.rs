// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashMap;
use std::hash::Hash;

/// Counts the elements of `items` per key returned by `key`.
///
/// # Arguments
///
/// - `items` - The elements to count.
/// - `key` - Returns the key to count each element under.
///
/// # Examples
///
/// ```
/// use helpers4::array::count_by;
///
/// let counts = count_by(&[1, 2, 3, 4, 5], |n| if n % 2 == 0 { "even" } else { "odd" });
/// assert_eq!(counts["odd"], 3);
/// assert_eq!(counts["even"], 2);
/// ```
pub fn count_by<T, K: Eq + Hash>(items: &[T], mut key: impl FnMut(&T) -> K) -> HashMap<K, usize> {
    let mut counts = HashMap::new();
    for item in items {
        *counts.entry(key(item)).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
#[path = "count_by.test.rs"]
mod tests;

#[cfg(test)]
#[path = "count_by.spec.rs"]
mod spec;
