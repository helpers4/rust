// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashSet;
use std::hash::{BuildHasher, Hash};

/// The Jaccard similarity of two sets: the size of their intersection over the size of their
/// union.
///
/// `1.0` means the sets are equal, `0.0` that they share nothing. Two empty sets are equal, so
/// they score `1.0`.
///
/// # Arguments
///
/// - `a` - The first set.
/// - `b` - The second set.
///
/// # Returns
///
/// A value between `0.0` and `1.0`.
///
/// # Examples
///
/// ```
/// use helpers4::set::jaccard;
/// use std::collections::HashSet;
///
/// let a = HashSet::from([1, 2, 3]);
/// let b = HashSet::from([2, 3, 4]);
/// assert_eq!(jaccard(&a, &b), 0.5); // {2, 3} out of {1, 2, 3, 4}
/// ```
#[must_use]
#[allow(clippy::cast_precision_loss)] // a set would need more than 2^53 elements to lose precision
pub fn jaccard<T: Eq + Hash, S: BuildHasher>(a: &HashSet<T, S>, b: &HashSet<T, S>) -> f64 {
    let shared = a.intersection(b).count();
    let total = a.len() + b.len() - shared;
    if total == 0 {
        return 1.0;
    }
    shared as f64 / total as f64
}

#[cfg(test)]
#[path = "jaccard.test.rs"]
mod tests;

#[cfg(test)]
#[path = "jaccard.spec.rs"]
mod spec;
