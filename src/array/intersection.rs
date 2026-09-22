// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashSet;
use std::hash::Hash;

/// Returns the elements of `a` that also appear in `b`, in `a`'s order.
///
/// Duplicates in `a` are kept: only membership in `b` decides whether an element stays.
///
/// # Arguments
///
/// - `a` - The first slice.
/// - `b` - The second slice.
///
/// # Examples
///
/// ```
/// use helpers4::array::intersection;
///
/// assert_eq!(intersection(&[1, 2, 3, 2], &[2, 3, 4]), vec![2, 3, 2]);
/// assert_eq!(intersection(&[1], &[2]), Vec::<i32>::new());
/// ```
pub fn intersection<T: Clone + Eq + Hash>(a: &[T], b: &[T]) -> Vec<T> {
    let wanted: HashSet<&T> = b.iter().collect();
    a.iter()
        .filter(|item| wanted.contains(item))
        .cloned()
        .collect()
}

#[cfg(test)]
#[path = "intersection.test.rs"]
mod tests;

#[cfg(test)]
#[path = "intersection.spec.rs"]
mod spec;
