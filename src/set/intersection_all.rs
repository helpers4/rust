// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashSet;
use std::hash::{BuildHasher, Hash};

/// The elements that appear in every set of `sets`.
///
/// [`HashSet::intersection`] only combines two sets and returns a lazy iterator; this takes any
/// number of sets and returns an owned set. An empty slice gives an empty set (there is no set
/// to draw elements from), not "everything".
///
/// # Arguments
///
/// - `sets` - The sets to intersect.
///
/// # Returns
///
/// A new set with the elements common to all of `sets`.
///
/// # Examples
///
/// ```
/// use helpers4::set::intersection_all;
/// use std::collections::HashSet;
///
/// let a = HashSet::from([1, 2, 3]);
/// let b = HashSet::from([2, 3, 4]);
/// let c = HashSet::from([3, 5]);
/// assert_eq!(intersection_all(&[a, b, c]), HashSet::from([3]));
/// ```
#[must_use]
pub fn intersection_all<T: Clone + Eq + Hash, S: BuildHasher>(
    sets: &[HashSet<T, S>],
) -> HashSet<T> {
    let Some((first, rest)) = sets.split_first() else {
        return HashSet::new();
    };
    first
        .iter()
        .filter(|item| rest.iter().all(|set| set.contains(*item)))
        .cloned()
        .collect()
}

#[cfg(test)]
#[path = "intersection_all.test.rs"]
mod tests;

#[cfg(test)]
#[path = "intersection_all.spec.rs"]
mod spec;
