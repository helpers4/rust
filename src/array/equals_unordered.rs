// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashMap;
use std::hash::Hash;

/// Returns `true` when `a` and `b` hold the same elements the same number of times, in any order.
///
/// Use it for collections where order is meaningless (tags, ids). For positional equality,
/// compare the slices with `==`.
///
/// # Examples
///
/// ```
/// use helpers4::array::equals_unordered;
///
/// assert!(equals_unordered(&[1, 2, 2, 3], &[3, 2, 1, 2]));
/// assert!(!equals_unordered(&[1, 2, 2], &[1, 1, 2]));
/// ```
pub fn equals_unordered<T: Eq + Hash>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut remaining: HashMap<&T, usize> = HashMap::new();
    for item in a {
        *remaining.entry(item).or_insert(0) += 1;
    }
    for item in b {
        match remaining.get_mut(item) {
            Some(count) if *count > 0 => *count -= 1,
            _ => return false,
        }
    }
    true
}

#[cfg(test)]
#[path = "equals_unordered.test.rs"]
mod tests;

#[cfg(test)]
#[path = "equals_unordered.spec.rs"]
mod spec;
