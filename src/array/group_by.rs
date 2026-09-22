// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashMap;
use std::hash::Hash;

/// Groups the elements of `items` by the key returned by `key`.
///
/// Within each group, elements keep their original order. The order of the groups themselves is
/// unspecified (`HashMap`).
///
/// # Arguments
///
/// - `items` - The elements to group.
/// - `key` - Returns the key to group each element under.
///
/// # Examples
///
/// ```
/// use helpers4::array::group_by;
///
/// let groups = group_by(&[1, 2, 3, 4, 5], |n| n % 2 == 0);
/// assert_eq!(groups[&true], vec![2, 4]);
/// assert_eq!(groups[&false], vec![1, 3, 5]);
/// ```
pub fn group_by<T: Clone, K: Eq + Hash>(
    items: &[T],
    mut key: impl FnMut(&T) -> K,
) -> HashMap<K, Vec<T>> {
    let mut groups: HashMap<K, Vec<T>> = HashMap::new();
    for item in items {
        groups.entry(key(item)).or_default().push(item.clone());
    }
    groups
}

#[cfg(test)]
#[path = "group_by.test.rs"]
mod tests;

#[cfg(test)]
#[path = "group_by.spec.rs"]
mod spec;
