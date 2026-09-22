// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashSet;
use std::hash::Hash;

/// Returns the elements present in exactly one of `a` and `b`.
///
/// The result is the elements of `a` missing from `b` (in `a`'s order), followed by the elements
/// of `b` missing from `a` (in `b`'s order). Duplicates are kept.
///
/// # Arguments
///
/// - `a` - The first slice.
/// - `b` - The second slice.
///
/// # Examples
///
/// ```
/// use helpers4::array::symmetric_difference;
///
/// assert_eq!(symmetric_difference(&[1, 2, 3], &[2, 3, 4]), vec![1, 4]);
/// ```
pub fn symmetric_difference<T: Clone + Eq + Hash>(a: &[T], b: &[T]) -> Vec<T> {
    let in_a: HashSet<&T> = a.iter().collect();
    let in_b: HashSet<&T> = b.iter().collect();
    a.iter()
        .filter(|item| !in_b.contains(item))
        .chain(b.iter().filter(|item| !in_a.contains(item)))
        .cloned()
        .collect()
}

#[cfg(test)]
#[path = "symmetric_difference.test.rs"]
mod tests;

#[cfg(test)]
#[path = "symmetric_difference.spec.rs"]
mod spec;
