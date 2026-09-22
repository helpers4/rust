// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::collections::HashSet;
use std::hash::Hash;

/// Returns `true` when `a` and `b` share at least one element.
///
/// # Arguments
///
/// - `a` - The first slice.
/// - `b` - The second slice.
///
/// # Examples
///
/// ```
/// use helpers4::array::intersects;
///
/// assert!(intersects(&[1, 2, 3], &[3, 4]));
/// assert!(!intersects(&[1, 2], &[3, 4]));
/// ```
pub fn intersects<T: Eq + Hash>(a: &[T], b: &[T]) -> bool {
    let in_b: HashSet<&T> = b.iter().collect();
    a.iter().any(|item| in_b.contains(item))
}

#[cfg(test)]
#[path = "intersects.test.rs"]
mod tests;

#[cfg(test)]
#[path = "intersects.spec.rs"]
mod spec;
