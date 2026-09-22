// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::hash::Hash;

/// Returns the first item of `iter` that has already appeared earlier in it, or `None` when
/// every item is unique.
///
/// Unlike [`array::duplicates`](crate::array::duplicates), this stops at the first repeat, so it
/// works on an infinite or otherwise unbounded iterator instead of requiring an already-collected
/// slice.
///
/// # Arguments
///
/// - `iter` - The items to scan, in order.
///
/// # Returns
///
/// The first repeated item, or `None` when there is none.
///
/// # Examples
///
/// ```
/// use helpers4::iter::first_duplicate;
///
/// assert_eq!(first_duplicate([1, 2, 3, 2, 1]), Some(2));
/// assert_eq!(first_duplicate(["a", "b", "c"]), None);
/// ```
pub fn first_duplicate<T: Eq + Hash + Clone>(iter: impl IntoIterator<Item = T>) -> Option<T> {
    let mut seen = std::collections::HashSet::new();
    iter.into_iter().find(|item| !seen.insert(item.clone()))
}

#[cfg(test)]
#[path = "first_duplicate.test.rs"]
mod tests;

#[cfg(test)]
#[path = "first_duplicate.spec.rs"]
mod spec;
