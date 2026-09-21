// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

/// Returns a new map with the entries of `map` except those whose key is in `keys`.
///
/// Keys that are not in `map` are ignored. The complement of [`pick`](super::pick).
///
/// # Examples
///
/// ```
/// use helpers4::map::omit;
/// use std::collections::HashMap;
///
/// let user = HashMap::from([("name", 1), ("email", 2), ("password", 3)]);
/// assert_eq!(omit(&user, &["password"]), HashMap::from([("name", 1), ("email", 2)]));
/// ```
#[must_use]
pub fn omit<K: Clone + Eq + Hash, V: Clone, S: BuildHasher>(
    map: &HashMap<K, V, S>,
    keys: &[K],
) -> HashMap<K, V> {
    map.iter()
        .filter(|(key, _)| !keys.contains(key))
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect()
}

#[cfg(test)]
#[path = "omit.test.rs"]
mod tests;

#[cfg(test)]
#[path = "omit.spec.rs"]
mod spec;
