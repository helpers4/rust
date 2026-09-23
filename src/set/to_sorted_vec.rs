// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashSet;
use std::hash::BuildHasher;

/// The elements of `set` as a sorted `Vec`.
///
/// A `HashSet` iterates in an unspecified order that changes between runs; use this wherever the
/// order is visible (output, snapshots, tests).
///
/// # Arguments
///
/// - `set` - The set to list.
///
/// # Returns
///
/// The elements in ascending order.
///
/// # Examples
///
/// ```
/// use helpers4::set::to_sorted_vec;
/// use std::collections::HashSet;
///
/// let set = HashSet::from([3, 1, 2]);
/// assert_eq!(to_sorted_vec(&set), vec![1, 2, 3]);
/// ```
#[must_use]
pub fn to_sorted_vec<T: Clone + Ord, S: BuildHasher>(set: &HashSet<T, S>) -> Vec<T> {
    let mut items: Vec<T> = set.iter().cloned().collect();
    items.sort();
    items
}

#[cfg(test)]
#[path = "to_sorted_vec.test.rs"]
mod tests;

#[cfg(test)]
#[path = "to_sorted_vec.spec.rs"]
mod spec;
