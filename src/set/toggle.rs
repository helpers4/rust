// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashSet;
use std::hash::{BuildHasher, Hash};

/// Removes `item` from `set` if it is there, inserts it otherwise.
///
/// # Arguments
///
/// - `set` - The set to change.
/// - `item` - The element to flip.
///
/// # Returns
///
/// `true` when `item` is in the set after the call (it was inserted), `false` when it was removed.
///
/// # Examples
///
/// ```
/// use helpers4::set::toggle;
/// use std::collections::HashSet;
///
/// let mut tags = HashSet::from(["rust"]);
/// assert!(toggle(&mut tags, "cli"));   // added
/// assert!(!toggle(&mut tags, "rust")); // removed
/// assert_eq!(tags, HashSet::from(["cli"]));
/// ```
pub fn toggle<T: Eq + Hash, S: BuildHasher>(set: &mut HashSet<T, S>, item: T) -> bool {
    if set.remove(&item) {
        false
    } else {
        set.insert(item);
        true
    }
}

#[cfg(test)]
#[path = "toggle.test.rs"]
mod tests;

#[cfg(test)]
#[path = "toggle.spec.rs"]
mod spec;
