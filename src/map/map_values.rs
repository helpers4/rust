// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

/// Returns a new map with the same keys and each value replaced by `f(value)`.
///
/// # Arguments
///
/// - `map` - The map to transform.
/// - `f` - Computes the new value from each old value.
///
/// # Examples
///
/// ```
/// use helpers4::map::map_values;
/// use std::collections::HashMap;
///
/// let prices = HashMap::from([("apple", 2), ("pear", 3)]);
/// let doubled = map_values(&prices, |price| price * 2);
/// assert_eq!(doubled, HashMap::from([("apple", 4), ("pear", 6)]));
/// ```
pub fn map_values<K: Clone + Eq + Hash, V, W, S: BuildHasher>(
    map: &HashMap<K, V, S>,
    mut f: impl FnMut(&V) -> W,
) -> HashMap<K, W> {
    map.iter()
        .map(|(key, value)| (key.clone(), f(value)))
        .collect()
}

#[cfg(test)]
#[path = "map_values.test.rs"]
mod tests;

#[cfg(test)]
#[path = "map_values.spec.rs"]
mod spec;
