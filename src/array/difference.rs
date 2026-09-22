// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashSet;
use std::hash::Hash;

/// Returns the elements of `a` that are not in `b`, in `a`'s order.
///
/// Duplicates in `a` are kept: only membership in `b` decides whether an element stays.
///
/// # Arguments
///
/// - `a` - The elements to keep from.
/// - `b` - The elements to remove.
///
/// # Examples
///
/// ```
/// use helpers4::array::difference;
///
/// assert_eq!(difference(&[1, 2, 3, 2], &[2]), vec![1, 3]);
/// assert_eq!(difference(&[1, 1, 2], &[3]), vec![1, 1, 2]);
/// ```
pub fn difference<T: Clone + Eq + Hash>(a: &[T], b: &[T]) -> Vec<T> {
    let excluded: HashSet<&T> = b.iter().collect();
    a.iter()
        .filter(|item| !excluded.contains(item))
        .cloned()
        .collect()
}

#[cfg(test)]
#[path = "difference.test.rs"]
mod tests;

#[cfg(test)]
#[path = "difference.spec.rs"]
mod spec;
