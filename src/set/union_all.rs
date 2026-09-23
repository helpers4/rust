// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashSet;
use std::hash::{BuildHasher, Hash};

/// The union of every set in `sets`.
///
/// [`HashSet::union`] only combines two sets and returns a lazy iterator; this takes any number
/// of sets and returns an owned set. An empty slice gives an empty set.
///
/// # Arguments
///
/// - `sets` - The sets to merge.
///
/// # Returns
///
/// A new set with every element that appears in at least one of `sets`.
///
/// # Examples
///
/// ```
/// use helpers4::set::union_all;
/// use std::collections::HashSet;
///
/// let a = HashSet::from([1, 2]);
/// let b = HashSet::from([2, 3]);
/// let c = HashSet::from([4]);
/// assert_eq!(union_all(&[a, b, c]), HashSet::from([1, 2, 3, 4]));
/// ```
#[must_use]
pub fn union_all<T: Clone + Eq + Hash, S: BuildHasher>(sets: &[HashSet<T, S>]) -> HashSet<T> {
    sets.iter().flatten().cloned().collect()
}

#[cfg(test)]
#[path = "union_all.test.rs"]
mod tests;

#[cfg(test)]
#[path = "union_all.spec.rs"]
mod spec;
