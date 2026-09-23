// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// The bytewise XOR of two slices of the same length.
///
/// # Arguments
///
/// - `a` - The first slice.
/// - `b` - The second slice.
///
/// # Returns
///
/// A new `Vec` whose byte `i` is `a[i] ^ b[i]`, or `None` when the lengths differ.
///
/// # Examples
///
/// ```
/// use helpers4::bytes::xor;
///
/// assert_eq!(xor(&[0b1100, 0b1010], &[0b1010, 0b1010]), Some(vec![0b0110, 0]));
/// assert_eq!(xor(&[1], &[1, 2]), None);
/// ```
#[must_use]
pub fn xor(a: &[u8], b: &[u8]) -> Option<Vec<u8>> {
    if a.len() != b.len() {
        return None;
    }
    Some(a.iter().zip(b).map(|(x, y)| x ^ y).collect())
}

#[cfg(test)]
#[path = "xor.test.rs"]
mod tests;

#[cfg(test)]
#[path = "xor.spec.rs"]
mod spec;
