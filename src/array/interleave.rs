// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Alternates the elements of `a` and `b`, starting with `a`; the leftover of the longer slice
/// goes at the end.
///
/// # Arguments
///
/// - `a` - The slice to take the first, third, … elements from.
/// - `b` - The slice to take the second, fourth, … elements from.
///
/// # Examples
///
/// ```
/// use helpers4::array::interleave;
///
/// assert_eq!(interleave(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
/// assert_eq!(interleave(&["a", "b", "c"], &["x"]), vec!["a", "x", "b", "c"]);
/// ```
#[must_use]
pub fn interleave<T: Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let shared = a.len().min(b.len());
    for i in 0..shared {
        out.push(a[i].clone());
        out.push(b[i].clone());
    }
    out.extend_from_slice(&a[shared..]);
    out.extend_from_slice(&b[shared..]);
    out
}

#[cfg(test)]
#[path = "interleave.test.rs"]
mod tests;

#[cfg(test)]
#[path = "interleave.spec.rs"]
mod spec;
