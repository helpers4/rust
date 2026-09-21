// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashMap;
use std::hash::Hash;

/// Indexes the elements of `items` by the key returned by `key`.
///
/// When several elements share a key, the last one wins. Use [`group_by`](super::group_by) to keep
/// them all.
///
/// # Examples
///
/// ```
/// use helpers4::array::key_by;
///
/// let users = [("ann", 1), ("bob", 2), ("cat", 1)];
/// let by_id = key_by(&users, |&(_, id)| id);
/// assert_eq!(by_id[&2], ("bob", 2));
/// assert_eq!(by_id[&1], ("cat", 1));
/// ```
pub fn key_by<T: Clone, K: Eq + Hash>(items: &[T], mut key: impl FnMut(&T) -> K) -> HashMap<K, T> {
    let mut indexed = HashMap::with_capacity(items.len());
    for item in items {
        indexed.insert(key(item), item.clone());
    }
    indexed
}

#[cfg(test)]
#[path = "key_by.test.rs"]
mod tests;

#[cfg(test)]
#[path = "key_by.spec.rs"]
mod spec;
