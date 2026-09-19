// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashSet;
use std::hash::Hash;

/// Removes duplicate values, keeping the first occurrence of each and the original order.
///
/// # Examples
///
/// ```
/// use helpers4::array::unique;
///
/// assert_eq!(unique(&[1, 2, 1, 3, 2]), vec![1, 2, 3]);
/// assert_eq!(unique::<i32>(&[]), Vec::<i32>::new());
/// ```
pub fn unique<T: Clone + Eq + Hash>(items: &[T]) -> Vec<T> {
    let mut seen = HashSet::with_capacity(items.len());
    items
        .iter()
        .filter(|item| seen.insert(*item))
        .cloned()
        .collect()
}

#[cfg(test)]
#[path = "unique.test.rs"]
mod tests;

#[cfg(test)]
#[path = "unique.spec.rs"]
mod spec;
