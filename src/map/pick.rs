// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

/// Returns a new map with only the entries of `map` whose key is in `keys`.
///
/// Keys that are not in `map` are ignored.
///
/// # Examples
///
/// ```
/// use helpers4::map::pick;
/// use std::collections::HashMap;
///
/// let user = HashMap::from([("name", 1), ("email", 2), ("password", 3)]);
/// let public = pick(&user, &["name", "email", "age"]);
/// assert_eq!(public, HashMap::from([("name", 1), ("email", 2)]));
/// ```
#[must_use]
pub fn pick<K: Clone + Eq + Hash, V: Clone, S: BuildHasher>(
    map: &HashMap<K, V, S>,
    keys: &[K],
) -> HashMap<K, V> {
    keys.iter()
        .filter_map(|key| map.get(key).map(|value| (key.clone(), value.clone())))
        .collect()
}

#[cfg(test)]
#[path = "pick.test.rs"]
mod tests;

#[cfg(test)]
#[path = "pick.spec.rs"]
mod spec;
